mod vec3;
mod color;

use std::io::{stdout, BufWriter, Write};
use std::fs::{File};

use color::{Color, write_color};

// TODO: Can I write to file without buffer and make it perfomance?

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

    // buffer -> file
    let mut buf: String = String::with_capacity(500*1024*1024*8);
    buf += &format!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n");

    // first -> width, second -> height. Filling to right bottom
    for j in 0..IMAGE_HEIGHT {
        // this is in log
        write!(handle, "\rScanlines remaining: {} ", IMAGE_HEIGHT-j).unwrap();
        // and necessary flush stdout
        handle.flush().unwrap();

        for i in 0..IMAGE_WIDTH {
            let pixel_color = Color::with((
                i as f32 / (IMAGE_WIDTH-1) as f32,
                j as f32 / (IMAGE_HEIGHT-1) as f32,
                0.0
            ));
            
            // write color to buffer
            write_color(pixel_color, &mut buf);
        }
    }

    // write to file
    if let Err(e) = image_file.write(buf.as_bytes()) {
        eprintln!("Write to file error: {e}!");
    }

    println!("\rDone.                           \n");
}
