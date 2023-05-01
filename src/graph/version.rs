use std::cmp::Ordering;

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

#[derive(PartialEq, Eq, Ord,Clone)]
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
    pub fn fork(&self,guard:& mut VersionGuard) -> VersionGuard {
        let next=guard.next();
        let mut padding=next.pfx.clone();
        padding.push(next.sfx);
        VersionGuard {
            pfx: padding,
            counter: 0,
        }
    }
}
