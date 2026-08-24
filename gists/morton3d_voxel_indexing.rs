//! # 3D Morton Z-Order Spatial Indexing: Hardware-Accelerated Voxel Cache Locality
//!
//! **Author**: Terrorify Engineering  
//! **License**: MIT / Apache-2.0  
//!
//! ## Overview
//! In dense multiplayer voxel worlds and physics simulation, traversing 3D linear arrays
//! `array[x][y][z]` causes severe cache misses because spatial neighbors along the Y and Z
//! axes are separated by large memory strides.
//!
//! **Morton Z-Order Indexing** maps 3D spatial coordinates $(x, y, z)$ into a 1D recursive
//! space-filling curve by interleaving their binary bits:
//!
//! ```text
//! 3D Coordinates (4-bit binary):
//!   X: .... .... x3 x2 x1 x0
//!   Y: .... .... y3 y2 y1 y0
//!   Z: .... .... z3 z2 z1 z0
//!
//! Interleaved Morton Index (12-bit Z-Order):
//!   Index: z3 y3 x3  z2 y2 x2  z1 y1 x1  z0 y0 x0
//! ```
//!
//! ### Key Performance Benefits:
//! * **High L1/L2 Cache Hit Rate**: 3D spatial neighbors remain contiguous in physical RAM cache lines.
//! * **Hardware Acceleration**: On x86_64 CPUs supporting BMI2, bit-interleaving executes in **single-cycle** via `PDEP` / `PEXT` instructions.
//! * **O(1) Spatial Range Queries**: Hierarchical octree and bounding box intersections simplify to linear slice operations.

#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::{_pdep_u64, _pext_u64};

/// 3D Morton encoder & decoder supporting up to 21 bits per dimension (fit in 64-bit integer).
pub struct Morton3D;

impl Morton3D {
    const X_MASK: u64 = 0x1249249249249249; // Bits: ..001001001001
    const Y_MASK: u64 = 0x2492492492492492; // Bits: ..010010010010
    const Z_MASK: u64 = 0x4924924924924924; // Bits: ..100100100100

    /// Encodes discrete 3D coordinates `(x, y, z)` into a 64-bit Morton Z-Order index.
    #[inline(always)]
    pub fn encode(x: u32, y: u32, z: u32) -> u64 {
        #[cfg(all(target_arch = "x86_64", target_feature = "bmi2"))]
        {
            unsafe {
                let px = _pdep_u64(x as u64, Self::X_MASK);
                let py = _pdep_u64(y as u64, Self::Y_MASK);
                let pz = _pdep_u64(z as u64, Self::Z_MASK);
                px | py | pz
            }
        }

        #[cfg(not(all(target_arch = "x86_64", target_feature = "bmi2")))]
        {
            (Self::spread_bits(z as u64) << 2)
                | (Self::spread_bits(y as u64) << 1)
                | Self::spread_bits(x as u64)
        }
    }

    /// Decodes a 64-bit Morton Z-Order index back into discrete 3D coordinates `(x, y, z)`.
    #[inline(always)]
    pub fn decode(code: u64) -> (u32, u32, u32) {
        #[cfg(all(target_arch = "x86_64", target_feature = "bmi2"))]
        {
            unsafe {
                let x = _pext_u64(code, Self::X_MASK) as u32;
                let y = _pext_u64(code, Self::Y_MASK) as u32;
                let z = _pext_u64(code, Self::Z_MASK) as u32;
                (x, y, z)
            }
        }

        #[cfg(not(all(target_arch = "x86_64", target_feature = "bmi2")))]
        {
            (
                Self::compact_bits(code) as u32,
                Self::compact_bits(code >> 1) as u32,
                Self::compact_bits(code >> 2) as u32,
            )
        }
    }

    /// Software fallback: Spreads the lowest 21 bits of a number so that every bit
    /// is separated by two zero bits (e.g. `....abcd` -> `..a..b..c..d`).
    #[inline(always)]
    fn spread_bits(mut x: u64) -> u64 {
        x &= 0x1fffff;
        x = (x | (x << 32)) & 0x1f00000000ffff;
        x = (x | (x << 16)) & 0x1f0000ff0000ff;
        x = (x | (x << 8))  & 0x100f00f00f00f00f;
        x = (x | (x << 4))  & 0x10c30c30c30c30c3;
        x = (x | (x << 2))  & 0x1249249249249249;
        x
    }

    /// Software fallback: Compacts every third bit of a 64-bit number back into contiguous bits.
    #[inline(always)]
    fn compact_bits(mut x: u64) -> u64 {
        x &= Self::X_MASK;
        x = (x ^ (x >> 2))  & 0x30c30c30c30c30c3;
        x = (x ^ (x >> 4))  & 0xf00f00f00f00f00f;
        x = (x ^ (x >> 8))  & 0x00ff0000ff0000ff;
        x = (x ^ (x >> 16)) & 0x0000ffff000000ff;
        x = (x ^ (x >> 32)) & 0x00000000001fffff;
        x
    }
}

/// A cache-aligned 16x16x16 chunk voxel buffer indexed with Morton Z-Order.
#[repr(align(64))]
pub struct MortonChunkVoxelBuffer<T: Copy + Default> {
    voxels: [T; 4096], // 16 * 16 * 16 = 4096 elements
}

impl<T: Copy + Default> MortonChunkVoxelBuffer<T> {
    pub fn new() -> Self {
        Self {
            voxels: [T::default(); 4096],
        }
    }

    /// O(1) get voxel within local 16³ chunk (coords 0..15).
    #[inline(always)]
    pub fn get(&self, x: u32, y: u32, z: u32) -> T {
        debug_assert!(x < 16 && y < 16 && z < 16);
        let morton_idx = Morton3D::encode(x, y, z) as usize;
        self.voxels[morton_idx]
    }

    /// O(1) set voxel within local 16³ chunk (coords 0..15).
    #[inline(always)]
    pub fn set(&mut self, x: u32, y: u32, z: u32, val: T) {
        debug_assert!(x < 16 && y < 16 && z < 16);
        let morton_idx = Morton3D::encode(x, y, z) as usize;
        self.voxels[morton_idx] = val;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_morton_roundtrip() {
        for x in [0, 1, 7, 15, 100, 2047] {
            for y in [0, 2, 8, 14, 50, 1024] {
                for z in [0, 3, 9, 13, 250, 4095] {
                    let code = Morton3D::encode(x, y, z);
                    let (dx, dy, dz) = Morton3D::decode(code);
                    assert_eq!((x, y, z), (dx, dy, dz), "Morton roundtrip failed for ({}, {}, {})", x, y, z);
                }
            }
        }
    }

    #[test]
    fn test_chunk_buffer_access() {
        let mut buffer = MortonChunkVoxelBuffer::<u16>::new();
        buffer.set(5, 7, 11, 42);
        assert_eq!(buffer.get(5, 7, 11), 42);
        assert_eq!(buffer.get(5, 7, 10), 0);
    }
}
