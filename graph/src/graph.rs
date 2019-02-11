use crate::Point;
use either::Either;
use std::fmt::Debug;
use std::iter::successors;

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
pub struct GraphVertex<T: Debug + Copy> {
    pub edge: usize,
    pub position: Either<usize, T>,
}

impl<T: Debug + Copy> GraphVertex<T> {
    #[inline]
    pub fn new(edge: usize, position: Either<usize, T>) -> GraphVertex<T> {
        GraphVertex { edge, position }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Edge<'a, T: Debug + Copy> {
    graph: &'a Graph<T>,
    id: usize,
}

impl<'a, T: Debug + Copy> Edge<'a, T> {
    #[inline]
    pub fn id(&self) -> usize {
        self.id / 2
    }

    #[inline]
    pub fn vertices(&self) -> (Vertex<'a, T>, Vertex<'a, T>) {
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
pub struct Face<'a, T: Debug + Copy> {
    graph: &'a Graph<T>,
    id: usize,
}

impl<'a, T: Debug + Copy> Face<'a, T> {
    #[inline]
    pub fn id(&self) -> usize {
        self.id
    }

    #[inline]
    pub fn edges(&self) -> impl Iterator<Item = Edge<'_, T>> {
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
    pub fn vertices(&self) -> impl Iterator<Item = Vertex<'_, T>> {
        self.edges().map(|edge| edge.vertices().0)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Vertex<'a, T: Debug + Copy> {
    graph: &'a Graph<T>,
    id: usize,
}

impl<'a, T: Debug + Copy> Vertex<'a, T> {
    #[inline]
    pub fn id(&self) -> usize {
        self.id
    }

    #[inline]
    pub fn position(&self) -> Either<Point, T> {
        self.graph.vertices[self.id]
            .position
            .map_left(|id| self.graph.points[id])
    }
}

#[derive(Debug)]
pub struct Graph<T: Debug + Copy> {
    pub(crate) points: Vec<Point>,

    pub(crate) edges: Vec<GraphEdge>,
    pub(crate) faces: Vec<GraphFace>,
    pub(crate) vertices: Vec<GraphVertex<T>>,
}

impl<T: Debug + Copy> Graph<T> {
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
    pub fn edge(&self, id: usize) -> Edge<'_, T> {
        Edge { graph: &self, id }
    }

    #[inline]
    pub fn face(&self, id: usize) -> Face<'_, T> {
        Face { graph: &self, id }
    }

    #[inline]
    pub fn vertex(&self, id: usize) -> Vertex<'_, T> {
        Vertex { graph: &self, id }
    }

    #[inline]
    pub fn edges(&self) -> impl Iterator<Item = Edge<'_, T>> {
        (0..self.edge_count()).map(move |id| self.edge(id << 1))
    }

    #[inline]
    pub fn faces(&self) -> impl Iterator<Item = Face<'_, T>> {
        (0..self.face_count()).map(move |id| self.face(id))
    }

    #[inline]
    pub fn vertices(&self) -> impl Iterator<Item = Vertex<'_, T>> {
        (0..self.vertex_count()).map(move |id| self.vertex(id))
    }
}
