use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::create("foo.ppm")?;
    let nx = 200;
    let ny = 100;
    file.write(format!("P3\n{} {}\n255\n", nx, ny).as_bytes())?;

    for j in 0..ny {
        for i in 0..nx {
            let j = ny - 1 - j;
            let r = i as f64 / nx as f64;
            let g = j as f64 / ny as f64;
            let b = 0.2_f64;

            let ir = (255.99 * r) as usize;
            let ig = (255.99 * b) as usize;
            let ib = (255.99 * g) as usize;

            file.write(format!("{} {} {}\n", ir, ig, ib).as_bytes())?;
        }
    }
    Ok(())
}
