use btr_async::prelude::*;
#[cfg(test)]
#[test]
fn main() {
    let mut node1: Node<&str, &str> = Node::new("n1");
    let mut node2: Node<&str, &str> = Node::new("n2");
    let mut node3: Node<&str, &str> = Node::new("n3");

    node1.add_child(&mut node2, "e1 2");
    node1.add_child(&mut node3, "e1 3");

    let result: Vec<&str> = node1.children().map(|(edge, _)| (*edge).clone()).collect();

    assert!(result.contains(&"e1 2"));
    assert!(result.contains(&"e1 3"));
}
