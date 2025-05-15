mod vec3;
mod color;
mod ray;

use std::io::{stdout, BufWriter, Write};
use std::fs::{File};

use color::{Color, write_color};
use vec3::{Point3, Vec3};
use ray::Ray;

// TODO: Can I write to file without buffer and make it perfomance?

pub fn ray_color(ray: &Ray) -> Color {
    let unit_direction = Vec3::unit_vector(ray.direction());
    let a = (unit_direction.y() + 1.0) * 0.5;

    Color::with((1.0, 1.0, 1.0))* (1.0 - a) + Color::with((0.5, 0.7, 1.0)) * a
}

fn main() {
    /* 
        ------------- work with file n stdout -------------
    */
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

    /*
        -----------------------------------------------------------------
    */

    // work with camera
    let aspect_ratio: f32 = 16.0 / 9.0;
    
    let image_width: u16 = 400;
    let image_height: u16 = (image_width as f32 / aspect_ratio) as u16;

    let focal_length: f32 = 1.0;
    let viewport_height: f32 = 2.0;
    let viewport_width: f32 = viewport_height * (image_width as f32 / image_height as f32);

    let camera_center = Point3::new();

    let viewport_u = Vec3::with((
        viewport_width, 0.0, 0.0
    ));

    let viewport_v = Vec3::with((
        0.0, -viewport_height, 0.0
    ));

    let pixel_delta_u = viewport_u / image_width as f32;
    let pixel_delta_v = viewport_v / image_height as f32;

    let viewport_upper_left = camera_center
                             - Vec3::with((0.0, 0.0, focal_length)) 
                             - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

    // buffer -> file
    let mut buf: String = String::with_capacity(500*1024*1024*8);
    buf += &format!("P3\n{image_width} {image_height}\n255\n");

    // first -> width, second -> height. Filling to right bottom
    for j in 0..image_height {
        // this is in log
        write!(handle, "\rScanlines remaining: {} ", image_height-j).unwrap();
        // and necessary flush stdout
        handle.flush().unwrap();

        for i in 0..image_width {
            let pixel_center = pixel00_loc + (pixel_delta_u * i as f32) + (pixel_delta_v * j as f32);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::with(&camera_center, &ray_direction);

            let pixel_color = ray_color(&r);
            
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
