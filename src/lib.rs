//! * Graph Representations
//!
//! This crate provides different representations for graphs, some of which are efficient to use,
//! and others that are efficient to construct.
//!
//! The crate supports conversion between the different representations while preserving node ids.
//!
//! If you are missing a feature or found a bug, please open an issue on [github](https://github.com/ISibboI/graphrepresentations/issues).

#![deny(missing_docs)]

use std::convert::TryInto;

pub mod adjacencyarray;
pub mod graph;
pub mod simplegraph;
mod util;

///////////////////////////////
///// IDENTIFIERS /////////////
///////////////////////////////

/// The internal type used for node and edge ids.
pub type IdType = u32;

/// Identifies a node in a graph.
///
/// This struct cannot be instantiated or modified by the client.
#[derive(Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub struct NodeId {
    id: IdType,
}

/// Identifies an edge in a graph.
///
/// This struct cannot be instantiated or modified by the client.
#[derive(Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub struct EdgeId {
    id: IdType,
}

impl NodeId {
    fn new(id: IdType) -> Self {
        let result = NodeId { id };
        assert_ne!(result, Self::invalid(), "Node id out of bounds");
        result
    }

    fn invalid() -> Self {
        NodeId {
            id: IdType::max_value(),
        }
    }

    /// Checks if this `NodeId` is valid.
    /// Does not account for id changes due to graph modifications.
    pub fn is_valid(&self) -> bool {
        #![allow(unused_comparisons)]
        self.id >= 0 && *self != Self::invalid()
    }
}

impl EdgeId {
    fn new(id: IdType) -> Self {
        let result = EdgeId { id };
        assert_ne!(result, Self::invalid(), "Edge id out of bounds");
        result
    }

    fn invalid() -> Self {
        EdgeId {
            id: IdType::max_value(),
        }
    }

    /// Checks if this `EdgeId` is valid.
    /// Does not account for id changes due to graph modifications.
    pub fn is_valid(&self) -> bool {
        #![allow(unused_comparisons)]
        self.id >= 0 && *self != Self::invalid()
    }
}

impl std::fmt::Debug for NodeId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "N{}", self.id)
    }
}

impl std::fmt::Debug for EdgeId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "E{}", self.id)
    }
}

impl From<NodeId> for usize {
    fn from(id: NodeId) -> Self {
        id.id.try_into().unwrap_or_else(|_| panic!("Node id not compatible with usize: {:?}", id))
    }
}

impl From<EdgeId> for usize {
    fn from(id: EdgeId) -> Self {
        id.id.try_into().unwrap_or_else(|_| panic!("Edge id not compatible with usize: {:?}", id))
    }
}

impl From<usize> for NodeId {
    fn from(id: usize) -> Self {
        NodeId::new(id.try_into().unwrap_or_else(|_| panic!("Node id not compatible with usize: {:?}", id)))
    }
}

impl From<usize> for EdgeId {
    fn from(id: usize) -> Self {
        EdgeId::new(id.try_into().unwrap_or_else(|_| panic!("Edge id not compatible with usize: {:?}", id)))
    }
}