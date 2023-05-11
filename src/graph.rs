use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use super::version::Version;

type RcNode<E, N> = Rc<RawNode<E, N>>;
type WeakNode<E, N> = Weak<RawNode<E, N>>;

/// Node handle
#[derive(Debug)]
pub struct Node<E, N> {
    version: Rc<Version<Edge<E, N>>>,
    raw: Rc<RawNode<E, N>>,
}

impl<E, N> Node<E, N> {
    pub fn new(data: N) -> Self {
        Self {
            version: Default::default(),
            raw: RawNode::new(data),
        }
    }
    pub fn add_child(&mut self, child: &mut Node<E, N>, value: E) {
        self.version = Rc::new(child.version.merge(&self.version));
        let edge = Edge {
            data: Rc::new(value),
            parent: self.raw.clone(),
            child: child.raw.clone(),
            version: Rc::downgrade(&self.version),
        };
        let edge = unsafe { child.version.migrate(edge) };
        self.raw.edges.borrow_mut().push(edge);
    }
    pub fn get(&self) -> &N {
        &self.raw.data
    }
    pub fn clone(&mut self) -> Self {
        let (a, b) = self.version.fork();
        self.version = a;
        Self {
            raw: Rc::new(RawNode {
                data: self.raw.data.clone(),
                edges: self.raw.edges.clone(),
            }),
            version: b,
        }
    }
    pub fn parent(&self) -> ParentsIterator<'_, E, N> {
        ParentsIterator { node: &self, i: 0 }
    }
    pub fn children(&self) -> ChildrenIterator<'_, E, N> {
        ChildrenIterator { node: &self, i: 0 }
    }
}

#[derive(Debug)]
pub struct ParentsIterator<'a, E, N> {
    node: &'a Node<E, N>,
    i: usize,
}

impl<'a, E, N> Iterator for ParentsIterator<'a, E, N> {
    type Item = (Rc<E>, Node<E, N>);

    fn next(&mut self) -> Option<Self::Item> {
        let version = &self.node.version;
        let edges = self.node.raw.edges.borrow();
        while self.i < edges.len() {
            let edge = &edges[self.i];
            self.i += 1;

            if let None = edge.upgrade() {
                continue;
            }

            let edge = edge.upgrade().unwrap();
            if !version.is_derivative_of(&edge.version.upgrade().unwrap()) {
                continue;
            }
            if !Rc::ptr_eq(&self.node.raw, &edge.child) {
                continue;
            }

            let node = Node {
                version: version.clone(),
                raw: edge.child.clone(),
            };
            return Some((edge.data.clone(), node));
        }
        None
    }
}

#[derive(Debug)]
pub struct ChildrenIterator<'a, E, N> {
    node: &'a Node<E, N>,
    i: usize,
}

impl<'a, E, N> Iterator for ChildrenIterator<'a, E, N> {
    type Item = (Rc<E>, Node<E, N>);

    fn next(&mut self) -> Option<Self::Item> {
        let version = &self.node.version;
        let edges = self.node.raw.edges.borrow();
        while self.i < edges.len() {
            let edge = &edges[self.i];
            self.i += 1;

            if let None = edge.upgrade() {
                continue;
            }

            let edge = edge.upgrade().unwrap();
            if !version.is_derivative_of(&edge.version.upgrade().unwrap()) {
                continue;
            }
            if !Rc::ptr_eq(&self.node.raw, &edge.parent) {
                continue;
            }

            let node = Node {
                version: version.clone(),
                raw: edge.parent.clone(),
            };
            return Some((edge.data.clone(), node));
        }
        None
    }
}

#[derive(Debug)]
struct RawNode<E, N> {
    data: Rc<N>,                           // immutable
    edges: RefCell<Vec<Weak<Edge<E, N>>>>, // mutable
}

impl<E, N> RawNode<E, N> {
    fn new(data: N) -> Rc<Self> {
        Rc::new(Self {
            data: Rc::new(data),
            edges: Default::default(),
        })
    }
}

#[derive(Clone, Debug)]
struct Edge<E, N> {
    data: Rc<E>,
    parent: Rc<RawNode<E, N>>,
    child: Rc<RawNode<E, N>>,
    version: Weak<Version<Edge<E, N>>>,
}
