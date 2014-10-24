use deuterium::*;

#[test]
fn select_group_by() {

    let jedi_table = TableDef::new("jedi");
    let name = NamedField::<String>::field_of("name", &jedi_table);
    let side = NamedField::<bool>::field_of("side", &jedi_table);

    let query = jedi_table.select_all().group_by(&[&name, &side]);
    assert_sql!(query, "SELECT * FROM jedi GROUP BY name, side;");

    // Edge case where `by` is empty
    let empty: &[&UntypedField] = &[];
    let query = jedi_table.select_all().group_by(empty);
    assert_sql!(query, "SELECT * FROM jedi;");
}