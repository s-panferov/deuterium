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
