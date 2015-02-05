
use predicate::{Predicate, ToAbstractPredicate, RcPredicate};
use expression::{self, Expression};
use field;
use sql::{ToPredicateValue};

#[derive(Clone)]
pub struct LikePredicate<F, T> {
    pub field: F,
    pub value: T,
    pub case_sensitive: bool
}

pub trait ToLikePredicate<T> {
    fn like<B>(&self, val: B) -> RcPredicate 
        where B: Expression<T> + ToPredicateValue + Clone + 'static;

    fn ilike<B>(&self, val: B) -> RcPredicate
        where B: Expression<T> + ToPredicateValue + Clone + 'static;
}

impl<F, T> Predicate for LikePredicate<F, T> 
    where F: ToPredicateValue,
          T: ToPredicateValue { }

macro_rules! impl_for {
    ($field:ty, $expr:ty) => (
        impl ToLikePredicate<$expr> for $field {
            fn like<B>(&self, val: B) -> RcPredicate 
                where B: Expression<$expr> + ToPredicateValue + Clone + 'static {
                LikePredicate { field: self.clone(), value: val, case_sensitive: true }.upcast()
            }

            fn ilike<B>(&self, val: B) -> RcPredicate 
                where B: Expression<$expr> + ToPredicateValue + Clone + 'static {
                LikePredicate { field: self.clone(), value: val, case_sensitive: false }.upcast()
            } 
        }
    )
}

impl_for!(field::StringField, String);
impl_for!(field::OptionalStringField, Option<String>);
impl_for!(expression::RawExpr, String);