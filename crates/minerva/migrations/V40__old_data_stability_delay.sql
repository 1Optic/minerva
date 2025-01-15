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
