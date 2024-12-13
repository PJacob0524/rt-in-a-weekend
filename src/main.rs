mod vec3;
mod ray;
mod hittable;
mod hitlist;
mod camera;

use vec3::*;
use ray::*;
use hittable::*;
use hitlist::*;
use camera::*;

fn main() {
    let camera = Camera::new(16.0 / 9.0, 400, 1.0, 2.0, Point3(0.0,0.0,0.0), 100, 50);
    
    //Objects
    let mut world = Hitlist::new();
    world.push(Box::new(Sphere::new(Point3(0.0,0.0,-1.0), 0.5)));
    world.push(Box::new(Sphere::new(Point3(0.0,-100.5,-1.0), 100.0)));

    camera.render(&mut world);
}