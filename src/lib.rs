#![deny(missing_debug_implementations, missing_copy_implementations,
        warnings,
        trivial_numeric_casts,
        unstable_features,
        unused, future_incompatible)]

extern crate serde_json;
extern crate chrono;

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
    BoxedField,
    SharedField,
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
    UuidField,

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
    RawExpression,
    ListExpression,
    ToExpression,
    ToListExpression
};

pub use sql::{SqlContext, ToSql, QueryToSql, FromToSql, ToPredicateValue};
#[cfg(feature = "postgres")] pub use sql::AsPostgresValue;
pub use from::{TableDef, Table, BoxedTable, SharedTable, From, BoxedFrom, SharedFrom};

pub use function::{
    Sum, SumArg,
    Min, MinArg,
    Max, MaxArg,
    Avg, AvgArg,
    Count, CountArg,
    CountAll
};

pub use placeholder::Placeholder;

macro_rules! with_clone {
    ($slf: ident, $v:ident, $ex:expr) => ({
        let mut $v = $slf.clone();
        $ex;
        $v
    })
}

mod field;
mod predicate;
mod select_query;
#[macro_use]
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
