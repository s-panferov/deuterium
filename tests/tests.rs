#![feature(globs)]
#![feature(macro_rules)]

extern crate deuterium;

use deuterium::*;

macro_rules! assert_sql(
    ($query:expr, $s:expr) => (
        assert_eq!($query.to_sql().as_slice(), $s)
    )
)

#[test]
fn it_works() {

    // Typed field
    let name = StringField { name: "name".to_string() };
    
    // Type is here only for sure it is right, it can be ommited in real code
    let query: SelectQuery<(String), LimitMany> = Query::select_1(&name, NamedFrom("jedi".to_string())).where_(
        &name.is("Luke".to_string()).exclude()
    );

    assert_sql!(query, "SELECT name FROM jedi WHERE name != 'Luke';");

}

#[test]
fn select_1_first() {

    let name = StringField { name: "name".to_string() };
    
    let query: SelectQuery<(String), LimitOne> = Query::select_1(&name, NamedFrom("jedi".to_string())).where_(
        &name.is("Luke".to_string()).exclude()
    ).first().offset(10);

    assert_sql!(query, "SELECT name FROM jedi WHERE name != 'Luke' LIMIT 1 OFFSET 10;");

}

#[test]
fn select_order() {

    let name = StringField { name: "name".to_string() };
    
    let query: SelectQuery<(String), LimitOne> = Query::select_1(&name, NamedFrom("jedi".to_string()))
        .first().order_by(&name);

    assert_sql!(query, "SELECT name FROM jedi ORDER BY name ASC LIMIT 1;");

}