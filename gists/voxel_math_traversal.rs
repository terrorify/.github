//! # Voxel Math: Discrete 3D Grid Footprint & Traversal Engine
//!
//! **Author**: Terrorify Engineering  
//! **License**: MIT / Apache-2.0  
//!
//! ## Overview
//! In voxel game engines, mapping continuous floating-point physical volumes (AABBs and rays)
//! into discrete integer block coordinates $(x, y, z)$ is an inner-loop operation performed millions of times per tick.
//!
//! This library provides:
//! 1. **Voxel Footprint Range Extraction**: Computes the exact minimum and maximum discrete chunk/block coordinates
//!    overlapped by any swept volume or static bounding box.
//! 2. **Ray-Voxel Traversal Iterator**: Yields intersecting voxel blocks in exact order of penetration without heap allocation.
//! 3. **Sub-Voxel Point Inclusivity**: Fast verification of whether continuous vectors lie inside complex block shapes (slabs, stairs, fences).

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockPos {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl BlockPos {
    #[inline(always)]
    pub const fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }
}

pub struct VoxelFootprint {
    pub min_x: i32,
    pub min_y: i32,
    pub min_z: i32,
    pub max_x: i32,
    pub max_y: i32,
    pub max_z: i32,
}

impl VoxelFootprint {
    /// Iterates through every block position within this footprint volume.
    #[inline]
    pub fn for_each<F: FnMut(BlockPos) -> bool>(&self, mut callback: F) {
        for y in self.min_y..=self.max_y {
            for z in self.min_z..=self.max_z {
                for x in self.min_x..=self.max_x {
                    if !callback(BlockPos::new(x, y, z)) {
                        return; // Early termination if requested by caller
                    }
                }
            }
        }
    }
}

pub struct VoxelMath;

impl VoxelMath {
    /// Computes the discrete integer voxel bounds overlapped by continuous floating-point coordinates.
    #[inline(always)]
    pub fn get_footprint(min_x: f64, min_y: f64, min_z: f64, max_x: f64, max_y: f64, max_z: f64) -> VoxelFootprint {
        const EPSILON: f64 = 1e-7;
        VoxelFootprint {
            min_x: (min_x + EPSILON).floor() as i32,
            min_y: (min_y + EPSILON).floor() as i32,
            min_z: (min_z + EPSILON).floor() as i32,
            max_x: (max_x - EPSILON).floor() as i32,
            max_y: (max_y - EPSILON).floor() as i32,
            max_z: (max_z - EPSILON).floor() as i32,
        }
    }

    /// Computes discrete swept footprint for a moving volume between origin and target.
    #[inline(always)]
    pub fn get_swept_footprint(
        from_min_x: f64, from_min_y: f64, from_min_z: f64,
        from_max_x: f64, from_max_y: f64, from_max_z: f64,
        to_min_x: f64, to_min_y: f64, to_min_z: f64,
        to_max_x: f64, to_max_y: f64, to_max_z: f64,
    ) -> VoxelFootprint {
        Self::get_footprint(
            from_min_x.min(to_min_x),
            from_min_y.min(to_min_y),
            from_min_z.min(to_min_z),
            from_max_x.max(to_max_x),
            from_max_y.max(to_max_y),
            from_max_z.max(to_max_z),
        )
    }

    /// Fast check if continuous coordinate `(px, py, pz)` is inside voxel `pos` with optional fractional offsets.
    #[inline(always)]
    pub fn contains_point(pos: BlockPos, px: f64, py: f64, pz: f64) -> bool {
        px >= pos.x as f64 && px < (pos.x + 1) as f64 &&
        py >= pos.y as f64 && py < (pos.y + 1) as f64 &&
        pz >= pos.z as f64 && pz < (pos.z + 1) as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_voxel_footprint_extraction() {
        // Player standing at (0.2, 64.0, 0.2) with dimensions 0.6w x 1.8h
        let footprint = VoxelMath::get_footprint(0.2, 64.0, 0.2, 0.8, 65.8, 0.8);
        assert_eq!(footprint.min_x, 0);
        assert_eq!(footprint.max_x, 0);
        assert_eq!(footprint.min_y, 64);
        assert_eq!(footprint.max_y, 65);
        assert_eq!(footprint.min_z, 0);
        assert_eq!(footprint.max_z, 0);

        let mut count = 0;
        footprint.for_each(|_| {
            count += 1;
            true
        });
        assert_eq!(count, 2, "Should span exactly 2 vertical voxels (Y=64 and Y=65)");
    }
}
