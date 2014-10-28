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
    query.push_untyped(&[&"Luke".to_string(), &true]);
    assert_sql!(query, "INSERT INTO jedi (name, side) VALUES\n    ('Luke', true);");

    let mut query = jedi_table.insert_1_for_test(&name);
    query.push((InsertValue::new(&"Luke".to_string()), ));
    query.push((DefaultValue, ));

    assert_sql!(query, "INSERT INTO jedi (name) VALUES\n    ('Luke'),\n    (DEFAULT);");
}