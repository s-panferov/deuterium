use deuterium::*;

#[test]
fn select_group_by() {

    let jedi_table = TableDef::new("jedi");
    let name = NamedField::<String>::field_of("name", &jedi_table);
    let side = NamedField::<bool>::field_of("side", &jedi_table);
    let force_level = NamedField::<i8>::field_of("force_level", &jedi_table);

    let query = jedi_table.select_all().group_by(&[&name, &side]);
    assert_sql!(query, "SELECT * FROM jedi GROUP BY name, side;");

    // Edge case where `by` is empty
    let empty: &[&UntypedExpression] = &[];
    let query = jedi_table.select_all().group_by(empty);
    assert_sql!(query, "SELECT * FROM jedi;");

    let query = jedi_table.select_2(&name, &force_level.sum()).group_by(&[&name]);
    assert_sql!(query, "SELECT name, SUM(force_level) FROM jedi GROUP BY name;");
}

#[test]
fn select_with_agg() {

    let jedi_table = TableDef::new("jedi");
    let name = NamedField::<String>::field_of("name", &jedi_table);
    let force_level = NamedField::<i8>::field_of("force_level", &jedi_table);

    let query = jedi_table.select_2(&name, &force_level.sum()).group_by(&[&name]);
    assert_sql!(query, "SELECT name, SUM(force_level) FROM jedi GROUP BY name;");
}