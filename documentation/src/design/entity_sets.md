# Entity Sets

## Entity Set History

For reprocessing of historical data and investigating previously calculated
data that uses entity sets, it is neccessary to keep track of the history of
entity sets.

Entity sets used to be stored as relations between 'entity_set' entities and regular
entities. Relations don't store a history of previous state and in this case we
even want to store the history of subsets (per entity_set entity) of the
relation table contents. So to enable proper history tracking, the storage of
entity sets is redesigned using dedicated tables and no longer uses the generic
relation mechanisms.

There will still be an entity type 'entity_set' by default and a regular
corresponding entity table `entity.entity_set`. Names of the entities will be a
combination of attributes that makes the entity set unique and formed as a
[Distinguished Name](https://datatracker.ietf.org/doc/html/rfc1779E) with
commas as separators and without optional spaces around the separators.

### Tables

There is one meta-data table storing which entity sets exist in the 'directory'
schema:

`directory.entity_set`

| Name                      | Data Type                                    |
|---------------------------|----------------------------------------------|
| id                        | serial primary key                           |
| name                      | text                                         |
| entity_type_id            | integer references directory.entity_type(id) |
| entity_id                 | integer                                      |
| owner                     | text                                         |
| group                     | text                                         |
| current_revision_id       | integer                                      |
| revision_retention_period | interval                                     |

There is a schema to store the actual entity set data and corresponding history
data named `entity_set`. Two tables are generated for each entity type; one
with the actual entity set data and one with the revision meta data. Entity
sets of the same entity type are stored in the same tables (data and revision
meta-data).

An example table for the v-cell entity type storing all revisions of all entity
sets of 'v-cell' entities:

`entity_set."v-cell"`

| Name          | Data Type                                          |
|---------------|----------------------------------------------------|
| entity_set_id | integer references directory.entity_set(id)        |
| revision_id   | bigint references entity_set."v-cell_revision"(id) |
| entity_id     | integer                                            |

An example table for the v-cell entity type revision meta data for all entity
sets of 'v-cell' entities:

`entity_set."v-cell_revision"`

| Name            | Data Type                                   |
|-----------------|---------------------------------------------|
| id              | bigserial primary key                       |
| entity_set_id   | integer references directory.entity_set(id) |
| validity_period | tstzrange                                   |

### Creation Of Entity Sets

1. Create a new entity_set entity with a name of 'group=<group>,owner=<owner>,name=<entity_set_name>'.
2. Insert record in the table `directory.entity_set` leaving
   `current_revision_id` with a NULL value.
3. Check if there are already tables in the 'entity_set' schema for entity set
   data and entity set revision meta data matching the entity type of the
   entity set and if not, create them.
4. Create a new revision record in the corresponding revision table setting the
   start of the `validity_period` to the output of the now() function and the end
   of the `validity_period` to NULL.
5. Insert records in the entity set data table for the corresponding entity
   type using the id of the newly created entity set and revision.
6. Set the `current_revision_id` of the entity set to the id of the previously
   created revision.

### Modifying Of Entity Sets

1. Create a new revision record in the corresponding revision table setting the
   start of the `validity_period` to the output of the now() function and the end
   of the validity_period to NULL.
2. If the `current_revision_id` of the entity set is not NULL, update the end
   of the `validity_period` of that revision with the start of the newly
   created revision.
3. Insert records for the updated set of entities in the entity set data table
   using the id of the newly created revision.
5. Set the `current_revision_id` of the entity set to the id of the newly  
   created revision.

### Deleting Of Entity Sets

It needs to be decided if entity sets can be deleted. When entity sets are used
for e.g. aggregation materializations, you want to keep the records of the
entity sets that the aggregations were created from. Otherwise, if you delete
the entity set, you will want to delete corresponding aggregated data.

### Example Data

`directory.entity_type`

| id | name   |
|----|--------|
| 1  | v-cell |

`directory.entity_set`

| id | name    | entity_type_id | owner    | group        | current_revision_id | revision_retention_period |
|----|---------|----------------|----------|--------------|---------------------|---------------------------|
| 1  | highway | 1              | John Doe | optimization | 3                   | 3mons                     |

`entity_set."v-cell"`

| entity_set_id | revision_id | entity_id |
|---------------|-------------|-----------|
| 1             | 1           | 97        |
| 1             | 1           | 109       |
| 1             | 1           | 236       |

`entity_set."v-cell_revision"`

| id | entity_set_id | validity_period                      |
|----|---------------|--------------------------------------|
| 1  | 1             | (2025-01-18 13:44, 2025-01-22 09:07] |
| 2  | 1             | (2025-01-22 09:07, 2025-01-23 08:31] |
| 3  | 1             | (2025-01-23 08:31, ]                 |
