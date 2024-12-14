#![allow(unused)]

use std::collections::btree_map::Range;
use std::ops::RangeBounds;

use crate::vec3::*;
use crate::ray::*;

pub struct HitRecord {pub collision: bool, pub p: Point3, pub normal: Vec3, pub t: f64, pub front_face: bool}

impl HitRecord {
    pub fn setfail() -> Self {
        HitRecord {
            collision: false,
            p: Vec3::empty(),
            normal: Vec3::empty(), 
            t: 0.0,
            front_face: false
        }
    }

    pub fn set(r: &Ray, t: f64, outward_normal: &Vec3) -> Self {
        let collision = true;
        let t = t;
        let p = r.at(t);
        let mut normal = *outward_normal;
    
        let front_face = r.direction.dot(outward_normal) < 0.0;
        if !front_face {
            normal = outward_normal.clone() * -1.0;
        }

        HitRecord { collision, t, p, normal, front_face }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, trange: std::ops::Range<f64>) -> HitRecord {
        HitRecord::setfail()
    }
}

pub struct Sphere {center: Point3, radius: f64}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, trange: std::ops::Range<f64>) -> HitRecord {
        let co = self.center - r.origin;

        let a = r.direction.dot(&r.direction);
        let h = r.direction.dot(&co);
        let c = co.dot(&co) - self.radius * self.radius;

        let discriminant = h * h - a * c;

        if discriminant < 0.0{
            return HitRecord::setfail();
        }

        let sqrtd = discriminant.sqrt();

        let mut root = (h - sqrtd) / a;
        if !trange.contains(&root) {
            root = (h + sqrtd) / a;
            if !trange.contains(&root) {
                return HitRecord::setfail();
            }
        }
        
        let outward_normal = (r.at(root) - self.center) / self.radius;
        HitRecord::set(r, root, &outward_normal)
    }
}