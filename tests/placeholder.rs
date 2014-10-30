use deuterium::*;

#[test]
fn placeholder() {

    let jedi_table = TableDef::new("jedi");
    let name = NamedField::<String>::field_of("name", &jedi_table);  

    let query = jedi_table.select_1(&name).where_(name.is(Placeholder::new(1)));
    assert_sql!(query, "SELECT name FROM jedi WHERE name = $1;");

    let query = jedi_table.select_1(&name).where_(name.is(Placeholder::new(1))).or(name.is("Luke Skywalker".to_string()));
    assert_sql!(query, "SELECT name FROM jedi WHERE (name = $1) OR (name = $2);");

    let query = jedi_table.select_1(&name).where_(name.is(Placeholder::new(1))).or(name.like(Placeholder::new(1))).or(name.is("Luke Skywalker".to_string()));
    assert_sql!(query, "SELECT name FROM jedi WHERE ((name = $1) OR (name LIKE $1)) OR (name = $2);");

    // edge and wrong!
    let query = jedi_table.select_1(&name).where_(name.is(Placeholder::new(1))).or(name.like(Placeholder::new(10))).or(name.is("Luke Skywalker".to_string()));
    assert_sql!(query, "SELECT name FROM jedi WHERE ((name = $1) OR (name LIKE $10)) OR (name = $11);");

}