#![allow(unused)]

use crate::*;

pub struct Camera {
    aspect_ratio: f64,
    image_width: i64,
    image_height: i64,
    focal_length: f64,
    viewport_width: f64,
    viewport_height: f64,
    camera_center: Point3,
    viewport_u: Vec3, 
    viewport_v: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    viewport_upper_left: Vec3,
    pixel00: Vec3
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: i64, focal_length: f64, viewport_height: f64, camera_center: Point3) -> Self {
        let aspect_ratio = aspect_ratio;
        let image_width = image_width;
        let image_height = ((image_width as f64 / aspect_ratio) as i64).max(1);

        //Camera
        let focal_length = focal_length;
        let viewport_height = viewport_height;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
        let camera_center = camera_center;

        //Vectors n stuff
        let viewport_u = Vec3(viewport_width, 0.0,0.0);
        let viewport_v = Vec3(0.0, -viewport_height, 0.0);

        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        let viewport_upper_left = camera_center - Vec3(0.0,0.0,focal_length) - viewport_u/2.0 - viewport_v/2.0;
        let pixel00 = viewport_upper_left + (pixel_delta_u + pixel_delta_v) / 2.0;
        Camera { aspect_ratio, image_width, image_height, focal_length, viewport_width, viewport_height, camera_center, viewport_u, viewport_v, pixel_delta_u, pixel_delta_v, viewport_upper_left, pixel00 }
    }

    fn ray_color(r: &Ray, objects: &mut dyn Hittable) -> Color {
        let mut rec: HitRecord = HitRecord{p: Point3(0.0,0.0,0.0), normal: Vec3(0.0,0.0,0.0), t: 0.0, front_face: false}; 
    
        if objects.hit(r, 0.0, 1000000000000.0, &mut rec) {
            return (rec.normal + Color(1.0,1.0,1.0)) / 2.0;
        }
    
        let a = (r.direction.unit().y() + 1.0) / 2.0;
        return Color(1.0,1.0,1.0)*(1.0-a) + Color(0.5,0.7,1.0)*a;
    }
    
    pub fn render(&self, world: &mut dyn Hittable) {
        println!("P3\n{} {}\n255", self.image_width, self.image_height);
        let mut remaining: i64;

        for y in 0..self.image_height {
            remaining = self.image_height - y;
            eprint!("\rScanlines remaining: {remaining} ");
            for x in 0..self.image_width {
                let pix_center = self.pixel00 + self.pixel_delta_u * x as f64 + self.pixel_delta_v * y as f64;
                let ray_direction = pix_center - self.camera_center;
                let ray: Ray = Ray{origin: self.camera_center, direction: ray_direction};
                Camera::ray_color(&ray, world).write();
            }
        }
        eprint!("\rDone.                            \n");
    }
}