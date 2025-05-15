const IMAGE_HEIGHT: i32 = 256;
const IMAGE_WIDTH: i32 = 256;

// first we must do ppm image from out stdout
fn main() {
    println!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255");

    let mut r: f64 = 0.0;
    let mut g: f64 = 0.0;
    let mut b: f64 = 0.0;

    for j in 0..IMAGE_HEIGHT {
        for i in 0..IMAGE_WIDTH {
            r = i as f64 / (IMAGE_WIDTH-1) as f64;
            g = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            b = 0.0;

            println!("{} {} {}", (r * 255.999) as u8, (g * 255.999) as u8, (b * 255.999) as u8)
        }
    }
}
