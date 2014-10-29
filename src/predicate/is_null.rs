
use predicate::{Predicate, RcPredicate};
use expression::{RawExpr};
use field::{
    BoolField,
    I8Field,
    I16Field,
    I32Field,
    I64Field,
    F32Field,
    F64Field,
    StringField,
    ByteListField,
    JsonField,
    TimespecField
};

#[deriving(Send, Clone)]
pub struct IsNullPredicate<F> {
    pub field: F,
    pub null: bool
}

pub trait ToIsNullPredicate {
    fn is_null(&self) -> RcPredicate;
    fn not_null(&self) -> RcPredicate;
}

macro_rules! impl_for(
    ($f:ty) => (

        impl Predicate for IsNullPredicate<$f> { }
        impl ToIsNullPredicate for $f {
            fn is_null(&self) -> RcPredicate {
                IsNullPredicate {
                    field: self.clone(),
                    null: true
                }.upcast()
            }

            fn not_null(&self) -> RcPredicate {
                IsNullPredicate {
                    field: self.clone(),
                    null: false
                }.upcast()
            }
        }

    )
)

impl_for!(BoolField)
impl_for!(I8Field)
impl_for!(I16Field)
impl_for!(I32Field)
impl_for!(I64Field)
impl_for!(F32Field)
impl_for!(F64Field)
impl_for!(StringField)
impl_for!(ByteListField)
impl_for!(JsonField)
impl_for!(TimespecField)
impl_for!(RawExpr)

