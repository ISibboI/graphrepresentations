//! Iterator types for the `AdjacencyArray`.

use crate::{
    graph::{Edge, Node},
    EdgeId, IdType, NodeId,
};

/// An iterator over the nodes of an `AdjacencyArray`.
pub type AdjacencyArrayNodeIterator = std::iter::Map<std::ops::Range<IdType>, fn(IdType) -> NodeId>;
/// An iterator over the edges of an `AdjacencyArray`.
pub type AdjacencyArrayEdgeIterator = std::iter::Map<std::ops::Range<IdType>, fn(IdType) -> EdgeId>;
