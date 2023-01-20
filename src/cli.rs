use anyhow::{Context, Result};
use std::io::BufRead;
use std::path::PathBuf;
use std::{fs, io};

use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long, value_name = "FILE")]
    pub input_file: Option<PathBuf>,

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
