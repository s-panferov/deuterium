
#![feature(tuple_indexing)]
#![feature(macro_rules)]

extern crate serialize;
extern crate time;

use std::rc::Rc;

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

pub use select_query::{SelectQuery, RcSelectQuery, ToSelectQuery, Select, SelectAll, SelectOnly};
pub use expression::{RawExpression};
pub use to_sql::{ToSql};

mod field;
mod predicate;
mod select_query;
mod to_sql;
mod expression;

#[deriving(Clone)]
pub enum From {
    QueryFrom(RcSelectQuery),
    NamedFrom(String)
}

struct Null;

struct Query;

impl Query {

    // FIXME: Unify select_N after [generics](https://github.com/rust-lang/rfcs/issues/376)

    pub fn select_1<T: Clone>(field: &Field<T>, from: From) -> SelectQuery<(T)> {
        SelectQuery::new(SelectOnly(vec![field.to_def().name()]), from)
    }

    pub fn select_2<T1: Clone, T2: Clone>(field1: &Field<T1>, field2: &Field<T2>, from: From) -> SelectQuery<(T1, T2)> {
        SelectQuery::new(SelectOnly(vec![field1.to_def().name(), field2.to_def().name()]), from)
    }

    pub fn select(fields: &[&UntypedField], from: From) -> SelectQuery<()> {
        SelectQuery::new(SelectOnly(fields.iter().map(|f| f.to_def().name()).collect()), from)
    }

    pub fn select_all<T: Clone>(from: From) -> SelectQuery<T> {
        SelectQuery::new(SelectAll, from)
    }
}

#[test]
fn it_works() {

    let name = StringField { name: "name".to_string() };
    let is_admin = BoolField { name: "is_admin".to_string() };
    let is_open = BoolField { name: "is_open".to_string() };
    let counter = I32Field { name: "counter".to_string() };

    let mut query = Query::select_1(&name, NamedFrom("table".to_string()));
    let predicate = name.is("Stas".to_string()).exclude().and(name.is_null());
    query = query.where_(&predicate);

    println!("{}", query.upcast().to_sql());
    fail!("")

}
