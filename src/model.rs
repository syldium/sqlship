use sqlparser::ast::{ColumnDef, ColumnOption};

pub struct TableDefinition {
    pub name: String,
    pub columns: Vec<ColumnDefinition>,
}

impl TableDefinition {
    pub fn new(name: String, columns: Vec<ColumnDefinition>) -> Self {
        Self { name, columns }
    }
}

pub struct ColumnDefinition {
    pub name: String,
    pub references: Option<String>,
}

impl ColumnDefinition {
    pub fn from_ast(column: ColumnDef) -> Self {
        let references: Option<String> = column.options.iter().find_map(|option| {
            if let ColumnOption::ForeignKey { foreign_table, .. } = &option.option {
                Some(foreign_table.to_string())
            } else {
                None
            }
        });
        Self {
            name: column.name.to_string(),
            references,
        }
    }
}
