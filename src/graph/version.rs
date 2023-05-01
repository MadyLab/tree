use std::{cmp::Ordering, rc::Rc, cell::RefCell};

use smallvec::SmallVec;

pub struct VersionGuard {
    pfx: SmallVec<[usize; 8]>,
    counter: usize,
}

impl Default for VersionGuard {
    fn default() -> Self {
        Self {
            pfx: Default::default(),
            counter: 0,
        }
    }
}

impl VersionGuard {
    pub fn next<'a>(&'a mut self) -> Version<'a> {
        let i = self.counter;
        self.counter += 1;

        Version {
            sfx: i,
            pfx: &self.pfx,
        }
    }
}

#[derive(PartialEq, Eq, Ord, Clone)]
pub struct Version<'a> {
    sfx: usize,
    pfx: &'a SmallVec<[usize; 8]>,
}

impl<'a> PartialOrd for Version<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // concat pfx and sfx
        let self_ = vec![self.sfx];
        let self_ = self.pfx.iter().chain(self_.iter());
        let other_ = vec![other.sfx];
        let other_ = other.pfx.iter().chain(other_.iter());
        // compare
        for (self_, other_) in self_.zip(other_) {
            if self_ > other_ {
                return Some(Ordering::Greater);
            } else if self_ < other_ {
                return Some(Ordering::Less);
            }
        }
        // return None if is actual fork
        None
    }
}

impl<'a> Version<'a> {
    pub fn fork(&self, guard: &mut VersionGuard) -> VersionGuard {
        let next = guard.next();
        let mut padding = next.pfx.clone();
        padding.push(next.sfx);
        VersionGuard {
            pfx: padding,
            counter: 0,
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
struct VersionNode {
    parents: RefCell<Vec<Rc<VersionNode>>>,
    depth: usize,
}

impl VersionNode {
    fn fork(self:Rc<VersionNode>)->Rc<VersionNode>{
        Rc::new(VersionNode{
            depth:self.depth+1,
            parents:RefCell::new(vec![self])
        })
    }
    fn get_parents(&self, step: usize) -> Rc<VersionNode> {
        if step == 0 {
            return Rc::new(self.clone());
        } else {
            let msb = MSB(step);

            let mut parents=self.parents.borrow_mut();
            let result=parents[msb].get_parents(step-(1<<(msb-1)));
            parents[msb]=result.clone();

            result
        }
    }
}
