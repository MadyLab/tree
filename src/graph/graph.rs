use std::rc::{Rc, Weak};

use super::{version::Version};

#[derive(Clone)]
struct Node<E, N> {
    data: Rc<N>,
    parent:*const Edge<E, N>,
    edges: Vec<*const Edge<E, N>>,
} 

#[derive(Clone)]
struct Edge< E, N> {
    data: Rc<E>,
    version: Version<Self>,
    parent:*const Node<E,N>,
    child: *const Node<E,N>,
}

