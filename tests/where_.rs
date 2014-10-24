
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
fn exclude_and() {

    let jedi_table = TableDef::new("jedi");
    let name = NamedField::<String>::field_of("name", &jedi_table);
    let side = NamedField::<bool>::field_of("side", &jedi_table);
    
    let query = jedi_table.select_all().exclude(name.is("Luke").and(side.is(true)));
    assert_sql!(query, "SELECT * FROM jedi WHERE (name != 'Luke') AND (side != true);"); 
}

#[test]
fn exclude_or() {

    let jedi_table = TableDef::new("jedi");
    let name = NamedField::<String>::field_of("name", &jedi_table);
    let side = NamedField::<bool>::field_of("side", &jedi_table);
    
    let query = jedi_table.select_all().exclude(name.is("Luke").or(side.is(true)));
    assert_sql!(query, "SELECT * FROM jedi WHERE (name != 'Luke') AND (side != true);"); 
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

#[test]
fn predicate_is_null() {

    let jedi_table = TableDef::new("jedi");
    let force_level = NamedField::<i8>::field_of("force_level", &jedi_table);
    
    let query = jedi_table.select_all().where_(force_level.is_null());
    assert_sql!(query, "SELECT * FROM jedi WHERE force_level IS NULL;"); 

    let query = jedi_table.select_all().exclude(force_level.is_null());
    assert_sql!(query, "SELECT * FROM jedi WHERE force_level IS NOT NULL;");     

}

#[test]
fn predicate_in() {

    let jedi_table = TableDef::new("jedi");
    let force_level = NamedField::<i8>::field_of("force_level", &jedi_table);
    
    let query = jedi_table.select_all().where_(force_level.in_(vec![100i8, 120i8]));
    assert_sql!(query, "SELECT * FROM jedi WHERE force_level IN (100, 120);");     

    let query = jedi_table.select_all().exclude(force_level.in_(vec![100i8, 120i8]));
    assert_sql!(query, "SELECT * FROM jedi WHERE force_level NOT IN (100, 120);");     

}

#[test]
fn predicate_in_subquery() {

    let jedi_table = TableDef::new("jedi");
    let name = NamedField::<String>::field_of("name", &jedi_table);
    let force_level = NamedField::<i8>::field_of("force_level", &jedi_table);
    
    let query = jedi_table.select_all().where_(force_level.in_(
        jedi_table.alias("j").select_1(&force_level).where_(name.is("Anakin"))
    ));

    assert_sql!(query, "SELECT * FROM jedi WHERE force_level IN (SELECT force_level FROM jedi AS j WHERE name = 'Anakin');");     

}

#[test]
fn predicate_in_range() {

    let jedi_table = TableDef::new("jedi");
    let force_level = NamedField::<i8>::field_of("force_level", &jedi_table);
    
    let query = jedi_table.select_all().where_(force_level.in_range(100i8, 120i8));
    assert_sql!(query, "SELECT * FROM jedi WHERE force_level >= 100 AND force_level <= 120;");     
    
    let query = jedi_table.select_all().where_(force_level.in_range_exclude(100i8, 120i8));
    assert_sql!(query, "SELECT * FROM jedi WHERE force_level > 100 AND force_level < 120;");     
    
    let query = jedi_table.select_all().where_(force_level.in_range_exclude_right(100i8, 120i8));
    assert_sql!(query, "SELECT * FROM jedi WHERE force_level >= 100 AND force_level < 120;");  

    let query = jedi_table.select_all().where_(force_level.in_range_exclude_left(100i8, 120i8));
    assert_sql!(query, "SELECT * FROM jedi WHERE force_level > 100 AND force_level <= 120;");     

}

#[test]
fn predicate_like() {

    let jedi_table = TableDef::new("jedi");
    let name = NamedField::<String>::field_of("name", &jedi_table);
    
    let query = jedi_table.select_all().where_(name.like("Luke%"));
    assert_sql!(query, "SELECT * FROM jedi WHERE name LIKE 'Luke%';");         

    let query = jedi_table.select_all().where_(name.ilike("Luke%"));
    assert_sql!(query, "SELECT * FROM jedi WHERE name ILIKE 'Luke%';");  

    let query = jedi_table.select_all().exclude(name.like("Luke%"));
    assert_sql!(query, "SELECT * FROM jedi WHERE name NOT LIKE 'Luke%';");         

    let query = jedi_table.select_all().exclude(name.ilike("Luke%"));
    assert_sql!(query, "SELECT * FROM jedi WHERE name NOT ILIKE 'Luke%';");        

}