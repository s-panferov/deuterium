use deuterium::*;

#[test]
fn conditioned_join() {

    let jedi_table = TableDef::new("jedi");
    let jedi_j = jedi_table.alias("j");
    let name = NamedField::<String>::field_of("name", &jedi_table);
    let name_j = NamedField::<String>::field_of("name", &jedi_j);
    
    let query = jedi_table.select_all().inner_join(&jedi_j, name.is(name_j));
    assert_sql!(query, "SELECT * FROM jedi INNER JOIN jedi AS j ON name = j.name;");

}

#[test]
fn unconditioned_join() {

    let jedi_table = TableDef::new("jedi");
    let jedi_j = jedi_table.alias("j");
    
    let query = jedi_table.select_all().cross_join(&jedi_j);
    assert_sql!(query, "SELECT * FROM jedi CROSS JOIN jedi AS j;");

}

