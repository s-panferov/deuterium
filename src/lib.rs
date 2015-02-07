#![feature(core)]
#![feature(concat_idents)]
// #![deny(warnings)]
// #![deny(bad_style)]

extern crate "rustc-serialize" as serialize;
extern crate time;

#[cfg(feature = "postgres")]
extern crate postgres;
extern crate uuid;

mod prelude {
    pub use {
        ToIsPredicate,
        ToOrPredicate,
        ToAndPredicate,
        ToInPredicate,
        ToInRangePredicate,
        ToInequalityPredicate,
        ToExcludePredicate,
        ToLikePredicate,
        ToIsNullPredicate,
        Selectable,
        Queryable,
        Orderable,
        ToSelectQuery,
        Updatable,
        ToFieldUpdate,
        ToInsertValue,
        Deletable,
        ToExpression,
        ToListExpression
    };
}

pub use field::{
    Field,
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
    SharedPredicate, 
    IsPredicate, ToIsPredicate, 
    OrPredicate, ToOrPredicate,
    AndPredicate, ToAndPredicate,
    InPredicate, ToInPredicate,
    InRangePredicate, ToInRangePredicate, InRangeBounds,
    InequalityPredicate, ToInequalityPredicate, Inequality,
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
    SharedSelectQuery, 
    ToSelectQuery, 
    Select, 
    NoResult, 
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
    ToInsertValue,
    InsertValue,
};

pub use delete_query::{
    DeleteQuery,
    Deletable,
};

pub use expression::{
    BoxedExpression, 
    UntypedExpression, 
    Expression,
    SharedExpression,
    RawExpr,
    ListExpression,
    ToExpression,
    ToListExpression
};

pub use sql::{SqlContext, ToSql, QueryToSql, FromToSql, ToPredicateValue};
#[cfg(feature = "postgres")] pub use sql::{AsPostgresValue};
pub use from::{TableDef, Table, BoxedTable, SharedTable, From, BoxedFrom, SharedFrom};

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

macro_rules! with_clone{
    ($slf: ident, $v:ident, $ex:expr) => ({
        let mut $v = $slf.clone();
        $ex;
        $v
    })
}

mod field;
mod predicate;
mod select_query;
mod insert_query;

#[macro_use]
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

