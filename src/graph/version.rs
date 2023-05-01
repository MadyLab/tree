use std::{cell::RefCell, rc::Rc};

pub struct Version(Rc<Node>);

impl Version {
    pub fn fork(&self) -> Version {
        Version(Rc::new(Node {
            parents: RefCell::new(vec![self.0.clone()]),
            depth: self.0.depth + 1,
        }))
    }
    pub fn is_derivative_of(&self, other: Version) -> bool {
        if other.0.depth < self.0.depth {
            return false;
        } else {
            return Rc::ptr_eq(&other.0, &self.0.get_parents(other.0.depth - self.0.depth));
        }
    }
}

fn MSB(mut i: usize) -> usize {
    debug_assert_ne!(i, 1, "0 is invaild in MSB");
    let mut o = 1;
    while i != 1 {
        i = i >> 1;
        o += 1;
    }
    o
}

#[derive(Clone)]
struct Node {
    parents: RefCell<Vec<Rc<Node>>>,
    depth: usize,
}

impl Node {
    fn get_parents(&self, step: usize) -> Rc<Node> {
        if step == 0 {
            return Rc::new(self.clone());
        } else {
            let msb = MSB(step);

            let mut parents = self.parents.borrow_mut();
            let result = parents[msb].get_parents(step - (1 << (msb - 1)));
            parents[msb] = result.clone();

            result
        }
    }
}
