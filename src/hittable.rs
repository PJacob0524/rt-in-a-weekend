#![allow(unused)]

use std::collections::btree_map::Range;
use std::ops::RangeBounds;

use crate::material;
use crate::vec3::*;
use crate::ray::*;
use crate::material::*;

pub struct HitRecord<'a> { pub p: Point3, pub normal: Vec3, pub t: f64, pub front_face: bool, pub mat: &'a Box<dyn Material>}

impl HitRecord<'_> {
    pub fn new<'a>(r: &Ray, t: f64, outward_normal: &Vec3, mat: &'a Box<dyn Material> ) -> HitRecord<'a> {
        let t = t;
        let p = r.at(t);
        let mut normal = *outward_normal;
    
        let front_face = r.direction.dot(outward_normal) < 0.0;
        if !front_face {
            normal = outward_normal.clone() * -1.0;
        }

        HitRecord { t, p, normal, front_face, mat }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, trange: std::ops::Range<f64>) -> Option<HitRecord> {
        None
    }
}

pub struct Sphere {center: Point3, radius: f64, mat: Box<dyn Material> }

impl Sphere {
    pub fn new(center: Point3, radius: f64, mat: Box<dyn Material>) -> Self {
        Sphere { center, radius, mat }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, trange: std::ops::Range<f64>) -> Option<HitRecord> {
        let co = self.center - r.origin;

        let a = r.direction.dot(&r.direction);
        let h = r.direction.dot(&co);
        let c = co.dot(&co) - self.radius * self.radius;

        let discriminant = h * h - a * c;

        if discriminant < 0.0{
            return None
        }

        let sqrtd = discriminant.sqrt();

        let mut root = (h - sqrtd) / a;
        if !trange.contains(&root) {
            root = (h + sqrtd) / a;
            if !trange.contains(&root) {
                return None
            }
        }
        
        let outward_normal = (r.at(root) - self.center) / self.radius;
        Some(HitRecord::new(r, root, &outward_normal, &self.mat))
    }
}