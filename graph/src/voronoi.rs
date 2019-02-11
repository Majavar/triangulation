use crate::graph::{Graph, GraphEdge, GraphFace, GraphVertex};
use crate::Delaunay;
use crate::{Point, Vector};
use either::{Left, Right};
use std::hint::unreachable_unchecked;
use std::ops::Deref;

#[derive(Debug)]
pub struct Voronoi(Graph<Vector>);

impl Deref for Voronoi {
    type Target = Graph<Vector>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<&Delaunay> for Voronoi {
    fn from(delaunay: &Delaunay) -> Self {
        let edges = delaunay
            .edges
            .iter()
            .map(|e| GraphEdge::new(e.face, e.next ^ 1, e.vertex))
            .collect::<Vec<_>>();
        let faces = delaunay
            .vertices
            .iter()
            .map(|v| GraphFace::new(v.edge))
            .collect::<Vec<_>>();
        let mut points = Vec::with_capacity(delaunay.faces.len());
        let mut vertices = Vec::with_capacity(delaunay.faces.len());

        delaunay.faces.iter().for_each(|f| {
            let e0 = f.edge;
            let e1 = delaunay.edges[e0].next ^ 1;
            let e2 = delaunay.edges[e1].next ^ 1;

            let v0 = delaunay.edges[e0].vertex;
            let v1 = delaunay.edges[e1].vertex;
            let v2 = delaunay.edges[e2].vertex;

            let p0 = delaunay.vertices[v0].position;
            let p1 = delaunay.vertices[v1].position;
            let p2 = delaunay.vertices[v2].position;

            match (p0, p1, p2) {
                (Left(i0), Left(i1), Left(i2)) => {
                    let i = points.len();
                    let c = Point::circumcenter(
                        &delaunay.points[i0],
                        &delaunay.points[i1],
                        &delaunay.points[i2],
                    );

                    points.push(c);
                    vertices.push(GraphVertex::new(e0, Left(i)));
                }
                (Left(i0), Left(i1), _) => {
                    let normal = Vector::new(
                        delaunay.points[i0].y - delaunay.points[i1].y,
                        delaunay.points[i1].x - delaunay.points[i0].x,
                    );

                    vertices.push(GraphVertex::new(e0, Right(normal)));
                }
                _ => unsafe { unreachable_unchecked() },
            }
        });

        Voronoi(Graph {
            points,
            edges,
            faces,
            vertices,
        })
    }
}
