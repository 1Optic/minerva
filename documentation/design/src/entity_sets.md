# Entity Sets

## Entity Set History

For reprocessing of historical data and investigating previously calculated
data that uses entity sets, it is neccessary to keep track of the history of
entity sets.

Entity sets are stored as relations between 'entity set' entities and regular
entities. Up until now, relations never stored a history of previous state.
Relations are periodically re-calculated and the old state was always
discarded.

### Storage Of Revisions

Original form of a relation table just stores a source entity Id and a target
entity Id for each relation:

| source_id  | target_id   |
| ---------: | ----------: |
| 1          | 4445        |
| 1          | 4893        |
| 1          | 4895        |
| 1          | 334         |
| 2          | 8899        |
| 2          | 8901        |

With the new history tracking, we store one extra piece of information in the
relation tables; the revision Id:

| Name        | Data Type |
| -----       | --------  |
| revision_id | integer   |
| source_id   | integer   |
| target_id   | integer   |

Example of a relation table with the extra information:

| revision_id  | source_id  | target_id   |
| -----------: | ---------: | ----------: |
| 1            | 1          | 4445        |
| 1            | 1          | 4893        |
| 1            | 1          | 4895        |
| 1            | 1          | 334         |
| 1            | 2          | 8899        |
| 1            | 2          | 8901        |
| 2            | 1          | 4445        |
| 2            | 1          | 334         |
| 2            | 2          | 8899        |
| 2            | 2          | 8901        |

Here we see that in the second revision, 2 relations have been removed.

There is a corresponding meta-data table `relation.revision` for relation types that stores more
information on the revisions:

| Name         | Data Type |
|--------------|-----------|
| type_id      | integer   |
| id           | integer   |
| valid_period | tstzrange |

The relation type table stores the current revision for each type in the column
`current_revision_id` and the retention for revisions in the column
`revision_retention`: 

| Name                | Data Type                                |
|---------------------|------------------------------------------|
| id                  | integer                                  |
| name                | name                                     |
| cardinality         | relation_directory.type_cardinality_enum |
| current_revision_id | integer                                  |
| revision_retention  | interval                                 |

Example of the type table with a relation type:

| id | name             | cardinality | current_revision_id | revision_retention |
|---:|------------------|-------------|--------------------:|--------------------|
|  1 | node->entity_set |             |                   2 | 3 mons             |

