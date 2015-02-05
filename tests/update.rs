use deuterium::*;

#[test]
fn update() {

    let jedi_table = TableDef::new("jedi");
    let name = NamedField::<String>::field_of("name", &jedi_table);

    let update: FieldUpdate<NamedField<String>, String> = name.set(&"Luke".to_string());
    let query = jedi_table.update().all().field(update);
    assert_sql!(query, "UPDATE jedi SET name = $1;");

    let query = jedi_table.update().field(name.set(&"Luke".to_string()));
    assert_sql!(query, "UPDATE jedi SET name = $1 WHERE true = false;");

    let query = jedi_table.update().field(name.set(&"Darth Vader".to_string())).where_(name.is("Anakin Skywalker".to_string()));
    assert_sql!(query, "UPDATE jedi SET name = $1 WHERE name = $2;");

    let table_b = TableDef::new("table_b");
    let name_b = NamedField::<String>::field_of("name", &table_b).qual();

    let query = jedi_table
        .update()
        .from(&table_b)
        .field(name.set_default())
        .where_(name.qual().is(name_b.qual()));

    assert_sql!(query, "UPDATE jedi SET name = DEFAULT FROM table_b WHERE jedi.name = table_b.name;");

}

#[test]
fn update_to_null() {
    let jedi_table = TableDef::new("jedi");
    let name = NamedField::<String>::field_of("name", &jedi_table);

    let query = jedi_table.update().all().field(name.set(&"jedi".to_string()));//.field(name.set());;
    assert_sql!(query, "UPDATE jedi SET name = $1;")
}

#[test]
fn update_returning() {

    let jedi_table = TableDef::new("jedi");
    let name = NamedField::<String>::field_of("name", &jedi_table);

    let query = jedi_table.update().all().field(name.set(&"Luke".to_string())).returning_all();
    assert_sql!(query, "UPDATE jedi SET name = $1 RETURNING *;");

    let query = jedi_table.update().all().field(name.set(&"Luke".to_string())).returning_1(&name.qual());
    assert_sql!(query, "UPDATE jedi SET name = $1 RETURNING jedi.name;")

}