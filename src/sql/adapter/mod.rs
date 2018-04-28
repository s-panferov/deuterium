use std::fmt;

pub trait SqlAdapter: fmt::Debug {
    fn placeholder(&self, idx: u8) -> String;
}

#[derive(Copy, Clone, Debug)]
pub struct PostgreSqlAdapter;

impl SqlAdapter for PostgreSqlAdapter {
    fn placeholder(&self, idx: u8) -> String {
        format!("${}", idx)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MysqlAdapter;

impl SqlAdapter for MysqlAdapter {
    fn placeholder(&self, _: u8) -> String {
        "?".to_owned()
    }
}