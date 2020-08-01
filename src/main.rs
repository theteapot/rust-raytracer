use rand::prelude::*;
use std::fs;

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

fn color(r: &Ray, world: &Vec<Box<dyn Hittable>>) -> Vec3 {
    match world.hit(r, 0.0, std::f64::MAX) {
        Some(hit) => {
            0.5 * Vec3::new(
                hit.normal.x() + 1.0,
                hit.normal.y() + 1.0,
                hit.normal.z() + 1.0,
            )
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
        Box::from(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)),
        Box::from(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)),
    ];

    for j in (0..=ny).rev() {
        for i in 0..nx {
            let mut col = Vec3::new(0., 0., 0.);
            for _s in 0..ns {
                let u = (i as f64 + random::<f64>()) / nx as f64;
                let v = (j as f64 + random::<f64>()) / ny as f64;
                let r = cam.get_ray(u, v);
                let _p = r.point_at_parameter(2.0);
                col += color(&r, &world);
            }

            col /= ns as f64;
            let ir = (255.99 * col[0]) as i32;
            let ig = (255.99 * col[1]) as i32;
            let ib = (255.99 * col[2]) as i32;

            contents.push(format!("{} {} {} \n", ir, ig, ib));
        }
    }

    fs::write(filename, contents.concat()).unwrap();
}
