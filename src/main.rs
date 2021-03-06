mod julia;
mod render;

use self::julia::JuliaSet;
use self::render::Render;
use image::ImageBuffer;
use std::f32::consts::PI;

const WIDTH: u32 = 4096;
const HEIGHT: u32 = 4096;

fn main() -> image::ImageResult<()> {
    let set = JuliaSet::new(WIDTH, HEIGHT, PI * 0.25);
    let img = ImageBuffer::from_fn(WIDTH, HEIGHT, |x, y| set.render(x, y));
    img.save("dist/julia.png")
}
