//! Defines different traits of graphs.

use crate::{EdgeId, IdType, NodeId};

/// A basic graph.
///
/// Graphs defining this trait can act as containers for nodes and edges.
/// Their functionality is very limited though, as not even navigation is defined.
pub trait Graph<'a, N, E> {
    /// An iterator over all nodes of a graph.
    type NodeIterator: Iterator<Item = NodeId> + 'a;
    /// An iterator over all edges of a graph.
    type EdgeIterator: Iterator<Item = EdgeId> + 'a;

    /// The amount of nodes in the graph.
    fn node_len(&self) -> IdType;

    /// The amount of edges in the graph.
    fn edge_len(&self) -> IdType;

    /// Returns an iterator over all nodes in the graph.
    fn node_iter(&self) -> Self::NodeIterator;

    /// Returns an iterator over all edges in the graph.
    fn edge_iter(&self) -> Self::EdgeIterator;

    /// Returns a reference to a nodes data, identified by the given id.
    fn node_data(&self, id: NodeId) -> &N;

    /// Returns a reference to an edges data, identified by the given id.
    fn edge_data(&self, id: EdgeId) -> &E;

    /// Returns an edge instance, identified by the given id.
    fn edge(&self, id: EdgeId) -> EdgeRef<'a, E>;

    /// Returns the start node of the edge identified by the given id.
    fn edge_start(&self, id: EdgeId) -> NodeId;

    /// Returns the end node of the edge identified by the given id.
    fn edge_end(&self, id: EdgeId) -> NodeId;
}

/// A forward navigable graph.
///
/// Graphs implementing this trait are able to output a nodes out-edges efficiently.
/// For undirected graphs, out-edges and in-edges are the same.
pub trait ForwardNavigableGraph<'a, N, E>: Graph<'a, N, E> {
    /// An iterator over the out-edges of a node.
    type OutEdgeIterator: Iterator<Item = EdgeId> + 'a;

    /// Returns an iterator over the out-edges of the node identified by the given id.
    fn out_edges(&self, id: NodeId) -> Self::OutEdgeIterator;
}

/// A backward navigable graph.
///
/// Graphs implementing this trait are able to output a nodes in-edges efficiently.
/// For undirected graphs, out-edges and in-edges are the same.
pub trait BackwardNavigableGraph<'a, N, E>: Graph<'a, N, E> {
    /// An iterator over the in-edges of a node.
    type InEdgeIterator: Iterator<Item = EdgeId> + 'a;

    /// Returns an iterator over the in-edges of the node identified by the given id.
    fn in_edges(&self, id: NodeId) -> Self::InEdgeIterator;
}

/// A mutable graph.
///
/// Graphs implementing this trait are able to be updated efficiently.
pub trait MutableGraph<N, E> {
    /// Creates a new empty graph.
    fn new() -> Self;

    /// Adds the given node to the graph.
    /// The return value is the id assigned to the new node.
    fn add_node(&mut self, node: Node<N>) -> NodeId;

    /// Adds the given edge to the graph.
    /// The return value is the id assigned to the new edge, or an error, if the edge refers a non-existing node.
    fn add_edge(&mut self, edge: Edge<E>) -> Result<EdgeId, GraphModificationError>;
}

/// An error type for graph modifications.
/// This type is used by the `MutableGraph` trait.
pub enum GraphModificationError {
    /// An edge that refers to a nonexistent start node was added to the graph
    StartNodeDoesNotExist,
    /// An edge that refers to a nonexistent end node was added to the graph
    EndNodeDoesNotExist,
}

/// A container for a node.
/// Can be used to add nodes to a `MutableGraph`.
#[derive(Debug)]
pub struct Node<N> {
    data: N,
}

/// A container for an edge.
/// Can be used to add nodes to a `MutableGraph`.
#[derive(Debug)]
pub struct Edge<E> {
    start: NodeId,
    end: NodeId,
    data: E,
}

/// A container for an edge.
/// Is returned by `Graph` when a complete edge instance is requested.
#[derive(Debug)]
pub struct EdgeRef<'a, E> {
    start: NodeId,
    end: NodeId,
    data: &'a E,
}

impl<N> Node<N> {
    /// Creates a new node with the given node data.
    pub fn new(data: N) -> Self {
        Self { data }
    }
}

impl<E> Edge<E> {
    /// Creates a new edge with the given edge data.
    pub fn new(start: NodeId, end: NodeId, data: E) -> Self {
        Self { start, end, data }
    }

    /// Returns the id of the start node of this edge.
    pub fn start(&self) -> NodeId {
        self.start
    }

    /// Returns the id of the end node of this edge.
    pub fn end(&self) -> NodeId {
        self.end
    }
}

impl<'a, E> EdgeRef<'a, E> {
    /// Creates a new edge ref with the given edge data.
    /// This method should not be used by the client.
    // TODO Change to crate visibility once stable
    pub fn new(start: NodeId, end: NodeId, data: &'a E) -> Self {
        Self { start, end, data }
    }

    /// Returns the id of the start node of this edge.
    pub fn start(&self) -> NodeId {
        self.start
    }

    /// Returns the id of the end node of this edge.
    pub fn end(&self) -> NodeId {
        self.end
    }

    /// Returns a reference to the data of this edge.
    pub fn data(&self) -> &'a E {
        self.data
    }
}