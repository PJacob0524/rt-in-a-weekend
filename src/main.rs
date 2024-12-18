mod vec3;
mod ray;
mod hittable;
mod hitlist;
mod camera;
mod material;
mod scene;
//mod bbox;

use rand::Rng;
use std::time::Instant;
use std::env;
use std::process::Command;
use std::fs::File;
use std::io::prelude::*;   

use vec3::*;
use ray::*;
use hittable::*;
use hitlist::*;
use camera::*;
use material::*;
use scene::*; 
//use bbox::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let time = Instant::now(); 

    let mut scene = Scene::load(&args[1]).unwrap();
    scene.render(&args[2]);

    let milli = time.elapsed().as_millis();
    let sec = milli as f64 / 1000.0; 
    let min = (sec /  60.0) as u128;
    let sec = sec % 60.0;
    if min > 0 {
        eprintln!("Time elapsed: {} minutes and {:.2} seconds", min, sec);
    } else {
        eprintln!("Time elapsed: {:.2} seconds", sec);
    }

    if &args[3] == "show" {
        Command::new("ImageGlass.exe").arg(&args[2]).spawn().expect("Failed to open file");
    } 
}