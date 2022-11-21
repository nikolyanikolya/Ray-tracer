use std::ops::{Add, Mul, Neg, Sub};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub trait PixelOperations {
    fn square(&self) -> f32;

    fn norm(&self) -> f32;

    fn normalized(&self) -> Vec3;
}

impl Mul<Vec3> for Vec3 {
    type Output = f32;

    fn mul(self, other: Vec3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, k: f32) -> Vec3 {
        Vec3 {
            x: self.x * k,
            y: self.y * k,
            z: self.z * k,
        }
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        self + (-other)
    }
}

impl PixelOperations for Vec3 {
    fn square(&self) -> f32 {
        *self * (*self)
    }

    fn norm(&self) -> f32 {
        (*self * (*self)).sqrt()
    }

    fn normalized(&self) -> Vec3 {
        *self * (1f32 / self.norm())
    }
}
