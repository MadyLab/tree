# Graph

A graph-like data structure to simplify clone on DAG.

## Get Started

Node store ``Version`` and data to find correspond ``RawNode``

``Version`` update after some operation(``clone``,``add_child``) to ensure clones of its ancestor is invisible its clone, and any change before the node been added to child is visible to its parents.

An basic example is shown below:

```rust
let mut node1: Node<&str, &str> = Node::new("n1");
let mut node2: Node<&str, &str> = Node::new("n2 or 3");

node1.add_child_sync(&mut node2, "e1 2");

let mut node3 = node2.clone();

let mut node4: Node<&str, &str> = Node::new("n4");
node3.add_child_sync(&mut node4, "e3 4");
```

In the example, we call ``add_child_sync`` instead of ``add_child`` to not only make the new edge visible to its parents but also visible to itself.



