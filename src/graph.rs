use std::rc::{Rc, Weak};

use super::version::Version;

type RcNode<E, N> = Rc<RawNode<E, N>>;
type WeakNode<E, N> = Weak<RawNode<E, N>>;

// prevent RawNode(the holding one), Version from being drop
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
        child.version.merge(&self.version);
        let edge = Edge {
            data: Rc::new(value),
            parent: self.raw.clone(),
            child: child.raw.clone(),
            version: Rc::downgrade(&child.version),
        };
        let edge=unsafe { child.version.migrate(edge) };
        self.raw.edges.push(edge);
    }
    pub fn get(&self) -> &N {
        &self.raw.data
    }
    pub fn set(&mut self, data: N) {
        self.raw = Rc::new(RawNode {
            data: Rc::new(data),
            edges: self.raw.edges.clone(),
        });
    }
    pub fn clone(&mut self) -> Self {
        let (a, b) = self.version.fork();
        self.version = a;
        Self {
            raw: self.raw.clone(),
            version: b,
        }
    }
    pub fn parents(&self) -> impl Iterator + '_ {
        self.raw
            .edges
            .iter()
            .filter(|edge| edge.upgrade().is_some())
            .filter_map(|edge| {
                let edge = edge.upgrade().unwrap();
                let version = edge.version.upgrade().unwrap();
                if self.version.is_derivative_of(&version) && Rc::ptr_eq(&edge.child, &self.raw) {
                    Some((edge.data.clone(), edge.parent.clone()))
                } else {
                    None
                }
            })
    }
    pub fn children(&self) -> impl Iterator<Item = (Rc<E>, Node<E, N>)> + '_ {
        self.raw
            .edges
            .iter()
            .filter(|edge| edge.upgrade().is_some())
            .filter_map(|edge| {
                let edge = edge.upgrade().unwrap();
                let version = edge.version.upgrade().unwrap();
                if self.version.is_derivative_of(&version) && Rc::ptr_eq(&edge.parent, &self.raw) {
                    let node = Node {
                        version: self.version.clone(),
                        raw: edge.parent.clone(),
                    };
                    Some((edge.data.clone(), node))
                } else {
                    None
                }
            })
    }
}

struct RawNode<E, N> {
    data: Rc<N>,
    edges: Vec<Weak<Edge<E, N>>>,
}

impl<E, N> RawNode<E, N> {
    fn new(data: N) -> Rc<Self> {
        Rc::new(Self {
            data: Rc::new(data),
            edges: Vec::new(),
        })
    }
}

#[derive(Clone)]
struct Edge<E, N> {
    data: Rc<E>,
    parent: Rc<RawNode<E, N>>,
    child: Rc<RawNode<E, N>>,
    version: Weak<Version<Edge<E, N>>>,
}
