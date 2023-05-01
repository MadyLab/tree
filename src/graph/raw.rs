use std::{
    cell::RefCell,
    rc::Rc,
    sync::atomic::{AtomicUsize, Ordering},
};

lazy_static::lazy_static! {
    static ref RAW_NODE_COUNTER: AtomicUsize = AtomicUsize::default();
}

type RcNode<N, E> = Rc<RawNode<N, E>>;

#[derive(Eq)]
pub struct RawNode<N, E> {
    id: usize,
    pub value: N,
    pub parents: RefCell<Option<RcNode<N, E>>>,
    pub children: RefCell<Vec<(E, RcNode<N, E>)>>,
}

impl<N, E> PartialEq for RawNode<N, E> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<N, E> RawNode<N, E> {
    pub fn new(value: N) -> Self {
        let id = RAW_NODE_COUNTER.fetch_add(1, Ordering::Relaxed);
        Self {
            value,
            id,
            parents: RefCell::new(None),
            children: Default::default(),
        }
    }
    pub fn disconnect(self: RcNode<N, E>) {
        if let Some(parent) = self.parents.take() {
            parent
                .children
                .borrow_mut()
                .retain(|(_, node)| node.id != self.id)
        }
        self.parents.replace(None);
    }
    pub fn append(self: RcNode<N, E>, value: E, children: &RcNode<N, E>) {
        if let Some(old_parents) = children.parents.take() {
            old_parents
                .children
                .borrow_mut()
                .retain(|(_, node)| node != children);
        }

        self.children
            .borrow_mut()
            .push((value, children.to_owned()));
        children.parents.replace(Some(self.clone()));
    }
}
