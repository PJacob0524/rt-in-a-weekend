#![allow(unused)]

use crate::vec3::*;
use crate::ray::*;

pub struct HitRecord {pub p: Point3, pub normal: Vec3, pub t: f64, pub front_face: bool}

impl HitRecord {
    fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = r.direction.dot(outward_normal) < 0.0;
        if self.front_face {
            self.normal = outward_normal.clone();
        } else {
            self.normal = outward_normal.clone() * -1.0;
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, tmin: f64, tmax: f64, record: &mut HitRecord) -> bool { false }
}

pub struct Sphere {center: Point3, radius: f64}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, tmin: f64, tmax: f64, hit_record: &mut HitRecord) -> bool {
        let co = self.center - r.origin;

        let a = r.direction.dot(&r.direction);
        let h = r.direction.dot(&co);
        let c = co.dot(&co) - self.radius * self.radius;

        let discriminant = h * h - a * c;

        if discriminant < 0.0{
            return false;
        }

        let sqrtd = discriminant.sqrt();

        let mut root = (h - sqrtd) / a;
        if root < tmin || root > tmax {
            root = (h + sqrtd) / a;
            if root < tmin || root > tmax {
                return false;
            }
        }

        hit_record.t = root;
        hit_record.p = r.at(root);
        
        let outward_normal = (hit_record.p - self.center) / self.radius;
        hit_record.set_face_normal(r, &outward_normal);

        true
    }
}