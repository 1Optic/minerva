# DDAs

Data Delivery Agreements (DDAs) are documents that describe multiple aspects of
the exchange of data between a producer and a consumer. In Minerva this applies
to both ends of the system: the end where data is ingested and the end where
data is exported. For each piece of data that a consumer sees, there will be
a DDA that states:

- Who is the owner
- What is the retention
- What are technical aspects of the data like datatype, value range etc.

```mermaid
erDiagram
  "trend_directory.table_trend" {
      serial id PK
      integer trend_store_part_id FK
      name name
      text data_type
      jsonb extra_data
      text description
      text time_aggregation
      text entity_aggregation
      integer dda_id FK
  }
  "trend_directory.generated_table_trend" {
      serial id PK
      integer trend_store_part_id FK
      name name
      text data_type
      text expression
      jsonb extra_data
      text description
      integer dda_id FK
  }
  "directory.dda" {
    integer id PK
    bigint current_revision_id FK
    timestamptz created
  }
  "directory.dda_revision" {
    bigint revision PK 
    timestamptz timestamp
    text title
    text contents
  }
  "directory.dda" ||--|{ "trend_directory.table_trend" : contains
  "directory.dda" ||--|{ "trend_directory.generated_table_trend" : contains
  "directory.dda" ||--|{ "directory.dda_revision" : has
```
