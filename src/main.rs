extern crate nalgebra;

use nalgebra::{Unit, Vector3};
use std::fs::File;
use std::io::prelude::*;

struct Ray {
    origin: Vector3<f64>,
    direction: Vector3<f64>,
}

impl Ray {
    fn new(origin: Vector3<f64>, direction: Vector3<f64>) -> Self {
        Ray { origin, direction }
    }
    fn point_at_parameter(&self, t: f64) -> Vector3<f64> {
        self.origin + t * self.direction
    }
}

fn color(ray: &Ray) -> Vector3<f64> {
    let unit_direction = Unit::new_normalize(ray.direction);
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0)
}

fn main() -> std::io::Result<()> {
    let mut file = File::create("foo.ppm")?;
    let nx = 200;
    let ny = 100;
    let lower_left_corner = Vector3::new(-2.0, -1.0, -1.0);
    let horizontal = Vector3::new(4.0, 0.0, 0.0);
    let vertical = Vector3::new(0.0, 2.0, 0.0);
    let origin = Vector3::new(0.0, 0.0, 0.0);

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
