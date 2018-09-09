extern crate nalgebra;

use nalgebra::Unit;
use std::fs::File;
use std::io::prelude::*;

type Vec3 = nalgebra::Vector3<f64>;

struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    fn new(origin: Vec3, direction: Vec3) -> Self {
        Ray { origin, direction }
    }
    fn point_at_parameter(&self, t: f64) -> Vec3 {
        self.origin + t * self.direction
    }
}

fn hit_sphere(center: Vec3, radius: f64, r: &Ray) -> bool {
    use nalgebra::dot;
    let oc = r.origin - center;
    let a = dot(&r.direction, &r.direction);
    let b = 2.0 * dot(&oc, &r.direction);
    let c = dot(&oc, &oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant > 0.0
}

fn color(ray: &Ray) -> Vec3 {
    if hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, &ray) {
        Vec3::new(1.0, 0.0, 0.0)
    } else {
        let unit_direction = Unit::new_normalize(ray.direction);
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
    }
}

fn main() -> std::io::Result<()> {
    let mut file = File::create("foo.ppm")?;
    let nx = 200;
    let ny = 100;
    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    file.write(format!("P3\n{} {}\n255\n", nx, ny).as_bytes())?;

    for j in 0..ny {
        for i in 0..nx {
            let j = ny - 1 - j;

            let u = i as f64 / nx as f64;
            let v = j as f64 / ny as f64;
            let r = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical);
            let col = color(&r);

            let ir = (255.99 * col.x) as usize;
            let ig = (255.99 * col.y) as usize;
            let ib = (255.99 * col.z) as usize;

            file.write(format!("{} {} {}\n", ir, ig, ib).as_bytes())?;
        }
    }
    Ok(())
}
