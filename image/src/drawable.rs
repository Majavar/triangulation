use crate::{Rgb, RgbImage};
use either::{Left, Right};
use graph::{Edge, Face, Graph, Vector, Vertex};
use imageproc::{drawing, point};
use std::fmt::Debug;

pub trait Drawable {
    fn draw(&self, image: &mut RgbImage);
}

impl<'a, T> Drawable for &'a Graph<T>
where
    T: Debug + Copy,
    Edge<'a, T>: Drawable,
    Face<'a, T>: Drawable,
    Vertex<'a, T>: Drawable,
{
    fn draw(&self, image: &mut RgbImage) {
        self.faces().for_each(|f| f.draw(image));
        self.edges().for_each(|e| e.draw(image));
        self.vertices().for_each(|v| v.draw(image));
    }
}

impl Drawable for Edge<'_, ()> {
    fn draw(&self, image: &mut RgbImage) {
        let black = Rgb([192u8, 192u8, 192u8]);
        let (w, h) = image.dimensions();

        let (v1, v2) = self.vertices();
        if let (Left(p1), Left(p2)) = (v1.position(), v2.position()) {
            drawing::draw_line_segment_mut(
                image,
                (p1.x as f32 * w as f32, p1.y as f32 * h as f32),
                (p2.x as f32 * w as f32, p2.y as f32 * h as f32),
                black,
            );
        };
    }
}

impl Drawable for Edge<'_, Vector> {
    fn draw(&self, image: &mut RgbImage) {
        let black = Rgb([0u8, 0u8, 0u8]);
        let red = Rgb([255u8, 0u8, 0u8]);
        let (w, h) = image.dimensions();

        let (v1, v2) = self.vertices();

        match (v1.position(), v2.position()) {
            (Left(p1), Left(p2)) => {
                drawing::draw_line_segment_mut(
                    image,
                    (p1.x as f32 * w as f32, p1.y as f32 * h as f32),
                    (p2.x as f32 * w as f32, p2.y as f32 * h as f32),
                    black,
                );
            }
            (Left(p1), Right(v)) => {
                let p2 = *p1 + *v;

                drawing::draw_line_segment_mut(
                    image,
                    (p1.x as f32 * w as f32, p1.y as f32 * h as f32),
                    (p2.x as f32 * w as f32, p2.y as f32 * h as f32),
                    red,
                );
            }
            _ => {}
        }
    }
}

impl Drawable for Face<'_, ()> {
    fn draw(&self, image: &mut RgbImage) {
        let color = Rgb([0u8, 255u8, 255u8]);
        let (w, h) = image.dimensions();

        let pos = self
            .vertices()
            .filter_map(|p| p.position().left())
            .map(|p| {
                (
                    (p.x as f32 * w as f32) as i32,
                    (p.y as f32 * h as f32) as i32,
                )
            })
            .collect::<Vec<_>>();

        let distinct = (0..pos.len())
            .filter(|&i| i == 0 || pos[0] != pos[i])
            .map(|i| point::Point::new(pos[i].0, pos[i].1))
            .collect::<Vec<_>>();

        if distinct.len() > 2 {
            drawing::draw_polygon_mut(image, distinct.as_slice(), color);
        }
    }
}

impl Drawable for Face<'_, Vector> {
    fn draw(&self, _image: &mut RgbImage) {}
}

impl<T: Debug + Copy> Drawable for Vertex<'_, T> {
    fn draw(&self, image: &mut RgbImage) {
        let black = Rgb([0u8, 0u8, 0u8]);
        let (w, h) = image.dimensions();

        if let Left(p) = self.position() {
            drawing::draw_filled_circle_mut(
                image,
                ((p.x * f64::from(w)) as i32, (p.y * f64::from(h)) as i32),
                2,
                black,
            );
        }
    }
}

impl<'a, T> Drawable for (&'a Graph<()>, &'a Graph<T>)
where
    T: Debug + Copy,
    Edge<'a, T>: Drawable,
    Face<'a, T>: Drawable,
    Vertex<'a, T>: Drawable,
{
    fn draw(&self, image: &mut RgbImage) {
        self.0.draw(image);
        self.1.draw(image);
    }
}
