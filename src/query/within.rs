
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

#[deriving(Send, Clone)]
pub struct InQuery<F, T> {
    pub field: F,
    pub values: T
}

pub trait ToInQuery<F, T> {
    fn within(&self, val: T) -> InQuery<F, T>;
}

impl<T: Clone> ToInQuery<NamedField<T>, Vec<T>> for NamedField<T> {
    fn within(&self, values: Vec<T>) -> InQuery<NamedField<T>, Vec<T>> {
        InQuery {
            field: self.clone(),
            values: values
        }
    }
}

impl Query for InQuery<I8Field, Vec<i8>> { }
impl Query for InQuery<I16Field, Vec<i16>> { }
impl Query for InQuery<I32Field, Vec<i32>> { }
impl Query for InQuery<I64Field, Vec<i64>> { }
impl Query for InQuery<F32Field, Vec<f32>> { }
impl Query for InQuery<F64Field, Vec<f64>> { }
impl Query for InQuery<StringField, Vec<String>> { }
impl Query for InQuery<TimespecField, Vec<Timespec>> { }

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
    fn in_range(&self, from: T, to: T) -> InRangeQuery<F, T>;
    fn in_range_exclude_left(&self, from: T, to: T) -> InRangeQuery<F, T>;
    fn in_range_exclude_right(&self, from: T, to: T) -> InRangeQuery<F, T>;
    fn in_range_exclude(&self, from: T, to: T) -> InRangeQuery<F, T>;
}

impl<T: Clone> ToInRangeQuery<NamedField<T>, T> for NamedField<T> {
    fn in_range(&self, from: T, to: T) -> InRangeQuery<NamedField<T>, T> {
        InRangeQuery {
            field: self.clone(),
            from: from,
            to: to,
            bounds: IncludeBoth
        }
    }

    fn in_range_exclude_left(&self, from: T, to: T) -> InRangeQuery<NamedField<T>, T> {
        InRangeQuery {
            field: self.clone(),
            from: from,
            to: to,
            bounds: ExcludeLeft
        }
    }

    fn in_range_exclude_right(&self, from: T, to: T) -> InRangeQuery<NamedField<T>, T> {
        InRangeQuery {
            field: self.clone(),
            from: from,
            to: to,
            bounds: ExcludeRight
        }
    }

    fn in_range_exclude(&self, from: T, to: T) -> InRangeQuery<NamedField<T>, T> {
        InRangeQuery {
            field: self.clone(),
            from: from,
            to: to,
            bounds: ExcludeBoth
        }
    }
}

impl Query for InRangeQuery<I8Field, i8> { }
impl Query for InRangeQuery<I16Field, i16> { }
impl Query for InRangeQuery<I32Field, i32> { }
impl Query for InRangeQuery<I64Field, i64> { }
impl Query for InRangeQuery<F32Field, f32> { }
impl Query for InRangeQuery<F64Field, f64> { }
impl Query for InRangeQuery<TimespecField, Timespec> { }