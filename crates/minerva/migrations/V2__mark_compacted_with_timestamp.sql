-- Cleanup deprecated dynamically generated views of existing attribute stores

CREATE FUNCTION attribute_directory.upgrade_attribute_store(attribute_directory.attribute_store)
    RETURNS void
AS $$
SELECT attribute_directory.drop_compacted_view($1);
SELECT attribute_directory.drop_run_length_view($1);
SELECT attribute_directory.drop_compacted_tmp_table($1);
SELECT attribute_directory.drop_changes_view($1);
$$ LANGUAGE sql VOLATILE;


SELECT attribute_directory.upgrade_attribute_store(attribute_store) FROM attribute_directory.attribute_store;

DROP FUNCTION attribute_directory.upgrade_attribute_store(attribute_directory.attribute_store);

-- End of cleanup logic for deprecated dynamically generated views


ALTER TABLE attribute_directory.attribute_store_compacted DROP COLUMN compacted;
ALTER TABLE attribute_directory.attribute_store_compacted ADD COLUMN compacted timestamptz;

DROP FUNCTION IF EXISTS "attribute_directory"."changes_view_name"(attribute_directory.attribute_store);
DROP FUNCTION IF EXISTS "attribute_directory"."run_length_view_name"(attribute_directory.attribute_store);
DROP FUNCTION IF EXISTS "attribute_directory"."compacted_view_name"(attribute_directory.attribute_store);

DROP FUNCTION IF EXISTS "attribute_directory"."create_run_length_view"(attribute_directory.attribute_store);
DROP FUNCTION IF EXISTS "attribute_directory"."drop_run_length_view_sql"(attribute_directory.attribute_store);
DROP FUNCTION IF EXISTS "attribute_directory"."drop_run_length_view"(attribute_directory.attribute_store);
DROP FUNCTION IF EXISTS "attribute_directory"."drop_changes_view"(attribute_directory.attribute_store);
DROP FUNCTION IF EXISTS "attribute_directory"."run_length_view_query"(attribute_directory.attribute_store);
DROP FUNCTION IF EXISTS "attribute_directory"."create_run_length_view_sql"(attribute_directory.attribute_store);

DROP FUNCTION IF EXISTS "attribute_directory"."update_compacted"("attribute_store_id" integer, "compacted" integer);
DROP FUNCTION IF EXISTS "attribute_directory"."store_compacted"("attribute_store_id" integer, "compacted" integer);
DROP FUNCTION IF EXISTS "attribute_directory"."mark_compacted"("attribute_store_id" integer, "compacted" integer);

DROP FUNCTION IF EXISTS "attribute_directory"."compacted_tmp_table_name"(attribute_directory.attribute_store);
DROP FUNCTION IF EXISTS "attribute_directory"."create_compacted_tmp_table_sql"(attribute_directory.attribute_store);
DROP FUNCTION IF EXISTS "attribute_directory"."create_compacted_tmp_table"(attribute_directory.attribute_store);
DROP FUNCTION IF EXISTS "attribute_directory"."drop_compacted_tmp_table_sql"(attribute_directory.attribute_store);
DROP FUNCTION IF EXISTS "attribute_directory"."drop_compacted_tmp_table"(attribute_directory.attribute_store);
DROP FUNCTION IF EXISTS "attribute_directory"."compacted_view_query"(attribute_directory.attribute_store);
DROP FUNCTION IF EXISTS "attribute_directory"."create_compacted_view_sql"(attribute_directory.attribute_store);
DROP FUNCTION IF EXISTS "attribute_directory"."create_compacted_view"(attribute_directory.attribute_store);
DROP FUNCTION IF EXISTS "attribute_directory"."drop_compacted_view_sql"(attribute_directory.attribute_store);
DROP FUNCTION IF EXISTS "attribute_directory"."drop_compacted_view"(attribute_directory.attribute_store);
DROP FUNCTION IF EXISTS "attribute_directory"."insert_into_compacted_sql"(attribute_directory.attribute_store);
DROP FUNCTION IF EXISTS "attribute_directory"."remove_from_compacted_sql"(attribute_directory.attribute_store);
DROP FUNCTION IF EXISTS "attribute_directory"."insert_into_compacted"(attribute_directory.attribute_store);
DROP FUNCTION IF EXISTS "attribute_directory"."remove_from_compacted"(attribute_directory.attribute_store);
DROP FUNCTION IF EXISTS "attribute_directory"."requires_compacting"("attribute_store_id" integer);
DROP FUNCTION IF EXISTS "attribute_directory"."requires_compacting"(attribute_directory.attribute_store);
DROP FUNCTION IF EXISTS "attribute_directory"."last_compacted"("attribute_store_id" integer);
DROP FUNCTION IF EXISTS "attribute_directory"."compact"(attribute_directory.attribute_store, "max_compacting" integer);
DROP FUNCTION IF EXISTS "attribute_directory"."compact"(attribute_directory.attribute_store);

DROP FUNCTION IF EXISTS "attribute_directory"."changes_view_query"(attribute_directory.attribute_store);
DROP FUNCTION IF EXISTS "attribute_directory"."create_changes_view_sql"(attribute_directory.attribute_store);
DROP FUNCTION IF EXISTS "attribute_directory"."create_changes_view"(attribute_directory.attribute_store);

CREATE OR REPLACE FUNCTION "attribute_directory"."create_dependees"(attribute_directory.attribute_store)
    RETURNS attribute_directory.attribute_store
AS $$
SELECT attribute_directory.create_staging_new_view($1);
SELECT attribute_directory.create_staging_modified_view($1);
SELECT attribute_directory.create_curr_ptr_view($1);
SELECT attribute_directory.create_curr_view($1);
SELECT $1;
$$ LANGUAGE sql VOLATILE;

CREATE OR REPLACE FUNCTION "attribute_directory"."drop_dependees"(attribute_directory.attribute_store)
    RETURNS attribute_directory.attribute_store
AS $$
SELECT attribute_directory.drop_curr_view($1);
SELECT attribute_directory.drop_curr_ptr_view($1);
SELECT attribute_directory.drop_staging_modified_view($1);
SELECT attribute_directory.drop_staging_new_view($1);
SELECT $1;
$$ LANGUAGE sql VOLATILE;

CREATE OR REPLACE FUNCTION "attribute_directory"."drop_hash"(attribute_directory.attribute_store)
    RETURNS attribute_directory.attribute_store
AS $$
SELECT public.action(
    $1,
    ARRAY[
        format('SELECT attribute_directory.drop_curr_view(%s)', $1),
        format('ALTER TABLE attribute_history.%I DROP COLUMN hash CASCADE', attribute_directory.attribute_store_to_char($1.id))
    ]
);
$$ LANGUAGE sql VOLATILE;

CREATE OR REPLACE FUNCTION "attribute_directory"."add_hash"(attribute_directory.attribute_store)
    RETURNS attribute_directory.attribute_store
AS $$
SELECT public.action(
    $1,
    ARRAY[
        format(
            'ALTER TABLE attribute_history.%I ADD COLUMN hash character varying GENERATED ALWAYS AS (%s) STORED',
            attribute_directory.attribute_store_to_char($1.id),
            attribute_directory.hash_query($1)
        ),
        format('SELECT attribute_directory.create_curr_view(%s)', $1)
    ]
);
$$ LANGUAGE sql VOLATILE;

CREATE OR REPLACE PROCEDURE "attribute_directory"."init"(attribute_directory.attribute_store)
AS $$
BEGIN
  -- Base table
  PERFORM attribute_directory.create_base_table($1);

  -- Dependent tables
  PERFORM attribute_directory.create_history_table($1);
  PERFORM attribute_directory.create_staging_table($1);

  -- Separate table
  PERFORM attribute_directory.create_curr_ptr_table($1);

  -- Other
  PERFORM attribute_directory.create_at_func_ptr($1);
  PERFORM attribute_directory.create_at_func($1);

  PERFORM attribute_directory.create_entity_at_func_ptr($1);
  PERFORM attribute_directory.create_entity_at_func($1);

  PERFORM attribute_directory.create_dependees($1);

END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION "attribute_directory"."deinit"(attribute_directory.attribute_store)
    RETURNS void
AS $$
-- Other
SELECT attribute_directory.drop_dependees($1);

SELECT attribute_directory.drop_entity_at_func($1);
SELECT attribute_directory.drop_entity_at_func_ptr($1);

SELECT attribute_directory.drop_at_func($1);
SELECT attribute_directory.drop_at_func_ptr($1);

SELECT attribute_directory.drop_curr_ptr_table($1);

-- Dependent tables
SELECT attribute_directory.drop_staging_table($1);
SELECT attribute_directory.drop_history_table($1);

-- Base/parent table
SELECT attribute_directory.drop_base_table($1);
$$ LANGUAGE sql VOLATILE;


CREATE OR REPLACE FUNCTION "attribute_directory"."materialize_curr_ptr"(attribute_directory.attribute_store)
    RETURNS integer
AS $$
DECLARE
    table_name name := attribute_directory.curr_ptr_table_name($1);
    view_name name := attribute_directory.curr_ptr_view_name($1);
    row_count integer;
BEGIN
    EXECUTE format('TRUNCATE attribute_history.%I', table_name);
    EXECUTE format(
        'INSERT INTO attribute_history.%I (id) '
        'SELECT id '
        'FROM attribute_history.%I', table_name, view_name
    );

    GET DIAGNOSTICS row_count = ROW_COUNT;

    PERFORM attribute_directory.mark_curr_materialized($1.id);

    RETURN row_count;
END;
$$ LANGUAGE plpgsql VOLATILE;


CREATE OR REPLACE FUNCTION "attribute_directory"."add_attribute_column"(attribute_directory.attribute_store, name, text)
    RETURNS attribute_directory.attribute_store
AS $$
SELECT public.action(
    $1,
    ARRAY[
        format('ALTER TABLE attribute_base.%I ADD COLUMN %I %s', attribute_directory.to_char($1), $2, $3),
        format('SELECT attribute_directory.drop_hash(%s::attribute_directory.attribute_store)', $1),
        format('ALTER TABLE attribute_history.%I ADD COLUMN %I %s', attribute_directory.to_char($1), $2, $3),
        format('SELECT attribute_directory.add_hash(%s::attribute_directory.attribute_store)', $1),
        format('SELECT attribute_directory.drop_staging_dependees(%s)', $1),
        format('ALTER TABLE attribute_staging.%I ADD COLUMN %I %s', attribute_directory.to_char($1), $2, $3),
        format('SELECT attribute_directory.add_staging_dependees(%s)', $1)
    ]
);
$$ LANGUAGE sql VOLATILE;


CREATE OR REPLACE FUNCTION "attribute_directory"."modify_data_type"(attribute_directory.attribute)
    RETURNS attribute_directory.attribute
AS $$
DECLARE
  store attribute_directory.attribute_store;
BEGIN
  SELECT * FROM attribute_directory.attribute_store WHERE id = $1.attribute_store_id INTO store;
  RETURN public.action(
      $1,
      ARRAY[
          format('ALTER TABLE attribute_base.%I ALTER %I TYPE %s', attribute_directory.to_char(store), $1.name, $1.data_type),
          format('SELECT attribute_directory.drop_hash(%s::attribute_directory.attribute_store)', store),
          format('ALTER TABLE attribute_history.%I ALTER %I TYPE %s', attribute_directory.to_char(store), $1.name, $1.data_type),
          format('SELECT attribute_directory.add_hash(%s::attribute_directory.attribute_store)', store),
          format('SELECT attribute_directory.drop_staging_dependees(%s)', store),
          format('ALTER TABLE attribute_staging.%I ALTER %I TYPE %s', attribute_directory.to_char(store), $1.name, $1.data_type),
          format('SELECT attribute_directory.add_staging_dependees(%s)', store)
      ]
  );
END;
$$ LANGUAGE plpgsql VOLATILE;


CREATE OR REPLACE FUNCTION "attribute_directory"."remove_attribute_column"(attribute_directory.attribute_store, name)
    RETURNS attribute_directory.attribute_store
AS $$
SELECT public.action(
    $1,
    ARRAY[
        format('SELECT attribute_directory.drop_hash(%s::attribute_directory.attribute_store)', $1),
        format('ALTER TABLE attribute_base.%I DROP COLUMN %I CASCADE', attribute_directory.to_char($1), $2),
        format('ALTER TABLE attribute_history.%I DROP COLUMN %I CASCADE', attribute_directory.to_char($1), $2),
        format('SELECT attribute_directory.add_hash(%s::attribute_directory.attribute_store)', $1),
        format('SELECT attribute_directory.drop_staging_dependees(%s)', $1),
        format('ALTER TABLE attribute_staging.%I DROP COLUMN %I CASCADE', attribute_directory.to_char($1), $2),
        format('SELECT attribute_directory.add_staging_dependees(%s)', $1)
    ]
);
$$ LANGUAGE sql VOLATILE;


