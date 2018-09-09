extern crate nalgebra;
extern crate rand;

use nalgebra::Unit;
use rand::random;
use std::fs::File;
use std::io::prelude::*;

type Vec3 = nalgebra::Vector3<f64>;

struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    fn new(origin: Vec3, lower_left_corner: Vec3, horizontal: Vec3, vertical: Vec3) -> Self {
        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}

struct Ray {
    origin: Vec3,
    direction: Vec3,
}

struct Hit {
    t: f64,
    p: Vec3,
    normal: Vec3,
}

struct Scene<'a> {
    objects: Vec<&'a Hittable>,
}

impl<'a> Scene<'a> {
    fn new(objects: Vec<&'a Hittable>) -> Self {
        Scene { objects }
    }
}

impl<'a> Hittable for Scene<'a> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let mut closest_hit: Option<Hit> = None;
        let mut closest_so_far = t_max;
        for i in 0..self.objects.len() {
            if let Some(hit) = self.objects[i].hit(&ray, t_min, closest_so_far) {
                closest_so_far = hit.t;
                closest_hit = Some(hit);
            }
        }
        closest_hit
    }
}

impl Hit {
    fn new(t: f64, p: Vec3, normal: Vec3) -> Self {
        Hit { t, p, normal }
    }
}

trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit>;
}

struct Sphere {
    center: Vec3,
    radius: f64,
}

impl Sphere {
    fn new(center: Vec3, radius: f64) -> Self {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        use nalgebra::dot;
        let oc = ray.origin - self.center;
        let a = dot(&ray.direction, &ray.direction);
        let b = dot(&oc, &ray.direction);
        let c = dot(&oc, &oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let temp = (-b - (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                let point = ray.point_at_parameter(temp);
                Some(Hit::new(temp, point, (point - self.center) / self.radius))
            } else {
                let temp = (-b + (b * b - a * c).sqrt()) / a;
                if temp < t_max && temp > t_min {
                    let point = ray.point_at_parameter(temp);
                    Some(Hit::new(temp, point, (point - self.center) / self.radius))
                } else {
                    None
                }
            }
        } else {
            None
        }
    }
}

impl Ray {
    fn new(origin: Vec3, direction: Vec3) -> Self {
        Ray { origin, direction }
    }
    fn point_at_parameter(&self, t: f64) -> Vec3 {
        self.origin + t * self.direction
    }
}

fn color(ray: &Ray, hittable: &Hittable) -> Vec3 {
    if let Some(hit) = hittable.hit(&ray, 0.0, std::f64::MAX) {
        0.5 * Vec3::new(hit.normal.x + 1.0, hit.normal.y + 1.0, hit.normal.z + 1.0)
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
    let ns = 100;
    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);
    let sphere1 = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5);
    let sphere2 = Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0);
    let scene = Scene::new(vec![&sphere1, &sphere2]);
    let camera = Camera::new(origin, lower_left_corner, horizontal, vertical);
    file.write(format!("P3\n{} {}\n255\n", nx, ny).as_bytes())?;

    for j in 0..ny {
        let j = ny - 1 - j;
        for i in 0..nx {
            let mut col = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..ns {
                let u = (i as f64 + random::<f64>()) / nx as f64;
                let v = (j as f64 + random::<f64>()) / ny as f64;
                let r = camera.get_ray(u, v);
                col += color(&r, &scene);
            }

            col /= ns as f64;
            let ir = (255.99 * col.x) as usize;
            let ig = (255.99 * col.y) as usize;
            let ib = (255.99 * col.z) as usize;

            file.write(format!("{} {} {}\n", ir, ig, ib).as_bytes())?;
        }
    }
    Ok(())
}
