#![allow(unused)]

use crate::*;
use std::{ops::RangeBounds, vec};

pub struct Hitlist{
    list: Vec<Box<dyn Hittable>>
}

impl Hitlist {
    pub fn clear(&mut self) {
        self.list.clear(); 
    }

    pub fn push(&mut self, item: Box<dyn Hittable>) {
        self.list.push(item);
    }

    pub fn new() -> Self{
        Hitlist { list: Vec::<Box<dyn Hittable>>::new() }
    }
}

impl Hittable for Hitlist {
    fn hit(&self, r: &Ray, trange: std::ops::Range<f64>) -> Option<HitRecord> {
        let collisions = self.list.iter()
            .map(|object| object.hit(r, trange.clone()))
            .filter(|object| !object.is_none())
            .map(|object| object.unwrap())
            .min_by(|rec1, rec2| rec1.t.total_cmp(&rec2.t));
        if !collisions.is_none() {
            return collisions
        }
        None
    }
}