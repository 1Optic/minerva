# Minerva Notification Triggers

## Description Format

The description of a trigger is stored in the description field as Markdown.

The Markdown flavor is a combination of CommonMark and GitHub Flavored Markdown as supported by [Marked.js](https://marked.js.org/#specifications)

It can be assumed that mermaid support is available when rendering the Markdown.

## Thresholds

### Threshold History

The history of threshold settings can be very usefull for explaining and
trouble-shooting previously created notifications because there is a record of
what the thresholds were at the time of the notification creation.

Each revision is a record with the threshold settings for a specific period.
The start of the period is the creation time and the end of the period is
filled in when a new revision of the thresholds is created with the start of
the new revision. The most recent revision always has an open end.

A table contains each revision of the thresholds with the respective start and
end of the validity period:

| Name               | Data Type               |
|--------------------|-------------------------|
| revision           | integer                 |
| valid_period       | tstzrange               |
| \<threshold_name\> | \<threshold_data_type\> |
| \<threshold_name\> | \<threshold_data_type\> |
| \<threshold_name\> | \<threshold_data_type\> |

The tables are named `<trigger_name>_history`.

An example of such a table for a specific trigger:

| revision | valid_period                             | max_power |
|---------:|------------------------------------------|----------:|
|        1 | ["2025-01-12 13:50", "2025-01-15 14:44") |      0.05 |
|        2 | ["2025-01-15 14:44", "2025-01-19 12:59") |     0.045 |
|        3 | ["2025-01-19 12:59", "2025-01-22 17:01") |     0.046 |
|        4 | ["2025-01-22 17:01",)                    |     0.047 |

The current threshold revision number is stored with the trigger rule:

| id | name                    | notification_store_id | granularity | default_interval | enabled | threshold_revision | description |
|---:|-------------------------|----------------------:|-------------|------------------|---------|-------------------:|-------------|
|  1 | node/15m/highpowerusage |                     1 | 00:15:00    |                  | t       |                  4 |             |

A view is used to deliver the current thresholds for use in trigger rules:

| max_power |
|----------:|
|     0.047 |

The view just selects the record matching the `threshold_revision` value with
the `revision` column in the threshold history table.

```
SELECT max_power
FROM trigger_rule."node/15m/highpowerusage_history" h
JOIN trigger.rule ON rule.threshold_revision = h.revision
WHERE rule.name = 'node/15m/highpowerusage';
```
