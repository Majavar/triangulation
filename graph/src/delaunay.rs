use crate::delaunator::Delaunator;
use crate::Graph;
use crate::Point;
use std::iter::FromIterator;
use std::ops::Deref;

#[derive(Debug)]
pub struct Delaunay(Graph<()>);

impl Deref for Delaunay {
    type Target = Graph<()>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Delaunay {
    #[inline]
    pub fn from(points: Vec<Point>) -> Result<Delaunay, ()> {
        let len = points.len() + 1;

        let mut edges = Vec::with_capacity(len * 6);
        let mut faces = Vec::with_capacity(len * 2);
        let mut vertices = Vec::with_capacity(len);

        Delaunator::new(&points, &mut edges, &mut faces, &mut vertices).process()?;

        Ok(Delaunay(Graph {
            points,
            edges,
            faces,
            vertices,
        }))
    }
}

impl FromIterator<Point> for Result<Delaunay, ()> {
    #[inline]
    fn from_iter<I: IntoIterator<Item = Point>>(iter: I) -> Self {
        let points = iter.into_iter().collect::<Vec<_>>();
        Delaunay::from(points)
    }
}
