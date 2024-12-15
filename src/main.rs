mod vec3;
mod ray;
mod hittable;
mod hitlist;
mod camera;
mod material;

use rand::Rng;

use vec3::*;
use ray::*;
use hittable::*;
use hitlist::*;
use camera::*;
use material::*;

fn main() {
    let camera = Camera::new(
        16.0 / 9.0,
        1200,
        Point3(13.0,2.0,3.0),
        500,
        50,
        35.6,
        Point3(0.0,0.0,0.0),
        Vec3(0.0,1.0,0.0),
        10.0,
        0.6);
    
    let mut world = Hitlist::new();

    let ground = Metal{color: Color(0.6, 0.6, 0.6), fuzziness: 0.7};
    world.push(Box::new(Sphere::new(Point3(0.0,-1000.0,0.0), 1000.0, Box::new(ground))));

    
    let mut rng = rand::thread_rng();
    for x in -11..11 {
        for z in -11..11 {
            let randmat = rng.gen_range(0.0..1.0);
            let center = Point3(x as f64 + 0.9 * rng.gen_range(0.0..1.0), 0.2, z as f64 + 0.9 * rng.gen_range(0.0..1.0));

            if (center - Point3(4.0, 0.2, 0.0)).length() > 0.9 {
                if randmat < 0.8 {
                    let mat = Matte{color: Color::random(0.0, 1.0) * Color::random(0.0, 1.0)};
                    let obj = Sphere::new(center, 0.2, Box::new(mat));
                    world.push(Box::new(obj));
                } else if randmat < 0.95 {
                    let mat = Metal{color: Color::random(0.5, 1.0), fuzziness: rng.gen_range(0.0..0.5)};
                    let obj = Sphere::new(center, 0.2, Box::new(mat));
                    world.push(Box::new(obj));
                } else {
                    let mat = Glass{color: Color::random(0.7, 1.0), refraction_index: rng.gen_range(0.75..1.5)};
                    let obj = Sphere::new(center, 0.2, Box::new(mat));
                    world.push(Box::new(obj));
                }
            }
        }
    }
    

    let mat1 = Glass{color: Color(1.0, 1.0, 1.0), refraction_index: 1.5};
    let obj1 = Sphere::new(Point3(0.0, 1.0, 0.0), 1.0, Box::new(mat1));
    world.push(Box::new(obj1));

    let mat2 = Matte{color: Color(0.4, 0.2, 0.1)};
    let obj2 = Sphere::new(Point3(-4.0, 1.0, 0.0), 1.0, Box::new(mat2));
    world.push(Box::new(obj2));

    let mat3 = Metal{color: Color(0.7, 0.6, 0.5), fuzziness: 0.0};
    let obj3 = Sphere::new(Point3(4.0, 1.0, 0.0), 1.0, Box::new(mat3));
    world.push(Box::new(obj3));

    camera.render(&mut world);
}