use super::*;

#[test] 
fn test_graph_add_edge(){
    let mut graph = Graph::new();
    graph.add_edge(0, 1);
    graph.add_edge(0, 2);
    graph.add_edge(1, 2);
    graph.add_edge(2, 0);
    assert_eq!(graph.nodes.len(), 3);
    assert_eq!(graph.nodes.get(&0), Some(&vec![1, 2]));
    assert_eq!(graph.nodes.get(&1), Some(&vec![2]));
    assert_eq!(graph.nodes.get(&2), Some(&vec![0]));
}

#[test]
fn test_graph_in_degree() {
    let mut graph = Graph::new();
    graph.add_edge(0, 1);
    graph.add_edge(0, 2);
    graph.add_edge(1, 2);
    graph.add_edge(2, 0);
    assert_eq!(graph.in_degree(0), 1);
    assert_eq!(graph.in_degree(1), 1);
    assert_eq!(graph.in_degree(2), 2);
}

#[test]
fn test_graph_out_degree() {
    let mut graph = Graph::new();
    graph.add_edge(0, 1);
    graph.add_edge(0, 2);
    graph.add_edge(1, 2);
    graph.add_edge(2, 0);
    assert_eq!(graph.out_degree(0), 2);
    assert_eq!(graph.out_degree(1), 1);
    assert_eq!(graph.out_degree(2), 1);
}

#[test]
fn test_graph_deg_centrality() {
    let mut graph = Graph::new();
    graph.add_edge(0, 1);
    graph.add_edge(0, 2);
    graph.add_edge(1, 2);
    graph.add_edge(2, 0);
    let centrality = graph.deg_centrality();
    assert_eq!(centrality.len(), 3);
    assert_eq!(centrality.get(&0), Some(&1.5));
    assert_eq!(centrality.get(&1), Some(&1.0));
    assert_eq!(centrality.get(&2), Some(&1.5));
}

#[test]
fn test_shortest_path() {
    let mut graph = Graph::new();
    graph.add_edge(0, 1);
    graph.add_edge(1, 2);
    graph.add_edge(2, 3);
    graph.add_edge(3, 4);
    graph.add_edge(0, 4);

    assert_eq!(graph.shortest_path(0, 4), Some(vec![0, 4]));
    assert_eq!(graph.shortest_path(2, 4), Some(vec![2, 3, 4]));
    assert_eq!(graph.shortest_path(1, 3), Some(vec![1, 2, 3]));
    assert_eq!(graph.shortest_path(0, 2), Some(vec![0, 1, 2]));
    assert_eq!(graph.shortest_path(0, 3), Some(vec![0, 1, 2, 3]));
    assert_eq!(graph.shortest_path(0, 5), None);
    assert_eq!(graph.shortest_path(5, 1), None);
}