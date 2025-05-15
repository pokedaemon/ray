use crate::vec3::Vec3;

pub type Color = Vec3;

pub fn write_color(pixel_color: Color, buf: &mut String) {
    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();

    *buf += &format!("{} {} {}\n", (r * 255.999) as i32, (g * 255.999) as i32, (b * 255.999) as i32)
}