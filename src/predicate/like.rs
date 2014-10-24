
use predicate::{Predicate, RcPredicate};
use expression::{RawExpression};
use field::{StringField};

#[deriving(Send, Clone)]
pub struct LikePredicate<F> {
    pub field: F,
    pub value: String,
    pub case_sensitive: bool
}

pub trait ToLikePredicate<F> {
    fn like(&self, val: &str) -> RcPredicate;
    fn ilike(&self, val: &str) -> RcPredicate;
}

macro_rules! impl_for(
    ($field:ident) => (
        impl Predicate for LikePredicate<$field> { }
        impl ToLikePredicate<$field> for $field { 
            fn like(&self, val: &str) -> RcPredicate {
                LikePredicate {
                    field: self.clone(),
                    value: val.to_string(),
                    case_sensitive: true
                }.upcast()
            }

            fn ilike(&self, val: &str) -> RcPredicate {
                LikePredicate {
                    field: self.clone(),
                    value: val.to_string(),
                    case_sensitive: false
                }.upcast()
            }
        }
    )
)

impl_for!(StringField)
impl_for!(RawExpression)