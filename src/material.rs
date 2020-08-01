use crate::*;
use crate::{hitable::*, ray::*, vec3::*};
use rand::prelude::*;

#[derive(Copy, Clone)]
pub enum Material {
    Lambertian(Vec3),
    Metal(Vec3, f32),
    Dielectric(f32),
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2. * dot(v, n) * n
}

fn refract(v: Vec3, n: Vec3, ni_over_nt: f32) -> Option<Vec3> {
    let uv = make_unit_vector(v);
    let dt = dot(uv, v);
    let discriminant = 1. - ni_over_nt * ni_over_nt * (1. - dt * dt);
    if discriminant > 0. {
        Some(ni_over_nt * (uv - n * dt) - n * discriminant.sqrt())
    } else {
        None
    }
}

fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let mut r0 = (1. - ref_idx) / (1. + ref_idx);
    r0 = r0 * r0;
    r0 + (1. - r0) * (1. - cosine).powi(5)
}

pub struct Scatter {
    pub attenuation: Vec3,
    pub scattered: Ray,
}

pub fn scatter(r_in: &Ray, rec: &HitRecord) -> Option<Scatter> {
    match rec.mat {
        Material::Lambertian(albedo) => {
            let target = rec.p + rec.normal + random_in_unit_sphere();
            let scattered = Ray::new(rec.p, target - rec.p);
            let attenuation = albedo;
            Some(Scatter {
                attenuation,
                scattered,
            })
        }
        Material::Metal(albedo, fuzz) => {
            let reflected = reflect(make_unit_vector(r_in.direction()), rec.normal);
            let scattered = Ray::new(rec.p, reflected + fuzz * random_in_unit_sphere());
            let attenuation = albedo;
            if dot(scattered.direction(), rec.normal) > 0. {
                Some(Scatter {
                    attenuation,
                    scattered,
                })
            } else {
                None
            }
        }
        Material::Dielectric(ref_idx) => {
            let outward_normal: Vec3;
            let mut scattered: Ray;
            let mut refracted: Vec3 = Vec3::new(0., 0., 0.);
            let ni_over_nt: f32;
            let reflect_prob: f32;
            let cosine: f32;

            let reflected = reflect(r_in.direction(), rec.normal);
            let attenuation = Vec3::new(1., 1., 1.);

            if dot(r_in.direction(), rec.normal) > 0. {
                outward_normal = -rec.normal;
                ni_over_nt = ref_idx;
                cosine = ref_idx * dot(r_in.direction(), rec.normal) / r_in.direction().length();
            } else {
                outward_normal = rec.normal;
                ni_over_nt = 1. / ref_idx;
                cosine = -dot(r_in.direction(), rec.normal) / r_in.direction().length();
            }

            match refract(r_in.direction(), outward_normal, ni_over_nt) {
                Some(r) => {
                    refracted = r;
                    reflect_prob = schlick(cosine, ref_idx);
                }
                None => {
                    reflect_prob = 1.;
                }
            }

            if random::<f32>() < reflect_prob {
                scattered = Ray::new(rec.p, reflected);
            } else {
                scattered = Ray::new(rec.p, refracted);
            }
            Some(Scatter {
                scattered,
                attenuation,
            })
        }
    }
}
