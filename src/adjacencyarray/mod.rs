//! Defines the adjacency array graph representation.
//!
//! This is a compact static graph representation that is often the most efficient solution if updates to the topology are rare.

use crate::{
    adjacencyarray::iterators::{AdjacencyArrayEdgeIdIterator, AdjacencyArrayNodeIdIterator},
    graph::{EdgeRef, Graph},
    simplegraph::SimpleGraph,
    util::PrefixSum,
    EdgeId, IdType, NodeId,
};
use std::convert::TryInto;
use superslice::Ext;
use crate::graph::ForwardNavigableGraph;

pub mod iterators;

/// A graph represented as adjacency array.
pub struct AdjacencyArray<N, E> {
    first_out: Vec<EdgeId>,
    edge_ends: Vec<NodeId>,
    node_data: Vec<N>,
    edge_data: Vec<E>,
}

impl<N, E> Graph<N, E> for AdjacencyArray<N, E> {
    type NodeIdIterator = AdjacencyArrayNodeIdIterator;
    type EdgeIdIterator = AdjacencyArrayEdgeIdIterator;

    fn node_len(&self) -> IdType {
        (self.first_out.len() - 1)
            .try_into()
            .unwrap_or_else(|_| panic!("Node len not compatible with usize"))
    }

    fn edge_len(&self) -> IdType {
        (self.edge_ends.len())
            .try_into()
            .unwrap_or_else(|_| panic!("Edge len not compatible with usize"))
    }

    fn node_id_iter(&self) -> Self::NodeIdIterator {
        (0..self.node_len()).map(|id| NodeId::new(id))
    }

    fn edge_id_iter(&self) -> Self::EdgeIdIterator {
        (0..self.edge_len()).map(|id| EdgeId::new(id))
    }

    fn node_data(&self, id: NodeId) -> &N {
        assert!(self.is_node_id_valid(id));
        &self.node_data[<NodeId as Into<usize>>::into(id)]
    }

    fn edge_data(&self, id: EdgeId) -> &E {
        assert!(self.is_edge_id_valid(id));
        &self.edge_data[<EdgeId as Into<usize>>::into(id)]
    }

    fn edge(&self, id: EdgeId) -> EdgeRef<E> {
        assert!(self.is_edge_id_valid(id));
        let start = self.edge_start(id);
        let end = self.edge_end(id);
        let data = self.edge_data(id);
        EdgeRef::new(start, end, data)
    }

    fn edge_start(&self, id: EdgeId) -> NodeId {
        assert!(self.is_edge_id_valid(id));
        (self.first_out.upper_bound(&id.into()) - 1).into()
    }

    fn edge_end(&self, id: EdgeId) -> NodeId {
        assert!(self.is_edge_id_valid(id));
        self.edge_ends[<EdgeId as Into<usize>>::into(id)]
    }

    fn is_node_id_valid(&self, id: NodeId) -> bool {
        id.is_valid() && id.id < self.node_len()
    }

    fn is_edge_id_valid(&self, id: EdgeId) -> bool {
        id.is_valid() && id.id < self.edge_len()
    }
}

impl<'a, N, E> ForwardNavigableGraph<'a, N, E> for AdjacencyArray<N, E> {
    type OutEdgeIterator = std::iter::Map<std::ops::Range<IdType>, fn(IdType) -> EdgeId>;

    fn out_edges(&self, id: NodeId) -> Self::OutEdgeIterator {
        assert!(self.is_node_id_valid(id));
        let node_index = <NodeId as Into<usize>>::into(id);
        let edge_id_offset = self.first_out[node_index].id;
        let edge_id_limit = self.first_out[node_index + 1].id;
        // TODO replace with Range<EdgeId> once Step API is stable (https://github.com/rust-lang/rust/issues/42168)
        (edge_id_offset..edge_id_limit).map(|id| EdgeId::new(id))
    }
}

fn convert_from<N: Clone, E: Default + Clone, G: Graph<N, E>>(source: &G) -> AdjacencyArray<N, E> {
    let node_len: usize = source
        .node_len()
        .try_into()
        .expect("Node len incompatible with usize");
    let edge_len: usize = source
        .edge_len()
        .try_into()
        .expect("Edge len incompatible with usize");
    let mut first_out = vec![EdgeId::new(0); node_len + 2];
    let mut edge_ends = vec![NodeId::invalid(); edge_len];
    let node_data: Vec<_> = source
        .node_id_iter()
        .map(|id| source.node_data(id).clone())
        .collect();
    let mut edge_data = vec![E::default(); edge_len];

    for edge in source.edge_id_iter().map(|id| source.edge(id)) {
        let count_index: usize = (edge.start().id + 2)
            .try_into()
            .expect("Node id out of bounds");
        assert!(count_index < first_out.len(), "Count index out of bounds");
        first_out[count_index].id += 1;
    }

    first_out.prefix_sum();

    for edge in source.edge_id_iter().map(|id| source.edge(id)) {
        let node_index: usize = (edge.start().id + 1)
            .try_into()
            .expect("Node id out of bounds");
        assert!(
            node_index < first_out.len() - 1,
            "Lookup index out of bounds"
        );
        let raw_edge_index = &mut first_out[node_index].id;
        let edge_index: usize = (*raw_edge_index).try_into().expect("Edge id out of bounds");
        edge_ends[edge_index] = edge.end();
        edge_data[edge_index] = edge.data().clone();
        *raw_edge_index += 1;
    }

    first_out.pop();

    AdjacencyArray {
        first_out,
        edge_ends,
        node_data,
        edge_data,
    }
}

impl<N: Clone, E: Default + Clone> From<&SimpleGraph<N, E>> for AdjacencyArray<N, E> {
    fn from(source: &SimpleGraph<N, E>) -> Self {
        convert_from(source)
    }
}
