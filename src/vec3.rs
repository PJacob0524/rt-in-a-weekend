#![allow(unused)]

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
    pub fn write(&self) {
        let r: i32 = (self.0 * 255.99) as i32;
        let g: i32 = (self.1 * 255.99) as i32;
        let b: i32 = (self.2 * 255.99) as i32;
        println!("{r} {g} {b}");
    }
}