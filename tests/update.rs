use deuterium::*;

#[test]
fn update() {

    let jedi_table = TableDef::new("jedi");
    let name = NamedField::<String>::field_of("name", &jedi_table);

    let query = jedi_table.update().all().field(name.set(&"Luke".to_string()));
    assert_sql!(query, "UPDATE jedi SET name = 'Luke';")

    let query = jedi_table.update().field(name.set(&"Luke".to_string()));
    assert_sql!(query, "UPDATE jedi SET name = 'Luke' WHERE true = false;");

    let query = jedi_table.update().field(name.set(&"Darth Vader".to_string())).where_(name.is("Anakin Skywalker".to_string()));
    assert_sql!(query, "UPDATE jedi SET name = 'Darth Vader' WHERE name = 'Anakin Skywalker';");

    let table_b = TableDef::new("table_b");
    let name_b = NamedField::<String>::field_of("name", &table_b).qual();

    let query = jedi_table
        .update()
        .from(&table_b)
        .field(name.set_default())
        .where_(name.qual().is(name_b.qual()));

    assert_sql!(query, "UPDATE jedi SET name = DEFAULT FROM table_b WHERE jedi.name = table_b.name;");

}