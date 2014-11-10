
use predicate::{Predicate, RcPredicate};
#[cfg(feature = "raw_expr")]
use expression::{RawExpr};
use field::{
    OptionalBoolField,
    OptionalI8Field,
    OptionalI16Field,
    OptionalI32Field,
    OptionalI64Field,
    OptionalF32Field,
    OptionalF64Field,
    OptionalStringField,
    OptionalByteListField,
    OptionalJsonField,
    OptionalTimespecField,
    OptionalUuidField,
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

impl_for!(OptionalBoolField)
impl_for!(OptionalI8Field)
impl_for!(OptionalI16Field)
impl_for!(OptionalI32Field)
impl_for!(OptionalI64Field)
impl_for!(OptionalF32Field)
impl_for!(OptionalF64Field)
impl_for!(OptionalStringField)
impl_for!(OptionalByteListField)
impl_for!(OptionalJsonField)
impl_for!(OptionalTimespecField)
impl_for!(OptionalUuidField)

#[cfg(feature = "raw_expr")]
impl_for!(RawExpr)

