use deuterium::*;

#[test]
fn simple_where() {

    let jedi_table = TableDef::new("jedi");
    let name = NamedField::<String>::field_of("name", &jedi_table);
    
    let query = jedi_table.select_all().where_(name.is("Luke".to_string()));
    assert_sql!(query, "SELECT * FROM jedi WHERE name = $1;");
}

#[test]
fn number_cast() {

    let jedi_table = TableDef::new("jedi");
    let count_i8 = NamedField::<i8>::field_of("count_i8", &jedi_table);
    
    let query = jedi_table.select_all().where_(count_i8.lt(40f64));
    assert_sql!(query, "SELECT * FROM jedi WHERE count_i8 < $1;");
}

#[test]
fn query_level_and() {

    let jedi_table = TableDef::new("jedi");
    let name = NamedField::<String>::field_of("name", &jedi_table);
    let side = NamedField::<bool>::field_of("side", &jedi_table);
    let force_level = NamedField::<i8>::field_of("force_level", &jedi_table);
    
    let and_sql = "SELECT * FROM jedi WHERE (name = $1) AND (side = $2);";

    let query = jedi_table.select_all().where_(name.is("Luke".to_string())).where_(side.is(true));
    assert_sql!(query, and_sql);

    let query = jedi_table.select_all().where_(name.is("Luke".to_string())).and(side.is(true));
    assert_sql!(query, and_sql);

    let query = query.and(force_level.lt(100i8));
    assert_sql!(query, "SELECT * FROM jedi WHERE ((name = $1) AND (side = $2)) AND (force_level < $3);");
}

#[test]
fn query_level_or() {

    let jedi_table = TableDef::new("jedi");
    let name = NamedField::<String>::field_of("name", &jedi_table);
    let side = NamedField::<bool>::field_of("side", &jedi_table);
    let force_level = NamedField::<i8>::field_of("force_level", &jedi_table);
    
    let query = jedi_table.select_all().where_(name.is("Luke".to_string())).or(side.is(true));
    assert_sql!(query, "SELECT * FROM jedi WHERE (name = $1) OR (side = $2);"); 

    let query = query.or(force_level.lt(100i8));
    assert_sql!(query, "SELECT * FROM jedi WHERE ((name = $1) OR (side = $2)) OR (force_level < $3);");
}

#[test]
fn predicate_and_or() {

    let jedi_table = TableDef::new("jedi");
    let name = NamedField::<String>::field_of("name", &jedi_table);
    let side = NamedField::<bool>::field_of("side", &jedi_table);
    
    let query = jedi_table.select_all().where_(name.is("Luke".to_string()).or(name.is("Joda".to_string()).and(side.is(true))));
    assert_sql!(query, "SELECT * FROM jedi WHERE (name = $1) OR ((name = $2) AND (side = $3));"); 
}

#[test]
fn exclude_and() {

    let jedi_table = TableDef::new("jedi");
    let name = NamedField::<String>::field_of("name", &jedi_table);
    let side = NamedField::<bool>::field_of("side", &jedi_table);
    
    let query = jedi_table.select_all().exclude(name.is("Luke".to_string()).and(side.is(true)));
    assert_sql!(query, "SELECT * FROM jedi WHERE (name != $1) AND (side != $2);"); 
}

#[test]
fn exclude_or() {

    let jedi_table = TableDef::new("jedi");
    let name = NamedField::<String>::field_of("name", &jedi_table);
    let side = NamedField::<bool>::field_of("side", &jedi_table);
    
    let query = jedi_table.select_all().exclude(name.is("Luke".to_string()).or(side.is(true)));
    assert_sql!(query, "SELECT * FROM jedi WHERE (name != $1) AND (side != $2);"); 
}

#[test]
fn predicate_inequality() {

    let jedi_table = TableDef::new("jedi");
    let force_level = NamedField::<i8>::field_of("force_level", &jedi_table);
    
    let query = jedi_table.select_all().where_(force_level.lt(100i8));
    assert_sql!(query, "SELECT * FROM jedi WHERE force_level < $1;");     

    let query = jedi_table.select_all().where_(force_level.lte(100i8));
    assert_sql!(query, "SELECT * FROM jedi WHERE force_level <= $1;"); 

    let query = jedi_table.select_all().where_(force_level.gt(100i8));
    assert_sql!(query, "SELECT * FROM jedi WHERE force_level > $1;"); 

    let query = jedi_table.select_all().where_(force_level.gte(100i8));
    assert_sql!(query, "SELECT * FROM jedi WHERE force_level >= $1;"); 
}

#[test]
fn predicate_inequality_exclude() {

    let jedi_table = TableDef::new("jedi");
    let force_level = NamedField::<i8>::field_of("force_level", &jedi_table);
    
    let query = jedi_table.select_all().exclude(force_level.lt(100i8));
    assert_sql!(query, "SELECT * FROM jedi WHERE force_level >= $1;");     

    let query = jedi_table.select_all().exclude(force_level.lte(100i8));
    assert_sql!(query, "SELECT * FROM jedi WHERE force_level > $1;"); 

    let query = jedi_table.select_all().exclude(force_level.gt(100i8));
    assert_sql!(query, "SELECT * FROM jedi WHERE force_level <= $1;"); 

    let query = jedi_table.select_all().exclude(force_level.gte(100i8));
    assert_sql!(query, "SELECT * FROM jedi WHERE force_level < $1;"); 
}

#[test]
fn predicate_is_null() {

    let jedi_table = TableDef::new("jedi");
    let force_level = NamedField::<Option<i8>>::field_of("force_level", &jedi_table);
    
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
    assert_sql!(query, "SELECT * FROM jedi WHERE force_level IN ($1, $2);");     

    let query = jedi_table.select_all().exclude(force_level.in_(vec![100i8, 120i8]));
    assert_sql!(query, "SELECT * FROM jedi WHERE force_level NOT IN ($1, $2);");     

}

#[test]
fn predicate_in_subquery() {

    let jedi_table = TableDef::new("jedi");
    let name = NamedField::<String>::field_of("name", &jedi_table);
    let force_level = NamedField::<i8>::field_of("force_level", &jedi_table);
    
    let query = jedi_table.select_all().where_(force_level.in_(
        jedi_table.alias("j").select_1(&force_level).where_(name.is("Anakin".to_string()))
    ));

    assert_sql!(query, "SELECT * FROM jedi WHERE force_level IN (SELECT force_level FROM jedi AS j WHERE name = $1);");     

}

#[test]
fn predicate_in_range() {

    let jedi_table = TableDef::new("jedi");
    let force_level = NamedField::<i8>::field_of("force_level", &jedi_table);
    
    let query = jedi_table.select_all().where_(force_level.in_range(100i8, 120i8));
    assert_sql!(query, "SELECT * FROM jedi WHERE force_level >= $1 AND force_level <= $2;");     
    
    let query = jedi_table.select_all().where_(force_level.in_range_exclude(100i8, 120i8));
    assert_sql!(query, "SELECT * FROM jedi WHERE force_level > $1 AND force_level < $2;");     
    
    let query = jedi_table.select_all().where_(force_level.in_range_exclude_right(100i8, 120i8));
    assert_sql!(query, "SELECT * FROM jedi WHERE force_level >= $1 AND force_level < $2;");  

    let query = jedi_table.select_all().where_(force_level.in_range_exclude_left(100i8, 120i8));
    assert_sql!(query, "SELECT * FROM jedi WHERE force_level > $1 AND force_level <= $2;");     

}

#[test]
fn predicate_like() {

    let jedi_table = TableDef::new("jedi");
    let name = NamedField::<String>::field_of("name", &jedi_table);
    
    let query = jedi_table.select_all().where_(name.like("Luke%".to_string()));
    assert_sql!(query, "SELECT * FROM jedi WHERE name LIKE $1;");         

    let query = jedi_table.select_all().where_(name.ilike("Luke%".to_string()));
    assert_sql!(query, "SELECT * FROM jedi WHERE name ILIKE $1;");  

    let query = jedi_table.select_all().exclude(name.like("Luke%".to_string()));
    assert_sql!(query, "SELECT * FROM jedi WHERE name NOT LIKE $1;");         

    let query = jedi_table.select_all().exclude(name.ilike("Luke%".to_string()));
    assert_sql!(query, "SELECT * FROM jedi WHERE name NOT ILIKE $1;");        

}

#[test]
fn predicate_like_another_string_field() {

    let jedi_table = TableDef::new("jedi");
    let name = NamedField::<String>::field_of("name", &jedi_table);
    
    let query = jedi_table.select_all().where_(name.like(name.clone()));
    assert_sql!(query, "SELECT * FROM jedi WHERE name LIKE name;");                 

}