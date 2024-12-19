#![allow(unused)]

use core::f64;
use rand::Rng;
use std::{array, f64::consts::PI, ops::Range};

use crate::*;

pub struct Camera {
    aspect_ratio: f64,
    image_width: i64,
    image_height: i64,
    viewport_width: f64,
    viewport_height: f64,
    camera_center: Point3,
    viewport_u: Vec3,
    viewport_v: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    viewport_upper_left: Vec3,
    pixel00: Vec3,
    samples_per_pixel: i64,
    max_depth: i64,
    fov: f64,
    look_at: Point3,
    vup: Vec3,
    frame_basis_u: Vec3,
    frame_basis_v: Vec3,
    frame_basis_w: Vec3,
    focus_dist: f64,
    defocus_angle: f64,
    defocus_u: Vec3,
    defocus_v: Vec3,
    defocus_radius: f64,
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        image_width: i64,
        camera_center: Point3,
        samples_per_pixel: i64,
        max_depth: i64,
        fov: f64,
        look_at: Point3,
        vup: Vec3,
        focus_dist: f64,
        defocus_angle: f64,
    ) -> Self {
        let aspect_ratio = aspect_ratio;
        let image_width = image_width;
        let image_height = ((image_width as f64 / aspect_ratio) as i64).max(1);

        //Camera
        let theta = (fov / 180.0 * PI) / 2.0;
        let h = theta.tan();
        let viewport_width = h * 2.0 * focus_dist;
        let viewport_height = viewport_width * (image_height as f64 / image_width as f64);

        //let viewport_height = viewport_height;
        //let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
        let camera_center = camera_center;

        let frame_basis_w = (camera_center - look_at).unit();
        let frame_basis_u = (vup.cross(&frame_basis_w)).unit();
        let frame_basis_v = frame_basis_w.cross(&frame_basis_u);

        //Vectors n stuff
        let viewport_u = frame_basis_u * viewport_width;
        let viewport_v = frame_basis_v * -viewport_height;

        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        let viewport_upper_left =
            camera_center - frame_basis_w * focus_dist - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00 = viewport_upper_left + (pixel_delta_u + pixel_delta_v) / 2.0;

        let samples_per_pixel = samples_per_pixel;
        let max_depth = max_depth;

        let defocus_radius = focus_dist * (defocus_angle / 2.0).to_radians().tan();
        let defocus_u = frame_basis_u * defocus_radius;
        let defocus_v = frame_basis_v * defocus_radius;

        Camera {
            aspect_ratio,
            image_width,
            image_height,
            viewport_width,
            viewport_height,
            camera_center,
            viewport_u,
            viewport_v,
            pixel_delta_u,
            pixel_delta_v,
            viewport_upper_left,
            pixel00,
            samples_per_pixel,
            max_depth,
            fov,
            look_at,
            vup,
            frame_basis_u,
            frame_basis_v,
            frame_basis_w,
            focus_dist,
            defocus_angle,
            defocus_u,
            defocus_v,
            defocus_radius,
        }
    }

    fn defocus(&self) -> Point3 {
        let defocus = Vec3::unitdiskrand();
        return self.camera_center + self.defocus_u * defocus.0 + self.defocus_v * defocus.1;
    }

    fn ray_color(r: &Ray, objects: &mut dyn Hittable, depth: i64) -> Color {
        if depth == 0 {
            return Vec3::empty();
        }

        let hit = objects.hit(
            r,
            Range {
                start: 0.001,
                end: f64::INFINITY,
            },
        );

        if !hit.is_none() {
            //return (rec.normal + Color(1.0,1.0,1.0)) / 2.0;
            let rec = hit.unwrap();
            let scatter = rec.mat.scatter(r, &rec);
            if !scatter.is_none() {
                return rec.mat.color() * Self::ray_color(&scatter.unwrap(), objects, depth - 1);
            }
            return Color::black();
        }

        let a = (r.direction.unit().y() + 1.0) * 0.5;
        return Color(1.0, 1.0, 1.0) * (1.0 - a) + Color(0.5, 0.7, 1.0) * a;
    }

    pub fn render(&self, world: &mut Hitlist, mut file: File) {
        writeln!(file, "P3\n{} {}\n255", self.image_width, self.image_height);
        let mut remaining: i64;
        let mut rng = rand::thread_rng();

        for y in 0..self.image_height {
            remaining = self.image_height - y;
            eprint!("\rScanlines remaining: {remaining} ");
            for x in 0..self.image_width {
                let pix_center: Point3 =
                    self.pixel00 + self.pixel_delta_u * x as f64 + self.pixel_delta_v * y as f64;
                let ray_origin: Point3 = self.defocus();
                let ray_direction: Vec3 = pix_center - ray_origin;
                let ray: Ray = Ray {
                    origin: ray_origin,
                    direction: ray_direction,
                };
                let mut color = Camera::ray_color(&ray, world, self.max_depth);
                for i in 0..self.samples_per_pixel {
                    let variance: Point3 = Point3(
                        rng.gen_range((-self.pixel_delta_u.0 / 2.0)..(self.pixel_delta_u.0 / 2.0)),
                        rng.gen_range((self.pixel_delta_v.1 / 2.0)..(-self.pixel_delta_v.1 / 2.0)),
                        0.0,
                    );
                    let pix_center: Point3 = self.pixel00
                        + self.pixel_delta_u * x as f64
                        + self.pixel_delta_v * y as f64
                        + variance;
                    let ray_origin = self.defocus();
                    let ray_direction: Vec3 = pix_center - ray_origin;
                    let ray: Ray = Ray {
                        origin: ray_origin,
                        direction: ray_direction,
                    };
                    color += Camera::ray_color(&ray, world, self.max_depth);
                }
                color = color / (self.samples_per_pixel as f64 + 1.0);
                writeln!(file, "{}", color.write());
            }
        }
        eprint!("\rDone.                            \n");
    }
}
