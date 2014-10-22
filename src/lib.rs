
#![feature(tuple_indexing)]
#![feature(macro_rules)]
#![feature(struct_variant)]
#![feature(globs)]

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

pub use expression::{RawExpression};
pub use to_sql::{ToSql, QueryToSql, FromToSql};
pub use from::{TableDef, Table, From, BoxedFrom, RcFrom};

mod field;
mod predicate;
mod select_query;
mod to_sql;
mod expression;
mod order_by;
mod from;
mod join;

pub struct Query;

impl Query {

    // FIXME: Unify select_N after [generics](https://github.com/rust-lang/rfcs/issues/376)

    pub fn select_1<T: Clone>(field: &Field<T>, from: &From) -> SelectQuery<(T), LimitMany, ()> {
        SelectQuery::new(SelectOnly(vec![field.to_def().clone_with_erase()]), from.upcast_from())
    }

    pub fn select_2<T1: Clone, T2: Clone>(field1: &Field<T1>, field2: &Field<T2>, from: &From) -> SelectQuery<(T1, T2), LimitMany, ()> {
        SelectQuery::new(SelectOnly(vec![field1.to_def().clone_with_erase(), field2.to_def().clone_with_erase()]), from.upcast_from())
    }

    pub fn select(fields: &[&UntypedField], from: &From) -> SelectQuery<(), LimitMany, ()> {
        SelectQuery::new(SelectOnly(fields.iter().map(|f| f.to_def().clone_with_erase()).collect()), from.upcast_from())
    }

    pub fn select_all(from: &From) -> SelectQuery<(), LimitMany, ()> {
        SelectQuery::new(SelectAll, from.upcast_from())
    }
}
