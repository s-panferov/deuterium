
#![feature(tuple_indexing)]
#![feature(macro_rules)]
#![feature(struct_variant)]
#![feature(concat_idents)]
#![feature(default_type_params)]
#![feature(globs)]

#![deny(warnings)]
#![deny(bad_style)]

extern crate serialize;
extern crate time;

#[cfg(feature = "postgres")]
extern crate postgres;

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
    IsNullPredicate, ToIsNullPredicate,
    RawPredicate
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

pub use update_query::{
    UpdateQuery,
    FieldUpdate,
    FieldUpd,
    Updatable,
    ToFieldUpdate,
};

pub use insert_query::{
    InsertQuery,
    Insertable,
};

pub use delete_query::{
    DeleteQuery,
    Deletable,
};

pub use expression::{
    ToExprValue,
    UntypedExpression, 
    Expression,
    RcExpression,
    ExprValue,
    ExpressionValue,
    DefaultValue,
    RawExpr
};

pub use sql::{SqlContext, ToSql, QueryToSql, FromToSql, ToPredicateValue};
#[cfg(feature = "postgres")] pub use sql::{AsPostgresValue};
pub use from::{TableDef, Table, BoxedTable, RcTable, From, BoxedFrom, RcFrom};

pub use function::{
    Sum, SumArg,
    Min, MinArg,
    Max, MaxArg,
    Avg, AvgArg,
    Count, CountArg,
    CountAll
};

pub use placeholder::{
    Placeholder
};

macro_rules! with_clone(
    ($slf: ident, $v:ident, $ex:expr) => ({
        let mut $v = $slf.clone();
        $ex;
        $v
    })
)

mod field;
mod predicate;
mod select_query;
mod insert_query;

#[macro_escape]
mod delete_query;
mod update_query;
pub mod sql;
mod expression;
mod order_by;
mod from;
mod join;
mod distinct;
mod group_by;
mod function;
mod placeholder;

pub struct Query;

impl Query {

    // FIXME: Unify select_N after [generics](https://github.com/rust-lang/rfcs/issues/376)

    pub fn select_1<T: Clone>(field: &Expression<T>, from: &From) -> SelectQuery<(T), LimitMany, ()> {
        SelectQuery::new(SelectOnly(vec![field.upcast_expression()]), from.upcast_from())
    }

    pub fn select_2<T1: Clone, T2: Clone>(field1: &Expression<T1>, field2: &Expression<T2>, from: &From) -> SelectQuery<(T1, T2), LimitMany, ()> {
        SelectQuery::new(SelectOnly(vec![field1.upcast_expression(), field2.upcast_expression()]), from.upcast_from())
    }

    pub fn select(fields: &[&UntypedExpression], from: &From) -> SelectQuery<(), LimitMany, ()> {
        SelectQuery::new(SelectOnly(fields.iter().map(|f| f.upcast_expression()).collect()), from.upcast_from())
    }

    pub fn select_all(from: &From) -> SelectQuery<(), LimitMany, ()> {
        SelectQuery::new(SelectAll, from.upcast_from())
    }
}
