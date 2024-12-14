#![allow(unused)]

use crate::*;

pub trait Material {
    fn scatter(&self, rin: &Ray, rec: &HitRecord) -> Option<Ray> {
        None
    }

    fn color(&self) -> Color {
        Vec3::empty()
    }
}

pub struct Matte {
    pub color: Color
}

impl Material for Matte {
    fn scatter(&self, rin: &Ray, rec: &HitRecord) -> Option<Ray> {
        Some(Ray{origin: rec.p, direction: rec.normal + Vec3::unitrand()})
    }

    fn color(&self) -> Vec3 {
        self.color
    }
}

pub struct Metal {
    pub color: Color
}

impl Material for Metal {
    fn scatter(&self, rin: &Ray, rec: &HitRecord) -> Option<Ray> {
        Some(Ray{origin: rec.p, direction: rin.direction.reflect(&rec.normal)})
    }

    fn color(&self) -> Vec3 {
        self.color
    }
}