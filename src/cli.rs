use anyhow::{Context, Result};
use std::io::BufRead;
use std::path::PathBuf;
use std::{fs, io};

use clap::{Parser, ValueEnum};
use sqlparser::dialect::*;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long, value_name = "FILE")]
    pub input_file: Option<PathBuf>,

    #[arg(
        short,
        long,
        value_name = "DIALECT",
        value_enum,
        default_value = "postgres"
    )]
    pub dialect: SqlDialect,

    #[arg(
        short,
        long,
        value_name = "SCHEMA",
        value_enum,
        default_value = "graphviz"
    )]
    pub generate_schema: SchemaGenerator,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum SchemaGenerator {
    Graphviz,
    Mocodo,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum SqlDialect {
    Ansi,
    BigQuery,
    Postgres,
    MsSql,
    MySql,
    Snowflake,
    Hive,
    Redshift,
    Generic,
}

impl Cli {
    pub fn read(&self) -> Result<String> {
        match &self.input_file {
            None => {
                let mut buffer = String::new();
                for line in io::stdin().lock().lines() {
                    buffer.push_str(&line?);
                }
                Ok(buffer)
            }
            Some(path) => {
                let content = fs::read_to_string(path).context("Failed to read file")?;
                Ok(content)
            }
        }
    }
}

impl From<SqlDialect> for Box<dyn Dialect> {
    fn from(value: SqlDialect) -> Self {
        match value {
            SqlDialect::Ansi => Box::new(AnsiDialect {}),
            SqlDialect::BigQuery => Box::new(BigQueryDialect {}),
            SqlDialect::Postgres => Box::new(PostgreSqlDialect {}),
            SqlDialect::MsSql => Box::new(MsSqlDialect {}),
            SqlDialect::MySql => Box::new(MySqlDialect {}),
            SqlDialect::Snowflake => Box::new(SnowflakeDialect {}),
            SqlDialect::Hive => Box::new(HiveDialect {}),
            SqlDialect::Redshift => Box::new(RedshiftSqlDialect {}),
            SqlDialect::Generic => Box::new(GenericDialect {}),
        }
    }
}
