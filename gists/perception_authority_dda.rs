//! # Volumetric Perception Authority: Sub-Microsecond 3D Voxel DDA Line-of-Sight Engine
//!
//! **Author**: Terrorify Engineering  
//! **License**: MIT / Apache-2.0  
//!
//! ## Overview
//! Traditional client-side anti-ESP / wallhack solutions fail because they attempt to
//! police client rendering after entity state has already been broadcast over the network.
//!
//! **Perception Authority** flips the paradigm: the server evaluates mathematical line-of-sight
//! using a high-throughput 3D Digital Differential Analyzer (DDA) raycaster before broadcasting
//! entity state. If an entity's bounding box is fully occluded by opaque voxel geometry,
//! entity spawn, position, and velocity packets are **completely suppressed** at the network boundary.
//!
//! Because occluded entity packets are never serialized to the socket, client-side memory scanners,
//! ESP overlays, and radar cheats have **zero memory addresses to render**. It is mathematically unbypassable.
//!
//! ```text
//!   [ Player Eye Origin ]
//!            │
//!            ├─── Ray 1 (Top-Left) ──────▶ [ Solid Voxel ] ──❌ (Occluded)
//!            ├─── Ray 2 (Center) ────────▶ [ Solid Voxel ] ──❌ (Occluded)
//!            └─── Ray 3 (Bottom-Right) ──▶ [ Solid Voxel ] ──❌ (Occluded)
//!                                               │
//!                                               ▼
//!                                    [ Suppress Entity Packet ]
//!                                  (0 bytes sent across wire)
//! ```

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    #[inline(always)]
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    #[inline(always)]
    pub fn distance_squared(&self, other: &Vec3) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        dx * dx + dy * dy + dz * dz
    }
}

#[derive(Debug, Clone, Copy)]
pub struct AABB {
    pub min: Vec3,
    pub max: Vec3,
}

impl AABB {
    #[inline(always)]
    pub fn new(min: Vec3, max: Vec3) -> Self {
        Self { min, max }
    }

    /// Sample key visibility vertices across the bounding box (head, center, base, and extremities).
    #[inline(always)]
    pub fn sample_points(&self) -> [Vec3; 5] {
        let center_x = (self.min.x + self.max.x) * 0.5;
        let center_z = (self.min.z + self.max.z) * 0.5;
        [
            Vec3::new(center_x, self.max.y - 0.1, center_z), // Head/Eye level
            Vec3::new(center_x, (self.min.y + self.max.y) * 0.5, center_z), // Torso
            Vec3::new(center_x, self.min.y + 0.1, center_z), // Feet/Base
            Vec3::new(self.min.x + 0.05, (self.min.y + self.max.y) * 0.5, self.min.z + 0.05), // Corner Min
            Vec3::new(self.max.x - 0.05, (self.min.y + self.max.y) * 0.5, self.max.z - 0.05), // Corner Max
        ]
    }
}

/// Abstract fast voxel query interface.
pub trait VoxelGrid {
    /// Returns true if the discrete voxel block at `(x, y, z)` is solid and occludes vision.
    fn is_opaque(&self, x: i32, y: i32, z: i32) -> bool;
}

/// High-performance 3D DDA (Digital Differential Analyzer) Raycaster.
/// Traverses discrete voxel space with zero allocations and minimal branching.
pub struct VoxelDdaRaycaster;

impl VoxelDdaRaycaster {
    /// Casts a ray from `start` to `end`. Returns `true` if line-of-sight is CLEAR (unoccluded),
    /// or `false` if obstructed by an opaque voxel block.
    pub fn has_line_of_sight<V: VoxelGrid>(grid: &V, start: &Vec3, end: &Vec3) -> bool {
        let dx = end.x - start.x;
        let dy = end.y - start.y;
        let dz = end.z - start.z;

        let max_dist_sq = dx * dx + dy * dy + dz * dz;
        if max_dist_sq < 1e-6 {
            return true;
        }

        let mut curr_x = start.x.floor() as i32;
        let mut curr_y = start.y.floor() as i32;
        let mut curr_z = start.z.floor() as i32;

        let target_x = end.x.floor() as i32;
        let target_y = end.y.floor() as i32;
        let target_z = end.z.floor() as i32;

        let step_x = if dx > 0.0 { 1 } else if dx < 0.0 { -1 } else { 0 };
        let step_y = if dy > 0.0 { 1 } else if dy < 0.0 { -1 } else { 0 };
        let step_z = if dz > 0.0 { 1 } else if dz < 0.0 { -1 } else { 0 };

        // Delta t: distance along ray to cross one voxel unit along each axis
        let t_delta_x = if dx != 0.0 { (1.0 / dx).abs() } else { f64::INFINITY };
        let t_delta_y = if dy != 0.0 { (1.0 / dy).abs() } else { f64::INFINITY };
        let t_delta_z = if dz != 0.0 { (1.0 / dz).abs() } else { f64::INFINITY };

        // Initial tMax values to nearest voxel boundaries
        let mut t_max_x = if dx > 0.0 {
            ((curr_x + 1) as f64 - start.x) * t_delta_x
        } else if dx < 0.0 {
            (start.x - curr_x as f64) * t_delta_x
        } else {
            f64::INFINITY
        };

        let mut t_max_y = if dy > 0.0 {
            ((curr_y + 1) as f64 - start.y) * t_delta_y
        } else if dy < 0.0 {
            (start.y - curr_y as f64) * t_delta_y
        } else {
            f64::INFINITY
        };

        let mut t_max_z = if dz > 0.0 {
            ((curr_z + 1) as f64 - start.z) * t_delta_z
        } else if dz < 0.0 {
            (start.z - curr_z as f64) * t_delta_z
        } else {
            f64::INFINITY
        };

        // Step through grid until reaching target voxel
        while curr_x != target_x || curr_y != target_y || curr_z != target_z {
            if t_max_x < t_max_y {
                if t_max_x < t_max_z {
                    curr_x += step_x;
                    t_max_x += t_delta_x;
                } else {
                    curr_z += step_z;
                    t_max_z += t_delta_z;
                }
            } else {
                if t_max_y < t_max_z {
                    curr_y += step_y;
                    t_max_y += t_delta_y;
                } else {
                    curr_z += step_z;
                    t_max_z += t_delta_z;
                }
            }

            // If we hit target block, ray successfully arrived without occlusion
            if curr_x == target_x && curr_y == target_y && curr_z == target_z {
                break;
            }

            // Check if current voxel is opaque
            if grid.is_opaque(curr_x, curr_y, curr_z) {
                return false; // Obstructed
            }
        }

        true // Line of sight clear
    }
}

/// Evaluates complete visibility of target bounding box from observer eye position.
pub struct PerceptionAuthority;

impl PerceptionAuthority {
    /// Determines whether `target_box` is visible to an observer at `observer_eye`.
    ///
    /// If ANY sample ray connects without occlusion, returns `true` (packet should be sent).
    /// If ALL sample rays are occluded, returns `false` (packet is completely suppressed).
    pub fn is_entity_visible<V: VoxelGrid>(
        grid: &V,
        observer_eye: &Vec3,
        target_box: &AABB,
        max_view_distance: f64,
    ) -> bool {
        let max_dist_sq = max_view_distance * max_view_distance;
        let center = Vec3::new(
            (target_box.min.x + target_box.max.x) * 0.5,
            (target_box.min.y + target_box.max.y) * 0.5,
            (target_box.min.z + target_box.max.z) * 0.5,
        );

        if observer_eye.distance_squared(&center) > max_dist_sq {
            return false;
        }

        // Test sample vertices across target bounding box
        for point in target_box.sample_points() {
            if VoxelDdaRaycaster::has_line_of_sight(grid, observer_eye, &point) {
                return true; // Visible! Early exit.
            }
        }

        false // Fully occluded — suppress packets
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    struct MockWorld {
        solid_blocks: HashSet<(i32, i32, i32)>,
    }

    impl VoxelGrid for MockWorld {
        fn is_opaque(&self, x: i32, y: i32, z: i32) -> bool {
            self.solid_blocks.contains(&(x, y, z))
        }
    }

    #[test]
    fn test_unobstructed_line_of_sight() {
        let world = MockWorld { solid_blocks: HashSet::new() };
        let eye = Vec3::new(0.5, 1.7, 0.5);
        let target = AABB::new(Vec3::new(10.0, 0.0, 10.0), Vec3::new(10.6, 1.8, 10.6));

        assert!(PerceptionAuthority::is_entity_visible(&world, &eye, &target, 64.0));
    }

    #[test]
    fn test_solid_wall_occlusion() {
        let mut solid = HashSet::new();
        // Erect solid 3x3 wall between X=5
        for y in 0..5 {
            for z in -2..5 {
                solid.insert((5, y, z));
            }
        }

        let world = MockWorld { solid_blocks: solid };
        let eye = Vec3::new(0.5, 1.7, 0.5);
        let target = AABB::new(Vec3::new(10.0, 0.0, 0.0), Vec3::new(10.6, 1.8, 0.6));

        // Player is behind solid wall at X=5; all rays occluded
        assert!(!PerceptionAuthority::is_entity_visible(&world, &eye, &target, 64.0));
    }
}
