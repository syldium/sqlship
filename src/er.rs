use crate::model::{Cardinality, TableDefinition, Uniqueness};

/// An entity model.
#[derive(Debug, Clone)]
pub struct Entity<'a> {
    /// The name of the entity.
    pub name: &'a str,

    /// The properties dedicated to the entity.
    pub properties: Vec<EntityProperty<'a>>,
}

/// A property of an entity in an entity relationship model.
#[derive(Debug, Clone)]
pub struct EntityProperty<'a> {
    /// The name of the property.
    pub name: &'a str,

    /// Whether the property is a primary key.
    pub primary: bool,
}

/// An entity relationship model.
#[derive(Debug, Clone)]
pub struct Relationship<'a> {
    /// The name of the relationship.
    pub name: Option<&'a str>,

    /// The properties dedicated to the relationship.
    pub properties: Vec<&'a str>,

    /// The references of the relationship.
    pub references: Vec<RelationshipReference<'a>>,
}

#[derive(Debug, Clone)]
pub struct RelationshipReference<'a> {
    pub to: &'a str,
    pub cardinality: Cardinality,
}

#[derive(Debug, Clone)]
pub struct EntityRelationships<'a> {
    pub entities: Vec<Entity<'a>>,
    pub relations: Vec<Relationship<'a>>,
}

fn related_name<'a>(relation: &Relationship<'a>, not: &str) -> &'a str {
    relation
        .references
        .iter()
        .find(|r| r.to != not)
        .map(|r| r.to)
        .expect("Invalid relation")
}

/// Remove elements from a vector that match a predicate, map and return them.
fn drain_filter_map<T, U>(
    from: &mut Vec<T>,
    filter: impl Fn(&T) -> bool,
    map: impl Fn(T) -> U,
) -> Vec<U> {
    let mut removed: Vec<U> = vec![];
    let mut i = 0;
    while i < from.len() {
        let e = &from[i];
        if filter(e) {
            let taken_e = from.swap_remove(i);
            removed.push(map(taken_e));
        } else {
            i += 1;
        }
    }
    removed
}

impl EntityRelationships<'_> {
    /// Create an entity relationship model from a list of table definitions.
    ///
    /// This function suppose that individual tables are entities, and that
    /// intermediate tables (where every reference is 1-1) are relationships.
    pub fn from_tables(tables: &Vec<TableDefinition>) -> EntityRelationships {
        let mut entities: Vec<Entity> = vec![];
        let mut relations: Vec<Relationship> = vec![];
        // First pass: create relationships
        for table in tables {
            for column in &table.columns {
                if let Some(reference) = column.references.as_ref() {
                    relations.push({
                        Relationship {
                            name: None,
                            properties: vec![],
                            references: vec![
                                {
                                    RelationshipReference {
                                        to: reference,
                                        cardinality: Cardinality::ZeroToMany,
                                    }
                                },
                                {
                                    RelationshipReference {
                                        to: &table.name,
                                        cardinality: column.cardinality(),
                                    }
                                },
                            ],
                        }
                    });
                }
            }
        }

        // Second pass: create entities or merge relationships
        for table in tables {
            let is_one_one = |relation: &Relationship| -> bool {
                relation.references.iter().any(|relation| {
                    relation.to == table.name && relation.cardinality == Cardinality::OneToOne
                })
            };
            let is_intermediate = relations
                .iter()
                .filter(|&relation| is_one_one(relation))
                .count()
                > 1;
            if is_intermediate {
                let references = drain_filter_map(&mut relations, is_one_one, |reference| {
                    RelationshipReference {
                        to: related_name(&reference, &table.name),
                        cardinality: Cardinality::ZeroToMany,
                    }
                });
                let fields: Vec<&str> = table
                    .columns
                    .iter()
                    .filter(|column| column.references.is_none())
                    .map(|f| f.name.as_ref())
                    .collect();
                relations.push(Relationship {
                    name: Some(&table.name),
                    properties: fields,
                    references,
                });
            } else {
                let mut fields: Vec<EntityProperty> = vec![];
                for column in &table.columns {
                    if column.references.is_none() {
                        fields.push(EntityProperty {
                            name: &column.name,
                            primary: column.uniqueness == Some(Uniqueness::PrimaryKey),
                        });
                    }
                }
                entities.push(Entity {
                    name: &table.name,
                    properties: fields,
                });
            }
        }

        EntityRelationships {
            entities,
            relations,
        }
    }
}
