use anyhow::Result;
use clap::Parser;
use sqlparser::ast::Statement;
use sqlparser::dialect::Dialect;
use sqlparser::parser::Parser as SqlParser;
use std::ops::Deref;

mod cli;
mod er;
mod model;
mod schema;

fn main() -> Result<()> {
    let cli = cli::Cli::parse();
    let input = cli.read()?;
    let dialect: Box<dyn Dialect> = cli.dialect.into();
    let statements = SqlParser::parse_sql(dialect.deref(), &input)?;
    let mut definitions = Vec::new();
    for statement in statements {
        if let Statement::CreateTable {
            name,
            columns,
            constraints,
            ..
        } = statement
        {
            definitions.push(model::TableDefinition::from_ast(name, columns, constraints));
        }
    }
    match cli.generate_schema {
        cli::SchemaGenerator::Graphviz => {
            schema::generate_graphviz(&mut std::io::stdout(), &definitions)?
        }
        cli::SchemaGenerator::Mocodo => {
            let model = er::EntityRelationships::from_tables(&definitions);
            schema::generate_mocodo(&mut std::io::stdout(), &model)?
        }
    }
    Ok(())
}
