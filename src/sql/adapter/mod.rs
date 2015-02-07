pub trait SqlAdapter {
    fn placeholder(&self, idx: u8) -> String;
}

#[derive(Copy)]
pub struct PostgreSqlAdapter;

impl SqlAdapter for PostgreSqlAdapter {
    fn placeholder(&self, idx: u8) -> String {
        format!("${}", idx)
    }
}