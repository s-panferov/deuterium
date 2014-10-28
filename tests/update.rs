use deuterium::*;

#[test]
fn update() {

    let jedi_table = TableDef::new("jedi");
    let name = NamedField::<String>::field_of("name", &jedi_table);

    let query = jedi_table.update().field(name.set("Luke".to_string()));
    assert_sql!(query, "UPDATE jedi SET name = 'Luke' WHERE true = false;");

    let query = jedi_table.update().field(name.set("Darth Vader".to_string())).where_(name.is("Anakin Skywalker"));
    assert_sql!(query, "UPDATE jedi SET name = 'Darth Vader' WHERE name = 'Anakin Skywalker';");

    let table_b = TableDef::new("table_b");

    let query = jedi_table
        .update()
        .from(&table_b)
        .field(name.set("Darth Vader".to_string()))
        .where_(name.is("Anakin Skywalker"));

    assert_sql!(query, "UPDATE jedi SET name = 'Darth Vader' FROM table_b WHERE name = 'Anakin Skywalker';");

}