use deuterium::*;

#[test]
fn delete() {

    let jedi_table = TableDef::new("jedi");
    let name = NamedField::<String>::field_of("name", &jedi_table);

    let query = jedi_table.delete().all();
    assert_sql!(query, "DELETE FROM jedi;")

    let query = jedi_table.delete();
    assert_sql!(query, "DELETE FROM jedi WHERE true = false;")

    let query = jedi_table.delete().where_(name.is("Anakin Skywalker".to_string()));
    assert_sql!(query, "DELETE FROM jedi WHERE name = $1;");

    let table_b = TableDef::new("table_b");
    let name_b = NamedField::<String>::field_of("name", &table_b).qual();

    let query = jedi_table
        .delete()
        .only()
        .using(&table_b)
        .where_(name.qual().is(name_b.qual()));

    assert_sql!(query, "DELETE FROM ONLY jedi USING table_b WHERE jedi.name = table_b.name;");

}


#[test]
fn delete_returning() {

    let jedi_table = TableDef::new("jedi");
    let name = NamedField::<String>::field_of("name", &jedi_table);

    let query: DeleteQuery<(), LimitMany, ()> = jedi_table.delete().all().returning_all();
    assert_sql!(query, "DELETE FROM jedi RETURNING *;")

    let query: DeleteQuery<(String), LimitMany, ()> = jedi_table.delete().all().returning_1(&name);
    assert_sql!(query, "DELETE FROM jedi RETURNING name;")

}