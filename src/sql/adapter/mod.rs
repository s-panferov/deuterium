

pub trait SqlAdapter {
    fn placeholder(&self, idx: uint) -> String;
}

pub struct PostgreSqlAdapter;

impl SqlAdapter for PostgreSqlAdapter {
    fn placeholder(&self, idx: uint) -> String {
        format!("${}", idx)
    }
}