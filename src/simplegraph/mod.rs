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
//! let e1 = simple_graph.add_edge(Edge::new(n1, n2, 'c')).unwrap_or_else(|error| panic!("The edge refers nonexistent nodes: {:?}", error));
//! let adjacency_array = AdjacencyArray::from(&simple_graph);
//!
//! let mut node_iter = adjacency_array.node_id_iter();
//! let mut edge_iter = adjacency_array.edge_id_iter();
//! assert_eq!(node_iter.next(), Some(n1)); // The order of the nodes is guaranteed to stay the same
//! assert_eq!(node_iter.next(), Some(n2));
//! assert_eq!(node_iter.next(), None);
//! assert_eq!(adjacency_array.edge(edge_iter.next().expect("Edge was not converted correctly")), simple_graph.edge(e1));
//! assert_eq!(edge_iter.next(), None);
//! ```

use crate::{
    graph::{Edge, EdgeRef, Graph, GraphModificationError, MutableGraph, Node},
    simplegraph::iterators::{SimpleGraphEdgeIdIterator, SimpleGraphNodeIdIterator},
    EdgeId, IdType, NodeId,
};
use std::{borrow::Borrow, convert::TryInto};

pub mod iterators;

/// A simple graph representation that is inefficient to use, but cheap to construct.
///
/// For actual usage, the graph should be converted into a different representation.
#[derive(Debug)]
pub struct SimpleGraph<N, E> {
    nodes: Vec<Node<N>>,
    edges: Vec<Edge<E>>,
}

impl<N, E> Graph<N, E> for SimpleGraph<N, E> {
    type NodeIdIterator = SimpleGraphNodeIdIterator;
    type EdgeIdIterator = SimpleGraphEdgeIdIterator;

    fn node_len(&self) -> IdType {
        self.nodes.len().try_into().expect("Node len out of range")
    }

    fn edge_len(&self) -> IdType {
        self.edges.len().try_into().expect("Edge len out of range")
    }

    fn node_id_iter(&self) -> Self::NodeIdIterator {
        (0..self.node_len()).map(|id| NodeId::new(id))
    }

    fn edge_id_iter(&self) -> Self::EdgeIdIterator {
        (0..self.edge_len()).map(|id| EdgeId::new(id))
    }

    fn node_data(&self, id: NodeId) -> &N {
        assert!(self.is_node_id_valid(id));
        self.nodes[<NodeId as Into<usize>>::into(id)].data()
    }

    fn edge_data(&self, id: EdgeId) -> &E {
        assert!(self.is_edge_id_valid(id));
        self.edges[<EdgeId as Into<usize>>::into(id)].data()
    }

    fn edge(&self, id: EdgeId) -> EdgeRef<E> {
        assert!(self.is_edge_id_valid(id));
        self.edges[<EdgeId as Into<usize>>::into(id)]
            .borrow()
            .into()
    }

    fn edge_start(&self, id: EdgeId) -> NodeId {
        assert!(self.is_edge_id_valid(id));
        self.edges[<EdgeId as Into<usize>>::into(id)].start()
    }

    fn edge_end(&self, id: EdgeId) -> NodeId {
        assert!(self.is_edge_id_valid(id));
        self.edges[<EdgeId as Into<usize>>::into(id)].end()
    }

    fn is_node_id_valid(&self, id: NodeId) -> bool {
        id.is_valid() && id.id < self.node_len()
    }

    fn is_edge_id_valid(&self, id: EdgeId) -> bool {
        id.is_valid() && id.id < self.edge_len()
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
        if !edge.start().is_valid() || edge.start().id >= self.node_len() {
            return Err(GraphModificationError::StartNodeDoesNotExist);
        } else if !edge.end().is_valid() || edge.end().id >= self.node_len() {
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
