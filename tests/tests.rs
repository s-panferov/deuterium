extern crate deuterium;
extern crate chrono;

use deuterium::*;

#[macro_export]
macro_rules! assert_sql {
    ($query:expr, $s:expr) => (
        assert_eq!(&$query.to_final_sql(&mut SqlContext::new(Box::new(sql::PostgreSqlAdapter))), $s)
    )
}

mod select;
mod where_;
mod order;
mod join;
mod group_by;
mod insert;
mod update;
mod delete;
mod placeholder;

#[test]
fn select_order() {

    let jedi_table = TableDef::new("jedi");
    let name = NamedField::<String>::field_of("name", &jedi_table);

    let query: SelectQuery<(String,), LimitOne, ()> = jedi_table.select_1(&name)
        .first().order_by(&name);

    assert_sql!(query, "SELECT name FROM jedi ORDER BY name ASC LIMIT 1;");

}

#[test]
fn select_left_join() {

    let jedi_table = TableDef::new("jedi");
    let name = NamedField::<String>::field_of("name", &jedi_table);

    let query = jedi_table.select_all().left_join(&jedi_table.alias("j"), name.is(name.clone()));
    assert_sql!(query, "SELECT * FROM jedi LEFT JOIN jedi AS j ON name = name;");

}
