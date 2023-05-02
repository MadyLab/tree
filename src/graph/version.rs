use std::{cell::RefCell, rc::{Rc, Weak}};

#[derive(Clone,Default)]
pub struct Version<T> {
    major: Rc<Node<T>>,
    sub: usize,
}

impl<T> Version<T> {
    pub fn derive(&self) -> Version<T> {
        Version {
            major: self.major.clone(),
            sub: self.sub + 1,
        }
    }
    pub fn fork(&self) -> Version<T> {
        Version {
            major: Rc::new(Node {
                parents: RefCell::new(vec![self.major.clone()]),
                depth: self.major.depth + 1,
                garbage: Default::default(),
            }),
            sub: 0,
        }
    }
    pub fn is_derivative_of(&self, other: Version<T>) -> bool {
        if Rc::ptr_eq(&self.major, &other.major) {
            other.sub <= self.sub
        } else {
            self.is_fork_of(other)
        }
    }
    pub unsafe fn migrate(&self, garbage: Box<T>) -> *const T {
        let result: *const T = &*garbage;
        self.major.garbage.borrow_mut().push(garbage);
        result
    }
    fn is_fork_of(&self, other: Version<T>) -> bool {
        if other.major.depth < self.major.depth {
            false
        } else {
            Rc::ptr_eq(
                &other.major,
                &self
                    .major
                    .clone()
                    .get_parents(other.major.depth - self.major.depth),
            )
        }
    }
}

pub struct WeakVersion<T>{
    major: Weak<Node<T>>,
    sub: usize,
}

fn msb(mut i: usize) -> usize {
    debug_assert_ne!(i, 1, "0 is invaild in MSB");
    let mut o = 1;
    while i != 1 {
        i = i >> 1;
        o += 1;
    }
    o
}

#[derive(Clone,Default)]
struct Node<T> {
    parents: RefCell<Vec<Rc<Node<T>>>>,
    depth: usize,
    garbage: RefCell<Vec<Box<T>>>,
}

impl<T> Node<T> {
    fn get_parents(self: Rc<Node<T>>, step: usize) -> Rc<Node<T>> {
        if step == 0 {
            return self.clone();
        } else {
            let msb = msb(step);

            let mut parents = self.parents.borrow_mut();
            let result = parents[msb].clone().get_parents(step - (1 << (msb - 1)));
            parents[msb] = result.clone();

            result
        }
    }
}
