use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use num_complex::Complex;

use crate::render::Render;

pub struct JuliaSet {
    width: u32,
    height: u32,
    scale: f32,
}

impl JuliaSet {
    pub fn new(width: u32, height: u32, scale: f32) -> Self {
        Self {
            width,
            height,
            scale,
        }
    }
}

impl Render for JuliaSet {
    fn render(&self, x: u32, y: u32) -> image::Rgb<u8> {
        let cx = x as f32 * (self.scale / self.width as f32) - (self.scale / 2.0);
        let cy = y as f32 * (self.scale / self.height as f32) - (self.scale / 2.0);

        let c = Complex::new(-0.5, 0.6);
        let z_start = Complex::new(cx, cy);

        let i = (0..u8::MAX)
            .into_iter()
            .enumerate()
            .fold_while((0, z_start), |acc, (i, _)| match acc.1 * acc.1 + c {
                z if z.norm() <= 2.0 => Continue((i, z)),
                _ => Done(acc),
            })
            .into_inner()
            .0 as f32;

        let pixel = if i < 24.0 {
            image::Rgb([4 * i as u8, 2 * i as u8, 4 * i as u8])
        } else {
            image::Rgb([
                (255_f32 * i.exp().sin()) as u8,
                (128_f32 * i.cos()) as u8,
                (255_f32 * i.tan()) as u8,
            ])
        };

        println!("rgb@{}:{} = {:?}", x, y, pixel);

        pixel
    }
}
