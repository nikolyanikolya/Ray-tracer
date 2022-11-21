use crate::image::sphere::cast_ray;
use crate::util::vec3::{PixelOperations, Vec3};
use image::{ImageBuffer, Rgb};

pub fn render(file_name: &String, width: u32, height: u32, fov: f32) {
    let mut framebuffer: Vec<Vec3> = Vec::with_capacity((width * height) as usize);
    framebuffer.resize(
        (width * height) as usize,
        Vec3 {
            x: 0.,
            y: 0.,
            z: 0.,
        },
    );

    for pixel in 0..height * width {
        let direction_x: f32 = ((pixel % width) as f32 + 0.5) - (width as f32) / 2.0;
        let direction_y: f32 = -((pixel as f32) / (width as f32) + 0.5) + (height as f32) / 2.0;
        let direction_z: f32 = -(height as f32) / (2.0 * (fov / 2.0).tan());
        framebuffer[pixel as usize] = cast_ray(
            &Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            &Vec3 {
                x: direction_x,
                y: direction_y,
                z: direction_z,
            }
            .normalized(),
            0,
        );
    }

    let img = ImageBuffer::from_fn(width as u32, height as u32, |x, y| {
        let pixel: usize = (y * width) as usize + (x as usize);
        let mut rgb = framebuffer[pixel];
        let max_color: f32 = rgb.y.max(rgb.z).max(rgb.x).max(1.);
        rgb = rgb * 255. * (1. / max_color);
        Rgb([rgb.x as u8, rgb.y as u8, rgb.z as u8])
    });

    img.save(file_name)
        .expect("Something went wrong when saving image to file");
}
