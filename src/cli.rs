use anyhow::{Context, Result};
use std::io::BufRead;
use std::path::PathBuf;
use std::{fs, io};

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long, value_name = "FILE")]
    pub input_file: Option<PathBuf>,
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
