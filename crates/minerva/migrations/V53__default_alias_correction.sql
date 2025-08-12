DROP FUNCTION directory.update_entity_type(text,text,text);

CREATE OR REPLACE FUNCTION "directory"."update_entity_type"(entity_type_name text, primary_alias text, "description" text)
    RETURNS directory.entity_type
AS $$
    DECLARE
        etype directory.entity_type;
    BEGIN
        SELECT * FROM directory.entity_type WHERE name = $1 INTO etype;
        IF etype IS NULL THEN
            RETURN NULL;
        END IF;
        UPDATE directory.entity_type SET primary_alias = $2, description = $3 WHERE name = $1;
        EXECUTE FORMAT(
            'ALTER TABLE entity.%I DROP COLUMN IF EXISTS primary_alias',
            $1
        );
        IF $2 IS NOT NULL THEN
            EXECUTE FORMAT (
                'ALTER TABLE entity.%I '
                'ADD COLUMN primary_alias text GENERATED ALWAYS AS (%s) STORED',
                $1, $2
            );
        END IF;
        SELECT * FROM directory.entity_type WHERE name = $1 INTO etype;
        RETURN etype;
    END;
$$ LANGUAGE plpgsql VOLATILE;

