use crate::er::EntityRelationships;
use crate::model::{TableDefinition, Uniqueness};
use anyhow::Result;
use std::io::Write;

pub fn generate_graphviz(writer: &mut dyn Write, tables: &Vec<TableDefinition>) -> Result<()> {
    writeln!(writer, "digraph {{")?;
    writeln!(writer, "    node [shape=plain]\n    rankdir=LR;")?;
    for table in tables {
        writeln!(writer, "    {}[label=<", table.name)?;
        writeln!(writer, "<table bgcolor=\"#FEE090\" border=\"0\" cellspacing=\"0\" cellpadding=\"4\" cellborder=\"0\">")?;
        writeln!(
            writer,
            "  <tr><td bgcolor=\"#FDAE61\">{}</td></tr>",
            table.name.to_uppercase()
        )?;
        for column in &table.columns {
            let mut prefix = "";
            if column.references.is_some() {
                prefix = "#";
            }
            if let Some(Uniqueness::PrimaryKey) = column.uniqueness {
                writeln!(
                    writer,
                    "  <tr><td><u>{}{}</u></td></tr>",
                    prefix, column.name
                )?;
            } else {
                writeln!(writer, "  <tr><td>{}{}</td></tr>", prefix, column.name)?;
            }
        }
        writeln!(writer, "</table>>];")?;
    }
    for table in tables {
        for column in &table.columns {
            if let Some(references) = &column.references {
                writeln!(writer, "    {} -> {};", table.name, references)?;
            }
        }
    }
    writeln!(writer, "}}")?;
    Ok(())
}

pub fn generate_mocodo(writer: &mut dyn Write, model: &EntityRelationships) -> Result<()> {
    // Write each entity on a single line
    // entity: primary1, _primary2, attribute1, attribute2
    for entity in &model.entities {
        write!(writer, "{}:", entity.name.to_uppercase())?;
        for (i, field) in entity.properties.iter().enumerate() {
            if i != 0 {
                write!(writer, ",")?;
            }
            if i != 0 && field.primary {
                write!(writer, " _{}", field.name)?;
            } else {
                write!(writer, " {}", field.name)?;
            }
        }
        writeln!(writer)?;
    }

    let mut df_counter = 0usize;

    // Write each association on a single line
    // association, 0N referred1, 01 referred2: attribute1
    for association in &model.relations {
        if association.name.is_none() {
            df_counter += 1;
            write!(writer, "DF{}", df_counter)?;
        } else {
            write!(writer, "{}", association.name.as_ref().unwrap())?;
        }
        for reference in &association.references {
            write!(
                writer,
                ", {} {}",
                reference.cardinality,
                reference.to.to_uppercase()
            )?;
        }
        write!(writer, ":")?;
        for (i, field) in association.properties.iter().enumerate() {
            if i != 0 {
                write!(writer, ",")?;
            }
            write!(writer, " {}", field)?;
        }
        writeln!(writer)?;
    }
    Ok(())
}
