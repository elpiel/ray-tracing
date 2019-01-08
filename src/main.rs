use std::fs::File;
use std::io::LineWriter;
use std::io::prelude::*;

fn main() -> Result<(), std::io::Error>  {
    let width = 200;
    let height = 100;
    let max_color = 255;

    let file = File::create("image.ppm")?;
    let mut file = LineWriter::new(file);

    let beginning_file = format!("P3\n{} {}\n{}\n", width, height, max_color);
    file.write_all(beginning_file.as_bytes())?;

    for j in (0..height).rev() {
        for i in 0..width {
            let r: f64 = i as f64 / width as f64;
            let g: f64 = j as f64 / height as f64;
            let b: f64 = 0.2;

            let ir = (255.99 * r) as i32;
            let ig = (255.99 * g) as i32;
            let ib = (255.99 * b) as i32;

            let pixel_color = format!("{} {} {}\n", ir, ig, ib);
            file.write_all(pixel_color.as_bytes())?;
        }
    }

    Ok(())
}
