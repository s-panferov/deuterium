
#![feature(tuple_indexing)]
#![feature(macro_rules)]

extern crate serialize;
extern crate time;

use std::rc::Rc;

pub use field::{
    FieldDef, 
    NamedField, 
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
pub use query::{
    Query, 
    RcQuery, 
    IsQuery, ToIsQuery, 
    OrQuery, ToOrQuery,
    AndQuery, ToAndQuery,
    InQuery, ToInQuery,
    InRangeQuery, ToInRangeQuery
};
pub use data_set::{SelectDataSet};
pub use to_sql::{ToSql};

mod field;
mod query;
mod data_set;
mod to_sql;

#[deriving(Clone)]
pub enum From {
    DataSetFrom(Box<SelectDataSet>),
    NamedFrom(String)
}

#[deriving(Clone)]
pub enum Select {
    SelectOnly(Vec<FieldDef>),
    SelectAll
}

struct DT;

impl DT {
    pub fn select(fields: &[&Field], from: From) -> SelectDataSet {
        let select = SelectOnly(fields.iter().map(|field| field.to_def()).collect());
        SelectDataSet::new(select, from)
    }

    pub fn select_all(from: From) -> SelectDataSet {
        SelectDataSet::new(SelectAll, from)
    }
}

#[test]
fn it_works() {

    let name = StringField { name: "name".to_string() };
    let is_admin = BoolField { name: "is_admin".to_string() };
    let is_open = BoolField { name: "is_open".to_string() };
    let counter = I32Field { name: "counter".to_string() };

    let mut dset = DT::select(&[&name], NamedFrom("table".to_string()));

    let query = name.is("test".to_string()).upcast().or(
        is_admin.is(true).upcast().and(is_open.is(true).upcast()).upcast().and(
            name.within(vec!["Marcus".to_string(), "Jane".to_string()]).upcast()
        ).upcast()
    ).upcast().or(counter.in_range(100, 200).upcast());

    dset = dset.where_(&query.upcast());

    println!("{}", dset.to_sql());
    fail!("")

}
