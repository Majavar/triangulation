use crate::{DynamicImage, Rgb, RgbImage};
use graph::Graph;
use imageproc::drawing;

pub trait ToImage {
    fn to_image(&self, width: u32, height: u32) -> DynamicImage;
}

impl ToImage for Graph {
    fn to_image(&self, width: u32, height: u32) -> DynamicImage {
        let black = Rgb([0u8, 0u8, 0u8]);
        let white = Rgb([255u8, 255u8, 255u8]);

        let light_cyan = Rgb([128u8, 255u8, 255u8]);
        let light_magenta = Rgb([255u8, 128u8, 255u8]);

        let mut image = RgbImage::from_pixel(width, height, white);
        let w = width as f32;
        let h = height as f32;

        self.faces().for_each(|f| {
            let vert = f.vertices().map(|v| v).collect::<Vec<_>>();
            if vert.iter().all(|v| v.position().is_some()) {
                let p = vert
                    .iter()
                    .map(|v| {
                        let pos = v.position().unwrap();
                        drawing::Point::new((pos.x as f32 * w) as i32, (pos.y as f32 * h) as i32)
                    })
                    .collect::<Vec<_>>();

                if f.id() % 2 == 0 {
                    drawing::draw_convex_polygon_mut(&mut image, p.as_slice(), light_cyan);
                } else {
                    drawing::draw_convex_polygon_mut(&mut image, p.as_slice(), light_magenta);
                }
            }
        });

        self.edges()
            .map(|edge| edge.vertices())
            .for_each(|(left, right)| {
                if let (Some(l), Some(r)) = (left.position(), right.position()) {
                    drawing::draw_line_segment_mut(
                        &mut image,
                        (l.x as f32 * w, l.y as f32 * h),
                        (r.x as f32 * w, r.y as f32 * h),
                        black,
                    );
                };
            });

        self.vertices().filter_map(|v| v.position()).for_each(|p| {
            drawing::draw_filled_circle_mut(
                &mut image,
                (
                    (p.x * f64::from(width)) as i32,
                    (p.y * f64::from(height)) as i32,
                ),
                2,
                black,
            );
        });

        DynamicImage::ImageRgb8(image)
    }
}
