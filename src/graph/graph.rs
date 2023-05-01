use std::rc::Rc;

use super::{version::Version, arena::Arena};

#[derive(Clone)]
struct DataNode<T>{
    data:Rc<T>
}

#[derive(Clone)]
struct Node<'a,E,N>{
    data:DataNode<N>,
    edges:Vec<Edge<'a,E,N>>,
}

#[derive(Clone)]
struct Edge<'a,E,N>{
    data:DataNode<E>,
    version:Version<'a>,
    node:Node<'a,E,N>
}

struct GraphGuard<'a,E,N>{
    arena:Arena<Node<'a,E,N>>,
}