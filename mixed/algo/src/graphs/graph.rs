use crate::graphs::edge::Edge;
use pyo3::prelude::*;
use std::collections::HashSet;

#[pyclass]
#[derive(Eq, PartialEq, Debug)]
pub struct Graph {
    nodes: HashSet<usize>,
    edges: HashSet<Edge>,
}

#[pymethods]
impl Graph {
    #[new]
    pub fn new() -> Graph {
        Graph {
            nodes: HashSet::new(),
            edges: HashSet::new(),
        }
    }

    pub fn add_node(&mut self, node: usize) {
        self.nodes.insert(node);
    }

    pub fn add_edge(&mut self, edge: Edge) {
        self.edges.insert(edge);
    }

    pub fn add_edge_plain(&mut self, from: usize, to: usize) {
        self.add_edge(Edge::new(from, to));
    }

    pub fn add_edge_bidirectional(&mut self, edge: Edge) {
        self.add_edge(edge);
        self.add_edge(edge.reverse());
    }

    pub fn add_edge_bidirectional_plain(&mut self, from: usize, to: usize) {
        self.add_edge_bidirectional(Edge::new(from, to));
    }

    pub fn get_nodes(&self) -> &HashSet<usize> {
        &self.nodes
    }

    pub fn get_edges(&self) -> HashSet<Edge> {
        //TODO: rethink strategy for this
        self.edges.clone()
    }
}
