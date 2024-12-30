CREATE OR REPLACE FUNCTION "trend_directory"."cleanup_for_view_materialization"()
    RETURNS trigger
AS $$
BEGIN
    EXECUTE format('DROP VIEW IF EXISTS %s', OLD.src_view);

    RETURN OLD;
END;
$$ LANGUAGE plpgsql VOLATILE;
