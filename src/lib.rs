
#![feature(tuple_indexing)]

use std::rc::Rc;

pub use field::{FieldDef, NamedField, Field};
pub use query::{Query, RcQuery, ToIsQuery};
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

    let name = NamedField::<String> { name: "name".to_string() };
    let mut dset = DT::select(&[&name], NamedFrom("table".to_string()));

    let query = name.is("test".to_string()).upcast();
    dset = dset.where_(&query);

    println!("{}", dset.to_sql());
    fail!("")

}
