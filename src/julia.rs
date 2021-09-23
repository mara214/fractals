use std::f32::consts::PI;

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
            .0 as u8;

        if i < 24 {
            return image::Rgb([0, 0, 0]);
        }

        let r = (255 as f32 * (i as f32).exp().sin()) as u8;
        let g = (128 as f32 * (i as f32).cos()) as u8;
        let b = (255 as f32 * (i as f32).tan()) as u8;

        println!("rgb@{}/{} = ({},{},{})", x, y, r, g, b);

        image::Rgb([r, g, b])
    }
}
