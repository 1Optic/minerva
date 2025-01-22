# Default Alias Column

The trend data is in most Minerva deployments the data with the highest volume
and the most queried. Because this data is often queried by end-users for e.g.
Grafana dashboards, it needs to be performant and easy to use. Currently, users
always need to join the trend data with at least an entity table or an alias
table to filter on user recognisable names.

To make it easier for the user and potentially quicker, we introduce an
optional 'default-alias' column for trend store tables, next to the existing
entity_id column. The goal is that this column should replace the entity_id
column and joined entity or alias table for most end-user use cases.

# Requirements

1. The default alias column can be optionally defined for each trend store
   part. A lot of trend store parts might not be used by end-users and
   therefore this would only introduce unused redundant data.
2. The addition of the default alias value must not result in extra lookups
   or joins to prevent performance degradation on ingestion.
3. The default alias column must not be included in a unique constraint to
   prevent unexpected collisions during ingestion.
4. The default alias values should correspond with entity names or aliases.

# Detailed Design

## Basic Form In The Database

Here is an example of a trend store part table with a default alias column:

| Name      | Data Type   |
|-----------|-------------|
| entity_id | integer     |
| timestamp | timestamptz |
| alias     | text        |
| created   | timestamptz |
| job_id    | bigint      |
| attempts  | bigint      |
| successes | bigint      |
| tx_bytes  | bigint      |

The data type of the alias column is always `text`, even when the value is
always numeric.


## Trend Store Part Meta Data

The configuration for a default alias column resides with the trend store part
as extra meta data in the form of a boolean column `default_alias`:

| Name           | Data Type    |
|----------------|--------------|
| id             | integer      |
| name           | name         |
| trend_store_id | integer      |
| default_alias  | boolean      |

When `default_alias` is set to True, an extra column named `alias` will be
created in the trend store part tables and otherwise not.

## Populating Default Alias Data

The values in the default alias column are loaded together with the entity_id,
timestamp, job_id and trend values in one go using COPY FROM or INSERT queries.
The values are calculated by the ingestion module that is used to load the
trend data.

Normally, the value will be derived from the same data that is used for the
entity lookup. This could be the name of the entity or a part of the name.
