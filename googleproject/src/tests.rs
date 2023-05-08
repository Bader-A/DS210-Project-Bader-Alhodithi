use std::collections::HashMap;
use crate::{calc_degree, Node};
use super::*;
use std::cell::RefCell;

#[test]
fn test_dfs() {
    let graph_data = &[
        (1, 2),
        (1, 3),
        (2, 3),
        (3, 4),
        (4, 5),
        (4, 6),
        (5, 6),
        (6, 7),
    ];

    let graph = create_test_graph(graph_data);
    let mut visited = std::collections::HashSet::new();
    let result = RefCell::new(Vec::new());

    dfs(&graph, 1, &mut visited, &mut |from, to| {
        result.borrow_mut().push((from, to));
        Ok(())
    }).unwrap();

    assert_eq!(result.borrow().len(), 8);
    assert!(!result.borrow().contains(&(7, 8)));
}





fn create_test_graph(edges: &[(Node, Node)]) -> HashMap<Node, Vec<Node>> {
    let mut graph = HashMap::new();
    for (n1, n2) in edges {
        graph.entry(*n1).or_insert_with(Vec::new).push(*n2);
        graph.entry(*n2).or_insert_with(Vec::new).push(*n1);
    }
    graph
}

#[test]
fn test_graph_properties() {
    let graph_data = &[
        (1, 2),
        (1, 3),
        (1, 4),
        (2, 3),
        (2, 5),
        (3, 4),
        (3, 5),
        (4, 5),
    ];

    let graph = create_test_graph(graph_data);

    let degree_centrality = calc_degree(&graph);

    assert!(degree_centrality.contains(&(1, 3)));
    assert!(degree_centrality.contains(&(2, 3)));
    assert!(degree_centrality.contains(&(3, 4)));
    assert!(degree_centrality.contains(&(4, 3)));
    assert!(degree_centrality.contains(&(5, 3)));

}
