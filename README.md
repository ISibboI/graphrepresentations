# Graph Representations

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

let mut node_iter = adjacency_array.node_iter();
let mut edge_iter = adjacency_array.edge_iter();
assert_eq!(node_iter.next(), Some(n1)); // The order of the nodes is guaranteed to stay the same
assert_eq!(node_iter.next(), Some(n2));
assert_eq!(node_iter.next(), None);
assert_eq!(adjacency_array.edge(edge_iter.next().expect("Edge was not converted correctly")), simple_graph.edge(e1));
assert_eq!(edge_iter.next(), None);
```

If you are missing a feature or found a bug, please open an issue on [github](https://github.com/ISibboI/graphrepresentations/issues).