use btr_async::prelude::*;
#[cfg(test)]
#[test]
fn main() {
    let mut node1: Node<&str, &str> = Node::new("n1");
    let mut node2: Node<&str, &str> = Node::new("n2");
    let mut node3: Node<&str, &str> = Node::new("n3");

    node1.add_child(&mut node3,"e1 3");
    node2.add_child(&mut node3,"e2 3");

    let mut node4: Node<&str, &str> = Node::new("n4");
    node4.add_child(&mut node1, "e1 4");
    node4.add_child(&mut node2, "e2 4");

    let result: Vec<&str> = node3.parent().map(|(_, node)| (*node.parent().next().unwrap().0).clone()).collect();

    assert_eq!(result,vec!["n4","n4"]);
}