use crate::*;
use std::{fs, path::Path}; 

pub struct Scene {
    pub camera: Camera,
    pub world: Hitlist
}

impl Scene {
    pub fn load(scene_name: &str) -> Option<Self> {
        match scene_name {
            "weekend" => {
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

                Some(Scene { world, camera })
            },
            "weekend-small" => {
                let camera = Camera::new(
                    16.0 / 9.0,
                    800,
                    Point3(13.0,2.0,3.0),
                    100,
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
                for x in -6..6 {
                    for z in -6..6 {
                        let randmat = rng.gen_range(0.0..1.0);
                        let center = Point3(x as f64 * 2.0 + 1.8 * rng.gen_range(0.0..1.0), 0.2, z as f64 * 2.0 + 1.8 * rng.gen_range(0.0..1.0));
            
                        if (center - Point3(4.0, 0.2, 0.0)).length() > 1.8 {
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

                Some(Scene { world, camera })
            },
            "ballsofsteel" => {
                let camera = Camera::new(16.0 / 9.0, 1920, Point3::empty(), 2000, 200, 110.0, Vec3(0.0, 0.0, -1.0), Vec3(0.0, 1.0, 0.0), 1.0, 0.0);
                let mut world = Hitlist::new();

                let ground_mat = Matte{color: Color(0.8, 0.8, 0.0)};
                let left_mat = Metal{color: Color(0.8, 0.8, 0.8), fuzziness: 0.0};
                let mid_mat = Matte{color: Color(0.1, 0.2, 0.5)};
                let right_mat = Metal{color: Color(0.8, 0.6, 0.2), fuzziness: 0.0};

                let ground = Sphere::new(Point3(0.0, -100.5, -1.0), 100.0, Box::new(ground_mat));
                let left = Sphere::new(Point3(-1.0, 0.0, -1.0), 0.5, Box::new(left_mat));
                let mid = Sphere::new(Point3(0.0, 0.0, -1.2), 0.5, Box::new(mid_mat));
                let right = Sphere::new(Point3(1.0, 0.0, -1.0), 0.5, Box::new(right_mat));

                world.push(Box::new(ground));
                world.push(Box::new(left));
                world.push(Box::new(mid));
                world.push(Box::new(right));

                Some(Scene { world, camera })
            },
            "mattest" => {
                let camera = Camera::new(16.0 / 9.0, 800, Point3::empty(), 100, 50, 120.0, Vec3(0.0, 0.0, -1.0), Vec3(0.0, 1.0, 0.0), 1.0, 0.0);
                let mut world = Hitlist::new();

                let ground_mat = Matte{color: Color(0.8, 0.8, 0.8)};
                let ground = Sphere::new(Point3(0.0, -100.5, -1.0), 100.0, Box::new(ground_mat));
                world.push(Box::new(ground));

                let matte_mat = Matte{color: Color(1.0, 0.0, 0.0)};
                let metal_clear_mat = Metal{color: Color(0.0, 1.0, 0.0), fuzziness: 0.0};
                let metal_fuz_mat = Metal{color: Color(0.0, 0.0, 1.0), fuzziness: 0.5};
                let glass_high_ri_mat = Glass{color: Color(0.9, 0.9, 0.0), refraction_index: 1.5};
                let glass_low_ri_mat = Glass{color: Color(0.9, 0.0, 0.9), refraction_index: 0.75};

                let matte = Sphere::new(Point3(-2.0, 0.0, -2.0), 0.5, Box::new(matte_mat));
                let metal_clear = Sphere::new(Point3(0.0, 0.0, -2.0), 0.5, Box::new(metal_clear_mat));
                let metal_fuz = Sphere::new(Point3(2.0, 0.0, -2.0), 0.5, Box::new(metal_fuz_mat));
                let glass_high_ri = Sphere::new(Point3(-1.2, 0.0, -2.5), 0.5, Box::new(glass_high_ri_mat));
                let glass_low_ri = Sphere::new(Point3(1.2, 0.0, -2.5), 0.5, Box::new(glass_low_ri_mat));

                world.push(Box::new(matte));
                world.push(Box::new(metal_clear));
                world.push(Box::new(metal_fuz));
                world.push(Box::new(glass_high_ri));
                world.push(Box::new(glass_low_ri));

                Some(Scene { world, camera })
            },
            _ => {
                eprint!("Invalid scene namse");
                None
            }
        }
    }

    pub fn render(&mut self, file_path: &String) {
        if Path::new(file_path).exists() {
            fs::remove_file(file_path).expect("Failed to remove old image file");
        } 
        let file = File::create_new(file_path);
        self.camera.render(&mut self.world, file.expect("Failed to load file"));
    }
}