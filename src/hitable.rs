use crate::{ray::*, vec3::*};
pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

#[derive(Copy, Clone)]
pub struct HitRecord {
    t: f64,
    p: Vec3,
    pub normal: Vec3,
}

impl Hittable for Vec<Box<dyn Hittable>> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit: Option<HitRecord> = None;

        for hitable in self.iter() {
            if let Some(cantidate_hit) = hitable.hit(r, t_min, t_max) {
                match hit {
                    None => hit = Some(cantidate_hit),
                    Some(prev) => {
                        if cantidate_hit.t < prev.t {
                            hit = Some(cantidate_hit);
                        }
                    }
                }
            }
        }

        hit
    }
}

pub struct Sphere {
    center: Vec3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Sphere {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = dot(r.direction(), r.direction());
        let b = dot(oc, r.direction());
        let c = dot(oc, oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let mut t = (-b - (discriminant).sqrt()) / a;
            if t > t_min && t < t_max {
                let p = r.point_at_parameter(t);
                return Some(HitRecord {
                    t,
                    p,
                    normal: (p - self.center) / self.radius,
                });
            }
            t = (-b + (discriminant).sqrt()) / a;
            if t > t_min && t < t_max {
                let p = r.point_at_parameter(t);
                return Some(HitRecord {
                    t,
                    p,
                    normal: (p - self.center) / self.radius,
                });
            }
        }

        None
    }
}
