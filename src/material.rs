#![allow(unused)]

use crate::*;

pub trait Material {

}

pub struct Matte {

}

impl Material for Matte {

}

pub struct NullMaterial {
    
}

impl Material for NullMaterial {
    
}

impl NullMaterial {
    pub fn new() -> Self {
        NullMaterial{}
    }
}