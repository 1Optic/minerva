CREATE OR REPLACE FUNCTION "trend_directory"."default_columnar_period"()
    RETURNS interval
AS $$
SELECT '2w'::interval;
$$ LANGUAGE sql IMMUTABLE;

ALTER TABLE "trend_directory"."partition" ADD COLUMN "is_columnar" boolean NOT NULL DEFAULT false;

CREATE OR REPLACE FUNCTION "trend_directory"."needs_columnar_store"(trend_directory.partition)
    RETURNS boolean
AS $$
SELECT not p.is_columnar and p.to + COALESCE(m.reprocessing_period, trend_directory.default_columnar_period()) < now()
FROM trend_directory.partition p
  JOIN trend_directory.trend_store_part tsp ON p.trend_store_part_id = tsp.id
  LEFT JOIN trend_directory.materialization m ON m.dst_trend_store_part_id = p.id
WHERE p.id = $1.id;
$$ LANGUAGE sql STABLE;

CREATE OR REPLACE FUNCTION "trend_directory"."convert_to_columnar"(trend_directory.partition)
    RETURNS void
AS $$
SELECT alter_table_set_access_method(format('%I.%I', trend_directory.partition_schema(), $1.name)::regclass, 'columnar');
UPDATE trend_directory.partition SET is_columnar = 'true' WHERE id = $1.id;
$$ LANGUAGE sql VOLATILE;

