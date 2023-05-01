use std::cell::RefCell;

pub struct Arena<T>(RefCell<Vec<Box<T>>>);

impl<T> Default for Arena<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<T> Arena<T> {
    pub fn leak(&self,data:T)->*const T{
        let boxed_data=Box::new(data);
        let raw_data:*const T=&*boxed_data;
        self.0.borrow_mut().push(boxed_data);
        raw_data
    }
}