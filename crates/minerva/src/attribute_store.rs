use postgres_protocol::message::frontend::execute;
use serde::{Deserialize, Serialize};
use std::boxed::Box;
use std::fmt;
use std::path::PathBuf;
use tokio_postgres::types::ToSql;
use tokio_postgres::{Client, GenericClient};

use async_trait::async_trait;

type PostgresName = String;

use super::change::{Change, ChangeResult};
use super::error::{ConfigurationError, DatabaseError, Error, RuntimeError};
use crate::meas_value::DataType;

pub mod compact;
pub mod materialize_curr_ptr;

#[derive(Debug, Serialize, Deserialize, Clone, ToSql)]
#[postgres(name = "attribute_descr")]
pub struct Attribute {
    pub name: PostgresName,
    pub data_type: DataType,
    #[serde(default = "default_empty_string")]
    pub description: String,
}

impl fmt::Display for Attribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Attribute({}: {})", &self.name, &self.data_type)
    }
}

fn default_empty_string() -> String {
    String::new()
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct AddAttributes {
    pub attribute_store: AttributeStore,
    pub attributes: Vec<Attribute>,
}

impl fmt::Display for AddAttributes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "AddAttributes({}, {}):",
            &self.attribute_store,
            &self.attributes.len()
        )?;

        for att in &self.attributes {
            writeln!(f, " - {}: {}", att.name, att.data_type)?;
        }

        Ok(())
    }
}

impl fmt::Debug for AddAttributes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "AddAttributes({}, {:?})",
            &self.attribute_store, &self.attributes
        )
    }
}

#[async_trait]
impl Change for AddAttributes {
    async fn apply(&self, client: &mut Client) -> ChangeResult {
        let tx = client.transaction().await?;

        tx.execute("SET citus.multi_shard_modify_mode TO 'sequential'", &[])
            .await?;

        let query = concat!(
            "SELECT attribute_directory.create_attribute(attribute_store, $1::name, $2::text, $3::text) ",
            "FROM attribute_directory.attribute_store ",
            "JOIN directory.data_source ON data_source.id = attribute_store.data_source_id ",
            "JOIN directory.entity_type ON entity_type.id = attribute_store.entity_type_id ",
            "WHERE data_source.name = $4 AND entity_type.name = $5",
        );

        for attribute in &self.attributes {
            tx.query_one(
                query,
                &[
                    &attribute.name,
                    &attribute.data_type.to_string(),
                    &attribute.description,
                    &self.attribute_store.data_source,
                    &self.attribute_store.entity_type,
                ],
            )
            .await
            .map_err(|e| {
                DatabaseError::from_msg(format!("Error adding attributes to attribute store: {e}"))
            })?;
        }

        tx.commit().await?;

        Ok(format!(
            "Added attributes to attribute store '{}'",
            &self.attribute_store
        ))
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct RemoveAttributes {
    pub attribute_store: AttributeStore,
    pub attributes: Vec<String>,
}

impl fmt::Display for RemoveAttributes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "RemoveAttributes({}, {})",
            &self.attribute_store,
            &self.attributes.len()
        )?;

        for att in &self.attributes {
            writeln!(f, " - {att}")?;
        }

        Ok(())
    }
}

impl fmt::Debug for RemoveAttributes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "RemoveAttributes({}, {:?})",
            &self.attribute_store, &self.attributes
        )
    }
}

#[async_trait]
impl Change for RemoveAttributes {
    async fn apply(&self, client: &mut Client) -> ChangeResult {
        let tx = client.transaction().await?;

        let query = concat!(
            "SELECT attribute_directory.drop_attribute(attribute_store, $1) ",
            "FROM attribute_directory.attribute_store ",
            "JOIN directory.data_source ON data_source.id = attribute_store.data_source_id ",
            "JOIN directory.entity_type ON entity_type.id = attribute_store.entity_type_id ",
            "WHERE data_source.name = $2 AND entity_type.name = $3",
        );

        for attribute in &self.attributes {
            tx.query(
                query,
                &[
                    &attribute,
                    &self.attribute_store.data_source,
                    &self.attribute_store.entity_type,
                ],
            )
            .await
            .map_err(|e| {
                DatabaseError::from_msg(format!(
                    "Error removing attribute '{attribute}' from attribute store: {e}"
                ))
            })?;
        }

        tx.commit().await?;

        Ok(format!(
            "Removed {} attributes from attribute store '{}'",
            &self.attributes.len(),
            &self.attribute_store
        ))
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct ChangeAttribute {
    pub attribute_store: AttributeStore,
    pub attribute: Attribute,
}

impl fmt::Display for ChangeAttribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ChangeAttribute({}, {})",
            &self.attribute_store, &self.attribute.name
        )
    }
}

impl fmt::Debug for ChangeAttribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ChangeAttribute({}, {})",
            &self.attribute_store, &self.attribute
        )
    }
}

#[async_trait]
impl Change for ChangeAttribute {
    async fn apply(&self, client: &mut Client) -> ChangeResult {
        let tx = client.transaction().await?;

        let query = concat!(
            "UPDATE attribute_directory.attribute ",
            "SET data_type = $1 ",
            "FROM attribute_directory.attribute_store ",
            "JOIN directory.data_source ON data_source.id = attribute_store.data_source_id ",
            "JOIN directory.entity_type ON entity_type.id = attribute_store.entity_type_id ",
            "WHERE attribute.attribute_store_id = attribute_store.id ",
            "AND attribute.name = $2 AND data_source.name = $3 AND entity_type.name = $4",
        );

        tx.execute(
            query,
            &[
                &self.attribute.data_type,
                &self.attribute.name,
                &self.attribute_store.data_source,
                &self.attribute_store.entity_type,
            ],
        )
        .await
        .map_err(|e| DatabaseError::from_msg(format!("Error changing trend data type: {e}")))?;

        tx.commit().await?;

        Ok(format!(
            "Changed type of attribute '{}' in store '{}'",
            &self.attribute, &self.attribute_store
        ))
    }
}

#[derive(Default, Debug)]
pub struct AttributeStoreDiffOptions {
    pub ignore_deletions: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AttributeStore {
    pub data_source: String,
    pub entity_type: String,
    pub attributes: Vec<Attribute>,
}

impl AttributeStore {
    #[must_use]
    pub fn diff(
        &self,
        other: &AttributeStore,
        options: AttributeStoreDiffOptions,
    ) -> Vec<Box<dyn Change + Send>> {
        let mut changes: Vec<Box<dyn Change + Send>> = Vec::new();

        let mut new_attributes: Vec<Attribute> = Vec::new();

        for other_attribute in &other.attributes {
            match self
                .attributes
                .iter()
                .find(|my_part| my_part.name == other_attribute.name)
            {
                Some(my_part) => {
                    if my_part.data_type != other_attribute.data_type {
                        changes.push(Box::new(ChangeAttribute {
                            attribute_store: self.clone(),
                            attribute: other_attribute.clone(),
                        }));
                    }
                }
                None => {
                    new_attributes.push(other_attribute.clone());
                }
            }
        }

        if !new_attributes.is_empty() {
            changes.push(Box::new(AddAttributes {
                attribute_store: self.clone(),
                attributes: new_attributes,
            }));
        }

        let mut removed_attributes: Vec<String> = Vec::new();

        for my_attribute in &self.attributes {
            match other
                .attributes
                .iter()
                .find(|other_attribute| other_attribute.name == my_attribute.name)
            {
                Some(_) => {
                    //println!("Still exists: '{}' - '{}' - '{}'", self.data_source, self.entity_type, my_attribute.name);
                    // Ok, the attribute still exists
                }
                None => {
                    //println!("No longer exists: '{}' - '{}' - '{}'", self.data_source, self.entity_type, my_attribute.name);
                    removed_attributes.push(my_attribute.name.clone());
                }
            }
        }

        if !options.ignore_deletions && !removed_attributes.is_empty() {
            changes.push(Box::new(RemoveAttributes {
                attribute_store: self.clone(),
                attributes: removed_attributes,
            }));
        }

        changes
    }
}

impl fmt::Display for AttributeStore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "AttributeStore({}, {})",
            &self.data_source, &self.entity_type
        )
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct AddAttributeStore {
    pub attribute_store: AttributeStore,
}

impl fmt::Display for AddAttributeStore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AddAttributeStore({})", &self.attribute_store)
    }
}

#[async_trait]
impl Change for AddAttributeStore {
    async fn apply(&self, client: &mut Client) -> ChangeResult {
        let tx = client.transaction().await?;

        tx.execute("SET citus.multi_shard_modify_mode TO 'sequential'", &[])
            .await?;

        let query = concat!(
            "CALL attribute_directory.create_attribute_store(",
            "$1::text, $2::text, ",
            "$3::attribute_directory.attribute_descr[]",
            ")"
        );

        tx.execute(
            query,
            &[
                &self.attribute_store.data_source,
                &self.attribute_store.entity_type,
                &self.attribute_store.attributes,
            ],
        )
        .await
        .map_err(|e| {
            DatabaseError::from_msg(format!(
                "Error creating attribute store '{}': {e}",
                &self.attribute_store
            ))
        })?;

        tx.commit().await?;

        Ok(format!(
            "Created attribute store '{}'",
            &self.attribute_store
        ))
    }
}

pub async fn load_attribute_stores<T: GenericClient + Send + Sync>(
    conn: &mut T,
) -> Result<Vec<AttributeStore>, Error> {
    let mut attribute_stores: Vec<AttributeStore> = Vec::new();

    let query = concat!(
        "SELECT attribute_store.id, data_source.name, entity_type.name ",
        "FROM attribute_directory.attribute_store ",
        "JOIN directory.data_source ON data_source.id = attribute_store.data_source_id ",
        "JOIN directory.entity_type ON entity_type.id = attribute_store.entity_type_id"
    );

    let result = conn
        .query(query, &[])
        .await
        .map_err(|e| DatabaseError::from_msg(format!("Error loading attribute stores: {e}")))?;

    for row in result {
        let attribute_store_id: i32 = row.get(0);
        let data_source: &str = row.get(1);
        let entity_type: &str = row.get(2);

        let attributes = load_attributes(conn, attribute_store_id).await;

        attribute_stores.push(AttributeStore {
            data_source: String::from(data_source),
            entity_type: String::from(entity_type),
            attributes,
        });
    }

    Ok(attribute_stores)
}

pub async fn load_attribute_store<T: GenericClient + Send + Sync>(
    conn: &mut T,
    data_source: &str,
    entity_type: &str,
) -> Result<AttributeStore, Error> {
    let query = concat!(
        "SELECT attribute_store.id ",
        "FROM attribute_directory.attribute_store ",
        "JOIN directory.data_source ON data_source.id = attribute_store.data_source_id ",
        "JOIN directory.entity_type ON entity_type.id = attribute_store.entity_type_id ",
        "WHERE data_source.name = $1 AND entity_type.name = $2"
    );

    let result = conn
        .query_one(query, &[&data_source, &entity_type])
        .await
        .map_err(|e| DatabaseError::from_msg(format!("Could not load attribute stores: {e}")))?;

    let attributes = load_attributes(conn, result.get::<usize, i32>(0)).await;

    Ok(AttributeStore {
        data_source: String::from(data_source),
        entity_type: String::from(entity_type),
        attributes,
    })
}

pub async fn load_attributes<T: GenericClient + Send + Sync>(
    conn: &T,
    attribute_store_id: i32,
) -> Vec<Attribute> {
    let attribute_query = "SELECT name, data_type, description FROM attribute_directory.attribute WHERE attribute_store_id = $1";
    let attribute_result = conn
        .query(attribute_query, &[&attribute_store_id])
        .await
        .unwrap();

    let mut attributes: Vec<Attribute> = Vec::new();

    for attribute_row in attribute_result {
        let attribute_name: &str = attribute_row.get(0);
        let attribute_data_type: &str = attribute_row.get(1);
        let attribute_description: Option<String> = attribute_row.get(2);

        attributes.push(Attribute {
            name: String::from(attribute_name),
            data_type: DataType::from(attribute_data_type),
            description: attribute_description.unwrap_or_default(),
        });
    }

    attributes
}

pub async fn load_attribute_names<T: GenericClient + Send + Sync>(
    conn: &T,
    attribute_store_id: i32,
) -> Result<Vec<String>, String> {
    let query = "SELECT name FROM attribute_directory.attribute WHERE attribute_store_id = $1";
    let rows = conn
        .query(query, &[&attribute_store_id])
        .await
        .map_err(|e| format!("Could not load attribute names: {e}"))?;

    Ok(rows.iter().map(|row| row.get::<usize, String>(0)).collect())
}

pub fn load_attribute_store_from_file(path: &PathBuf) -> Result<AttributeStore, Error> {
    let f = std::fs::File::open(path).map_err(|e| {
        ConfigurationError::from_msg(format!(
            "Could not open attribute store definition file '{}': {}",
            path.display(),
            e
        ))
    })?;

    let trend_store: AttributeStore = serde_yaml::from_reader(f).map_err(|e| {
        RuntimeError::from_msg(format!(
            "Could not read trend store definition from file '{}': {}",
            path.display(),
            e
        ))
    })?;

    Ok(trend_store)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diff_added_attributes() {
        let my_attribute_store = AttributeStore {
            data_source: "test".to_string(),
            entity_type: "node".to_string(),
            attributes: vec![],
        };

        let other_attribute_store = AttributeStore {
            data_source: "test".to_string(),
            entity_type: "node".to_string(),
            attributes: vec![Attribute {
                name: "equipment_type".to_string(),
                data_type: DataType::Text,
                description: "Type name from vendor".to_string(),
            }],
        };

        let diff_options = AttributeStoreDiffOptions {
            ignore_deletions: false,
        };

        let changes = my_attribute_store.diff(&other_attribute_store, diff_options);

        assert_eq!(changes.len(), 1);
        let first_change = changes.first().expect("Should have a change");

        assert_eq!(
            first_change.to_string(),
            "AddAttributes(AttributeStore(test, node), 1):\n - equipment_type: text\n"
        );
    }

    #[test]
    fn test_diff_removed_attributes() {
        let my_attribute_store = AttributeStore {
            data_source: "test".to_string(),
            entity_type: "node".to_string(),
            attributes: vec![Attribute {
                name: "equipment_type".to_string(),
                data_type: DataType::Text,
                description: "Type name from vendor".to_string(),
            }],
        };

        let other_attribute_store = AttributeStore {
            data_source: "test".to_string(),
            entity_type: "node".to_string(),
            attributes: vec![],
        };

        let diff_options = AttributeStoreDiffOptions {
            ignore_deletions: false,
        };

        let changes = my_attribute_store.diff(&other_attribute_store, diff_options);

        assert_eq!(changes.len(), 1);
        let first_change = changes.first().expect("Should have a change");

        assert_eq!(
            first_change.to_string(),
            "RemoveAttributes(AttributeStore(test, node), 1)\n - equipment_type\n"
        );
    }
}
