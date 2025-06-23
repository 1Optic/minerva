# Generic Event Based Updating

For a data platform as Minerva, it is essential that all data is as up-to-date
as possible using as little resources as possible. For trend materialization,
an event-based updating mechanism has been in use for a long time and this
works really well. We want to use a similar mechanism for all such updating
based on changes. A few concrete examples:

1. Updating of relation tables
2. Updating of alias tables
3. External data that is based on data in Minerva

## Approach

1. A generic stream of change events is stored in a table as an audit log with an
   autoincrement Id.
2. The state of Because the different types of data have different identifiable chunks of
   data to trigger changes on, we treat them differently.


### Streams Of Events Per Data Type

The change events are stored in a separate log table for each type of data.

### Entity Relationship Diagram

```mermaid
erDiagram
    trend_event_log {
        id bigserial PK
        timestamp timestamptz
        trend_store_part_id integer FK
        data_timestamp timestamptz
    }

    event_subscriber {
        id serial PK
        name text UK
        created timestamptz
    }

    trend_event_subscription {
        id serial PK
        name text
        created timestamptz
        event_subscriber_id integer FK
        trend_store_part_id integer FK
    }

    event_subscriber ||--o{ trend_event_subscription : has

    trend_event_subscription_state {
        trend_event_subscription_id integer PK,FK
        timestamp timestamptz
        last_processed_event_id bigint FK
    }

    trend_event_subscription ||--|| trend_event_subscription_state : has_state
    trend_event_subscription_state }o--|| trend_event_log : tracks

    attribute_event_log {
        id bigserial PK
        timestamp timestamptz
        attribute_store_id integer FK
        max_attribute_id bigint
    }

    attribute_event_subscription {
        id serial PK
        name text
        created timestamptz
        event_subscriber_id integer FK
        attribute_store_id integer FK
    }

    event_subscriber ||--o{ attribute_event_subscription : has

    attribute_event_subscription_state {
        attribute_event_subscription_id integer PK,FK
        timestamp timestamptz
        last_processed_event_id bigint FK
    }

    attribute_event_subscription ||--|| attribute_event_subscription_state : has_state
    attribute_event_subscription_state }o--|| attribute_event_log : tracks

    notification_event_log {
        id bigserial PK
        timestamp timestamptz
        notification_store_id integer FK
        max_notification_id bigint
    }

    notification_event_subscription {
        id serial PK
        name text
        created timestamptz
        event_subscriber_id integer FK
        notification_store_id integer FK
    }

    event_subscriber ||--o{ notification_event_subscription : has

    notification_event_subscription_state {
        notification_event_subscription_id integer PK,FK
        timestamp timestamptz
        last_processed_event_id bigint FK
    }

    notification_event_subscription ||--|| notification_event_subscription_state : has_state
    notification_event_subscription_state }o--|| notification_event_log : tracks

    relation_event_log {
        id bigserial PK
        timestamp timestamptz
        type_id integer FK
    }

    relation_event_subscription {
        id serial PK
        name text
        created timestamptz
        event_subscriber_id integer FK
        type_id integer FK
    }

    relation_event_subscription_state {
        notification_event_subscription_id integer PK,FK
        timestamp timestamptz
        last_processed_event_id bigint FK
    }

    event_subscriber ||--o{ relation_event_subscription : has
    relation_event_subscription ||--|| relation_event_subscription_state : has_state
    relation_event_subscription_state }o--|| relation_event_log : tracks

    entity_event_log {
        id bigserial PK
        timestamp timestamptz
        entity_type_id integer FK
        max_entity_id bigint
    }

    entity_event_subscription {
        id serial PK
        name text
        created timestamptz
        event_subscriber_id integer FK
        entity_type_id integer FK
    }

    entity_event_subscription_state {
        entity_event_subscription_id integer PK,FK
        timestamp timestamptz
        last_processed_event_id bigint FK
    }

    event_subscriber ||--o{ entity_event_subscription : has
    entity_event_subscription ||--|| entity_event_subscription_state : has_state
    entity_event_subscription_state }o--|| entity_event_log : tracks
```
