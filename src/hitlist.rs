#![allow(unused)]

use crate::*;
use std::vec;

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
    fn hit(&self, r: &Ray, tmin: f64, tmax: f64, record: &mut HitRecord) -> bool {
        let mut result = false;
        let mut closest = tmax;

        for item in self.list.iter() {
            if item.hit(r, tmin, closest, record) {
                result = true;
                closest = record.t;
            }
        }

        return result
    }
}