
use sql::{ToPredicateValue};
use predicate::{Predicate, ToAbstractPredicate, RcPredicate};

use expression::{RawExpr};
use field;

#[derive(Clone)]
pub struct IsNullPredicate<F> {
    pub field: F,
    pub null: bool
}

pub trait ToIsNullPredicate {
    fn is_null(&self) -> RcPredicate;
    fn not_null(&self) -> RcPredicate;
}

impl<F> Predicate for IsNullPredicate<F> where F: ToPredicateValue {}

impl<T> ToIsNullPredicate for field::NamedField<Option<T>> where T: ToPredicateValue + Clone {
    fn is_null(&self) -> RcPredicate {
        IsNullPredicate { field: self.clone(), null: true }.upcast()
    }

    fn not_null(&self) -> RcPredicate {
        IsNullPredicate { field: self.clone(), null: false }.upcast()
    }
}

impl ToIsNullPredicate for RawExpr {
    fn is_null(&self) -> RcPredicate {
        IsNullPredicate { field: self.clone(), null: true }.upcast()
    }

    fn not_null(&self) -> RcPredicate {
        IsNullPredicate { field: self.clone(), null: false }.upcast()
    }
}