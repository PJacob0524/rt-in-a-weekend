mod vec3;
mod ray;
mod hittable;
mod hitlist;

use vec3::*;
use ray::*;
use hittable::*;
use hitlist::*;

fn hit_sphere(center: Point3, radius: f64, ray: &Ray) -> f64 {
    let co = center - ray.origin;

    let a = ray.direction.dot(&ray.direction);
    let h = ray.direction.dot(&co);
    let c = co.dot(&co) - radius * radius;

    let discriminant = h * h - a * c;

    if discriminant >= 0.0{
        return (h - discriminant.sqrt()) / a;
    }
    return -1.0;
}

fn ray_color(r: &Ray) -> Color {
    let c = Point3(0.0,0.0,-1.0);
    let t = hit_sphere(c, 0.5, r);
    if t != -1.0{
        return (Color(1.0, 1.0, 1.0) + (r.at(t) - c).unit()) / 2.0;
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

    //Code :)
    println!("P3\n{image_width} {image_height}\n255");
    let mut remaining: i32;

    for y in 0..image_height {
        remaining = image_height - y;
        eprint!("\rScanlines remaining: {remaining} ");
        for x in 0..image_width {
            let pix_center = pixel00 + pixel_delta_u * x as f64 + pixel_delta_v * y as f64;
            let ray_direction = pix_center - camera_center;
            let ray: Ray = Ray{origin: pix_center, direction: ray_direction};
            ray_color(&ray).write();
        }
    }
    eprint!("\rDone.                            \n");
}