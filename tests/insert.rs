use deuterium::*;

#[test]
fn insert_default() {
    let jedi_table = TableDef::new("jedi");
    assert_sql!(jedi_table.insert_all(), "INSERT INTO jedi DEFAULT VALUES;")
}

#[test]
fn insert() {

    let jedi_table = TableDef::new("jedi");
    let name = NamedField::<String>::field_of("name", &jedi_table);
    let side = NamedField::<bool>::field_of("side", &jedi_table);

    let mut query = jedi_table.insert_fields(&[&name, &side]);
    query.push_untyped(&["Luke".to_string().as_expr(), true.as_expr()]);
    assert_sql!(query, "INSERT INTO jedi (name, side) VALUES\n    ($1, $2);");

    let mut query = jedi_table.insert_1_for_test(&name);
    query.push((InsertValue::new("Luke".to_string().as_expr()), ));
    query.push((InsertValue::Default, ));

    assert_sql!(query, "INSERT INTO jedi (name) VALUES\n    ($1),\n    (DEFAULT);");
}

#[test]
fn insert_returning() {

    let jedi_table = TableDef::new("jedi");
    let name = NamedField::<String>::field_of("name", &jedi_table);
    let side = NamedField::<bool>::field_of("side", &jedi_table);

    let mut query = jedi_table.insert_fields(&[&name, &side]).returning_1(&name.qual());
    query.push_untyped(&["Luke".to_string().as_expr(), true.as_expr()]);
    assert_sql!(query, "INSERT INTO jedi (name, side) VALUES\n    ($1, $2) RETURNING jedi.name;");
}