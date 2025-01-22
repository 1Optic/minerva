CREATE VIEW trend."hub_v-network_main_15m" AS
SELECT
  r.target_id AS entity_id,
  timestamp,
  count(*) AS samples,
  SUM("outside_temp") AS "outside_temp",
  SUM("inside_temp") AS "inside_temp",
  SUM("power_kwh") AS "power_kwh",
  SUM("freq_power") AS "freq_power"
FROM trend."hub_node_main_15m" t
JOIN relation."node->v-network" r ON t.entity_id = r.source_id
GROUP BY timestamp, r.target_id;

GRANT SELECT ON trend."hub_v-network_main_15m" TO minerva;

INSERT INTO trend_directory.trend_view(entity_type_id, data_source_id, granularity)
(
  SELECT et.id, ds.id, '15m'
  FROM directory.entity_type et, directory.data_source ds
  WHERE et.name = 'v-network' AND ds.name = 'hub'
) ON CONFLICT DO NOTHING;

INSERT INTO trend_directory.trend_view_part(name, trend_view_id)
(
  SELECT 'hub_v-network_main_15m', tv.id
  FROM trend_directory.trend_view tv
  JOIN directory.entity_type et ON et.id = tv.entity_type_id
  JOIN directory.data_source ds ON ds.id = tv.data_source_id
  WHERE et.name = 'v-network' AND ds.name = 'hub'
  AND tv.granularity = '15m'
);

INSERT INTO trend_directory.view_trend(trend_view_part_id, name, data_type, extra_data, description, time_aggregation, entity_aggregation)
(
  SELECT tvp.id, attname, format_type(atttypid, atttypmod), '{}', '', 'SUM', 'SUM'
  FROM pg_class c
  JOIN pg_namespace ns ON ns.oid = c.relnamespace
  JOIN pg_attribute a ON a.attrelid = c.oid
  JOIN trend_directory.trend_view_part tvp ON tvp.name = relname
  JOIN trend_directory.trend_view tv ON tv.id = tvp.trend_view_id
  JOIN directory.entity_type et ON et.id = tv.entity_type_id
  JOIN directory.data_source ds ON ds.id = tv.data_source_id
  WHERE tvp.name = 'hub_v-network_main_15m'
  AND ns.nspname = 'trend'
  AND NOT (attname = any(array['entity_id', 'timestamp']))
  AND attnum > 0
  AND et.name = 'v-network' AND ds.name = 'hub'
  AND tv.granularity = '15m'
) ON CONFLICT DO NOTHING;

