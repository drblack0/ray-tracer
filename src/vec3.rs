use std::{
    f64, fmt,
    ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign},
};

use crate::utility::{random_float, random_float_by_range};

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    elements: [f64; 3],
}

pub type Point3 = Vec3;

impl Default for Vec3 {
    fn default() -> Self {
        Vec3 {
            elements: [0.0, 0.0, 0.0],
        }
    }
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 {
            elements: [x, y, z],
        }
    }

    pub fn length_squared(&self) -> f64 {
        self.elements[0] * self.elements[0]
            + self.elements[1] * self.elements[1]
            + self.elements[2] * self.elements[2]
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        f64::abs(self.elements[0]) < s
            && f64::abs(self.elements[1]) < s
            && f64::abs(self.elements[2]) < s
    }

    pub fn random() -> Vec3 {
        Vec3 {
            elements: [random_float(), random_float(), random_float()],
        }
    }

    pub fn random_from_range(min: f64, max: f64) -> Vec3 {
        Vec3 {
            elements: [
                random_float_by_range(min, max),
                random_float_by_range(min, max),
                random_float_by_range(min, max),
            ],
        }
    }
    pub fn x(&self) -> f64 {
        self.elements[0]
    }
    pub fn y(&self) -> f64 {
        self.elements[1]
    }
    pub fn z(&self) -> f64 {
        self.elements[2]
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 {
            elements: [-self.elements[0], -self.elements[1], -self.elements[2]],
        }
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;
    fn index(&self, index: usize) -> &Self::Output {
        &self.elements[index]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut f64 {
        &mut self.elements[index]
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Self) -> Self::Output {
        Vec3::new(
            self.elements[0] + rhs.elements[0],
            self.elements[1] + rhs.elements[1],
            self.elements[2] + rhs.elements[2],
        )
    }
}

impl Add<Vec3> for &Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3::new(
            self.elements[0] + rhs.elements[0],
            self.elements[1] + rhs.elements[1],
            self.elements[2] + rhs.elements[2],
        )
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.elements[0] += rhs.elements[0];
        self.elements[1] += rhs.elements[1];
        self.elements[2] += rhs.elements[2];
    }
}

impl Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Self) -> Self::Output {
        Vec3::new(
            self.elements[0] * rhs.elements[0],
            self.elements[1] * rhs.elements[1],
            self.elements[2] * rhs.elements[2],
        )
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Self::Output {
        Vec3::new(
            self.elements[0] * rhs,
            self.elements[1] * rhs,
            self.elements[2] * rhs,
        )
    }
}

impl Mul<f64> for &Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Self::Output {
        Vec3::new(
            self.elements[0] * rhs,
            self.elements[1] * rhs,
            self.elements[2] * rhs,
        )
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl Mul<&Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: &Vec3) -> Self::Output {
        rhs * self
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        self.elements[0] *= rhs.elements[0];
        self.elements[1] *= rhs.elements[1];
        self.elements[2] *= rhs.elements[2];
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.elements[0] *= rhs;
        self.elements[1] *= rhs;
        self.elements[2] *= rhs;
    }
}

impl Div for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: Self) -> Self::Output {
        Vec3::new(
            self.elements[0] / rhs.elements[0],
            self.elements[1] / rhs.elements[1],
            self.elements[2] / rhs.elements[2],
        )
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl Div<f64> for &Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        // self is &Vec3, we have to access elements via &self.elements
        Vec3::new(
            self.elements[0] / rhs,
            self.elements[1] / rhs,
            self.elements[2] / rhs,
        )
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Self) {
        self.elements[0] /= rhs.elements[0];
        self.elements[1] /= rhs.elements[1];
        self.elements[2] /= rhs.elements[2];
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        // The C++ code optimizes this: *this *= 1/t
        let inv = 1.0 / rhs;
        self.elements[0] *= inv;
        self.elements[1] *= inv;
        self.elements[2] *= inv;
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec3::new(
            self.elements[0] - rhs.elements[0],
            self.elements[1] - rhs.elements[1],
            self.elements[2] - rhs.elements[2],
        )
    }
}

impl Sub<&Vec3> for &Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: &Vec3) -> Self::Output {
        Vec3::new(
            self.elements[0] - rhs.elements[0],
            self.elements[1] - rhs.elements[1],
            self.elements[2] - rhs.elements[2],
        )
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.elements[0] -= rhs.elements[0];
        self.elements[1] -= rhs.elements[1];
        self.elements[2] -= rhs.elements[2];
    }
}

pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
    u.elements[0] * v.elements[0] + u.elements[1] * v.elements[1] + u.elements[2] * v.elements[2]
}

pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
    Vec3::new(
        u.elements[1] * v.elements[2] - u.elements[2] * v.elements[1],
        u.elements[2] * v.elements[0] - u.elements[0] * v.elements[2],
        u.elements[0] * v.elements[1] - u.elements[1] * v.elements[0],
    )
}

pub fn unit_vector(v: &Vec3) -> Vec3 {
    v / v.length()
}

pub fn random_unit_vector() -> Vec3 {
    loop {
        let p = Vec3::random();
        let lensq = p.length_squared();

        if 1e-160 < lensq && lensq <= 1.0 {
            return p / f64::sqrt(lensq);
        }
    }
}

pub fn random_on_hemisphere(normal: &Vec3) -> Vec3 {
    let on_unit_sphere = random_unit_vector();
    if dot(&on_unit_sphere, normal) > 0.0 {
        return on_unit_sphere;
    } else {
        -on_unit_sphere
    }
}

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    v - &(2.0 * (dot(v, n) * n))
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.elements[0], self.elements[1], self.elements[2]
        )
    }
}
