use raytracer::image::api::render;
use std::env;
use std::f32::consts::PI;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 4 {
        println!(
            "Expected: ./raytracer <file_name> <width> <height>. Actual: {}",
            args.join(" ")
        )
    }

    render(
        &args[1],
        args[2].parse().unwrap(),
        args[3].parse().unwrap(),
        PI / 2.,
    )
}
