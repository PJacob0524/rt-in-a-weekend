use std::vec;

use crate::*;

struct BBox {
    objects: Hitlist,
    xrange: std::ops::Range<f64>,
    yrange: std::ops::Range<f64>,
    zrange: std::ops::Range<f64>
}

impl Hittable for BBox {
    fn hit(&self, r: &Ray, trange: std::ops::Range<f64>) -> Option<HitRecord> {
        let xhit = ((self.xrange.start - r.origin.x()) / r.direction.x()).min((self.xrange.end - r.origin.x()) / r.direction.x());
        if trange.contains(&xhit){
            let yhit = ((self.yrange.start - r.origin.y()) / r.direction.y()).min((self.yrange.end - r.origin.y()) / r.direction.y());
            if trange.contains(&yhit){
                let zhit = ((self.zrange.start - r.origin.z()) / r.direction.z()).min((self.zrange.end - r.origin.z()) / r.direction.z());
                if trange.contains(&zhit){
                    return self.objects.hit(r, trange);
                }
            }
        }
        None 
    }
}

impl BBox {
    pub fn new(list: Vec<&dyn Hittable>, xrange: std::ops::Range<f64>, yrange: std::ops::Range<f64>, zrange: std::ops::Range<f64>) -> Self {
        let objects = Hitlist::new();

        for obj in list {
            let mut x = xrange.start;
            'objloop: while x <= xrange.end {
                let mut y = xrange.start;
                while y <= yrange.end {
                    if !obj.hit(&Ray{origin:Point3(x,y,zrange.start),direction:Vec3(0.0,0.0,1.0)}, zrange.clone()).is_none() {
                        objects.push(Box::new(obj));
                    }
                    y += 0.1;
                }
                x += 0.1;
            }
        }

        BBox { objects, xrange, yrange, zrange }
    }
}