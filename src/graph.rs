use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use super::version::Version;

/// A Node in Directed Acyclic Graph
///
/// Being cyclic would make all participated ``rawNode`` visible to other, which make clone meaningsless
///
/// First generic: Edge value
///
/// Second generic: Node value
///
#[derive(Debug)]
pub struct Node<E, N> {
    version: Rc<Version<Edge<E, N>>>,
    raw: Rc<RawNode<E, N>>,
}

impl<E, N> Node<E, N> {
    /// construct new Node
    ///
    pub fn new(data: N) -> Self {
        Self {
            version: Default::default(),
            raw: RawNode::new(data),
        }
    }
    /// Synchronize Node's ``Version`` with other
    /// 
    /// Also of note that the ``Version`` here is different from that of ``RawNode``,
    /// so changing ``Node``'s ``Version`` won't change how the underlying graph.
    /// 
    pub fn sync(&mut self, base: &mut Node<E, N>) {
        if self.version.is_derivative_of(&base.version) {
            unsafe { self.extend_version(base) };
        } else if base.version.is_derivative_of(&self.version) {
            unsafe { base.extend_version(self) };
        } else {
            panic!("base's version might be a fork of self(and never merge)");
        }
    }
    /// Extend Node's ``Version`` to the future
    /// 
    /// Caller should ensure ``base`` is older than ``self``(not strictly).
    ///
    /// It's recommanded to call ``sync`` instead.
    ///
    pub unsafe fn extend_version(&mut self, base: &mut Node<E, N>) {
        debug_assert!(self.version.is_derivative_of(&base.version));
        self.version = base.version.clone();
    }
    /// Optimized the ``Node`` to speed up the process of finding its edge
    /// 
    /// Only need to call once, and returned all ``Node`` in both
    ///  ``ParentsIterator`` and ``ChildrenIterator`` would be optimized.
    /// 
    pub fn optimize(&mut self) {
        self.version = self.version.optimize();
    }
    /// Add child to the ``Node``
    /// 
    /// Also of note that the child's version won't be sync, to sync the child
    /// , call ``add_child_sync`` instead
    /// 
    pub fn add_child(&mut self, child: &mut Node<E, N>, value: E) {
        self.version = Rc::new(child.version.merge(&self.version));
        let edge = Edge {
            data: Rc::new(value),
            parent: self.raw.clone(),
            child: child.raw.clone(),
            version: Rc::downgrade(&self.version),
        };
        let edge = unsafe { child.version.migrate(edge) };
        self.raw.edges.borrow_mut().push(edge.clone());
        child.raw.edges.borrow_mut().push(edge);
    }
    /// Add child and sync
    /// 
    /// See ``add_child`` and ``sync`` for further info.
    /// 
    pub fn add_child_sync(&mut self, child: &mut Node<E, N>, value: E) {
        self.add_child(child, value);
        unsafe { child.extend_version(self) };
    }
    /// Get value of the ``Node``
    /// 
    pub fn get(&self) -> &N {
        &self.raw.data
    }
    /// Clone the node
    /// 
    pub fn clone(&mut self) -> Self {
        Self {
            raw: Rc::new(RawNode {
                data: self.raw.data.clone(),
                edges: self.raw.edges.clone(),
            }),
            version: self.version.derive(),
        }
    }
    pub fn parents(&self) -> ParentsIterator<'_, E, N> {
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
                raw: edge.parent.clone(),
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
                raw: edge.child.clone(),
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
