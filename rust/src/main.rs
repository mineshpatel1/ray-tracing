use std::fs::File;
use std::io::prelude::*;
use indicatif::ProgressBar;

const IMAGE_WIDTH: i32 = 256;
const IMAGE_HEIGHT: i32 = 256;
const IMAGES_DIR: &str = "images";
const OUTPUT_IMAGE: &str = "sample";


fn write_file(path: &String, content: &String) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;
    return Ok(());
}

fn main() {
    let fpath = format!("{}/{}.ppm", IMAGES_DIR, OUTPUT_IMAGE);
    let mut out = format!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);
    let bar = ProgressBar::new(IMAGE_HEIGHT as u64);

    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let r = (i as f64) / ((IMAGE_WIDTH - 1) as f64);
            let g = (j as f64) / ((IMAGE_HEIGHT - 1) as f64);
            let b = 0.8;
            
            let ir = (255.999 * r) as u64;
            let ig = (255.999 * g) as u64;
            let ib = (255.999 * b) as u64;
            out.push_str(&format!("{} {} {}\n", ir, ig, ib)[..]);
        }
        bar.inc(1);
    }
    write_file(&fpath, &out).expect("Failed when writing file.");
    bar.finish();
}
