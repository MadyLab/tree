use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    rc::{Rc, Weak},
};

#[derive(Debug)]
pub struct Version<T> {
    cache: Option<HashSet<*const Version<T>>>,
    parents: Vec<Rc<Version<T>>>,
    garbage: RefCell<Vec<Rc<T>>>,
}

impl<T> Clone for Version<T> {
    fn clone(&self) -> Self {
        Self {
            cache:Default::default(),
            parents: self.parents.clone(),
            garbage: self.garbage.clone(),
        }
    }
}

impl<T> Default for Version<T> {
    fn default() -> Self {
        Self {
            cache:Default::default(),
            parents: Default::default(),
            garbage: Default::default(),
        }
    }
}

impl<T> Version<T> {
    pub fn merge(self: &Rc<Version<T>>, base: &Rc<Version<T>>) -> Version<T> {
        let mut version = Version::default();
        version.parents = vec![self.clone(), base.clone()];
        version
    }
    pub unsafe fn migrate(self: &Version<T>, garbage: T) -> Weak<T> {
        let garbage = Rc::new(garbage);
        let result = Rc::<T>::downgrade(&garbage);
        self.garbage.borrow_mut().push(garbage);
        result
    }
    pub fn optimize(self: &Rc<Version<T>>)->Rc<Version<T>>{
        let mut cache=HashSet::new();
        let mut stack=vec![self];
        loop{
            if let Some(item)=stack.pop(){
                let ptr=Rc::as_ptr(item);
                if !cache.contains(&ptr){
                    cache.insert(ptr);
                    for parent in &item.parents{
                        stack.push(&parent)
                    }
                }
            }else{
                break;
            }
        }
        Rc::new(Version{ cache:Some(cache), parents: self.parents.clone(), garbage: self.garbage.clone() })
    }
    pub fn is_derivative_of(self: &Rc<Version<T>>, other: &Rc<Version<T>>) -> bool {
        if let Some(cache)=&self.cache{
            return cache.contains(&Rc::as_ptr(&other));
        }
        if Rc::ptr_eq(self, other) {
            return true;
        }
        for parent in &*self.parents {
            if parent.is_derivative_of(other) {
                return true;
            }
        }
        return false;
    }
    pub fn derive(self: &Rc<Version<T>>) -> Rc<Version<T>> {
        Rc::new(Version {
            cache:Default::default(),
            parents: vec![self.clone()],
            garbage: Default::default(),
        })
    }
}
