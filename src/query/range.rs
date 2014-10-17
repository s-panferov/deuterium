
use serialize::json::Json;
use time::Timespec;

use query::{Query, RcQuery};

use field::{
    NamedField,

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
    TimespecField,
};

#[deriving(Clone)]
pub enum InRangeBounds {
    ExcludeBoth,
    IncludeBoth,
    ExcludeRight,
    ExcludeLeft
}

#[deriving(Send, Clone)]
pub struct InRangeQuery<F, T> {
    pub field: F,
    pub from: T,
    pub to: T,
    pub bounds: InRangeBounds
}

pub trait ToInRangeQuery<F, T> {
    fn in_range(&self, from: T, to: T) -> RcQuery;
    fn in_range_exclude_left(&self, from: T, to: T) -> RcQuery;
    fn in_range_exclude_right(&self, from: T, to: T) -> RcQuery;
    fn in_range_exclude(&self, from: T, to: T) -> RcQuery;
}

macro_rules! impl_for(
    ($field:ty, $v:ty) => (

        impl Query for InRangeQuery<$field, $v> { }

        impl ToInRangeQuery<$field, $v> for $field {
            fn in_range(&self, from: $v, to: $v) -> RcQuery {
                InRangeQuery {
                    field: self.clone(),
                    from: from,
                    to: to,
                    bounds: IncludeBoth
                }.upcast()
            }

            fn in_range_exclude_left(&self, from: $v, to: $v) -> RcQuery {
                InRangeQuery {
                    field: self.clone(),
                    from: from,
                    to: to,
                    bounds: ExcludeLeft
                }.upcast()
            }

            fn in_range_exclude_right(&self, from: $v, to: $v) -> RcQuery {
                InRangeQuery {
                    field: self.clone(),
                    from: from,
                    to: to,
                    bounds: ExcludeRight
                }.upcast()
            }

            fn in_range_exclude(&self, from: $v, to: $v) -> RcQuery {
                InRangeQuery {
                    field: self.clone(),
                    from: from,
                    to: to,
                    bounds: ExcludeBoth
                }.upcast()
            }
        }

    )
)

impl_for!(I8Field, i8)
impl_for!(I16Field, i16)
impl_for!(I32Field, i32)
impl_for!(I64Field, i64)
impl_for!(F32Field, f32)
impl_for!(F64Field, f64)
impl_for!(TimespecField, Timespec)