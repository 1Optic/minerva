-- For now, this is just the minimum to make the test fail in the way we want it to fail,
-- defining the relevant functions in a non-functional but non-failing way

ALTER TABLE "directory"."entity_type"
    ADD COLUMN "primary_alias" text DEFAULT NULL;

CREATE FUNCTION "entity"."create_entity_table_sql"(directory.entity_type, primary_alias text)
    RETURNS text[]
AS $$
SELECT ARRAY[
    format(
      'CREATE TABLE IF NOT EXISTS entity.%I('
      'id serial,'
      'name text UNIQUE,'
      'created timestamp with time zone default now(),'
      'primary_alias text GENERATED ALWAYS AS (%s) STORED'
      ');',
      $1.name,
      $2
    ),
    format(
       'SELECT create_reference_table(''entity.%I'');',
       $1.name
    )
];
$$ LANGUAGE sql VOLATILE;

CREATE FUNCTION "entity"."create_entity_table"(directory.entity_type, primary_alias text)
    RETURNS directory.entity_type
AS $$
    SELECT public.action($1, 
        CASE WHEN $2 IS NULL THEN entity.create_entity_table_sql($1)
        ELSE entity.create_entity_table_sql($1, $2)
        END
    );
$$ LANGUAGE sql VOLATILE;

CREATE FUNCTION "directory"."init_entity_type"(directory.entity_type, primary_alias text)
    RETURNS directory.entity_type
AS $$
    SELECT entity.create_entity_table($1, $2);
    SELECT entity.create_get_entity_function($1);
    SELECT entity.create_create_entity_function($1);
    SELECT entity.create_to_entity_function($1);
$$ LANGUAGE sql VOLATILE STRICT;

CREATE FUNCTION "directory"."define_entity_type"(entity_type_name text, primary_alias text, "description" text)
    RETURNS directory.entity_type
AS $$
    INSERT INTO directory.entity_type(name, primary_alias, description)
    VALUES ($1, $2, $3)
    ON CONFLICT DO NOTHING;
    SELECT * FROM directory.entity_type WHERE name = $1;
$$ LANGUAGE sql VOLATILE STRICT;

CREATE FUNCTION "directory"."create_entity_type"(entity_type_name text, primary_alias text)
    RETURNS directory.entity_type
AS $$
    SELECT directory.init_entity_type(directory.define_entity_type($1, $2, ''), $2);
$$ LANGUAGE sql VOLATILE STRICT;

CREATE OR REPLACE FUNCTION "directory"."create_entity_type"(text)
    RETURNS directory.entity_type
AS $$
    SELECT directory.create_entity_type($1, NULL);
$$ LANGUAGE sql VOLATILE STRICT;

CREATE OR REPLACE FUNCTION "directory"."update_entity_type"(entity_type_name text, primary_alias text, "description" text)
    RETURNS directory.entity_type
AS $$
    DECLARE
        etype directory.entity_type;
    BEGIN
        SELECT * FROM directory.entity_type WHERE name = $1 INTO etype;
        IF etype IS NOT NULL THEN
            UPDATE directory.entity_type SET primary_alias = $2, description = $3 WHERE name = $1;
            EXECUTE FORMAT(
                "ALTER TABLE entity.%I DROP COLUMN IF EXISTS primary_alias",
                $1
            );
            IF $2 IS NOT NULL THEN
                EXECUTE FORMAT (
                    'ALTER TABLE entity.%I '
                    'ADD COLUMN primary_alias text GENERATED ALWAYS AS (%s) STORED',
                    $1, $2
                );
            END IF;
        END IF;
    END;
$$ LANGUAGE plpgsql VOLATILE STRICT;


DROP FUNCTION "directory"."init_entity_type"(directory.entity_type);

DROP FUNCTION "directory"."define_entity_type"(text);

DROP FUNCTION "entity"."create_entity_table"(directory.entity_type);

ALTER TABLE "trend_directory"."trend_store_part"
    ADD COLUMN "primary_alias" boolean NOT NULL DEFAULT false;