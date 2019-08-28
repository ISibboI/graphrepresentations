//! A mutable graph representation.
//!
//! While cheap to update, this representation should not be used in any algorithms.
//! It does not implement any of the more interesting feature traits like `NavigableGraph`.
//!
//! The `SimpleGraphs` main use is to aid construction of static graph representations.
//!
//! * Example
//!
//! Constructing an `AdjacencyArray` using a `SimpleGraph`.
//!
//! ```
//! use graphrepresentations::simplegraph::SimpleGraph;
//! use graphrepresentations::graph::{MutableGraph, Node, Edge, Graph};
//! use graphrepresentations::adjacencyarray::AdjacencyArray;
//!
//! let mut simple_graph = SimpleGraph::new();
//! let n1 = simple_graph.add_node(Node::new(5));
//! let n2 = simple_graph.add_node(Node::new(7));
//! let e1 = simple_graph.add_edge(Edge::new(n1, n2, 'c')).unwrap_or_else(|error| panic!("The edge refers nonexistent nodes: {}", error));
//! let adjacency_array = AdjacencyArray::from(&simple_graph);
//!
//! let mut node_iter = adjacency_array.node_iter();
//! let mut edge_iter = adjacency_array.edge_iter();
//! assert_eq!(node_iter.next(), Some(n1)); // The order of the nodes is guaranteed to stay the same
//! assert_eq!(node_iter.next(), Some(n2));
//! assert_eq!(node_iter.next(), None);
//! assert_eq!(edge_iter.next(), Some(simple_graph.edge(e1)));
//! assert_eq!(edge_iter.next(), None);
//! ```

use crate::{
    graph::{Edge, EdgeRef, Graph, GraphModificationError, MutableGraph, Node},
    simplegraph::iterators::{SimpleGraphEdgeIterator, SimpleGraphNodeIterator},
    EdgeId, IdType, NodeId,
};
use std::{convert::TryInto, iter::Map, ops::Range};

pub mod iterators;

/// A simple graph representation that is inefficient to use, but cheap to construct.
///
/// For actual usage, the graph should be converted into a different representation.
#[derive(Debug)]
pub struct SimpleGraph<N, E> {
    nodes: Vec<Node<N>>,
    edges: Vec<Edge<E>>,
}

impl<'a, N, E: 'a> Graph<'a, N, E> for SimpleGraph<N, E> {
    type NodeIterator = SimpleGraphNodeIterator;
    type EdgeIterator = SimpleGraphEdgeIterator;

    fn node_len(&self) -> IdType {
        self.nodes.len().try_into().expect("Node len out of range")
    }

    fn edge_len(&self) -> IdType {
        self.edges.len().try_into().expect("Edge len out of range")
    }

    fn node_iter(&self) -> Self::NodeIterator {
        (0..self.node_len().try_into().expect("Node id out of bounds")).map(|id| NodeId::new(id))
    }

    fn edge_iter(&self) -> Self::EdgeIterator {
        unimplemented!()
    }

    fn node_data(&self, id: NodeId) -> &N {
        unimplemented!()
    }

    fn edge_data(&self, id: EdgeId) -> &E {
        unimplemented!()
    }

    fn edge(&self, id: EdgeId) -> EdgeRef<'a, E> {
        unimplemented!()
    }

    fn edge_start(&self, id: EdgeId) -> NodeId {
        unimplemented!()
    }

    fn edge_end(&self, id: EdgeId) -> NodeId {
        unimplemented!()
    }
}

impl<N, E> MutableGraph<N, E> for SimpleGraph<N, E> {
    fn new() -> Self {
        Default::default()
    }

    fn add_node(&mut self, node: Node<N>) -> NodeId {
        self.nodes.push(node);
        NodeId::new(
            (self.nodes.len() - 1)
                .try_into()
                .expect("Node id out of bounds"),
        )
    }

    fn add_edge(&mut self, edge: Edge<E>) -> Result<EdgeId, GraphModificationError> {
        if edge.start().is_valid() || edge.start().id >= self.node_len() {
            return Err(GraphModificationError::StartNodeDoesNotExist);
        } else if edge.end().is_valid() || edge.end().id >= self.node_len() {
            return Err(GraphModificationError::EndNodeDoesNotExist);
        }

        self.edges.push(edge);
        Ok(EdgeId::new(
            (self.edges.len() - 1)
                .try_into()
                .expect("Edge id out of bounds"),
        ))
    }
}

impl<N, E> Default for SimpleGraph<N, E> {
    fn default() -> Self {
        SimpleGraph {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }
}
