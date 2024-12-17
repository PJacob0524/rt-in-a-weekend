#![allow(unused)]

use rand::{Rng, RngCore}; 
use std::cell::Ref;
use std::ops;

#[derive(Copy, Clone, Debug)]
pub struct Vec3 (pub  f64, pub  f64, pub  f64);

impl Vec3 {
    pub fn x(&self) ->  f64 {self.0}
    pub fn y(&self) ->  f64 {self.1}
    pub fn z(&self) ->  f64 {self.2}
    pub fn r(&self) ->  f64 {self.0}
    pub fn g(&self) ->  f64 {self.1}
    pub fn b(&self) ->  f64 {self.2}


    pub fn length_squared(&self) ->  f64 {
        self.0*self.0 + self.1*self.1 + self.2*self.2
    }

    pub fn length(&self) ->  f64 {
        self.length_squared().sqrt()
    }

    pub fn dot(&self, other: &Vec3) ->  f64 {
        (self.0*other.0) + (self.1*other.1) + (self.2*other.2)
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3(self.1 * other.2 - self.2 * other.1, self.2 * other.0 - self.0 * other.2, self.0 * other.1 - self.1 * other.0)
    }

    pub fn unit(&self) -> Vec3 {
        *self / self.length()
    }

    pub fn random(min: f64, max: f64) -> Self {
        let mut rng = rand::thread_rng(); 
        Point3(rng.gen_range(min..=max),rng.gen_range(min..=max),rng.gen_range(min..=max)) 
    }

    pub fn unitrand() -> Self {
        loop {
            let p = Self::random(-1.0, 1.0);
            let plensqrt = p.length_squared(); 
            if (1e-160..1.0).contains(&plensqrt) {
                return p / plensqrt;
            }
        }
    }

    pub fn hemi_rand(normal: &Vec3) -> Self {
        let rand = Self::unitrand();
        if rand.dot(normal) > 0.0 {
            return rand;
        }
        return -rand; 
    }

    pub fn empty() -> Self {
        Vec3(0.0,0.0,0.0)
    }

    pub fn reflect(&self, surface: &Vec3) -> Self {
        return *self - (*surface * surface.dot(self) * 2.0)
    }

    pub fn refract(&self, surface: &Vec3, refrac_ratio: f64) -> Self {
        let cos_theta: f64 = (*self * -1.0).dot(surface).min(1.0);
        let perpendicular: Vec3 = (*self + (*surface * cos_theta)) * refrac_ratio;
        let parallel: Vec3 = *surface * -1.0 * (1.0 - perpendicular.length_squared()).sqrt();
        return perpendicular + parallel;
    }

    pub fn unitdiskrand() -> Self {
        loop {
            let mut rand = Vec3::random(-1.0, 1.0);
            rand.2 = 0.0;

            if rand.length_squared() < 1.0 {
                return rand;
            }
        }
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}
impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}
impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3(self.0*rhs.0, self.1*rhs.1, self.2*rhs.2)
    }
}
impl ops::Div<Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Self::Output {
        Vec3(self.0 / rhs.0, self.1 / rhs.1, self.2 / rhs.2)
    }
}
impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3(-self.0, -self.1, -self.2)
    }
}
impl ops::AddAssign<Vec3> for Vec3 {

    fn add_assign(&mut self, rhs: Vec3) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}
impl ops::SubAssign<Vec3> for Vec3 {

    fn sub_assign(&mut self, rhs: Vec3) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
        self.2 -= rhs.2;
    }
}
impl ops::MulAssign<Vec3> for Vec3 {

    fn mul_assign(&mut self, rhs: Vec3) {
        self.0 *= rhs.0;
        self.1 *= rhs.1;
        self.2 *= rhs.2;
    }
}
impl ops::DivAssign<Vec3> for Vec3 {

    fn div_assign(&mut self, rhs: Vec3) {
        self.0 /= rhs.0;
        self.1 /= rhs.1;
        self.2 /= rhs.2;
    }
}
impl ops::Index<usize> for Vec3 {
    type Output =  f64;

    fn index(&self, idx: usize) -> &Self::Output {
        match idx {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            _ => panic!("index out of bounds")
        }
    }
}
impl ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        match idx {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            _ => panic!("index out of bounds")
        }
    }
}
impl ops::Mul< f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs:  f64) -> Self::Output {
        Vec3(self.0*rhs, self.1*rhs, self.2*rhs)
    }
}
impl ops::Div< f64> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs:  f64) -> Self::Output {
        Vec3(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

pub use Vec3 as Point3;

pub use Vec3 as Color;

impl Color{
    fn lin_to_gamma(&self) -> Self {
        Color(self.0.sqrt(), self.1.sqrt(), self.2.sqrt())
    }

    pub fn write(&self) -> String {
        let color = self.lin_to_gamma(); 
        let r: i32 = (color.0 * 255.99) as i32;
        let g: i32 = (color.1 * 255.99) as i32;
        let b: i32 = (color.2 * 255.99) as i32;
        format!("{r} {g} {b}")
    }

    pub fn black() -> Self {
        Color(0.0,0.0,0.0)
    }
}