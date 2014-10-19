#![feature(globs)]
#![feature(macro_rules)]

extern crate deuterium;

use deuterium::*;

macro_rules! assert_sql(
    ($query:expr, $s:expr) => (
        assert_eq!($query.to_final_sql().as_slice(), $s)
    )
)

#[test]
fn it_works() {

    let jedi_table = TableDef::new("jedi".to_string());
    let name = NamedField::<String>::new("name");
    
    // Type is here only for sure it is right, it can be ommited in real code
    let query: SelectQuery<(String), LimitMany> = jedi_table.select_1(&name).where_(
        name.is("Luke".to_string()).exclude()
    );

    assert_sql!(query, "SELECT name FROM jedi WHERE name != 'Luke';");

}

#[test]
fn select_1_first() {

    let jedi_table = TableDef::new("jedi".to_string());
    let name = NamedField::<String>::new("name");
    
    let query: SelectQuery<(String), LimitOne> = jedi_table.select_1(&name).where_(
        name.is("Luke".to_string()).exclude()
    ).first().offset(10);

    assert_sql!(query, "SELECT name FROM jedi WHERE name != 'Luke' LIMIT 1 OFFSET 10;");

}

#[test]
fn select_order() {

    let jedi_table = TableDef::new("jedi".to_string());
    let name = NamedField::<String>::new("name");
    
    let query: SelectQuery<(String), LimitOne> = jedi_table.select_1(&name)
        .first().order_by(&name);

    assert_sql!(query, "SELECT name FROM jedi ORDER BY name ASC LIMIT 1;");

}

#[test]
fn select_within() {

    let jedi_table = TableDef::new("jedi".to_string());
    let name = NamedField::<String>::new("name");
    
    let query = jedi_table.select_all().where_(name.within(vec!["Luke".to_string()]));
    assert_sql!(query, "SELECT * FROM jedi WHERE name IN ('Luke');");

}

#[test]
fn select_within_select() {

    let jedi_table = TableDef::new("jedi".to_string());
    let name = NamedField::<String>::new("name");

    let query = jedi_table.alias("j".to_string()).select_all().where_(name.within(
        jedi_table.select_1(&name)
    ));

    assert_sql!(query, "SELECT * FROM jedi AS j WHERE name IN (SELECT name FROM jedi);");

}

#[test]
fn select_from_select() {

    let jedi_table = TableDef::new("jedi".to_string());
    
    let query = jedi_table.select_all().alias("jedi_list".to_string()).select_all();
    assert_sql!(query, "SELECT * FROM (SELECT * FROM jedi) as jedi_list;");

}

#[test]
fn select_left_join() {

    let jedi_table = TableDef::new("jedi".to_string());
    let name = NamedField::<String>::new("name");
    
    let query = jedi_table.select_all().left_join(&jedi_table.alias("j".to_string()), name.is(name.clone()));
    assert_sql!(query, "SELECT * FROM jedi LEFT JOIN jedi AS j ON name = name;");

}
