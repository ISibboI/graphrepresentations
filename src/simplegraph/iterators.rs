//! Iterator types for the `SimpleGraph`.

use crate::{EdgeId, IdType, NodeId};

/// An iterator over the nodes of a `SimpleGraph`.
pub type SimpleGraphNodeIdIterator = std::iter::Map<std::ops::Range<IdType>, fn(IdType) -> NodeId>;
/// An iterator over the edges of a `SimpleGraph`.
pub type SimpleGraphEdgeIdIterator = std::iter::Map<std::ops::Range<IdType>, fn(IdType) -> EdgeId>;
