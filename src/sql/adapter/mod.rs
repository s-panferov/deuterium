pub trait SqlAdapter {
    fn placeholder(&self, idx: u8) -> String;
}

#[derive(Copy, Clone)]
pub struct PostgreSqlAdapter;

impl SqlAdapter for PostgreSqlAdapter {
    fn placeholder(&self, idx: u8) -> String {
        format!("${}", idx)
    }
}

#[derive(Copy, Clone)]
pub struct MysqlAdapter;

impl SqlAdapter for MysqlAdapter {
    fn placeholder(&self, _: u8) -> String {
        "?".to_owned()
    }
}