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
    let name = StringField { name: "name".to_string() };
    
    // Type is here only for sure it is right, it can be ommited in real code
    let query: SelectQuery<(String), LimitMany> = Query::select_1(&name, &jedi_table).where_(
        name.is("Luke".to_string()).exclude()
    );

    assert_sql!(query, "SELECT name FROM jedi WHERE name != 'Luke';");

}

#[test]
fn select_1_first() {

    let jedi_table = TableDef::new("jedi".to_string());
    let name = StringField { name: "name".to_string() };
    
    let query: SelectQuery<(String), LimitOne> = Query::select_1(&name, &jedi_table).where_(
        name.is("Luke".to_string()).exclude()
    ).first().offset(10);

    assert_sql!(query, "SELECT name FROM jedi WHERE name != 'Luke' LIMIT 1 OFFSET 10;");

}

#[test]
fn select_order() {

    let jedi_table = TableDef::new("jedi".to_string());
    let name = StringField { name: "name".to_string() };
    
    let query: SelectQuery<(String), LimitOne> = Query::select_1(&name, &jedi_table)
        .first().order_by(&name);

    assert_sql!(query, "SELECT name FROM jedi ORDER BY name ASC LIMIT 1;");

}

#[test]
fn select_within() {

    let jedi_table = TableDef::new("jedi".to_string());
    let name = StringField { name: "name".to_string() };
    
    let query = Query::select_all(&jedi_table).where_(name.within(vec!["Luke".to_string()]));
    assert_sql!(query, "SELECT * FROM jedi WHERE name IN ('Luke');");

}

#[test]
fn select_within_select() {

    let jedi_table = TableDef::new("jedi".to_string());
    let name = StringField { name: "name".to_string() };

    let query = Query::select_all(&jedi_table.alias("j".to_string())).where_(name.within(
        Query::select_1(&name, &jedi_table)
    ));

    assert_sql!(query, "SELECT * FROM jedi AS j WHERE name IN (SELECT name FROM jedi);");

}

#[test]
fn select_from_select() {

    let jedi_table = TableDef::new("jedi".to_string());
    
    let query = Query::select_all(&Query::select_all(&jedi_table).as_alias("jedi_list".to_string()));
    assert_sql!(query, "SELECT * FROM (SELECT * FROM jedi) as jedi_list;");

}

#[test]
fn select_left_join() {

    let jedi_table = TableDef::new("jedi".to_string());
    let name = StringField { name: "name".to_string() };
    
    let query = Query::select_all(&jedi_table).left_join(&jedi_table.alias("j".to_string()), name.is(name.clone()));
    assert_sql!(query, "SELECT * FROM (SELECT * FROM jedi) as jedi_list;");

}