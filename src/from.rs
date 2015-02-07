use std::rc;

use super::sql;
use super::select_query;
use super::insert_query;
use super::update_query;
use super::delete_query;
use super::field::{self, Field};

pub trait From { 
    fn as_sql(&self) -> &sql::FromToSql;
    fn upcast_from(&self) -> SharedFrom;
}

pub type BoxedFrom = Box<From + 'static>;
pub type SharedFrom = rc::Rc<BoxedFrom>;

pub trait Table {
    fn upcast_table(&self) -> SharedTable;
    fn get_table_name(&self) -> &String;
    fn get_table_alias(&self) -> &Option<String>;
}

pub type BoxedTable = Box<Table + 'static>;
pub type SharedTable = rc::Rc<BoxedTable>;

#[derive(Clone)]
pub struct TableDef {
    name: String,
    alias: Option<String>
}

// FIXME: Remove after all stuff in insert_query::InsertQuery will be fixed
macro_rules! insert {
    ($name:ident, $(($t:ident, $arg:ident)),+) => (
        #[doc(hidden)]
        fn $name<$($t:Clone,)+>(&self, $($arg: &field::NamedField<$t>,)+) -> insert_query::InsertQuery<($($t,)+), ($(insert_query::InsertValue<$t>,)+), (), (), ()> {
            let mut cols = vec![];
            $(cols.push((*$arg).upcast_field());)+
            insert_query::InsertQuery::new_with_cols(self, cols)
        }
    )
}

#[allow(dead_code)]
impl TableDef {
    pub fn new(name: &str) -> TableDef {
        TableDef { name: name.to_string(), alias: None }
    }

    pub fn new_with_alias(name: &str, alias: &str) -> TableDef {
        TableDef { name: name.to_string(), alias: Some(alias.to_string()) }
    }

    pub fn alias(&self, alias: &str) -> TableDef {
        let mut table_def = self.clone();
        table_def.alias = Some(alias.to_string());
        table_def
    }

    // FIXME: Remove after all stuff in insert_query::InsertQuery will be fixed
    insert!(insert_1, (T0, _t0));

    #[doc(hidden)]
    pub fn insert_1_for_test(&self, name: &field::NamedField<String>) -> insert_query::InsertQuery<(String,), (insert_query::InsertValue<String>,), (), (), ()> {
        self.insert_1(name)
    }
}

impl Table for TableDef {
    fn upcast_table(&self) -> SharedTable {
        rc::Rc::new(Box::new(self.clone()))
    }

    fn get_table_name(&self) -> &String {
        &self.name
    }

    fn get_table_alias(&self) -> &Option<String> {
        &self.alias
    }
}

impl From for TableDef {
    fn as_sql(&self) -> &sql::FromToSql {
        self
    }

    fn upcast_from(&self) -> SharedFrom {
        rc::Rc::new(Box::new(self.clone()))
    }
}

impl select_query::Selectable<()> for TableDef {}
impl insert_query::Insertable<()> for TableDef {}
impl update_query::Updatable<()> for TableDef {}
impl delete_query::Deletable<()> for TableDef {}

#[derive(Clone)]
pub struct FromSelect<T, L, M> {
    pub select: select_query::SelectQuery<T, L, M>,
    pub alias: String 
}

impl<T: Clone, L: Clone, M: Clone> From for FromSelect<T, L, M> {
    fn as_sql(&self) -> &sql::FromToSql {
        self
    }

    fn upcast_from(&self) -> SharedFrom {
        rc::Rc::new(Box::new(self.clone()))
    }
}

impl<T: Clone, L: Clone, M: Clone> select_query::Selectable<M> for FromSelect<T, L, M> {}

