# DDAs

## Introduction

As for every data platform, it is a given that there is data incoming and data
outgoing. For both sides, you want to have agreements in place about aspects
such as when data is provided, what format it has and who to contact in case of
issues or planned changes.

We want to have these agreements stored next to the data and linked to the
data. this way, users and system operators have quick access to the details of
these agreements.

## Requirements

The following requirements need to be fulfilled.

1. Each trend and attribute has a corresponding dda
2. A DDA can change over time and the history needs to be kept

## Detailed Design

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
