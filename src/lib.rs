
#![feature(tuple_indexing)]
#![feature(macro_rules)]
#![feature(struct_variant)]
#![feature(concat_idents)]
#![feature(globs)]

#![deny(warnings)]
#![deny(bad_style)]

extern crate serialize;
extern crate time;

pub use field::{
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
    Inequality, LessThan, LessThanEqual, GreaterThan, GreaterThanEqual,
    ExcludePredicate, ToExcludePredicate,
    LikePredicate, ToLikePredicate,
    IsNullPredicate, ToIsNullPredicate
};

pub use select_query::{
    Selectable,
    Queryable,
    Orderable,
    SelectQuery, 
    RcSelectQuery, 
    ToSelectQuery, 
    Select, 
    SelectAll, 
    SelectOnly, 
    LimitOne, 
    LimitTwo, 
    LimitMany
};

pub use insert_query::{
    InsertQuery
};

pub use expression::{
    UntypedExpression, 
    Expression,
    RcExpression
};

pub use raw_expression::{RawExpression};
pub use to_sql::{ToSql, QueryToSql, FromToSql};
pub use from::{TableDef, Table, From, BoxedFrom, RcFrom};

pub use function::{
    Sum, SumArg,
    Min, MinArg,
    Max, MaxArg,
    Avg, AvgArg,
    Count, CountArg,
    CountAll
};

mod field;
pub mod predicate;
mod select_query;
mod insert_query;
mod to_sql;
mod expression;
mod raw_expression;
mod order_by;
mod from;
mod join;
mod distinct;
mod group_by;
mod function;

pub struct Query;

impl Query {

    // FIXME: Unify select_N after [generics](https://github.com/rust-lang/rfcs/issues/376)

    pub fn select_1<T: Clone>(field: &Expression<T>, from: &From) -> SelectQuery<(T), LimitMany, ()> {
        SelectQuery::new(SelectOnly(vec![field.upcast()]), from.upcast_from())
    }

    pub fn select_2<T1: Clone, T2: Clone>(field1: &Expression<T1>, field2: &Expression<T2>, from: &From) -> SelectQuery<(T1, T2), LimitMany, ()> {
        SelectQuery::new(SelectOnly(vec![field1.upcast(), field2.upcast()]), from.upcast_from())
    }

    pub fn select(fields: &[&UntypedExpression], from: &From) -> SelectQuery<(), LimitMany, ()> {
        SelectQuery::new(SelectOnly(fields.iter().map(|f| f.upcast()).collect()), from.upcast_from())
    }

    pub fn select_all(from: &From) -> SelectQuery<(), LimitMany, ()> {
        SelectQuery::new(SelectAll, from.upcast_from())
    }
}
