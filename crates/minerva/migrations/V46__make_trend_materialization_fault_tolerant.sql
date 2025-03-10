CREATE FUNCTION "trend_directory"."function_materialization_columns"("materialization_id" integer)
    RETURNS TABLE (name name)
AS $$
WITH function_columns AS (
  SELECT unnest(proargnames[2:]) AS name
  FROM trend_directory.materialization m
  JOIN trend_directory.trend_store_part tsp ON tsp.id = m.dst_trend_store_part_id,
  pg_proc
  JOIN pg_namespace ON pg_proc.pronamespace = pg_namespace.oid
  WHERE m.id = $1 AND nspname = 'trend' AND proname = tsp.name
),
trend_store_part_columns AS (
  SELECT t.name
  FROM trend_directory.materialization m
  JOIN trend_directory.table_trend t ON t.trend_store_part_id = m.dst_trend_store_part_id
  WHERE m.id = $1
)
SELECT f.name
FROM function_columns AS f
JOIN trend_store_part_columns AS t ON t.name = f.name
$$ LANGUAGE sql STABLE;

COMMENT ON FUNCTION "trend_directory"."function_materialization_columns"("materialization_id" integer) IS 'Return the names of column that are both in the target trend store part and materialization function to be used in queries';
