use rand::prelude::*;
use std::fs;
use std::rc::Rc;

mod camera;
mod hitable;
mod material;
mod ray;
mod vec3;

use camera::*;
use hitable::*;
use material::*;
use ray::*;
use vec3::*;

fn main() {
    create_ppm();
}

fn random_in_unit_sphere() -> Vec3 {
    let mut p = Vec3::new(1.0, 1.0, 1.0);
    while {
        p = 2.0 * Vec3::new(random::<f32>(), random::<f32>(), random::<f32>())
            - Vec3::new(1., 1., 1.);
        p.squared_length() >= 1.0
    } {}
    p
}

fn color(r: &Ray, world: &Vec<Box<dyn Hittable>>, depth: i32) -> Vec3 {
    let hit = world.hit(r, 0.001, std::f32::MAX);
    match hit {
        Some(hit) => {
            if depth < 50 {
                match scatter(r, &hit) {
                    Some(scattered) => {
                        scattered.attenuation * color(&scattered.scattered, world, depth + 1)
                    }
                    None => Vec3::new(0., 0., 0.),
                }
            } else {
                Vec3::new(0., 0., 0.)
            }
        }
        None => {
            let unit_direction = make_unit_vector(r.direction());
            let t = 0.5 * (unit_direction.y() + 1.0);
            (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
        }
    }
}

fn create_ppm() -> () {
    let nx = 200;
    let ny = 100;
    let ns = 100;
    let filename = "hello_world.ppm";

    let cam = Camera::new();

    let mut contents = vec![
        String::from("P3\n"),
        nx.to_string(),
        String::from(" "),
        ny.to_string(),
        String::from("\n"),
        String::from("\n255\n"),
    ];

    let world: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere::new(
            Vec3::new(0.0, 0.0, -1.0),
            0.5,
            Material::Lambertian(Vec3::new(0.1, 0.2, 0.5)),
        )),
        Box::new(Sphere::new(
            Vec3::new(0.0, -100.5, -1.0),
            100.0,
            Material::Lambertian(Vec3::new(0.8, 0.8, 0.0)),
        )),
        Box::new(Sphere::new(
            Vec3::new(1.0, 0., -1.),
            0.5,
            Material::Metal(Vec3::new(0.8, 0.6, 0.2), 0.3),
        )),
        Box::new(Sphere::new(
            Vec3::new(-1., 0., -1.),
            0.5,
            Material::Dielectric(1.5),
        )),
    ];

    for j in (0..=ny).rev() {
        for i in 0..nx {
            let mut col = Vec3::new(0., 0., 0.);
            for _s in 0..ns {
                let u = (i as f32 + random::<f32>()) / nx as f32;
                let v = (j as f32 + random::<f32>()) / ny as f32;
                let r = cam.get_ray(u, v);
                let _p = r.point_at_parameter(2.0);
                col += color(&r, &world, 0);
            }

            col /= ns as f32;
            col = Vec3::new(col[0].sqrt(), col[1].sqrt(), col[2].sqrt());
            let ir = (255.99 * col[0]) as i32;
            let ig = (255.99 * col[1]) as i32;
            let ib = (255.99 * col[2]) as i32;

            contents.push(format!("{} {} {} \n", ir, ig, ib));
        }
    }

    fs::write(filename, contents.concat()).unwrap();
}
