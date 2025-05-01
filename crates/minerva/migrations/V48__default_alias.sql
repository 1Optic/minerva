-- For now, this is just the minimum to make the test fail in the way we want it to fail,
-- defining the relevant functions in a non-functional but non-failing way

CREATE FUNCTION "directory"."create_entity_type"(text, text)
    RETURNS directory.entity_type
AS $$
DECLARE
    et directory.entity_type;
BEGIN
    SELECT * FROM directory.create_entity_type($1) INTO et;
    EXECUTE FORMAT (
        'ALTER TABLE entity.%I '
        'ADD COLUMN primary_alias text DEFAULT ''test''',
        $1
    );
    RETURN et;
END;
$$ LANGUAGE plpgsql VOLATILE STRICT;

ALTER TABLE "trend_directory"."trend_store_part"
    ADD COLUMN "primary_alias" boolean NOT NULL DEFAULT false;