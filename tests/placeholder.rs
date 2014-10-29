use deuterium::*;

#[test]
fn placeholder() {

    let jedi_table = TableDef::new("jedi");
    let name = NamedField::<String>::field_of("name", &jedi_table);
    
    let query = jedi_table.select_all().order_by(&Placeholder::new(1));
    assert_sql!(query, "SELECT * FROM jedi ORDER BY $1 ASC;");    

    let query = jedi_table.select_1(&name).where_(name.is(Placeholder::new(1)));
    assert_sql!(query, "SELECT name FROM jedi WHERE name = $1;");

}