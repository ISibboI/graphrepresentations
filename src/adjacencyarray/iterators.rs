//! Iterator types for the `AdjacencyArray`.

use crate::{EdgeId, IdType, NodeId};
use crate::graph::EdgeRef;

/// An iterator over the nodes of an `AdjacencyArray`.
pub type AdjacencyArrayNodeIdIterator =
    std::iter::Map<std::ops::Range<IdType>, fn(IdType) -> NodeId>;
/// An iterator over the edges of an `AdjacencyArray`.
pub type AdjacencyArrayEdgeIdIterator =
    std::iter::Map<std::ops::Range<IdType>, fn(IdType) -> EdgeId>;

/// An iterator over the out-edges of a node.
pub struct OutEdgeIterator<'a, E> {
    start: NodeId,
    edge_ends: &'a [NodeId],
    edge_data: &'a [E],
}

impl<'a, E> Iterator for OutEdgeIterator<'a, E> {
    type Item = EdgeId;

    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!()
    }
}