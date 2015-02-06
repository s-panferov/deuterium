use time::Timespec;
use uuid::Uuid;

use predicate::{Predicate, ToAbstractPredicate, RcPredicate};
use expression::{ToListExpression};

use expression::{RawExpr};
use sql::{ToPredicateValue};
use field;

#[derive(Clone)]
pub struct InPredicate<F, T> {
    pub field: F,
    pub values: T
}

pub trait ToInPredicate<T> {
    fn in_<B>(&self, values: B) -> RcPredicate where B: ToListExpression<T> + ToPredicateValue + Clone + 'static;
}

impl<F, T> Predicate for InPredicate<F, T> 
    where F: ToPredicateValue,
          T: ToPredicateValue { }

impl<T> ToInPredicate<T> for field::NamedField<T> where T: ToPredicateValue + Clone {
    fn in_<B: ToListExpression<T> + ToPredicateValue + Clone + 'static>(&self, val: B) -> RcPredicate {
        InPredicate { field: self.clone(), values: val }.upcast()
    }
}

impl<T> ToInPredicate<T> for RawExpr where T: ToPredicateValue + Clone {
    fn in_<B: ToListExpression<T> + ToPredicateValue + Clone + 'static>(&self, val: B) -> RcPredicate {
        InPredicate { field: self.clone(), values: val }.upcast()
    }
}