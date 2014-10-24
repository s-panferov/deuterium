use deuterium::*;

#[test]
fn order_asc() {

    let jedi_table = TableDef::new("jedi");
    let name = NamedField::<String>::field_of("name", &jedi_table);
    let force_level = NamedField::<i8>::field_of("force_level", &jedi_table);
    let side = NamedField::<bool>::field_of("side", &jedi_table);
    
    let query = jedi_table.select_all().order_by(&name);
    assert_sql!(query, "SELECT * FROM jedi ORDER BY name ASC;");

    let query = query.order_append(&force_level);
    assert_sql!(query, "SELECT * FROM jedi ORDER BY name ASC, force_level ASC;");    

    let query = query.order_prepend(&side);
    assert_sql!(query, "SELECT * FROM jedi ORDER BY side ASC, name ASC, force_level ASC;");

}

#[test]
fn order_desc() {

    let jedi_table = TableDef::new("jedi");
    let name = NamedField::<String>::field_of("name", &jedi_table);
    let force_level = NamedField::<i8>::field_of("force_level", &jedi_table);
    let side = NamedField::<bool>::field_of("side", &jedi_table);
    
    let query = jedi_table.select_all().reverse_by(&name);
    assert_sql!(query, "SELECT * FROM jedi ORDER BY name DESC;");

    let query = query.reverse_append(&force_level);
    assert_sql!(query, "SELECT * FROM jedi ORDER BY name DESC, force_level DESC;");    

    let query = query.reverse_prepend(&side);
    assert_sql!(query, "SELECT * FROM jedi ORDER BY side DESC, name DESC, force_level DESC;");

}