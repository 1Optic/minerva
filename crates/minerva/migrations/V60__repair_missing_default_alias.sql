CREATE OR REPLACE FUNCTION "trend_directory"."ensure_name_column"(trend_directory."trend_store_part")
    RETURNS VOID
AS $$
    DECLARE
        entity_type text;
    BEGIN
        SELECT et.name FROM trend_directory.trend_store ts JOIN directory.entity_type et 
            ON ts.entity_type_id = et.id WHERE ts.id = $1.trend_store_id INTO entity_type;
        IF NOT $1.primary_alias THEN
            UPDATE trend_directory."trend_store_part" SET primary_alias = true WHERE id = $1.id;
            EXECUTE FORMAT(
                'ALTER TABLE trend.%I '
                'ADD COLUMN name text',
                $1.name
            );
            EXECUTE FORMAT(
                'UPDATE trend.%I t SET name = e.primary_alias FROM entity.%I e WHERE e.id = t.entity_id',
                $1.name, entity_type
            );
        ELSE
            EXECUTE FORMAT(
                'UPDATE trend.%I t SET name = e.primary_alias FROM entity.%I e WHERE t.name IS NULL AND e.id = t.entity_id',
                $1.name, entity_type
            );
        END IF;
    END;
$$ LANGUAGE plpgsql VOLATILE;
