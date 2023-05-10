use std::{
    cell::RefCell,
    collections::{HashMap},
    rc::{Rc, Weak},
};

pub struct Version<T> {
    cache: RefCell<HashMap<*const Version<T>, bool>>,
    parents: RefCell<Vec<Rc<Version<T>>>>,
    garbage: RefCell<Vec<Rc<T>>>,
}

impl<T> Clone for Version<T> {
    fn clone(&self) -> Self {
        Self {
            cache: self.cache.clone(),
            parents: self.parents.clone(),
            garbage: self.garbage.clone(),
        }
    }
}

impl<T> Default for Version<T> {
    fn default() -> Self {
        Self {
            cache: Default::default(),
            parents: Default::default(),
            garbage: Default::default(),
        }
    }
}

impl<T> Version<T> {
    pub fn merge(self: &Rc<Version<T>>, base: &Rc<Version<T>>) {
        self.parents.borrow_mut().push(base.clone());
    }
    pub fn fork(self: &Rc<Version<T>>) -> (Rc<Version<T>>, Rc<Version<T>>) {
        (self.derive(), self.derive())
    }
    pub unsafe fn migrate(self: &Version<T>, garbage: T) -> Weak<T> {
        let garbage = Rc::new(garbage);
        let result = Rc::<T>::downgrade(&garbage);
        self.garbage.borrow_mut().push(garbage);
        result
    }
    pub fn is_derivative_of(self: &Rc<Version<T>>, other: &Rc<Version<T>>) -> bool {
        let other_ptr = Rc::<Version<T>>::as_ptr(&other);
        let mut cache = self.cache.borrow_mut();

        if let Some(a) = cache.get(&other_ptr) {
            return *a;
        } else if Rc::ptr_eq(self, other) {
            return true;
        }
        for parent in &*self.parents.borrow() {
            if !Rc::ptr_eq(parent, self) && parent.is_derivative_of(other) {
                cache.insert(other_ptr, true);
                return true;
            }
        }
        cache.insert(other_ptr, false);
        return false;
    }
    pub fn derive(self: &Rc<Version<T>>) -> Rc<Version<T>> {
        Rc::new(Version {
            cache: Default::default(),
            parents: RefCell::new(vec![self.clone()]),
            garbage: Default::default(),
        })
    }
}
