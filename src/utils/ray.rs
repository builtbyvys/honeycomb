use crate::world::Chunk;
use crate::utils::math::{Vec3f, Matrix};
use egui::Direction;
use glam::Vec3;
use wgpu::naga::back::RayIntersectionType;

// ray with origin & direction
#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Vec3f,
    pub direction: Vec3f,
    pub distance: f32,
}

impl Ray {
    pub fn new(origin: Vec3f, direction: Vec3f) -> Self {
        Self {
            origin,
            direction: direction.normalize(),
            distance: 1000.0,
        }
    }

    // get the point alongside ray at distance "t"
    pub fn at(&self, t: f32) -> Vec3f {
        Vec3f(
            self.origin.0 + self.direction.0 * t,
            self.origin.1 + self.direction.1 * t,
            self.origin.2 + self.direction.2 * t,
        )
    }

    // using the infamous 3D-DDA ray traversal algorithm
    // https://www.researchgate.net/publication/233899848
    pub fn march(&self, chunk: &Chunk) -> Option<RaycastHit> {
        let mut t = 0.0;
        let step = Vec3f(
            self.direction.0.signum(),
            self.direction.1.signum(),
            self.direction.2.signum(),
        );

        let mut voxel_pos = self.origin.floor();
        let mut t_max = Self::calc_t_max(self.origin, self.direction, step);
        let mut t_delta = Self::calc_t_delta(self.direction);

        while t < self.distance {
            if let Some(block) = chunk.get_block(
                voxel_pos.0 as i32,
                voxel_pos.1 as i32,
                voxel_pos.2 as i32,
            ) {
                if block != 0 {
                    return Some(RaycastHit {
                        position: self.at(t),
                        normal: self.calculate_normal(voxel_pos),
                        distance: t,
                        voxel: block,
                    });
                }
            }

            let (axis, min_t) = Self::find_next_axis(t_max);
            t = min_t;
            voxel_pos = Self::step_voxel(voxel_pos, axis, step);
            t_max.0 += t_delta.0 * (axis == 0) as i32 as f32;
            t_max.1 += t_delta.1 * (axis == 1) as i32 as f32;
            t_max.2 += t_delta.2 * (axis == 2) as i32 as f32;
        }

        None
    }

    // helper functions
    fn calc_t_max(origin: Vec3f, dir: Vec3f, step: Vec3f) -> Vec3f {
        Vec3f(
            (step.0 - (origin.0 % 1.0)) / dir.0,
            (step.1 - (origin.1 % 1.0)) / dir.1,
            (step.2 - (origin.2 % 1.0)) / dir.2,
        )
    }

    fn calc_t_delta(dir: Vec3f) -> Vec3f {
        Vec3f(1.0 / dir.0.abs(), 1.0 / dir.1.abs(), 1.0 / dir.2.abs())
    }

    fn find_next_axis(t_max: Vec3f) -> (usize, f32) {
        let axis = if t_max.0 < t_max.1 && t_max.0 < t_max.2 {
            0
        } else if t_max.1 < t_max.2 {
            1
        } else {
            2
        };
        (axis, t_max[axis])
    }

    fn step_voxel(mut pos: Vec3f, axis: usize, step: Vec3f) -> Vec3f {
        match axis {
            0 => pos.0 += step.0,
            1 => pos.1 += step.1,
            2 => pos.2 += step.2,
            _ => unreachable!(),
        }
        pos
    }

    fn calculate_normal(&self, voxel_pos: Vec3f) -> Vec3f {
        let delta = self.at(0.001).floor() - voxel_pos;
        Vec3f(
            -delta.0.signum(),
            -delta.1.signum(),
            -delta.2.signum(),
        )
    }
}

// the result of a ray-voxel interaction
#[derive(Debug, Clone, Copy)]
pub struct RaycastHit {
    pub position: Vec3f,
    pub normal: Vec3f,
    pub distance: f32,
    pub voxel: u8,
}

pub type VoxelRayResult = Option<RaycastHit>;
