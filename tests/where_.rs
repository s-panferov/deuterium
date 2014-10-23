
use deuterium::*;

#[test]
fn simple_where() {

    let jedi_table = TableDef::new("jedi");
    let name = NamedField::<String>::field_of("name", &jedi_table);
    
    let query = jedi_table.select_all().where_(name.is("Luke"));
    assert_sql!(query, "SELECT * FROM jedi WHERE name = 'Luke';");
}

#[test]
fn query_level_and() {

    let jedi_table = TableDef::new("jedi");
    let name = NamedField::<String>::field_of("name", &jedi_table);
    let side = NamedField::<bool>::field_of("side", &jedi_table);
    let force_level = NamedField::<i8>::field_of("force_level", &jedi_table);
    
    let and_sql = "SELECT * FROM jedi WHERE (name = 'Luke') AND (side = true);";

    let query = jedi_table.select_all().where_(name.is("Luke")).where_(side.is(true));
    assert_sql!(query, and_sql);

    let query = jedi_table.select_all().where_(name.is("Luke")).and(side.is(true));
    assert_sql!(query, and_sql);

    let query = query.and(force_level.lt(100i8));
    assert_sql!(query, "SELECT * FROM jedi WHERE ((name = 'Luke') AND (side = true)) AND (force_level < 100);");
}

#[test]
fn query_level_or() {

    let jedi_table = TableDef::new("jedi");
    let name = NamedField::<String>::field_of("name", &jedi_table);
    let side = NamedField::<bool>::field_of("side", &jedi_table);
    let force_level = NamedField::<i8>::field_of("force_level", &jedi_table);
    
    let query = jedi_table.select_all().where_(name.is("Luke")).or(side.is(true));
    assert_sql!(query, "SELECT * FROM jedi WHERE (name = 'Luke') OR (side = true);"); 

    let query = query.or(force_level.lt(100i8));
    assert_sql!(query, "SELECT * FROM jedi WHERE ((name = 'Luke') OR (side = true)) OR (force_level < 100);");
}

#[test]
fn predicate_and_or() {

    let jedi_table = TableDef::new("jedi");
    let name = NamedField::<String>::field_of("name", &jedi_table);
    let side = NamedField::<bool>::field_of("side", &jedi_table);
    
    let query = jedi_table.select_all().where_(name.is("Luke").or(name.is("Joda").and(side.is(true))));
    assert_sql!(query, "SELECT * FROM jedi WHERE (name = 'Luke') OR ((name = 'Joda') AND (side = true));"); 
}

#[test]
fn predicate_inequality() {

    let jedi_table = TableDef::new("jedi");
    let force_level = NamedField::<i8>::field_of("force_level", &jedi_table);
    
    let query = jedi_table.select_all().where_(force_level.lt(100i8));
    assert_sql!(query, "SELECT * FROM jedi WHERE force_level < 100;");     

    let query = jedi_table.select_all().where_(force_level.lte(100i8));
    assert_sql!(query, "SELECT * FROM jedi WHERE force_level <= 100;"); 

    let query = jedi_table.select_all().where_(force_level.gt(100i8));
    assert_sql!(query, "SELECT * FROM jedi WHERE force_level > 100;"); 

    let query = jedi_table.select_all().where_(force_level.gte(100i8));
    assert_sql!(query, "SELECT * FROM jedi WHERE force_level >= 100;"); 
}

#[test]
fn predicate_inequality_exclude() {

    let jedi_table = TableDef::new("jedi");
    let force_level = NamedField::<i8>::field_of("force_level", &jedi_table);
    
    let query = jedi_table.select_all().exclude(force_level.lt(100i8));
    assert_sql!(query, "SELECT * FROM jedi WHERE force_level >= 100;");     

    let query = jedi_table.select_all().exclude(force_level.lte(100i8));
    assert_sql!(query, "SELECT * FROM jedi WHERE force_level > 100;"); 

    let query = jedi_table.select_all().exclude(force_level.gt(100i8));
    assert_sql!(query, "SELECT * FROM jedi WHERE force_level <= 100;"); 

    let query = jedi_table.select_all().exclude(force_level.gte(100i8));
    assert_sql!(query, "SELECT * FROM jedi WHERE force_level < 100;"); 
}