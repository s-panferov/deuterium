#![feature(globs)]
#![feature(macro_rules)]

extern crate deuterium;
extern crate time;

use deuterium::*;

#[macro_export]
macro_rules! assert_sql(
    ($query:expr, $s:expr) => (
        assert_eq!($query.to_final_sql().as_slice(), $s)
    )
)

mod simple_select;
mod where_;

#[test]
fn select_1_first() {

    let jedi_table = TableDef::new("jedi");
    let name = NamedField::<String>::field_of("name", &jedi_table);
    
    let query = jedi_table.select_1(&name).where_(
        name.is("Luke".to_string()).exclude()
    ).first().offset(10);

    assert_sql!(query, "SELECT name FROM jedi WHERE name != 'Luke' LIMIT 1 OFFSET 10;");

}

#[test]
fn select_order() {

    let jedi_table = TableDef::new("jedi");
    let name = NamedField::<String>::field_of("name", &jedi_table);
    
    let query: SelectQuery<(String), LimitOne, ()> = jedi_table.select_1(&name)
        .first().order_by(&name);

    assert_sql!(query, "SELECT name FROM jedi ORDER BY name ASC LIMIT 1;");

}

#[test]
fn select_within() {

    let jedi_table = TableDef::new("jedi");
    let name = NamedField::<String>::field_of("name", &jedi_table);
    
    let query = jedi_table.select_all().where_(name.in_(vec!["Luke".to_string()]));
    assert_sql!(query, "SELECT * FROM jedi WHERE name IN ('Luke');");

}

#[test]
fn select_within_select() {

    let jedi_table = TableDef::new("jedi");
    let name = NamedField::<String>::field_of("name", &jedi_table);

    let query = jedi_table.alias("j").select_all().where_(name.in_(
        jedi_table.select_1(&name)
    ));

    assert_sql!(query, "SELECT * FROM jedi AS j WHERE name IN (SELECT name FROM jedi);");

}

#[test]
fn select_from_select() {

    let jedi_table = TableDef::new("jedi");
    
    let query = jedi_table.select_all().alias("jedi_list").select_all();
    assert_sql!(query, "SELECT * FROM (SELECT * FROM jedi) as jedi_list;");

}

#[test]
fn select_left_join() {

    let jedi_table = TableDef::new("jedi");
    let name = NamedField::<String>::field_of("name", &jedi_table);
    
    let query = jedi_table.select_all().left_join(&jedi_table.alias("j"), name.is(name.clone()));
    assert_sql!(query, "SELECT * FROM jedi LEFT JOIN jedi AS j ON name = name;");

}

#[test]
fn aliases() {

    let jedi_table = TableDef::new("jedi");
    let jedi_a = jedi_table.alias("a");
    let jedi_b = jedi_table.alias("b");
    let name_a = NamedField::<String>::field_of("name", &jedi_a);
    let name_b = NamedField::<String>::field_of("name", &jedi_b);
    
    let query = jedi_a.select_all().inner_join(&jedi_b, name_a.is(name_b));
    assert_sql!(query, "SELECT * FROM jedi AS a INNER JOIN jedi AS b ON a.name = b.name;");

}