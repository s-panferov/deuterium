
use std::rc::Rc;

pub use field::{FieldDef, NamedField, Field};
pub use query::{Query, RcQuery, ToIsQuery};
pub use data_set::{DataSet};

mod field;
mod query;
mod data_set;

#[deriving(Clone)]
pub enum From {
    NotDefinedFrom,
    DataSetFrom(Box<DataSet>),
    NamedFrom(String)
}

#[deriving(Clone)]
pub enum Select {
    SelectOnly(Vec<FieldDef>),
    SelectAll
}


#[test]
fn it_works() {

    let name = NamedField::<String> { name: "name".to_string() };
    let mut dset = DataSet::new().select(&[&name]);

    let query = name.is("test".to_string()).upcast();
    dset = dset.where_(&query);

}
