//! Iterator types for the `AdjacencyArray`.

use crate::{EdgeId, IdType, NodeId};

/// An iterator over the nodes of an `AdjacencyArray`.
pub type AdjacencyArrayNodeIdIterator =
    std::iter::Map<std::ops::Range<IdType>, fn(IdType) -> NodeId>;
/// An iterator over the edges of an `AdjacencyArray`.
pub type AdjacencyArrayEdgeIdIterator =
    std::iter::Map<std::ops::Range<IdType>, fn(IdType) -> EdgeId>;