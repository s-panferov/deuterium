

pub trait SqlAdapter {
    fn placeholder(&self, idx: uint) -> String;
}

#[deriving(Copy)]
pub struct PostgreSqlAdapter;

impl SqlAdapter for PostgreSqlAdapter {
    fn placeholder(&self, idx: uint) -> String {
        format!("${}", idx)
    }
}