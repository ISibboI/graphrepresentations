# Graph Representations

[![Project Status: WIP – Initial development is in progress, but there has not yet been a stable, usable release suitable for the public.](https://www.repostatus.org/badges/latest/wip.svg)](https://www.repostatus.org/#wip)
[![](http://meritbadge.herokuapp.com/graphrepresentations)](https://crates.io/crates/graphrepresentations)
[![](https://docs.rs/graphrepresentations/badge.svg)](https://docs.rs/graphrepresentations)

This crate provides different representations for graphs, some of which are efficient to use,
and others that are efficient to construct.

The crate supports conversion between the different representations while preserving node ids.

## Quick Start Examples

Creating an adjacency array.

```rust
use graphrepresentations::simplegraph::SimpleGraph;
use graphrepresentations::graph::{MutableGraph, Node, Edge, Graph};
use graphrepresentations::adjacencyarray::AdjacencyArray;

let mut simple_graph = SimpleGraph::new();
let n1 = simple_graph.add_node(Node::new(5));
let n2 = simple_graph.add_node(Node::new(7));
let e1 = simple_graph.add_edge(Edge::new(n1, n2, 'c')).unwrap_or_else(|error| panic!("The edge refers nonexistent nodes: {:?}", error));
let adjacency_array = AdjacencyArray::from(&simple_graph);

let mut node_iter = adjacency_array.node_id_iter();
let mut edge_iter = adjacency_array.edge_id_iter();
assert_eq!(simple_graph.node_data(node_iter.next().expect("Node got lost")), adjacency_array.node_data(n1)); // The order of the nodes is guaranteed to stay the same
assert_eq!(simple_graph.node_data(node_iter.next().expect("Node got lost")), adjacency_array.node_data(n2));
assert_eq!(node_iter.next(), None);
assert_eq!(adjacency_array.edge(edge_iter.next().expect("Edge was not converted correctly")), simple_graph.edge(e1));
assert_eq!(edge_iter.next(), None);
```

Navigating that same adjacency array.

```rust
// The same adjacency array as above
let mut neighbors = adjacency_array.out_edges(n1);
assert_eq!(adjacency_array.edge(neighbors.next().expect("Edge was not converted correctly")), simple_graph.edge(e1));
assert_eq!(neighbors.next(), None);
```

## Graph Traits

 * `Graph` The basic trait for graphs.
   It cannot do much more than acting as an unmodifiable container.
   Supported are iteration of node and edge ids and lookup of node and edge data by id.
 * `MutableGraph` A trait for mutable graphs.
   Graphs implementing this trait are supposed to allow efficient mutations.
   At the moment, the trait only requires `add`-methods.
 * `ForwardNavigableGraph` A graph that can be navigated forward.
   It requires the method `out_edges` that returns an iterator over all out-edges of a node.
 * `IterableGraph` A graph that supports efficient iteration of complete node and edge data.
   This is not implemented and blocked by [#29661](https://github.com/rust-lang/rust/issues/29661).
   
## Graph Representations

At the moment, this crate supports only two graph representations.
One dynamic, and one static.

 * `SimpleGraph: Graph + MutableGraph` A dynamic graph representation, that allows efficient modification, but is not very useful to implement any algorithms.
 * `AdjacencyArray: Graph + ForwardNavigableGraph` A static graph representation that is efficient to use in graph algorithms, but inefficient to modify.
   At the moment, modification needs to be done by rebuilding it from a `SimpleGraph`. 

## Ids Explained

This crate uses ids to refer to nodes and edges.
This comes natural when implementing adjacency arrays or edge lists.
It is a fast and easy solution to pass around node and edge data in client code without having to worry about borrowing.
Ids are 32 bit (or maybe also 64 bit in the future), so passing them around is basically zero-cost.

**Keep in mind** that ids are not bound in any way to the graph instance that created them.
There is no protection against using an id for one graph in another, resulting in unpredictable behavior.
**But** using the node ids of one graph instance with a converted version of that same graph is guaranteed to work consistently, if none of the graphs were modified after conversion.

##

If you are missing a feature or found a bug, please open an issue on [github](https://github.com/ISibboI/graphrepresentations/issues).