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
      'id serial, '
      'name text UNIQUE, '
      'created timestamp with time zone default now(), '
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
$$ LANGUAGE sql VOLATILE STRICT;

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
$$ LANGUAGE sql VOLATILE;

CREATE FUNCTION "directory"."define_entity_type"(entity_type_name text, primary_alias text, "description" text)
    RETURNS directory.entity_type
AS $$
    INSERT INTO directory.entity_type(name, primary_alias, description)
        VALUES ($1, $2, $3) ON CONFLICT DO NOTHING;
    SELECT * FROM directory.entity_type WHERE name = $1;
$$ LANGUAGE sql VOLATILE;

CREATE FUNCTION "directory"."create_entity_type"(entity_type_name text, primary_alias text)
    RETURNS directory.entity_type
AS $$
    SELECT directory.init_entity_type(directory.define_entity_type($1, $2, ''), $2);
$$ LANGUAGE sql VOLATILE;

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
$$ LANGUAGE plpgsql VOLATILE;

DROP FUNCTION "directory"."init_entity_type"(directory.entity_type);

DROP FUNCTION "directory"."define_entity_type"(text);

DROP FUNCTION "entity"."create_entity_table"(directory.entity_type);


ALTER TYPE "trend_directory"."trend_store_part_descr" ADD ATTRIBUTE "primary_alias" boolean;

ALTER TABLE "trend_directory"."trend_store_part"
    ADD COLUMN "primary_alias" boolean NOT NULL DEFAULT false;

CREATE FUNCTION "trend_directory"."define_trend_store_part"("trend_store_id" integer, "name" name, "primary_alias" boolean)
    RETURNS trend_directory.trend_store_part
AS $$
    INSERT INTO trend_directory.trend_store_part (trend_store_id, name, primary_alias)
        VALUES ($1, $2, $3)
    RETURNING *;
$$ LANGUAGE sql VOLATILE;

CREATE FUNCTION "trend_directory"."define_trend_store_part"("trend_store_id" integer, "name" name, "primary_alias" boolean, "trends" trend_directory.trend_descr[], "generated_trends" trend_directory.generated_trend_descr[])
    RETURNS trend_directory.trend_store_part
AS $$
    SELECT trend_directory.define_generated_table_trends(
        trend_directory.define_table_trends(
            trend_directory.define_trend_store_part($1, $2, $3),
            $4
        ),
        $5
    );
$$ LANGUAGE sql VOLATILE;

CREATE OR REPLACE FUNCTION "trend_directory"."create_trend_store_part"("trend_store_id" integer, "name" name)
    RETURNS trend_directory.trend_store_part
AS $$
    SELECT trend_directory.initialize_trend_store_part(
        trend_directory.define_trend_store_part($1, $2, false)
    );
$$ LANGUAGE sql VOLATILE;

CREATE OR REPLACE FUNCTION "trend_directory"."define_trend_store"(trend_directory.trend_store, trend_directory.trend_store_part_descr[])
    RETURNS trend_directory.trend_store
AS $$
    SELECT trend_directory.define_trend_store_part($1.id, name, primary_alias, trends, generated_trends)
        FROM unnest($2);
    SELECT $1;
$$ LANGUAGE sql VOLATILE;

CREATE FUNCTION "trend_directory"."ensure_name_column"(trend_directory."trend_store_part")
    RETURNS VOID
AS $$
    DECLARE
        entity_type text;
    BEGIN
        IF NOT $1.primary_alias THEN
            SELECT et.name FROM trend_directory.trend_store ts JOIN directory.entity_type et 
                ON ts.entity_type_id = et.id WHERE ts.id = $1.trend_store_id INTO entity_type;
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
        END IF;
    END;
$$ LANGUAGE plpgsql VOLATILE;

CREATE FUNCTION "trend_directory"."remove_name_column"(trend_directory."trend_store_part")
    RETURNS VOID
AS $$
    BEGIN
        IF $1.primary_alias THEN
            UPDATE trend_directory."trend_store_part" SET primary_alias = false WHERE id = $1.id;
            EXECUTE FORMAT(
                'ALTER TABLE trend.%I '
                'DROP COLUMN IF EXISTS name',
                $1.name
            );
        END IF;
    END;
$$ LANGUAGE plpgsql VOLATILE;


DROP FUNCTION "trend_directory"."remove_extra_trends"("part" trend_directory.trend_store_part_descr);

DROP FUNCTION "trend_directory"."add_trends"(trend_directory.trend_store, "parts" trend_directory.trend_store_part_descr[]);

DROP FUNCTION "trend_directory"."add_trends"("part" trend_directory.trend_store_part_descr);

DROP FUNCTION "trend_directory"."assure_table_trends_exist"(integer, text, trend_directory.trend_descr[], trend_directory.generated_trend_descr[]);

DROP FUNCTION  "trend_directory"."define_trend_store_part"(integer, name, trend_directory.trend_descr[], trend_directory.generated_trend_descr[]);

DROP FUNCTION "trend_directory"."define_trend_store_part"(integer, name);
