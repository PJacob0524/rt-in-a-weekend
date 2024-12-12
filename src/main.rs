mod vec3;
mod ray;
mod hittable;
mod hitlist;

use vec3::*;
use ray::*;
use hittable::*;
use hitlist::*;

fn ray_color(r: &Ray, objects: &mut dyn Hittable) -> Color {
    let mut rec: HitRecord = HitRecord{p: Point3(0.0,0.0,0.0), normal: Vec3(0.0,0.0,0.0), t: 0.0, front_face: false}; 

    if objects.hit(r, 0.0, 1000000000000.0, &mut rec) {
        return (rec.normal + Color(1.0,1.0,1.0)) / 2.0;
    }

    let a = (r.direction.unit().y() + 1.0) / 2.0;
    return Color(1.0,1.0,1.0)*(1.0-a) + Color(0.5,0.7,1.0)*a;
}

fn main() {
    //Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = ((image_width as f64 / aspect_ratio) as i32).max(1);

    //Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Point3(0.0,0.0,0.0);

    //Vectors n stuff
    let viewport_u = Vec3(viewport_width, 0.0,0.0);
    let viewport_v = Vec3(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    let viewport_upper_left = camera_center - Vec3(0.0,0.0,focal_length) - viewport_u/2.0 - viewport_v/2.0;
    let pixel00 = viewport_upper_left + (pixel_delta_u + pixel_delta_v) / 2.0;

    //Objects
    let mut world = Hitlist::new();
    world.push(Box::new(Sphere::new(Point3(0.0,0.0,-1.0), 0.5)));
    world.push(Box::new(Sphere::new(Point3(0.0,-100.5,-1.0), 100.0)));

    //Code :)
    println!("P3\n{image_width} {image_height}\n255");
    let mut remaining: i32;

    for y in 0..image_height {
        remaining = image_height - y;
        eprint!("\rScanlines remaining: {remaining} ");
        for x in 0..image_width {
            let pix_center = pixel00 + pixel_delta_u * x as f64 + pixel_delta_v * y as f64;
            let ray_direction = pix_center - camera_center;
            let ray: Ray = Ray{origin: camera_center, direction: ray_direction};
            ray_color(&ray, &mut world).write();
        }
    }
    eprint!("\rDone.                            \n");
}