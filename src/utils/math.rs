use cgmath::PerspectiveFov;
use glam::{Vec3, Mat4, Vec4};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Vec3f(pub f32, pub f32, pub f32);

impl Vec3f {
    pub const ZERO: Self = Self(0.0, 0.0, 0.0);
    
    pub fn to_glam(self) -> Vec3 {
        Vec3::new(self.0, self.1, self.2)
    }

    pub fn from_glam(v: Vec3) -> Self {
        Self(v.x, v.y, v.z)
    }

    pub fn floor(&self) -> Self {
        Self(
            self.0.floor(),
            self.1.floor(),
            self.2.floor(),
        )
    }

    pub fn chunk_coords(&self, chunk_size: f32) -> (i32, i32, i32) {
        (
            (self.0 / chunk_size).floor() as i32,
            (self.1 / chunk_size).floor() as i32,
            (self.2 / chunk_size).floor() as i32,
        )
    }
}

// 4x4 matrix
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix(pub Mat4);

impl Matrix {
    pub fn perspective(fov: f32, aspect: f32, near: f32, far: f32) -> Self {
        Self(Mat4::perspective_rh(fov, aspect, near, far))
    }

    pub fn look_at(eye: Vec3f, target: Vec3f, up: Vec3f) -> Self {
        Self(Mat4::look_at_rh(eye.to_glam(), target.to_glam(), up.to_glam()))
    }

    pub fn multiply(&self, other: &Self) -> Self {
        Self(self.0.mul_mat4(&other.0))
    }
}

// convert world pos to normalized device coords
pub fn project_to_ndc(pos: Vec3f, view_proj: &Matrix) -> Vec3f {
    let pos = view_proj.0.project_point3(pos.to_glam());
    Vec3f::from_glam(pos)
}
