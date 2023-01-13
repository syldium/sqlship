use crate::model::TableDefinition;
use anyhow::Result;
use std::io::Write;

pub fn generate_graphviz(writer: &mut dyn Write, tables: Vec<TableDefinition>) -> Result<()> {
    writeln!(writer, "digraph {{")?;
    writeln!(writer, "    node [shape=plain]\n    rankdir=LR;")?;
    for table in &tables {
        writeln!(writer, "    {}[label=<", table.name)?;
        writeln!(writer, "<table bgcolor=\"#FEE090\" border=\"0\" cellspacing=\"0\" cellpadding=\"4\" cellborder=\"0\">")?;
        writeln!(
            writer,
            "  <tr><td bgcolor=\"#FDAE61\">{}</td></tr>",
            table.name.to_uppercase()
        )?;
        for column in &table.columns {
            writeln!(writer, "  <tr><td>{}</td></tr>", column.name)?;
        }
        writeln!(writer, "</table>>];")?;
    }
    for table in &tables {
        for column in &table.columns {
            if let Some(references) = &column.references {
                writeln!(writer, "    {} -> {};", table.name, references)?;
            }
        }
    }
    writeln!(writer, "}}")?;
    Ok(())
}
