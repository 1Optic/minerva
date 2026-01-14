use std::boxed::Box;
use std::fmt;
use std::fmt::Display;
use std::path::PathBuf;

use async_trait::async_trait;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL_CONDENSED;
use comfy_table::*;
use postgres_protocol::escape::escape_identifier;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio_postgres::types::ToSql;
use tokio_postgres::{Client, GenericClient};

type PostgresName = String;

use super::change::{Change, ChangeResult, InformationOption};
use super::error::{ConfigurationError, DatabaseError, Error, RuntimeError};
use crate::change::{Changed, MinervaObjectRef};
use crate::entity::{default_entity_id_type, EntityIdType};

use crate::meas_value::DataType;

pub mod compact;
pub mod materialize_curr_ptr;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Attribute {
    pub name: PostgresName,
    pub data_type: DataType,
    #[serde(default = "default_empty_string")]
    pub description: String,
    pub extra_data: Value,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSql)]
#[postgres(name = "attribute_descr")]
pub struct AttributeDescr {
    pub name: PostgresName,
    pub data_type: DataType,
    #[serde(default = "default_empty_string")]
    pub description: String,
}

impl From<&Attribute> for AttributeDescr {
    fn from(value: &Attribute) -> Self {
        AttributeDescr {
            name: value.name.clone(),
            data_type: value.data_type,
            description: value.description.clone(),
        }
    }
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
pub struct AddAttributeStoreAttributes {
    pub attribute_store: AttributeStoreRef,
    pub attributes: Vec<Attribute>,
}

impl fmt::Display for AddAttributeStoreAttributes {
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

impl fmt::Debug for AddAttributeStoreAttributes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "AddAttributes({}, {:?})",
            &self.attribute_store, &self.attributes
        )
    }
}

#[async_trait]
#[typetag::serde]
impl Change for AddAttributeStoreAttributes {
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

        Ok(Box::new(AddedAttributes {
            attribute_store: self.attribute_store.clone(),
            attributes: self.attributes.iter().map(|a| a.name.clone()).collect(),
        }))
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
struct AddedAttributes {
    pub attribute_store: AttributeStoreRef,
    pub attributes: Vec<String>,
}

impl fmt::Display for AddedAttributes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Added attributes to attribute store '{}'",
            self.attribute_store
        )
    }
}

#[typetag::serde]
impl Changed for AddedAttributes {
    fn revert(&self) -> Option<Box<dyn Change>> {
        Some(Box::new(RemoveAttributes {
            attribute_store: self.attribute_store.clone(),
            attributes: self.attributes.clone(),
        }))
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct RemoveAttributes {
    pub attribute_store: AttributeStoreRef,
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
#[typetag::serde]
impl Change for RemoveAttributes {
    async fn apply(&self, client: &mut Client) -> ChangeResult {
        let mut attributes: Vec<Attribute> = Vec::new();
        let tx = client.transaction().await?;

        let query = concat!(
            "SELECT attribute_directory.drop_attribute(attribute_store, $1) ",
            "FROM attribute_directory.attribute_store ",
            "JOIN directory.data_source ON data_source.id = attribute_store.data_source_id ",
            "JOIN directory.entity_type ON entity_type.id = attribute_store.entity_type_id ",
            "WHERE data_source.name = $2 AND entity_type.name = $3",
        );

        for attribute in &self.attributes {
            let full_attribute = load_attribute(&tx, &self.attribute_store, attribute).await?;

            attributes.push(full_attribute);

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

        Ok(Box::new(RemovedAttributes {
            attribute_store: self.attribute_store.clone(),
            attributes,
        }))
    }

    fn information_options(&self) -> Vec<Box<dyn InformationOption>> {
        vec![Box::new(AttributeRemoveValueInformation {
            data_source: self.attribute_store.data_source.clone(),
            entity_type: self.attribute_store.entity_type.clone(),
            attribute_names: self.attributes.clone(),
        })]
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
struct RemovedAttributes {
    pub attribute_store: AttributeStoreRef,
    pub attributes: Vec<Attribute>,
}

impl Display for RemovedAttributes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Removed {} attributes from attribute store '{}'",
            &self.attributes.len(),
            &self.attribute_store
        )
    }
}

#[typetag::serde]
impl Changed for RemovedAttributes {
    fn revert(&self) -> Option<Box<dyn Change>> {
        Some(Box::new(AddAttributeStoreAttributes {
            attribute_store: self.attribute_store.clone(),
            attributes: self.attributes.clone(),
        }))
    }
}

pub struct AttributeRemoveValueInformation {
    pub data_source: String,
    pub entity_type: String,
    pub attribute_names: Vec<String>,
}

#[async_trait]
impl InformationOption for AttributeRemoveValueInformation {
    fn name(&self) -> String {
        "Show attribute value information".to_string()
    }

    async fn retrieve(&self, client: &mut Client) -> Vec<String> {
        let attribute_store_name = format!("{}_{}", &self.data_source, &self.entity_type);
        let expressions: Vec<String> = self
            .attribute_names
            .iter()
            .map(|attribute_name| format!("{}::text", escape_identifier(attribute_name)))
            .collect();
        let expressions_part: String = expressions.join(", ");
        let query = format!(
            "SELECT {} FROM attribute.{} LIMIT 1",
            expressions_part,
            escape_identifier(&attribute_store_name)
        );

        let rows = client.query(&query, &[]).await.unwrap();

        if rows.is_empty() {
            vec!["No data".to_string()]
        } else {
            let row = rows.first().unwrap();

            let values: Vec<Option<String>> = self
                .attribute_names
                .iter()
                .enumerate()
                .map(|(index, _attribute_name)| row.get::<usize, Option<String>>(index))
                .collect();

            let mut table = Table::new();

            table
                .load_preset(UTF8_FULL_CONDENSED)
                .apply_modifier(UTF8_ROUND_CORNERS)
                .set_header(vec!["attribute", "value"]);

            for (index, attribute_name) in self.attribute_names.iter().enumerate() {
                let value = values.get(index);

                table.add_row(vec![
                    Cell::new(attribute_name.clone()),
                    Cell::new(format!("{value:?}")),
                ]);
            }

            table.lines().collect()
        }
    }
}

impl Display for AttributeRemoveValueInformation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.name())
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct ChangeAttribute {
    pub attribute_store: AttributeStoreRef,
    pub attribute_name: String,
    pub data_type: DataType,
}

impl fmt::Display for ChangeAttribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ChangeAttribute({}, {})",
            &self.attribute_store, &self.attribute_name
        )
    }
}

impl fmt::Debug for ChangeAttribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ChangeAttribute({}, {})",
            &self.attribute_store, &self.attribute_name
        )
    }
}

#[async_trait]
#[typetag::serde]
impl Change for ChangeAttribute {
    async fn apply(&self, client: &mut Client) -> ChangeResult {
        let tx = client.transaction().await?;

        tx.execute("SET citus.multi_shard_modify_mode TO 'sequential'", &[])
            .await?;

        // Load information for later reverting this change
        let original_attribute = load_attribute(&tx, &self.attribute_store, &self.attribute_name)
            .await
            .map_err(|e| RuntimeError::from_msg(format!("Could not load attribute: {e}")))?;

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
                &self.data_type,
                &self.attribute_name,
                &self.attribute_store.data_source,
                &self.attribute_store.entity_type,
            ],
        )
        .await
        .map_err(|e| DatabaseError::from_postgres_error("Error changing attribute data type", e))?;

        tx.commit().await?;

        Ok(Box::new(ChangedAttributeDataType {
            attribute_store: self.attribute_store.clone(),
            attribute_name: self.attribute_name.clone(),
            original_data_type: original_attribute.data_type,
            new_data_type: self.data_type,
        }))
    }

    fn existing_object(&self) -> Option<MinervaObjectRef> {
        Some(MinervaObjectRef::AttributeStore(format!(
            "{}_{}",
            self.attribute_store.data_source, self.attribute_store.entity_type
        )))
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct ChangedAttributeDataType {
    pub attribute_store: AttributeStoreRef,
    pub attribute_name: String,
    pub original_data_type: DataType,
    pub new_data_type: DataType,
}

impl Display for ChangedAttributeDataType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Changed type of attribute '{}' in store '{}'",
            &self.attribute_name, &self.attribute_store
        )
    }
}

#[typetag::serde]
impl Changed for ChangedAttributeDataType {
    fn revert(&self) -> Option<Box<dyn Change>> {
        Some(Box::new(ChangeAttribute {
            attribute_store: self.attribute_store.clone(),
            attribute_name: self.attribute_name.clone(),
            data_type: self.original_data_type,
        }))
    }
}

#[derive(Default, Debug)]
pub struct AttributeStoreDiffOptions {
    pub ignore_deletions: bool,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct AttributeStoreRef {
    pub data_source: String,
    pub entity_type: String,
}

impl fmt::Display for AttributeStoreRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "AttributeStore({}, {})",
            &self.data_source, &self.entity_type
        )
    }
}

impl From<&AttributeStore> for AttributeStoreRef {
    fn from(value: &AttributeStore) -> Self {
        AttributeStoreRef {
            data_source: value.data_source.clone(),
            entity_type: value.entity_type.clone(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AttributeStore {
    pub data_source: String,
    pub entity_type: String,
    pub attributes: Vec<Attribute>,
    #[serde(default = "default_entity_id_type")]
    pub entity_id_type: EntityIdType,
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
                            attribute_store: self.into(),
                            attribute_name: other_attribute.name.clone(),
                            data_type: other_attribute.data_type,
                        }));
                    }
                }
                None => {
                    new_attributes.push(other_attribute.clone());
                }
            }
        }

        if !new_attributes.is_empty() {
            changes.push(Box::new(AddAttributeStoreAttributes {
                attribute_store: self.into(),
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
                    // Ok, the attribute still exists
                }
                None => {
                    removed_attributes.push(my_attribute.name.clone());
                }
            }
        }

        if !options.ignore_deletions && !removed_attributes.is_empty() {
            changes.push(Box::new(RemoveAttributes {
                attribute_store: self.into(),
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
#[typetag::serde]
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
                &self
                    .attribute_store
                    .attributes
                    .iter()
                    .map(AttributeDescr::from)
                    .collect::<Vec<AttributeDescr>>(),
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

        Ok(Box::new(AddedAttributeStore {
            attribute_store: (&self.attribute_store).into(),
        }))
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct AddedAttributeStore {
    pub attribute_store: AttributeStoreRef,
}

impl Display for AddedAttributeStore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Created attribute store '{}'", &self.attribute_store)
    }
}

#[typetag::serde]
impl Changed for AddedAttributeStore {
    fn revert(&self) -> Option<Box<dyn Change>> {
        Some(Box::new(RemoveAttributeStore {
            attribute_store: self.attribute_store.clone(),
        }))
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct RemoveAttributeStore {
    pub attribute_store: AttributeStoreRef,
}

impl Display for RemoveAttributeStore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RemoveAttributeStore({})", &self.attribute_store)
    }
}

#[async_trait]
#[typetag::serde]
impl Change for RemoveAttributeStore {
    async fn apply(&self, _client: &mut Client) -> ChangeResult {
        Err(Error::Runtime(RuntimeError {
            msg: "Not implemented".to_string(),
        }))
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

        let entity_id_type_query = concat!(
            "SELECT format_type(atttypid, null) ",
            "FROM pg_attribute ",
            "JOIN pg_class ON pg_class.oid = pg_attribute.attrelid ",
            "JOIN pg_namespace ON pg_namespace.oid = pg_class.relnamespace ",
            "WHERE pg_class.relname = $1 ",
            "AND pg_attribute.attname = 'entity_id' ",
            "AND pg_namespace.nspname = 'attribute_base'",
        );

        let table_name = format!("{}_{}", data_source, entity_type);

        let entity_id_type_result = conn
            .query(entity_id_type_query, &[&table_name])
            .await
            .unwrap();
        let entity_id_type_row = entity_id_type_result.first().unwrap();
        let entity_id_type_str: &str = entity_id_type_row.get(0);
        let entity_id_type = match entity_id_type_str {
            "int4" => EntityIdType::I32,
            "int8" => EntityIdType::I64,
            "integer" => EntityIdType::I32,
            "bigint" => EntityIdType::I64,
            _ => panic!("Unknown entity_id type '{}'", entity_id_type_str),
        };

        let attributes = load_attributes(conn, attribute_store_id).await;

        attribute_stores.push(AttributeStore {
            data_source: String::from(data_source),
            entity_type: String::from(entity_type),
            attributes,
            entity_id_type,
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

    let entity_id_type_query = concat!(
        "SELECT format_type(atttypid, null) ",
        "FROM pg_attribute ",
        "JOIN pg_class ON pg_class.oid = pg_attribute.attrelid ",
        "JOIN pg_namespace ON pg_namespace.oid = pg_class.relnamespace ",
        "WHERE pg_class.relname = $1 ",
        "AND pg_attribute.attname = 'entity_id' ",
        "AND pg_namespace.nspname = 'attribute_base'",
    );

    let table_name = format!("{}_{}", data_source, entity_type);

    let entity_id_type_result = conn
        .query(entity_id_type_query, &[&table_name])
        .await
        .unwrap();
    let entity_id_type_row = entity_id_type_result.first().unwrap();
    let entity_id_type_str: &str = entity_id_type_row.get(0);
    let entity_id_type = match entity_id_type_str {
        "int4" => EntityIdType::I32,
        "int8" => EntityIdType::I64,
        "integer" => EntityIdType::I32,
        "bigint" => EntityIdType::I64,
        _ => panic!("Unknown entity_id type '{}'", entity_id_type_str),
    };

    let attributes = load_attributes(conn, result.get::<usize, i32>(0)).await;

    Ok(AttributeStore {
        data_source: String::from(data_source),
        entity_type: String::from(entity_type),
        attributes,
        entity_id_type,
    })
}

pub async fn load_attributes<T: GenericClient + Send + Sync>(
    conn: &T,
    attribute_store_id: i32,
) -> Vec<Attribute> {
    let attribute_query = "SELECT name, data_type, description, extra_data FROM attribute_directory.attribute WHERE attribute_store_id = $1";
    let attribute_result = conn
        .query(attribute_query, &[&attribute_store_id])
        .await
        .unwrap();

    let mut attributes: Vec<Attribute> = Vec::new();

    for attribute_row in attribute_result {
        let attribute_name: &str = attribute_row.get(0);
        let attribute_data_type: &str = attribute_row.get(1);
        let attribute_description: Option<String> = attribute_row.get(2);
        let extra_data: Value = attribute_row.get(3);

        attributes.push(Attribute {
            name: String::from(attribute_name),
            data_type: DataType::from(attribute_data_type),
            description: attribute_description.unwrap_or_default(),
            extra_data,
        });
    }

    attributes
}

pub async fn load_attribute<T: GenericClient + Send + Sync>(
    conn: &T,
    attribute_store: &AttributeStoreRef,
    attribute_name: &str,
) -> Result<Attribute, String> {
    let attribute_query = concat!(
        "SELECT att.name, att.data_type, att.description, att.extra_data ",
        "FROM attribute_directory.attribute att ",
        "JOIN attribute_directory.attribute_store ast ON att.attribute_store_id = ast.id ",
        "JOIN directory.data_source ds ON ds.id = ast.data_source_id ",
        "JOIN directory.entity_type et ON et.id = ast.entity_type_id ",
        "WHERE ds.name = $1 AND et.name = $2 AND att.name = $3"
    );

    let attribute_result = conn
        .query(
            attribute_query,
            &[
                &attribute_store.data_source,
                &attribute_store.entity_type,
                &attribute_name,
            ],
        )
        .await
        .unwrap();

    if attribute_result.is_empty() {
        return Err(format!(
            "No such attribute '{}_{}'.'{}'",
            attribute_store.data_source, attribute_store.entity_type, attribute_name
        ));
    }

    let attribute_row = attribute_result.first().unwrap();

    let attribute_name: &str = attribute_row.get(0);
    let attribute_data_type: &str = attribute_row.get(1);
    let attribute_description: Option<String> = attribute_row.get(2);
    let extra_data: Value = attribute_row.get(3);

    Ok(Attribute {
        name: String::from(attribute_name),
        data_type: DataType::from(attribute_data_type),
        description: attribute_description.unwrap_or_default(),
        extra_data,
    })
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
            entity_id_type: default_entity_id_type(),
        };

        let other_attribute_store = AttributeStore {
            data_source: "test".to_string(),
            entity_type: "node".to_string(),
            attributes: vec![Attribute {
                name: "equipment_type".to_string(),
                data_type: DataType::Text,
                description: "Type name from vendor".to_string(),
                extra_data: Value::Null,
            }],
            entity_id_type: default_entity_id_type(),
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
                extra_data: Value::Null,
            }],
            entity_id_type: default_entity_id_type(),
        };

        let other_attribute_store = AttributeStore {
            data_source: "test".to_string(),
            entity_type: "node".to_string(),
            attributes: vec![],
            entity_id_type: default_entity_id_type(),
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
