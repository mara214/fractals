pub trait Render {
    fn render(&self, x: u32, y: u32) -> image::Rgb<u8>;
}
