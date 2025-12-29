CREATE TYPE trend_directory.timestamp_statistics AS (
	partition_name name,
	stats real
);

CREATE FUNCTION trend_directory.timestamp_statistics(trend_store_part name, timestamp with time zone) RETURNS trend_directory.timestamp_statistics
    LANGUAGE sql
AS $$
    SELECT partition.name, stanumbers1[array_position(stavalues1::text::text[], $2::text)] AS stats
    FROM trend_directory.trend_store_part tsp
    JOIN trend_directory.partition ON partition.trend_store_part_id = tsp.id
    JOIN pg_statistic ON starelid = format('trend_partition.%I', partition.name)::regclass::oid
    JOIN pg_attribute a ON a.attrelid = starelid and a.attnum = staattnum
    WHERE tsp.name = $1
    AND partition."from" <= $2
    AND partition."to" > $2
    AND attname = 'timestamp'
$$;

-- Remove use of action_count function and get the count conventially

CREATE OR REPLACE FUNCTION attribute_directory.stage_sample(attribute_directory.sampled_view_materialization) RETURNS integer
    LANGUAGE plpgsql
    AS $$
DECLARE
    query text;
    row_count integer;
BEGIN
SELECT attribute_directory.view_to_attribute_staging_sql($1.src_view, attribute_store) INTO query
FROM attribute_directory.attribute_store
WHERE id = $1.attribute_store_id;

EXECUTE query;

GET DIAGNOSTICS row_count = ROW_COUNT;
RETURN row_count;
END;
$$;
