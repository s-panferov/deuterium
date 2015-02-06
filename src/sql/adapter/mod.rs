pub trait SqlAdapter {
    fn placeholder(&self, idx: usize) -> String;
}

#[derive(Copy)]
pub struct PostgreSqlAdapter;

impl SqlAdapter for PostgreSqlAdapter {
    fn placeholder(&self, idx: usize) -> String {
        format!("${}", idx)
    }
}