
pub trait TableSource {
    fn source(&self) -> String;
}

pub struct TableDef {
    name: String
}

impl TableDef {
    pub fn new(name: String) -> TableDef {
        TableDef { name: name }
    }
}

impl TableSource for TableDef {
    fn source(&self) -> String {
        self.name.to_string()
    }
}