
use std::sync::Arc;
pub use field::{NamedField};

pub trait Query: Sync + Send { 
    fn upcast(self) -> RcQuery {
        Arc::new(box self as BoxedQuery)
    }
}

#[deriving(Send, Clone)]
pub struct IsQuery<F, T> {
    pub field: F,
    pub value: T
}

impl<F: Send+Sync, T: Send+Sync> Query for IsQuery<F, T> {

}

pub trait ToIsQuery<F, T> {
    fn is(&self, val: T) -> IsQuery<F, T>;
}

impl<T: Clone> ToIsQuery<NamedField<T>, T> for NamedField<T> {
    fn is(&self, val: T) -> IsQuery<NamedField<T>, T> {
        IsQuery {
            field: self.clone(),
            value: val
        }
    }
}

pub type BoxedQuery = Box<Query + Send + Sync>;
pub type RcQuery = Arc<BoxedQuery>;