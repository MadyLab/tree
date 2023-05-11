use btr_async::prelude::*;
#[cfg(test)]
#[test]
fn main() {
    let mut node1: Node<&str, &str> = Node::new("n1");
    let mut node2: Node<&str, &str> = Node::new("n2");
    let mut node3: Node<&str, &str> = Node::new("n3");

    node1.add_child(&mut node2, "e1 2");
    node1.add_child(&mut node3, "e1 3");

    let result: Vec<(&str, &str)> = node1
        .children()
        .map(|(edge, node)| ((*edge).clone(), node.get().clone()))
        .collect();
    assert!(result.contains(&("e1 2", "n2")));
    assert!(result.contains(&("e1 3", "n3")));

    let result: Vec<(&str, &str)> = node1
        .parents()
        .map(|(edge, node)| ((*edge).clone(), node.get().clone()))
        .collect();
    assert!(result.len() == 0);

    let result: Vec<(&str, &str)> = node2
        .children()
        .map(|(edge, node)| ((*edge).clone(), node.get().clone()))
        .collect();
    assert!(result.len() == 0);

    let result: Vec<(&str, &str)> = node2
        .parents()
        .map(|(edge, node)| ((*edge).clone(), node.get().clone()))
        .collect();
    assert!(result.contains(&("e1 2", "n1")));

    let result: Vec<(&str, &str)> = node3
        .children()
        .map(|(edge, node)| ((*edge).clone(), node.get().clone()))
        .collect();
    assert!(result.len() == 0);

    let result: Vec<(&str, &str)> = node3
        .parents()
        .map(|(edge, node)| ((*edge).clone(), node.get().clone()))
        .collect();
    assert!(result.contains(&("e1 3", "n1")));
}
