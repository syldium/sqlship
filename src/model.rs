use sqlparser::ast::{ColumnDef, ColumnOption, ObjectName, TableConstraint};

#[derive(Debug, Clone)]
pub struct TableDefinition {
    pub name: String,
    pub columns: Vec<ColumnDefinition>,
}

impl TableDefinition {
    pub fn from_ast(
        name: ObjectName,
        columns: Vec<ColumnDef>,
        constraints: Vec<TableConstraint>,
    ) -> Self {
        let mut cols: Vec<ColumnDefinition> = columns
            .into_iter()
            .map(ColumnDefinition::from_ast)
            .collect();
        for constraint in constraints {
            match constraint {
                TableConstraint::ForeignKey {
                    name: _,
                    columns,
                    foreign_table,
                    ..
                } => {
                    cols.iter_mut()
                        .filter(|column| columns.iter().any(|c| c.to_string() == column.name))
                        .for_each(|column| {
                            column.references = Some(foreign_table.to_string());
                        });
                }
                TableConstraint::Unique {
                    columns,
                    is_primary,
                    ..
                } => {
                    cols.iter_mut()
                        .filter(|column| columns.iter().any(|c| c.to_string() == column.name))
                        .for_each(|column| {
                            column.uniqueness = Some(if is_primary {
                                Uniqueness::PrimaryKey
                            } else {
                                Uniqueness::Unique
                            });
                        });
                }
                _ => {}
            }
        }
        Self {
            name: name.to_string(),
            columns: cols,
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Uniqueness {
    Unique,
    PrimaryKey,
}

#[derive(Debug, Clone)]
pub struct ColumnDefinition {
    pub name: String,
    pub uniqueness: Option<Uniqueness>,
    pub references: Option<String>,
    non_null: bool,
}

impl ColumnDefinition {
    pub fn from_ast(column: ColumnDef) -> Self {
        let mut col = Self {
            name: column.name.to_string(),
            uniqueness: None,
            references: None,
            non_null: false,
        };
        for option in &column.options {
            match &option.option {
                ColumnOption::ForeignKey { foreign_table, .. } => {
                    col.references = Some(foreign_table.to_string());
                }
                ColumnOption::NotNull => {
                    col.non_null = true;
                }
                ColumnOption::Unique { is_primary } => {
                    col.uniqueness = Some(if *is_primary {
                        Uniqueness::PrimaryKey
                    } else {
                        Uniqueness::Unique
                    });
                }
                _ => {}
            }
        }
        col
    }

    pub fn _is_non_null(&self) -> bool {
        self.non_null || self.uniqueness == Some(Uniqueness::PrimaryKey)
    }
}
