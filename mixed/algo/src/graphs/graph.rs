use crate::graphs::edge::Edge;
use pyo3::prelude::*;
use pyo3_stub_gen::derive::*;
use std::collections::HashSet;

#[gen_stub_pyclass_enum]
#[pyclass(eq, eq_int)]
#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub enum GraphType {
    OneDirectional,
    BiDirectional,
}

#[gen_stub_pyclass]
#[pyclass]
#[derive(Eq, PartialEq, Debug)]
pub struct Graph {
    nodes: HashSet<usize>,
    edges: HashSet<Edge>,
    graph_type: GraphType,
}

#[gen_stub_pymethods]
#[pymethods]
impl Graph {
    #[new]
    pub fn new(graph_type: GraphType) -> Self {
        Self {
            nodes: HashSet::new(),
            edges: HashSet::new(),
            graph_type,
        }
    }

    pub fn add_node(&mut self, node: usize) {
        self.nodes.insert(node);
    }

    pub fn add_edge(&mut self, edge: Edge) {
        match self.graph_type {
            GraphType::OneDirectional => {
                self.edges.insert(edge);
            }
            GraphType::BiDirectional => {
                self.edges.insert(edge);
                self.edges.insert(edge.reverse());
            }
        }

        self.nodes.insert(edge.from_node);
        self.nodes.insert(edge.to_node);
    }

    pub fn add_edge_by_coordinates(&mut self, from_node: usize, to_node: usize) {
        self.add_edge(Edge::new(from_node, to_node));
    }

    pub fn get_nodes(&self) -> &HashSet<usize> {
        &self.nodes
    }

    pub fn get_edges(&self) -> HashSet<Edge> {
        //TODO: rethink strategy for this
        self.edges.clone()
    }

    pub fn get_edges_from_node(&self, node: usize) -> HashSet<Edge> {
        //TODO: rethink cloned for this
        self.edges
            .iter()
            .filter(|edge| edge.from_node == node)
            .cloned()
            .collect()
    }

    pub fn get_edges_to_node(&self, node: usize) -> HashSet<Edge> {
        //TODO: rethink cloned for this
        self.edges
            .iter()
            .filter(|edge| edge.to_node == node)
            .cloned()
            .collect()
    }

    pub fn get_node_neighbours(&self, node: usize) -> HashSet<usize> {
        self.edges
            .iter()
            .filter(|edge| edge.from_node == node)
            .map(|edge| edge.to_node)
            .collect()
    }

    pub fn is_node_exists(&self, node: usize) -> bool {
        self.nodes.contains(&node)
    }

    pub fn is_edge_exists(&self, edge: Edge) -> bool {
        self.edges.contains(&edge)
    }

    pub fn is_edge_exists_by_coordinates(&self, from_node: usize, to_node: usize) -> bool {
        self.edges.contains(&Edge::new(from_node, to_node))
    }

    pub fn remove_edge(&mut self, edge: Edge) {
        match self.graph_type {
            GraphType::OneDirectional => {
                self.edges.remove(&edge);
            }
            GraphType::BiDirectional => {
                self.edges.remove(&edge);
                self.edges.remove(&edge.reverse());
            }
        }
    }

    pub fn remove_edge_by_coordinates(&mut self, from_node: usize, to_node: usize) {
        self.remove_edge(Edge::new(from_node, to_node));
    }

    pub fn remove_node(&mut self, node: usize) {
        self.nodes.remove(&node);
        self.edges
            .retain(|edge| edge.from_node != node && edge.to_node != node);
    }
}
