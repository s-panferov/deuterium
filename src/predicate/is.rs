
// use time::Timespec;
// use serialize::json::Json;
// use uuid::Uuid;

use sql::{ToPredicateValue};
use expression::{Expression};

use expression::{RawExpr};
use predicate::{Predicate, ToAbstractPredicate, RcPredicate};
use field::{
    NamedField,
};

#[derive(Clone)]
pub struct IsPredicate<F, T> {
    pub field: F,
    pub value: T
}

pub trait ToIsPredicate<T> {
    fn is<B: Expression<T> + ToPredicateValue + Clone + 'static>(&self, val: B) -> RcPredicate;
}

impl<F, T> Predicate for IsPredicate<F, T> 
    where F: ToPredicateValue,
          T: ToPredicateValue { }

impl<T> ToIsPredicate<T> for NamedField<T> where T: ToPredicateValue + Clone {
    fn is<B: Expression<T> + ToPredicateValue + Clone + 'static>(&self, val: B) -> RcPredicate {
        IsPredicate { field: self.clone(), value: val }.upcast()
    }
}

impl<T> ToIsPredicate<T> for RawExpr where T: ToPredicateValue + Clone {
    fn is<B: Expression<T> + ToPredicateValue + Clone + 'static>(&self, val: B) -> RcPredicate {
        IsPredicate { field: self.clone(), value: val }.upcast()
    }
}