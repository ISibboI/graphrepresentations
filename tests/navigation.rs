use graphrepresentations::simplegraph::SimpleGraph;
use graphrepresentations::graph::{MutableGraph, Node, Edge, ForwardNavigableGraph, Graph, EdgeRef};
use graphrepresentations::adjacencyarray::AdjacencyArray;

#[test]
fn test_adjacency_array_navigation_simple_example() {
    let mut simple_graph = SimpleGraph::new();
    let n0 = simple_graph.add_node(Node::new('a'));
    let n1 = simple_graph.add_node(Node::new('b'));
    let n2 = simple_graph.add_node(Node::new('c'));
    let n3 = simple_graph.add_node(Node::new('d'));
    let n4 = simple_graph.add_node(Node::new('e'));
    simple_graph.add_edge(Edge::new(n0, n1, 1)).unwrap();
    simple_graph.add_edge(Edge::new(n1, n0, 2)).unwrap();
    simple_graph.add_edge(Edge::new(n2, n3, 5)).unwrap();
    simple_graph.add_edge(Edge::new(n1, n4, 3)).unwrap();
    simple_graph.add_edge(Edge::new(n1, n2, 4)).unwrap();
    simple_graph.add_edge(Edge::new(n3, n3, 6)).unwrap();
    let adjacency_array = AdjacencyArray::from(&simple_graph);

    let n1_out_edges: Vec<_> = adjacency_array.out_edges(n1).map(|id| adjacency_array.edge(id)).collect();
    assert_eq!(n1_out_edges, vec![EdgeRef::new(n1, n0, &2), EdgeRef::new(n1, n4, &3), EdgeRef::new(n1, n2, &4)]);
}