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

    /// lerp, but chopped at t = 0 and t = 1
    #[inline(always)]
    pub fn lerp_with_chop(&self, other: Self, t: f64) -> Self {
        if t > 1.0 {return *self}
        if t < 0.0 {return other}
        *self*t + other*(1.0-t)
    }

    /// returns self*t + other*(1-t)
    #[inline(always)]
    pub fn lerp(&self, other: Self, t: f64) -> Self {
        *self*t + other*(1.0-t)
    }

    #[inline(always)]
    pub fn size(&self) -> f64 {
        ((*self).dot(*self)).sqrt()
    }

    #[inline(always)]
    pub fn unit_assign(&mut self) {
        *self *= 1.0/self.size();
    }

    #[inline(always)]
    pub fn unit(&self) -> Self {
        *self * (1.0/self.size())
    }

    /// returns a new vector which is the 3d cross product of the 2, self at left, w component is also of self (other.w is ignored)
    #[inline(always)]
    pub fn cross3d(&self, other: Self) -> Self {
        Vec4d {
            x: self.y*other.z - other.y*self.z,
            y: -self.x*other.z + other.x*self.z,
            z: self.x*other.y - other.x*self.y,
            w: self.w,
        }
    }

    #[inline(always)]
    pub fn dot(&self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    // a wierd multiplication but idk where its useful
    #[inline(always)]
    pub fn mul(&self, other: Self) -> Self {
        Vec4d {
            x: self.x*other.x,
            y: self.y*other.y,
            z: self.z*other.z,
            w: self.w*other.w,
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

// this isnt cross cuz cross is a 3d func, not 4d. so vec1*vec2 -> f64 makes more sense
// impl Mul for Vec4d {
//     type Output = f64;

//     #[inline(always)]
//     fn mul(self, other: Self) -> f64 {
//         self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
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
