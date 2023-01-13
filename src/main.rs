use anyhow::Result;
use clap::Parser;
use sqlparser::ast::Statement;
use sqlparser::dialect::PostgreSqlDialect;
use sqlparser::parser::Parser as SqlParser;

mod cli;
mod model;
mod schema;

fn main() -> Result<()> {
    let cli = cli::Cli::parse();
    let input = cli.read()?;
    let statements = SqlParser::parse_sql(&PostgreSqlDialect {}, &input)?;
    let mut definitions = Vec::new();
    for statement in statements {
        if let Statement::CreateTable { name, columns, .. } = statement {
            definitions.push(model::TableDefinition::new(
                name.to_string(),
                columns
                    .into_iter()
                    .map(model::ColumnDefinition::from_ast)
                    .collect(),
            ));
        }
    }
    schema::generate_graphviz(&mut std::io::stdout(), definitions)?;
    Ok(())
}
