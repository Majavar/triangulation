use crate::drawable::Drawable;
use crate::{DynamicImage, Rgb, RgbImage};

pub trait ToImage {
    fn to_image(&self, width: u32, height: u32) -> DynamicImage;
}

impl<T> ToImage for T
where
    T: Drawable,
{
    fn to_image(&self, width: u32, height: u32) -> DynamicImage {
        let white = Rgb([255u8, 255u8, 255u8]);
        let mut image = RgbImage::from_pixel(width, height, white);

        self.draw(&mut image);
        DynamicImage::ImageRgb8(image)
    }
}
