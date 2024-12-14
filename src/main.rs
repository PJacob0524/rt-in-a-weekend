mod vec3;
mod ray;
mod hittable;
mod hitlist;
mod camera;
mod material;

use vec3::*;
use ray::*;
use hittable::*;
use hitlist::*;
use camera::*;
use material::*;

fn main() {
    let camera = Camera::new(16.0 / 9.0, 400, 1.0, 2.0, Point3(0.0,0.0,0.0), 100, 50);
    
    //Objects
    let ground = Matte{color: Color(0.8, 0.8, 0.0)};
    let center= Matte{color: Color(0.1, 0.2, 0.5)};
    let left   = Metal{color: Color(0.8, 0.8, 0.8)};
    let right  = Metal{color: Color(0.8, 0.6, 0.2)};

    let mut world = Hitlist::new();
    world.push(Box::new(Sphere::new(Point3(0.0,-100.5,-1.0), 100.0, Box::new(ground))));
    world.push(Box::new(Sphere::new(Point3(0.0,0.0,-1.0), 0.5, Box::new(center))));
    world.push(Box::new(Sphere::new(Point3(-1.0,0.0,-1.0), 0.5, Box::new(left))));
    world.push(Box::new(Sphere::new(Point3(1.0,0.0,-1.0), 0.5, Box::new(right))));

    camera.render(&mut world);
}