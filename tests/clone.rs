use btr_async::prelude::*;
#[cfg(test)]
#[test]
fn main() {
    let mut node1: Node<&str, &str> = Node::new("n1");
    let mut node2: Node<&str, &str> = Node::new("n2 or 3");

    node1.add_child(&mut node2, "e1 2");

    let mut node3 = node2.clone();

    let mut node4: Node<&str, &str> = Node::new("n4");
    node3.add_child(&mut node4, "e3 4");

    let node2_result: Vec<&str> = node2.children().map(|(edge, _)| (*edge).clone()).collect();
    assert!(node2_result.len() == 0);

    let node3_result: Vec<&str> = node3.children().map(|(edge, _)| (*edge).clone()).collect();
    assert!(node3_result.contains(&"e3 4"));
}
