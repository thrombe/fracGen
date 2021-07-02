// should i inline everything???

#[derive(Debug, Clone, Copy)] // is copy good for this???
pub struct Vec4d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Vec4d {

    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self {x, y, z, w}
    }

    #[inline(always)]
    pub fn lerp(&self, other: Self, t: f64) -> Self {
        if t > 1.0 {return *self}
        if t < 0.0 {return other}
        *self*t + other*(1.0-t)
    }

    #[inline(always)]
    pub fn lerp_without_check(&self, other: Self, t: f64) -> Self {
        *self*t + other*(1.0-t)
    }

    #[inline(always)]
    pub fn size(&self) -> f64 {
        (*self * *self).sqrt()
    }

    #[inline(always)]
    pub fn unit_assign(&mut self) {
        *self *= 1.0/self.size();
    }

    #[inline(always)]
    pub fn unit(&self) -> Self {
        *self * (1.0/self.size())
    }

    pub fn cross(self, other: Vec4d) -> Vec4d {
        Vec4d {
            x: self.y*other.z - other.y*self.z,
            y: self.x*other.z - other.x*self.z,
            z: self.x*other.y - other.x*self.y,
            w: self.w,
        }
    }
}

use std::ops::{Add, Sub, Mul, AddAssign, SubAssign, MulAssign};
impl Add for Vec4d {
    type Output = Self;

    #[inline(always)]
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}
impl Sub for Vec4d {
    type Output = Self;

    #[inline(always)]
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}
impl Mul for Vec4d {
    type Output = f64;

    #[inline(always)]
    fn mul(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }
}

// /// 3d vector multiplication
// impl Mul<Vec4d> for &Vec4d { // this is ugly
//     type Output = Vec4d;

//     /// ugly implementation of 3d vector multiplication
//     fn mul(self, other: Vec4d) -> Vec4d {
//         Vec4d {
//             x: self.y*other.z - other.y*self.z,
//             y: self.x*other.z - other.x*self.z,
//             z: self.x*other.y - other.x*self.y,
//             w: self.w,
//         }
//     }
// }

impl Mul<f64> for Vec4d {
    type Output = Self;

    #[inline(always)]
    fn mul(self, t: f64) -> Self {
        Self {
            x: self.x * t,
            y: self.y * t,
            z: self.z * t,
            w: self.w * t,
        }
    }
}
impl AddAssign for Vec4d {
    #[inline(always)]
    fn add_assign(&mut self, other: Self) {
        self.x = self.x + other.x;
        self.y = self.y + other.y;
        self.z = self.z + other.z;
        self.w = self.w + other.w;
    }
}
impl SubAssign for Vec4d {
    #[inline(always)]
    fn sub_assign(&mut self, other: Self) {
        self.x = self.x - other.x;
        self.y = self.y - other.y;
        self.z = self.z - other.z;
        self.w = self.w - other.w;
    }
}
impl MulAssign<f64> for Vec4d {
    #[inline(always)]
    fn mul_assign(&mut self, t: f64) {
        self.x = self.x * t;
        self.y = self.y * t;
        self.z = self.z * t;
        self.w = self.w * t;
    }
}
