CREATE VIEW trend."hub_v-network_main_1d" AS
SELECT
  r.target_id AS entity_id,
  timestamp,
  SUM("freq_power") AS "freq_power",
  SUM("inside_temp") AS "inside_temp",
  SUM("outside_temp") AS "outside_temp",
  SUM("power_kwh") AS "power_kwh",
  sum("samples") AS "samples"
FROM trend."hub_node_main_1d" t
JOIN relation."node->v-network" r ON t.entity_id = r.source_id
GROUP BY timestamp, r.target_id;

GRANT SELECT ON trend."hub_v-network_main_1d" TO minerva;
