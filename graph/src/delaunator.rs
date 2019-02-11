use crate::graph::{GraphEdge, GraphFace, GraphVertex};
use crate::point::Point;
use either::{Left, Right};
use nalgebra::distance_squared;

use std::hint::unreachable_unchecked;

#[cfg(feature = "use-rayon")]
use rayon::prelude::*;

#[derive(Debug)]
pub struct Delaunator<'a> {
    points: &'a [Point],

    edges: &'a mut Vec<GraphEdge>,
    faces: &'a mut Vec<GraphFace>,
    vertices: &'a mut Vec<GraphVertex<()>>,
}

impl<'a> Delaunator<'a> {
    #[inline]
    pub fn new(
        points: &'a [Point],
        edges: &'a mut Vec<GraphEdge>,
        faces: &'a mut Vec<GraphFace>,
        vertices: &'a mut Vec<GraphVertex<()>>,
    ) -> Self {
        Delaunator {
            points,
            edges,
            faces,
            vertices,
        }
    }

    #[inline]
    fn circumradius(&self, v0: usize, v1: usize, v2: usize) -> f64 {
        Point::circumradius(&self.points[v0], &self.points[v1], &self.points[v2])
    }

    #[inline]
    fn circumcenter(&self, v0: usize, v1: usize, v2: usize) -> Point {
        Point::circumcenter(&self.points[v0], &self.points[v1], &self.points[v2])
    }

    #[inline]
    fn is_ccw(&self, v0: usize, v1: usize, v2: usize) -> bool {
        Point::is_ccw(&self.points[v0], &self.points[v1], &self.points[v2])
    }

    #[inline]
    fn nearly_equals(&self, v0: usize, v1: usize) -> bool {
        Point::nearly_equals(&self.points[v0], &self.points[v1])
    }

    #[inline]
    fn in_circle(&self, v0: usize, v1: usize, v2: usize, p: usize) -> bool {
        Point::in_circle(
            &self.points[v0],
            &self.points[v1],
            &self.points[v2],
            &self.points[p],
        )
    }

    #[cfg(feature = "use-rayon")]
    #[inline]
    fn calc_bounding_box_center(&self) -> Option<Point> {
        self.points
            .par_iter()
            .map(|p| (p.x, p.y, p.x, p.y))
            .reduce_with(
                |(lminx, lminy, lmaxx, lmaxy), (rminx, rminy, rmaxx, rmaxy)| {
                    let minx = if lminx < rminx { lminx } else { rminx };
                    let maxx = if lmaxx > rmaxx { lmaxx } else { rmaxx };
                    let miny = if lminy < rminy { lminy } else { rminy };
                    let maxy = if lmaxy > rmaxy { lmaxy } else { rmaxy };

                    (minx, miny, maxx, maxy)
                },
            )
            .map(|(minx, miny, maxx, maxy)| Point::new((minx + maxx) / 2., (miny + maxy) / 2.))
    }

    #[cfg(not(feature = "use-rayon"))]
    #[inline]
    fn calc_bounding_box_center(&self) -> Option<Point> {
        let mut iter = self.points.iter();

        iter.next()
            .map(|first| {
                iter.fold(
                    (first.x, first.x, first.y, first.y),
                    |(x1, x2, y1, y2), p| {
                        let (minx, maxx) = if p.x < x1 {
                            (p.x, x2)
                        } else if p.x < x2 {
                            (x1, x2)
                        } else {
                            (x1, p.x)
                        };
                        let (miny, maxy) = if p.y < y1 {
                            (p.y, y2)
                        } else if p.y < y2 {
                            (y1, y2)
                        } else {
                            (y1, p.y)
                        };

                        (minx, maxx, miny, maxy)
                    },
                )
            })
            .map(|(minx, miny, maxx, maxy)| Point::new((minx + maxx) / 2., (miny + maxy) / 2.))
    }

    #[cfg(feature = "use-rayon")]
    #[inline]
    fn find_closest_to_position(&self, center: Point) -> Option<usize> {
        (0..(self.points.len()))
            .into_par_iter()
            .map(|id| (id, distance_squared(&*center, &*self.points[id])))
            .reduce_with(|(l, ld), (r, rd)| if ld < rd { (l, ld) } else { (r, rd) })
            .map(|(id, _)| id)
    }

    #[cfg(not(feature = "use-rayon"))]
    #[inline]
    fn find_closest_to_position(&self, center: Point) -> Option<usize> {
        let mut iter =
            (0..(self.points.len())).map(|id| (id, distance_squared(&*center, &*self.points[id])));

        iter.next()
            .map(|first| {
                iter.fold(
                    first,
                    |(l, ld), (r, rd)| if ld < rd { (l, ld) } else { (r, rd) },
                )
            })
            .map(|(id, _)| id)
    }

    #[cfg(feature = "use-rayon")]
    #[inline]
    fn find_closest_to_vertex(&self, index: usize) -> Option<(usize, usize)> {
        (0..(self.points.len()))
            .into_par_iter()
            .filter_map(|i| {
                if index == i || self.nearly_equals(index, i) {
                    None
                } else {
                    Some((i, distance_squared(&*self.points[index], &*self.points[i])))
                }
            })
            .reduce_with(|(l, ld), (r, rd)| if ld < rd { (l, ld) } else { (r, rd) })
            .map(|(id, _)| (index, id))
    }

    #[cfg(not(feature = "use-rayon"))]
    #[inline]
    fn find_closest_to_vertex(&self, index: usize) -> Option<(usize, usize)> {
        let mut iter = (0..(self.points.len())).filter_map(|i| {
            if index == i || self.nearly_equals(index, i) {
                None
            } else {
                Some((i, distance_squared(&*self.points[index], &*self.points[i])))
            }
        });

        iter.next()
            .map(|first| {
                iter.fold(
                    first,
                    |(l, ld), (r, rd)| if ld < rd { (l, ld) } else { (r, rd) },
                )
            })
            .map(|(id, _)| (index, id))
    }

    #[cfg(feature = "use-rayon")]
    #[inline]
    fn find_delaunay_triangle(&self, v1: usize, v2: usize) -> Option<(usize, usize, usize)> {
        (0..(self.points.len()))
            .into_par_iter()
            .filter_map(|i| {
                if self.nearly_equals(i, v1) || self.nearly_equals(i, v2) {
                    None
                } else {
                    Some((i, self.circumradius(v1, v2, i)))
                }
            })
            .reduce_with(|(l, ld), (r, rd)| if ld < rd { (l, ld) } else { (r, rd) })
            .map(|(id, _)| (v1, v2, id))
    }

    #[cfg(not(feature = "use-rayon"))]
    #[inline]
    fn find_delaunay_triangle(&self, v1: usize, v2: usize) -> Option<(usize, usize, usize)> {
        let mut iter = (0..(self.points.len())).filter_map(|i| {
            if v1 == i || v2 == i || self.nearly_equals(i, v1) || self.nearly_equals(i, v2) {
                None
            } else {
                Some((i, self.circumradius(v1, v2, i)))
            }
        });

        iter.next()
            .map(|first| {
                iter.fold(
                    first,
                    |(l, ld), (r, rd)| if ld < rd { (l, ld) } else { (r, rd) },
                )
            })
            .map(|(id, _)| (v1, v2, id))
    }

    #[inline]
    fn find_seed_triangle(&self) -> Option<(usize, usize, usize)> {
        self.calc_bounding_box_center()
            .and_then(|center| self.find_closest_to_position(center))
            .and_then(|vertex| self.find_closest_to_vertex(vertex))
            .and_then(|(v0, v1)| self.find_delaunay_triangle(v0, v1))
            .map(|(v0, v1, v2)| {
                if self.is_ccw(v0, v1, v2) {
                    (v0, v1, v2)
                } else {
                    (v0, v2, v1)
                }
            })
    }

    #[inline]
    fn add_seed_triangle(&mut self, v0: usize, v1: usize, v2: usize) {
        self.vertices.push(GraphVertex::new(3, Right(())));
        self.vertices.push(GraphVertex::new(0, Left(v2)));
        self.vertices.push(GraphVertex::new(7, Left(v1)));
        self.vertices.push(GraphVertex::new(1, Left(v0)));

        self.faces.push(GraphFace::new(0));
        self.faces.push(GraphFace::new(1));
        self.faces.push(GraphFace::new(6));
        self.faces.push(GraphFace::new(8));

        self.edges.push(GraphEdge::new(3, 2, 1));
        self.edges.push(GraphEdge::new(1, 6, 0));
        self.edges.push(GraphEdge::new(0, 9, 0));
        self.edges.push(GraphEdge::new(1, 4, 3));
        self.edges.push(GraphEdge::new(3, 10, 0));
        self.edges.push(GraphEdge::new(0, 1, 2));
        self.edges.push(GraphEdge::new(2, 5, 1));
        self.edges.push(GraphEdge::new(3, 8, 2));
        self.edges.push(GraphEdge::new(1, 11, 1));
        self.edges.push(GraphEdge::new(2, 0, 3));
        self.edges.push(GraphEdge::new(2, 3, 2));
        self.edges.push(GraphEdge::new(0, 7, 3));
    }

    #[inline]
    fn find_visible_edge(&self, position: usize) -> Option<(usize, bool)> {
        let initial = self.vertices[0].edge;
        let mut current = initial;
        let mut current_position = self.vertices[self.edges[current].vertex]
            .position
            .left()
            .unwrap_or_else(|| unsafe { unreachable_unchecked() });

        loop {
            let next = self.edges[current].next;
            let next_position = self.vertices[self.edges[next].vertex]
                .position
                .left()
                .unwrap_or_else(|| unsafe { unreachable_unchecked() });

            if !self.is_ccw(position, current_position, next_position) {
                break Some((current, current == initial));
            }

            current = next;
            current_position = next_position;

            if current == initial {
                break None;
            };
        }
    }

    #[inline]
    fn add_triangle(&mut self, vertex: usize, current_edge: usize, next_edge: usize) -> usize {
        let current_vertex = self.edges[current_edge].vertex;
        let next_vertex = self.edges[next_edge].vertex;
        let face = self.edges[next_edge].face;
        let opposite_edge = self.edges[next_edge ^ 1].next;

        let new_current_face = self.faces.len();
        let new_next_face = new_current_face + 1;

        let edge = self.edges.len();

        self.faces.push(GraphFace::new(edge));
        self.faces.push(GraphFace::new(edge + 4));

        self.edges
            .push(GraphEdge::new(vertex, current_edge ^ 1, face));
        self.edges
            .push(GraphEdge::new(current_vertex, edge + 4, new_current_face));
        self.edges.push(GraphEdge::new(0, edge + 1, new_next_face));
        self.edges
            .push(GraphEdge::new(vertex, next_edge, new_current_face));
        self.edges.push(GraphEdge::new(next_vertex, edge + 2, face));
        self.edges
            .push(GraphEdge::new(vertex, opposite_edge, new_next_face));

        self.edges[next_edge ^ 1].next = edge + 5;
        self.edges[current_edge].next = edge + 3;
        self.edges[opposite_edge ^ 1].next = edge;

        self.edges[current_edge ^ 1].face = new_current_face;
        self.edges[next_edge].face = new_next_face;

        self.vertices[vertex].edge = edge + 1;
        self.vertices[0].edge = edge + 3;

        opposite_edge
    }

    fn legalize(&mut self, t0e0: usize) {
        let t1e1 = self.edges[t0e0].next;
        let p = self.edges[t1e1].vertex;

        if p != 0 {
            let t1e0 = t0e0 ^ 1;
            let t0e1 = self.edges[t1e0].next;
            let t0e2 = self.edges[t0e1 ^ 1].next;

            let va = self.edges[t0e0].vertex;
            let vb = self.edges[t1e0].vertex;
            let v0 = self.edges[t0e1].vertex;
            let v1 = self.edges[t1e1].vertex;

            let p0 = self.vertices[v0]
                .position
                .left()
                .unwrap_or_else(|| unsafe { unreachable_unchecked() });
            let pa = self.vertices[va]
                .position
                .left()
                .unwrap_or_else(|| unsafe { unreachable_unchecked() });
            let pb = self.vertices[vb]
                .position
                .left()
                .unwrap_or_else(|| unsafe { unreachable_unchecked() });
            let p1 = self.vertices[v1]
                .position
                .left()
                .unwrap_or_else(|| unsafe { unreachable_unchecked() });

            if self.in_circle(p0, pa, pb, p1) {
                let t1e2 = self.edges[t1e1 ^ 1].next;
                let t0 = self.edges[t0e2].face;
                let t1 = self.edges[t1e2].face;

                self.vertices[va].edge = t0e1;
                self.vertices[vb].edge = t1e1;

                self.edges[t0e2 ^ 1].next = t1e1;
                self.edges[t1e2 ^ 1].next = t0e1;

                self.edges[t0e0].vertex = self.edges[t1e1].vertex;
                self.edges[t1e0].vertex = self.edges[t0e1].vertex;

                self.edges[t0e0].next = t0e2;
                self.edges[t1e0].next = t1e2;
                self.edges[t0e1 ^ 1].next = t0e0;
                self.edges[t1e1 ^ 1].next = t1e0;

                self.edges[t0e2].face = t1;
                self.edges[t1e2].face = t0;

                self.faces[t0].edge = t0e1 ^ 1;
                self.faces[t1].edge = t1e1 ^ 1;

                self.legalize(t1e1);
                self.legalize(t1e2);
            }
        }
    }

    pub fn process(&mut self) -> Result<(), ()> {
        let (i0, i1, i2) = self.find_seed_triangle().ok_or(())?;

        self.add_seed_triangle(i0, i1, i2);
        let center = self.circumcenter(i0, i1, i2);

        let mut dists = (0..self.points.len())
            .map(|i| (i, distance_squared(&*center, &*self.points[i])))
            .collect::<Vec<_>>();
        #[cfg(feature = "use-rayon")]
        dists.par_sort_unstable_by(|&(_, da), &(_, db)| da.partial_cmp(&db).unwrap());
        #[cfg(not(feature = "use-rayon"))]
        dists.sort_unstable_by(|&(_, da), &(_, db)| da.partial_cmp(&db).unwrap());

        for i in 3..dists.len() {
            let new_point = dists[i].0;
            if new_point == i0
                || new_point == i1
                || new_point == i2
                || self.nearly_equals(dists[i - 1].0, new_point)
            {
                continue;
            };

            if let Some((edge, walk_back)) = self.find_visible_edge(new_point) {
                let vertex = self.vertices.len();
                self.vertices.push(GraphVertex::new(99, Left(new_point)));

                let mut current = edge;
                let mut current_position;

                let mut next = self.edges[current].next;
                let mut next_vertex = self.edges[next].vertex;
                let mut next_position = self.vertices[next_vertex]
                    .position
                    .left()
                    .unwrap_or_else(|| unsafe { unreachable_unchecked() });

                let mut previous = self.edges[self.edges[current ^ 1].next ^ 1].next ^ 1;

                let e = self.add_triangle(vertex, current, next);
                self.legalize(e);

                let new_edge = self.vertices[0].edge;

                loop {
                    current = next;
                    current_position = next_position;

                    next = self.edges[current].next;
                    next_vertex = self.edges[next].vertex;
                    next_position = self.vertices[next_vertex]
                        .position
                        .left()
                        .unwrap_or_else(|| unsafe { unreachable_unchecked() });

                    if self.is_ccw(new_point, current_position, next_position) {
                        break;
                    };

                    let edge_1 = self.edges[next ^ 1].next;
                    let edge_2 = self.edges[current ^ 1].next;
                    let face_1 = self.edges[next].face;
                    let face_2 = self.edges[current].face;

                    self.edges[new_edge].next = next;

                    self.edges[edge_2].face = face_1;
                    self.faces[face_2].edge = current ^ 1;

                    self.edges[current].vertex = vertex;
                    self.edges[current ^ 1].vertex = self.edges[next].vertex;

                    self.edges[current].next = edge_1;
                    self.edges[current ^ 1].next = new_edge ^ 1;

                    self.edges[next].face = face_2;
                    self.edges[next ^ 1].next = current;
                    self.edges[edge_2 ^ 1].next = current ^ 1;
                    self.edges[edge_1 ^ 1].next = edge_2;

                    self.legalize(edge_1);
                }

                if walk_back {
                    let mut current = edge;
                    let current_vertex = self.edges[current].vertex;
                    let mut current_position = self.vertices[current_vertex]
                        .position
                        .left()
                        .unwrap_or_else(|| unsafe { unreachable_unchecked() });

                    let mut previous_vertex = self.edges[previous].vertex;
                    let mut previous_position = self.vertices[previous_vertex]
                        .position
                        .left()
                        .unwrap_or_else(|| unsafe { unreachable_unchecked() });

                    while !self.is_ccw(new_point, previous_position, current_position) {
                        let edge_1 = self.edges[new_edge ^ 1].next;
                        let edge_2 = self.edges[current ^ 1].next;
                        let face_1 = self.edges[new_edge].face;
                        let face_2 = self.edges[current].face;

                        self.edges[previous].next = new_edge;

                        self.edges[edge_2].face = face_1;
                        self.faces[face_2].edge = current ^ 1;

                        self.edges[current].vertex = self.edges[previous].vertex;
                        self.edges[current ^ 1].vertex = vertex;

                        self.edges[current].next = edge_1;
                        self.edges[current ^ 1].next = previous ^ 1;

                        self.edges[new_edge].face = face_2;
                        self.edges[new_edge ^ 1].next = current;
                        self.edges[edge_2 ^ 1].next = current ^ 1;
                        self.edges[edge_1 ^ 1].next = edge_2;

                        self.legalize(edge_2);

                        current = previous;
                        current_position = previous_position;

                        previous = self.edges[self.edges[current ^ 1].next ^ 1].next ^ 1;
                        previous_vertex = self.edges[previous].vertex;
                        previous_position = self.vertices[previous_vertex]
                            .position
                            .left()
                            .unwrap_or_else(|| unsafe { unreachable_unchecked() });
                    }
                }
            }
        }
        Ok(())
    }
}
