
#![feature(tuple_indexing)]
#![feature(macro_rules)]

#![deny(warnings)]
#![deny(bad_style)]

extern crate serialize;
extern crate time;

pub use field::{
    FieldDef, 
    NamedField,
    UntypedField, 
    Field,

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

pub use predicate::{
    Predicate, 
    RcPredicate, 
    IsPredicate, ToIsPredicate, 
    OrPredicate, ToOrPredicate,
    AndPredicate, ToAndPredicate,
    InPredicate, ToInPredicate,
    InRangePredicate, ToInRangePredicate, 
    InRangeBounds, ExcludeBoth, IncludeBoth, ExcludeRight, ExcludeLeft,
    InequalityPredicate, ToInequalityPredicate, 
    Inequality, LessThan, LessThanEqual, GreaterThan, GreaterTranEqual,
    ExcludePredicate, ToExcludePredicate,
    IsNullPredicate, ToIsNullPredicate
};

pub use select_query::{SelectQuery, RcSelectQuery, ToSelectQuery, Select, SelectAll, SelectOnly, LimitOne, LimitTwo, LimitMany};
pub use expression::{RawExpression};
pub use to_sql::{ToSql};

mod field;
mod predicate;
mod select_query;
mod to_sql;
mod expression;
mod order_by;

#[deriving(Clone)]
pub enum From {
    QueryFrom(RcSelectQuery),
    NamedFrom(String)
}

pub struct Query;

impl Query {

    // FIXME: Unify select_N after [generics](https://github.com/rust-lang/rfcs/issues/376)

    pub fn select_1<T: Clone>(field: &Field<T>, from: From) -> SelectQuery<(T), LimitMany> {
        SelectQuery::new(SelectOnly(vec![field.to_def().name()]), from)
    }

    pub fn select_2<T1: Clone, T2: Clone>(field1: &Field<T1>, field2: &Field<T2>, from: From) -> SelectQuery<(T1, T2), LimitMany> {
        SelectQuery::new(SelectOnly(vec![field1.to_def().name(), field2.to_def().name()]), from)
    }

    pub fn select(fields: &[&UntypedField], from: From) -> SelectQuery<(), LimitMany> {
        SelectQuery::new(SelectOnly(fields.iter().map(|f| f.to_def().name()).collect()), from)
    }

    pub fn select_all(from: From) -> SelectQuery<(), LimitMany> {
        SelectQuery::new(SelectAll, from)
    }
}
