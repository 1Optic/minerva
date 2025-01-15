ALTER TABLE trend_directory.materialization
  ADD COLUMN old_data_stability_delay interval DEFAULT NULL,
  ADD COLUMN old_data_threshold interval DEFAULT NULL;

ALTER TABLE trend_directory.materialization_state
  ADD COLUMN materialization_count integer NOT NULL DEFAULT 0;

CREATE OR REPLACE FUNCTION "trend_directory"."materialize"("materialization_id" integer, "timestamp" timestamp with time zone)
    RETURNS trend_directory.transfer_result
AS $$
DECLARE
    mat trend_directory.materialization;
    start timestamp with time zone;
    duration interval;
    columns_part text;
    result trend_directory.transfer_result;
BEGIN
    SELECT * FROM trend_directory.materialization WHERE id = $1 INTO mat;

    start = clock_timestamp();

    -- Remove all records in the target table for the timestamp to materialize
    PERFORM trend_directory.clear_trend_store_part(
        mat.dst_trend_store_part_id, $2
    );

    result.row_count = trend_directory.transfer($1, $2);

    -- Update the state of this materialization
    UPDATE trend_directory.materialization_state vms
    SET processed_fingerprint = vms.source_fingerprint,
      materialization_count = materialization_count + 1
    WHERE vms.materialization_id = $1 AND vms.timestamp = $2;

    -- Log the change in the target trend store part
    PERFORM trend_directory.mark_modified(mat.dst_trend_store_part_id, $2, now());

    duration = clock_timestamp() - start;

    UPDATE trend_directory.materialization_metrics
    SET execution_count = execution_count + 1, total_duration = total_duration + duration
    WHERE materialization_metrics.materialization_id = $1;

    RETURN result;
END;
$$ LANGUAGE plpgsql VOLATILE;

CREATE FUNCTION "trend_directory"."define_materialization"("dst_trend_store_part_id" integer, "processing_delay" interval, "stability_delay" interval, "reprocessing_period" interval, "description" jsonb, "old_data_threshold" interval, "old_data_stability_delay" interval)
    RETURNS trend_directory.materialization
AS $$
INSERT INTO trend_directory.materialization(dst_trend_store_part_id, processing_delay, stability_delay, reprocessing_period, description, old_data_threshold, old_data_stability_delay)
VALUES ($1, $2, $3, $4, $5, $6, $7)
ON CONFLICT DO NOTHING
RETURNING *;
$$ LANGUAGE sql VOLATILE;

COMMENT ON FUNCTION "trend_directory"."define_materialization"("dst_trend_store_part_id" integer, "processing_delay" interval, "stability_delay" interval, "reprocessing_period" interval, "description" jsonb, "old_data_threshold" interval, "old_data_stability_delay" interval) IS 'Define a materialization';

CREATE FUNCTION "trend_directory"."define_view_materialization"("dst_trend_store_part_id" integer, "processing_delay" interval, "stability_delay" interval, "reprocessing_period" interval, "src_view" regclass, "description" jsonb, "old_data_threshold" interval, "old_data_stability_delay" interval)
    RETURNS trend_directory.view_materialization
AS $$
INSERT INTO trend_directory.view_materialization(materialization_id, src_view)
VALUES((trend_directory.define_materialization($1, $2, $3, $4, $6, $7, $8)).id, $5) RETURNING *;
$$ LANGUAGE sql VOLATILE;

COMMENT ON FUNCTION "trend_directory"."define_view_materialization"("dst_trend_store_part_id" integer, "processing_delay" interval, "stability_delay" interval, "reprocessing_period" interval, "src_view" regclass, "description" jsonb, "old_data_threshold" interval, "old_data_stability_delay" interval) IS 'Define a materialization that uses a view as source';

CREATE OR REPLACE FUNCTION "trend_directory"."define_view_materialization"("dst_trend_store_part_id" integer, "processing_delay" interval, "stability_delay" interval, "reprocessing_period" interval, "src_view" regclass, "description" jsonb)
    RETURNS trend_directory.view_materialization
AS $$
SELECT trend_directory.define_view_materialization($1, $2, $3, $4, $5, $6, NULL, NULL);
$$ LANGUAGE sql VOLATILE;

CREATE FUNCTION "trend_directory"."define_function_materialization"("dst_trend_store_part_id" integer, "processing_delay" interval, "stability_delay" interval, "reprocessing_period" interval, "src_function" regproc, "description" jsonb, "old_data_threshold" interval, "old_data_stability_delay" interval)
    RETURNS trend_directory.function_materialization
AS $$
INSERT INTO trend_directory.function_materialization(materialization_id, src_function)
VALUES((trend_directory.define_materialization($1, $2, $3, $4, $6, $7, $8)).id, $5::text)
ON CONFLICT DO NOTHING
RETURNING *;
$$ LANGUAGE sql VOLATILE;

COMMENT ON FUNCTION "trend_directory"."define_function_materialization"("dst_trend_store_part_id" integer, "processing_delay" interval, "stability_delay" interval, "reprocessing_period" interval, "src_function" regproc, "description" jsonb, "old_data_threshold" interval, "old_data_stability_delay" interval) IS 'Define a materialization that uses a function as source';

CREATE OR REPLACE FUNCTION "trend_directory"."define_function_materialization"("dst_trend_store_part_id" integer, "processing_delay" interval, "stability_delay" interval, "reprocessing_period" interval, "src_function" regproc, "description" jsonb)
    RETURNS trend_directory.function_materialization
AS $$
SELECT trend_directory.define_function_materialization($1, $2, $3, $4, $5, $6, NULL, NULL)
$$ LANGUAGE sql VOLATILE;

DROP FUNCTION "trend_directory"."define_materialization"("dst_trend_store_part_id" integer, "processing_delay" interval, "stability_delay" interval, "reprocessing_period" interval, "description" jsonb);

