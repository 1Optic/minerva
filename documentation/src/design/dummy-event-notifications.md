# Dummy Event Notifications

There are 4 main use cases that drive the need to be able to generate 'dummy' notifications for specific triggers:

1. Document/communicate the expected notifications from a trigger.
2. Being able to test connected services before a trigger implementation is complete.
3. Being able to test connected services without having data that triggers an actual notification.
4. Being able to test connected services with specific values, like edge-cases.

## Requirements

1. The ability to define the exact properties of a dummy notification.
2. The ability to easily recognize dummy notifications so that they can be ignored quickly when you are not interested in them (i.e. when performing regular business tasks).
3. A method to describe the dummy notifications with the trigger specification so that it is versioned together with the trigger.
4. The ability to define multiple variants of dummy notifications.
5. The ability to trigger the generation of a dummy notification at any time (ad-hoc).
6. The ability to choose what variant to generate when triggering a dummy notification.

## Specification Format

The dummy notifications will be defined together with the trigger in the same YAML file under the key `dummy_notifications`. The full schema is specified [here](../design/trigger-schema.md)

Example:
```
name: node/15m/highpowerusage
kpi_data:
  - name: power_kwh
    data_type: numeric
kpi_function: |-
  BEGIN
      RETURN QUERY EXECUTE $query$
      SELECT
          t.entity_id,
          t.timestamp,
          t.power_kwh
      FROM trend."hub_node_main_15m" AS t
      WHERE
          t.timestamp = $1
      $query$ USING $1;
  END; 
thresholds:
  - name: max_power
    data_type: numeric
    value: 0.05
condition: |-
  power_kwh > max_power
weight: |-
  SELECT
      CASE
          WHEN $1.power_kwh > 1 THEN 500
          WHEN $1.power_kwh > 2 THEN 800
          ELSE 300
      END
notification: |-
  SELECT array_to_string(
      ARRAY[
          'HighPowerUsage',
          format('%s > %s', $1.power_kwh, $1.max_power)
      ],
      E'\n'
  )
data: |-
  SELECT json_build_object(
    'power_kwh', $1.power_kwh
  )
tags: ['online']
fingerprint: |-
  SELECT trigger.modified_to_fingerprint(
      ARRAY[
          trend.modified(trend.to_trendstore('hub_node_main_15m'), $1)
      ]::timestamptz[]
  )
notification_store: trigger-notification
trend_store_links:
  - part_name: hub_node_main_15m
    mapping_function: mapping_id
mapping_functions: []
granularity: 15m
description: |-
  |||
  | --- | --- |
  | Description | A sample trigger |
dummy_notifications:
  default_entity: node=Dummy
  variants:
    - name: low
      weight: 100
      data:
        power_kwh: 13.82
    - name: high
      weight: 400
      data:
        power_kwh: 106.29
```

## Deployment

No specific deployment is required for the dummy notifications, except for optional creation of the default entity used for dummy notifications. The dummy entity will be created by default when the trigger is deployed:

```
$ minerva trigger create examples/tiny_instance_v1/trigger/node-15m-highpowerusage.yaml
```

Suppress creation of the dummy entity using the `--no-dummy-entity`:

```
$ minerva trigger create --no-dummy-entity examples/tiny_instance_v1/trigger/node-15m-highpowerusage.yaml
```

## Generating Notifications

Generating notifications can be done in 2 ways:

1. Using the Minerva CLI
2. Using the Minerva Web Console

### Minerva CLI

There will be a new sub-command for the Minerva administration command `minerva trigger dummy-notification`. Example usage of this command:

```
$ minerva trigger dummy-notification 'node/15m/highpowerusage' low
Dummy notification created:

  timestamp: 2025-11-01 14:45:00+01
  created:   2025-11-01 14:55:12+01
  entity:    node=Dummy
  weight:    100
  data:      {"power_kwh": 13.82}
```

This will trigger the creation of a notification for the default dummy entity using the definition for a dummy notification for trigger 'node/15m/highpowerusage' of the variant 'low' as defined in the YAML.

Optionally, a dummy notification can be generated for an entity other than the default dummy entity:

```
$ minerva trigger dummy-notification 'node/15m/highpowerusage' low 'node=n2303'
Dummy notification created:

  timestamp: 2025-11-01 14:45:00+01
  created:   2025-11-01 14:55:12+01
  entity:    node=n2303
  weight:    100
  data:      {"power_kwh": 13.82}
```

So in this case, a notification is generated for an entity with name 'node=n2303'.

### Minerva Web Console

The Minerva Web Console (to be created) it will be possible to select a trigger in the web GUI and choose a dummy notification type to generate from a list.

## Recognizing Dummy Notifications

Generated dummy notifications can only be recognized by the data or the entity for which they are generated. No other special marker is available. For this reason, you should normally only generate dummy notifications for designated dummy entities.

## Heartbeat Notifications

Using the Minerva CLI command in combination with a systemd service and timer, a periodic heartbeat notification can be generated.
