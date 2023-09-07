use crate::er::EntityRelationships;
use crate::model::{TableDefinition, Uniqueness};
use anyhow::Result;
use std::io::Write;

pub fn generate_graphviz(writer: &mut impl Write, tables: &Vec<TableDefinition>) -> Result<()> {
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

pub fn generate_mocodo(writer: &mut impl Write, model: &EntityRelationships) -> Result<()> {
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
            write!(writer, "DF{df_counter}")?;
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
            write!(writer, " {field}")?;
        }
        writeln!(writer)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::er::{Entity, EntityProperty, Relationship, RelationshipReference};
    use crate::model::Cardinality;
    use std::io::Cursor;

    #[test]
    fn test_generate_mocodo() {
        let mut buffer = Cursor::new(vec![]);
        let model = EntityRelationships {
            entities: vec![
                Entity {
                    name: "Entity1",
                    properties: vec![
                        EntityProperty {
                            name: "primary1",
                            primary: true,
                        },
                        EntityProperty {
                            name: "primary2",
                            primary: true,
                        },
                        EntityProperty {
                            name: "attribute1",
                            primary: false,
                        },
                        EntityProperty {
                            name: "attribute2",
                            primary: false,
                        },
                    ],
                },
                Entity {
                    name: "Entity2",
                    properties: vec![
                        EntityProperty {
                            name: "primary1",
                            primary: true,
                        },
                        EntityProperty {
                            name: "attribute1",
                            primary: false,
                        },
                        EntityProperty {
                            name: "attribute2",
                            primary: false,
                        },
                    ],
                },
            ],
            relations: vec![Relationship {
                name: None,
                references: vec![
                    RelationshipReference {
                        cardinality: Cardinality::OneToMany,
                        to: "Entity1",
                    },
                    RelationshipReference {
                        cardinality: Cardinality::OneToOne,
                        to: "Entity2",
                    },
                ],
                properties: vec!["attribute2", "attribute3"],
            }],
        };

        assert!(generate_mocodo(&mut buffer, &model).is_ok());
        assert_eq!(
            String::from_utf8(buffer.into_inner()).unwrap(),
            "ENTITY1: primary1, _primary2, attribute1, attribute2
ENTITY2: primary1, attribute1, attribute2
DF1, 1N ENTITY1, 11 ENTITY2: attribute2, attribute3\n"
        );
    }
}
