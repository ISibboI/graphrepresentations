use graphrepresentations::{
    adjacencyarray::AdjacencyArray,
    graph::{Edge, EdgeRef, Graph, MutableGraph, Node},
    simplegraph::SimpleGraph,
};

#[test]
fn test_simple_graph_construction() {
    let mut simple_graph = SimpleGraph::new();
    let n1 = simple_graph.add_node(Node::new(4));
    let n2 = simple_graph.add_node(Node::new(5));
    assert_ne!(n1, n2);
    let e1 = simple_graph.add_edge(Edge::new(n1, n2, 'x')).unwrap();

    let mut node_iter = simple_graph.node_id_iter();
    assert_eq!(node_iter.next(), Some(n1));
    assert_eq!(node_iter.next(), Some(n2));
    assert_eq!(node_iter.next(), None);

    let mut edge_iter = simple_graph.edge_id_iter();
    assert_eq!(edge_iter.next(), Some(e1));
    assert_eq!(edge_iter.next(), None);

    assert_eq!(simple_graph.node_data(n1), &4);
    assert_eq!(simple_graph.node_data(n2), &5);
    assert_eq!(simple_graph.edge(e1), EdgeRef::new(n1, n2, &'x'));
}

#[test]
fn test_adjacency_array_construction_simple_example() {
    let mut simple_graph = SimpleGraph::new();
    let n1 = simple_graph.add_node(Node::new(4));
    let n2 = simple_graph.add_node(Node::new(5));
    simple_graph.add_edge(Edge::new(n1, n2, 'x')).unwrap();
    let adjacency_array = AdjacencyArray::from(&simple_graph);

    let sg_node_iter = simple_graph.node_id_iter();
    let aa_node_iter = adjacency_array.node_id_iter();

    for (sg, aa) in sg_node_iter.zip(aa_node_iter) {
        assert_eq!(simple_graph.node_data(sg), adjacency_array.node_data(aa));
    }
    assert_eq!(simple_graph.node_len(), adjacency_array.node_len());

    let sg_edge_iter = simple_graph.edge_id_iter();
    let aa_edge_iter = adjacency_array.edge_id_iter();

    for (sg, aa) in sg_edge_iter.zip(aa_edge_iter) {
        assert_eq!(simple_graph.edge(sg), adjacency_array.edge(aa));
    }
    assert_eq!(simple_graph.edge_len(), adjacency_array.edge_len());
}