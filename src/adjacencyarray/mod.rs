//! Defines the adjacency array graph representation.
//!
//! This is a compact static graph representation that is often the most efficient solution if updates to the topology are rare.

use crate::{graph::Graph, simplegraph::SimpleGraph, util::PrefixSum, EdgeId, NodeId};
use std::convert::TryInto;

/// A graph represented as adjacency array.
pub struct AdjacencyArray<N, E> {
    first_out: Vec<EdgeId>,
    edge_ends: Vec<NodeId>,
    node_data: Vec<N>,
    edge_data: Vec<E>,
}

impl<N: Clone, E: Default + Clone> From<&SimpleGraph<N, E>> for AdjacencyArray<N, E> {
    fn from(source: &SimpleGraph<N, E>) -> Self {
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
            .node_iter()
            .map(|id| source.node_data(id).clone())
            .collect();
        let mut edge_data = vec![E::default(); edge_len];

        for (id, edge) in source.edge_iter().map(|id| source.edge(id)).enumerate() {
            let count_index: usize = (edge.start().id + 2)
                .try_into()
                .expect("Node id out of bounds");
            assert!(count_index < first_out.len(), "Count index out of bounds");
            first_out[count_index].id += 1;
        }

        first_out.prefix_sum();

        for edge in source.edge_iter().map(|id| source.edge(id)) {
            let node_index: usize = (edge.start().id + 1)
                .try_into()
                .expect("Node id out of bounds");
            assert!(
                node_index < first_out.len() - 1,
                "Lookup index out of bounds"
            );
            let raw_edge_index = &mut first_out[node_index].id;
            let edge_index: usize = (*raw_edge_index)
                .try_into()
                .expect(("Edge id out of bounds"));
            edge_ends[edge_index] = edge.end();
            edge_data[edge_index] = edge.data().clone();
            *raw_edge_index += 1;
        }

        first_out.pop();

        Self {
            first_out,
            edge_ends,
            node_data,
            edge_data,
        }
    }
}
