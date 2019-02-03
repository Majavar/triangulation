use crate::Point;
use std::iter::{successors, FromIterator};

#[derive(Debug)]
pub struct GraphEdge {
    pub vertex: usize,
    pub next: usize,
    pub face: usize,
}

impl GraphEdge {
    #[inline]
    pub fn new(vertex: usize, next: usize, face: usize) -> GraphEdge {
        GraphEdge { vertex, next, face }
    }
}

#[derive(Debug)]
pub struct GraphFace {
    pub edge: usize,
}

impl GraphFace {
    #[inline]
    pub fn new(edge: usize) -> GraphFace {
        GraphFace { edge }
    }
}

#[derive(Debug)]
pub struct GraphVertex {
    pub edge: usize,
    pub position: Option<usize>,
}

impl GraphVertex {
    #[inline]
    pub fn new(edge: usize, position: Option<usize>) -> GraphVertex {
        GraphVertex { edge, position }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Edge<'a> {
    graph: &'a Graph,
    id: usize,
}

impl<'a> Edge<'a> {
    #[inline]
    pub fn id(&self) -> usize {
        self.id / 2
    }

    #[inline]
    pub fn vertices(&self) -> (Vertex<'a>, Vertex<'a>) {
        let id = self.id;

        let left = Vertex {
            graph: &self.graph,
            id: self.graph.edges[id].vertex,
        };

        let right = Vertex {
            graph: &self.graph,
            id: self.graph.edges[id ^ 1].vertex,
        };

        (left, right)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Face<'a> {
    graph: &'a Graph,
    id: usize,
}

impl<'a> Face<'a> {
    #[inline]
    pub fn id(&self) -> usize {
        self.id
    }

    #[inline]
    pub fn edges(&self) -> impl Iterator<Item = Edge<'_>> {
        let edge = self.graph.faces[self.id].edge;

        successors(Some(self.graph.edge(edge)), move |p| {
            let n = self.graph.edges[p.id].next ^ 1;
            if edge == n {
                None
            } else {
                Some(self.graph.edge(n))
            }
        })
    }

    #[inline]
    pub fn vertices(&self) -> impl Iterator<Item = Vertex<'_>> {
        self.edges().map(|edge| edge.vertices().0)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Vertex<'a> {
    graph: &'a Graph,
    id: usize,
}

impl<'a> Vertex<'a> {
    #[inline]
    pub fn id(&self) -> usize {
        self.id
    }

    #[inline]
    pub fn position(&self) -> Option<Point> {
        self.graph.vertices[self.id]
            .position
            .map(|id| self.graph.points[id])
    }
}

#[derive(Debug)]
pub struct Graph {
    points: Vec<Point>,

    edges: Vec<GraphEdge>,
    faces: Vec<GraphFace>,
    vertices: Vec<GraphVertex>,
}

impl Graph {
    #[inline]
    pub fn from(points: Vec<Point>) -> Result<Graph, ()> {
        let mut delaunator = crate::delaunator::Delaunator::new(points);
        delaunator.process()?;
        Ok(delaunator.into())
    }

    #[inline]
    pub fn edge_count(&self) -> usize {
        self.edges.len() / 2
    }

    #[inline]
    pub fn face_count(&self) -> usize {
        self.faces.len()
    }

    #[inline]
    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    #[inline]
    pub fn edge(&self, id: usize) -> Edge<'_> {
        Edge { graph: &self, id }
    }

    #[inline]
    pub fn face(&self, id: usize) -> Face<'_> {
        Face { graph: &self, id }
    }

    #[inline]
    pub fn vertex(&self, id: usize) -> Vertex<'_> {
        Vertex { graph: &self, id }
    }

    #[inline]
    pub fn edges(&self) -> impl Iterator<Item = Edge<'_>> {
        (0..self.edge_count()).map(move |id| self.edge(id << 1))
    }

    #[inline]
    pub fn faces(&self) -> impl Iterator<Item = Face<'_>> {
        (0..self.face_count()).map(move |id| self.face(id))
    }

    #[inline]
    pub fn vertices(&self) -> impl Iterator<Item = Vertex<'_>> {
        (0..self.vertex_count()).map(move |id| self.vertex(id))
    }
}

impl FromIterator<Point> for Result<Graph, ()> {
    #[inline]
    fn from_iter<I: IntoIterator<Item = Point>>(iter: I) -> Self {
        let points = iter.into_iter().collect::<Vec<_>>();
        Graph::from(points)
    }
}

#[inline]
pub fn build_from(
    points: Vec<Point>,
    edges: Vec<GraphEdge>,
    faces: Vec<GraphFace>,
    vertices: Vec<GraphVertex>,
) -> Graph {
    Graph {
        points,
        edges,
        faces,
        vertices,
    }
}
