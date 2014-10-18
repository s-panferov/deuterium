#![feature(globs)]

extern crate deuterium;

use deuterium::*;

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