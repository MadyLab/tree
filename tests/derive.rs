macro_rules! check_children {
    ($c:expr,$edge:expr,$node:expr) => {
        let result: Vec<(&str, &str)> = $c
            .children()
            .map(|(edge, node)| ((*edge).clone(), node.get().clone()))
            .collect();
        assert!(result.contains(&($edge, $node)));
    };
    ($c:expr,$len:expr) => {
        let result: Vec<(&str, &str)> = $c
            .children()
            .map(|(edge, node)| ((*edge).clone(), node.get().clone()))
            .collect();
        assert!(result.len() == $len);
    };
}

macro_rules! check_parents {
    ($c:expr,$edge:expr,$node:expr) => {
        let result: Vec<(&str, &str)> = $c
            .parents()
            .map(|(edge, node)| ((*edge).clone(), node.get().clone()))
            .collect();
        assert!(result.contains(&($edge, $node)));
    };
    ($c:expr,$len:expr) => {
        let result: Vec<(&str, &str)> = $c
            .parents()
            .map(|(edge, node)| ((*edge).clone(), node.get().clone()))
            .collect();
        assert!(result.len() == $len);
    };
}

use btr_async::prelude::*;
#[cfg(test)]
#[test]
fn main() {
    let mut node1: Node<&str, &str> = Node::new("n1");
    let mut node2: Node<&str, &str> = Node::new("n2");
    let mut node3: Node<&str, &str> = Node::new("n3");

    node1.add_child_sync(&mut node2, "e1 2");
    node1.add_child_sync(&mut node3, "e1 3");

    node1.optimize();
    check_children!(node1, "e1 2", "n2");
    check_children!(node1, "e1 3", "n3");
    check_children!(node1, 2);
    check_parents!(node1, 0);

    check_children!(node2, 0);
    check_parents!(node2, "e1 2", "n1");
    check_parents!(node2, 1);

    check_children!(node3, 0);
    check_parents!(node3, "e1 3", "n1");
    check_parents!(node3, 1);
}
