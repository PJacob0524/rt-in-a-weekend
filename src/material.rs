#![allow(unused)]

use rand::Rng;

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
    pub color: Color,
}

impl Material for Matte {
    fn scatter(&self, rin: &Ray, rec: &HitRecord) -> Option<Ray> {
        Some(Ray {
            origin: rec.p,
            direction: rec.normal + Vec3::unitrand(),
        })
    }

    fn color(&self) -> Vec3 {
        self.color
    }
}

pub struct Metal {
    pub color: Color,
    pub fuzziness: f64,
}

impl Material for Metal {
    fn scatter(&self, rin: &Ray, rec: &HitRecord) -> Option<Ray> {
        let reflection = rin.direction.reflect(&rec.normal) + Vec3::unitrand() * self.fuzziness;
        if reflection.dot(&rec.normal) < 0.0 {
            return None;
        }
        Some(Ray {
            origin: rec.p,
            direction: reflection,
        })
    }

    fn color(&self) -> Vec3 {
        self.color
    }
}

pub struct Glass {
    pub color: Color,
    pub refraction_index: f64,
}

impl Material for Glass {
    fn scatter(&self, rin: &Ray, rec: &HitRecord) -> Option<Ray> {
        let mut ri = self.refraction_index;
        if rec.front_face {
            ri = 1.0 / ri;
        }
        let unit_rin = rin.direction.unit();
        let cos_theta: f64 = (unit_rin * -1.0).dot(&rec.normal).min(1.0);
        if ri * (1.0 - (cos_theta * cos_theta)).sqrt() > 1.0 || Self::schlick_approx(cos_theta, ri)
        {
            return Some(Ray {
                origin: rec.p,
                direction: unit_rin.reflect(&rec.normal),
            });
        }
        Some(Ray {
            origin: rec.p,
            direction: unit_rin.refract(&rec.normal, ri),
        })
    }

    fn color(&self) -> Vec3 {
        self.color
    }
}

impl Glass {
    fn schlick_approx(cos: f64, ri: f64) -> bool {
        let mut rng = rand::thread_rng();

        let r0 = ((1.0 - ri) / (1.0 + ri)).powf(2.0);
        r0 + (1.0 - r0) * (1.0 - cos).powf(5.0) > rng.gen_range((0.0..1.0))
    }
}
