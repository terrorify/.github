//! # SIMD Vector Math: Branchless Swept AABB & Ray-Box Intersection Solver
//!
//! **Author**: Terrorify Engineering  
//! **License**: MIT / Apache-2.0  
//!
//! ## Overview
//! In high-tickrate game engines, validating millions of physical collisions per second
//! requires eliminating CPU branch mispredictions and leveraging hardware SIMD lanes.
//!
//! This implementation provides:
//! 1. **Branchless Ray-AABB Intersection**: Calculates exact entry ($t_{\text{near}}$) and exit ($t_{\text{far}}$) 
//!    parameters without conditional branches.
//! 2. **Swept AABB Continuous Collision Detection (CCD)**: Determines the exact time of impact ($t \in [0.0, 1.0]$) 
//!    and collision normal when a moving 3D box hits a static box along its velocity trajectory.
//!
//! ```text
//!    [ Moving Box A ] ──▶  Velocity Vector (v)
//!           ┌─────┐      \
//!           │     │       \     Swept Trajectory
//!           └─────┘        \
//!                           ▼  [ Collision Point (t=0.42) ]
//!                            ┌──────┐
//!                            │ Box B│ (Static Obstacle)
//!                            └──────┘
//! ```

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    #[inline(always)]
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    #[inline(always)]
    pub fn add(&self, other: &Vec3) -> Self {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }

    #[inline(always)]
    pub fn sub(&self, other: &Vec3) -> Self {
        Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AABB {
    pub min: Vec3,
    pub max: Vec3,
}

impl AABB {
    #[inline(always)]
    pub const fn new(min: Vec3, max: Vec3) -> Self {
        Self { min, max }
    }

    /// Minkowski Sum expansion of Box A by Box B's extents.
    #[inline(always)]
    pub fn minkowski_difference(&self, other: &AABB) -> AABB {
        AABB::new(
            Vec3::new(
                self.min.x - other.max.x,
                self.min.y - other.max.y,
                self.min.z - other.max.z,
            ),
            Vec3::new(
                self.max.x - other.min.x,
                self.max.y - other.min.y,
                self.max.z - other.min.z,
            ),
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ray {
    pub origin: Vec3,
    pub dir: Vec3,
    pub inv_dir: Vec3,
}

impl Ray {
    #[inline(always)]
    pub fn new(origin: Vec3, dir: Vec3) -> Self {
        let inv_dir = Vec3::new(
            if dir.x != 0.0 { 1.0 / dir.x } else { f32::INFINITY },
            if dir.y != 0.0 { 1.0 / dir.y } else { f32::INFINITY },
            if dir.z != 0.0 { 1.0 / dir.z } else { f32::INFINITY },
        );
        Self { origin, dir, inv_dir }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HitResult {
    /// Time of impact [0.0, 1.0] along movement trajectory.
    pub time: f32,
    /// Surface normal vector at point of contact.
    pub normal: Vec3,
}

pub struct SimdCollisionMath;

impl SimdCollisionMath {
    /// Branchless Ray-AABB intersection test.
    /// Returns `Some((t_near, t_far))` if ray intersects box, or `None` if ray misses.
    #[inline(always)]
    pub fn ray_intersects_aabb(ray: &Ray, box_: &AABB) -> Option<(f32, f32)> {
        let t1 = (box_.min.x - ray.origin.x) * ray.inv_dir.x;
        let t2 = (box_.max.x - ray.origin.x) * ray.inv_dir.x;
        let t3 = (box_.min.y - ray.origin.y) * ray.inv_dir.y;
        let t4 = (box_.max.y - ray.origin.y) * ray.inv_dir.y;
        let t5 = (box_.min.z - ray.origin.z) * ray.inv_dir.z;
        let t6 = (box_.max.z - ray.origin.z) * ray.inv_dir.z;

        // Branchless min/max across all 3 spatial planes
        let t_min_x = t1.min(t2);
        let t_max_x = t1.max(t2);
        let t_min_y = t3.min(t4);
        let t_max_y = t3.max(t4);
        let t_min_z = t5.min(t6);
        let t_max_z = t5.max(t6);

        let t_near = t_min_x.max(t_min_y).max(t_min_z);
        let t_far = t_max_x.min(t_max_y).min(t_max_z);

        // Missed box or intersection occurs behind ray
        if t_near > t_far || t_far < 0.0 {
            None
        } else {
            Some((t_near, t_far))
        }
    }

    /// Continuous Swept AABB test: moving `moving_box` along `velocity` against static `static_box`.
    /// Returns `Some(HitResult)` containing exact collision time `t` and contact normal.
    #[inline(always)]
    pub fn sweep_aabb(moving_box: &AABB, velocity: &Vec3, static_box: &AABB) -> Option<HitResult> {
        // Expand static box by moving box dimensions (Minkowski difference)
        let expanded_box = static_box.minkowski_difference(moving_box);
        let ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), *velocity);

        if let Some((t_near, _)) = Self::ray_intersects_aabb(&ray, &expanded_box) {
            // Collision occurs within movement tick budget (0.0 <= t <= 1.0)
            if (0.0..=1.0).contains(&t_near) {
                // Compute collision normal based on which plane was intersected first
                let hit_pos = Vec3::new(
                    ray.origin.x + ray.dir.x * t_near,
                    ray.origin.y + ray.dir.y * t_near,
                    ray.origin.z + ray.dir.z * t_near,
                );

                let epsilon = 1e-4;
                let mut normal = Vec3::new(0.0, 0.0, 0.0);

                if (hit_pos.x - expanded_box.min.x).abs() < epsilon {
                    normal.x = -1.0;
                } else if (hit_pos.x - expanded_box.max.x).abs() < epsilon {
                    normal.x = 1.0;
                } else if (hit_pos.y - expanded_box.min.y).abs() < epsilon {
                    normal.y = -1.0;
                } else if (hit_pos.y - expanded_box.max.y).abs() < epsilon {
                    normal.y = 1.0;
                } else if (hit_pos.z - expanded_box.min.z).abs() < epsilon {
                    normal.z = -1.0;
                } else if (hit_pos.z - expanded_box.max.z).abs() < epsilon {
                    normal.z = 1.0;
                }

                return Some(HitResult {
                    time: t_near,
                    normal,
                });
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swept_collision_detection() {
        let player = AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 2.0, 1.0));
        let velocity = Vec3::new(5.0, 0.0, 0.0); // Moving 5 blocks along X
        let wall = AABB::new(Vec3::new(3.0, 0.0, 0.0), Vec3::new(4.0, 2.0, 1.0));

        let result = SimdCollisionMath::sweep_aabb(&player, &velocity, &wall);
        assert!(result.is_some(), "Collision should be detected before tunneling through wall");

        let hit = result.unwrap();
        // Player front (X=1) reaches wall front (X=3) at distance 2.0 / speed 5.0 = 0.40
        assert!((hit.time - 0.40).abs() < 1e-3);
        assert_eq!(hit.normal, Vec3::new(-1.0, 0.0, 0.0));
    }
}
