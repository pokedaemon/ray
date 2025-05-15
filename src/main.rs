use std::io::{stdout, BufWriter, Write};
use std::fs::{File};

fn main() {
    // Graphic Hello World
    const IMAGE_HEIGHT: i32 = 256;
    const IMAGE_WIDTH: i32 = 256;

    let image_output_name = String::from("./output/hello_world.ppm");

    // stdout handler
    let stdout = stdout();
    let mut handle = BufWriter::new(stdout.lock());

    // create our file in output directory
    let mut image_file = match File::create(image_output_name) {
        Ok(v) => {
            v
        },
        Err(e) => {
            panic!("Creating file error: {e}!")
        }
    };

    // create here our scalers
    let mut r: f64 = 0.0;
    let mut g: f64 = 0.0;
    let mut b: f64 = 0.0;

    // buffer our
    let mut buf: String = String::with_capacity(500*1024*1024*8);
    buf += &format!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n");

    // first -> width, second -> height. Filling to right bottom
    for j in 0..IMAGE_HEIGHT {
        write!(handle, "\rScanlines remaining: {} ", IMAGE_HEIGHT-j).unwrap();
        handle.flush().unwrap();
        for i in 0..IMAGE_WIDTH {
            r = i as f64 / (IMAGE_WIDTH-1) as f64;
            g = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            b = 0.0;
            
            // in stdout. used ">>" operator for creating image
            buf += &format!("{} {} {}\n", (r * 255.999) as u8, (g * 255.999) as u8, (b * 255.999) as u8);
        }
    }

    if let Err(e) = image_file.write(buf.as_bytes()) {
        eprintln!("Write to file error: {e}!");
    }

    println!("\rDone.                           \n");
}
