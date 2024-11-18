

CREATE OR REPLACE FUNCTION "relation_directory"."entity_set_exists"("owner" text, "name" text)
    RETURNS boolean
AS $$
DECLARE
  row_count integer;
BEGIN
  SELECT action_count(format(
    'SELECT * FROM attribute.minerva_entity_set '
    'WHERE owner = %L AND name = %L;',
    $1,
    $2
  )) INTO row_count;
  RETURN CASE row_count
    WHEN 0 THEN false
    ELSE true
  END;
END;
$$ LANGUAGE plpgsql STABLE;


CREATE OR REPLACE FUNCTION "relation_directory"."add_entity_to_set"("minerva_entity_set_id" integer, "entity" text)
    RETURNS attribute.minerva_entity_set
AS $$
DECLARE
  set attribute.minerva_entity_set;
BEGIN
  SELECT * FROM attribute.minerva_entity_set WHERE entity_id = $1 INTO set;
  PERFORM relation_directory.update_entity_set_attributes($1);
  EXECUTE FORMAT(
    'INSERT INTO relation.%I (source_id, target_id) '
    'SELECT source.id AS source_id, $1 AS target '
    'FROM entity.%I source '
    'WHERE source.name = $2 '
    'ON CONFLICT DO NOTHING;',
    set.source_entity_type || '->entity_set',
    set.source_entity_type
  ) USING set.entity_id, $2;
  RETURN set;
END;
$$ LANGUAGE plpgsql VOLATILE;


CREATE OR REPLACE FUNCTION "relation_directory"."remove_entity_from_set"("minerva_entity_set_id" integer, "entity" text)
    RETURNS void
AS $$
DECLARE
  set attribute.minerva_entity_set;
BEGIN
  SELECT * FROM attribute.minerva_entity_set WHERE entity_id = $1 INTO set;
  PERFORM relation_directory.update_entity_set_attributes($1);
  EXECUTE FORMAT(
    'DELETE es FROM relation.%I es '
    'JOIN entity.%I source ON es.source_id = source.id '
    'WHERE source.name = $1 AND target_id = $2',
    set.source_entity_type || '->entity_set',
    set.source_entity_type
  ) USING $2, set.entity_id;
END;
$$ LANGUAGE plpgsql VOLATILE;


CREATE OR REPLACE FUNCTION "relation_directory"."change_set_entities_guarded"("minerva_entity_set_id" integer, "entities" text[])
    RETURNS text[]
AS $$
DECLARE
  set attribute.minerva_entity_set;
  entity text;
  real_entity text;
  result text[];
  newresult text[];
BEGIN
  SELECT * FROM attribute.minerva_entity_set WHERE entity_id = $1 INTO set;
  SELECT $2 INTO result;
  FOREACH entity IN ARRAY $2 LOOP
    EXECUTE FORMAT(
      'SELECT name FROM entity.%I WHERE name = $1;',
      set.source_entity_type
    ) INTO real_entity USING entity;
    SELECT array_remove(result, real_entity) INTO result;
  END LOOP;
  IF ARRAY_LENGTH(result, 1) IS NULL THEN
    PERFORM relation_directory.change_set_entities($1, $2);
  END IF;
  RETURN result;
END;
$$ LANGUAGE plpgsql VOLATILE;

COMMENT ON FUNCTION "relation_directory"."change_set_entities_guarded"("minerva_entity_set_id" integer, "entities" text[]) IS 'Only sets the entities if all specified entities are actually valid.
Returns those entities that were invalid.';


CREATE OR REPLACE FUNCTION "relation_directory"."create_entity_set_guarded"("name" text, "group" text, "entity_type_name" text, "owner" text, "description" text, "entities" text[])
    RETURNS text[]
AS $$
DECLARE
  entity text;
  real_entity text;
  result text[];
  newresult text[];
  entityset integer;
BEGIN
  SELECT $6 INTO result;
  FOREACH entity IN ARRAY $6 LOOP
    EXECUTE FORMAT(
      'SELECT name FROM entity.%I WHERE name = $1;',
      entity_type_name
    ) INTO real_entity USING entity;
    SELECT array_remove(result, real_entity) INTO result;
  END LOOP;
  IF ARRAY_LENGTH(result, 1) IS NULL THEN
    SELECT entity_id FROM relation_directory.create_entity_set($1, $2, $3, $4, $5) INTO entityset;
    PERFORM relation_directory.change_set_entities(entityset, $6);
  END IF;
  RETURN result;
END;
$$ LANGUAGE plpgsql VOLATILE;


CREATE OR REPLACE FUNCTION "trend_directory"."rename_partitions"(trend_directory.trend_store_part, "new_name" name)
    RETURNS trend_directory.trend_store_part
AS $$
DECLARE
  partition trend_directory.partition;
BEGIN
  FOR partition in SELECT * FROM trend_directory.partition WHERE trend_store_part_id = $1.id
  LOOP
    EXECUTE format(
        'ALTER TABLE trend_partition.%I RENAME TO %I',
        partition.name,
        $2 || '_' || partition.index
    );
    EXECUTE 'UPDATE trend_directory.partition SET name = $1 WHERE id = $2'
        USING $2 || '_' || partition.index, partition.id;
  END LOOP;
  RETURN $1;
END;
$$ LANGUAGE plpgsql VOLATILE;


CREATE OR REPLACE FUNCTION "trend_directory"."rename_trend_store_part_full"(trend_directory.trend_store_part, name)
    RETURNS trend_directory.trend_store_part
AS $$
DECLARE
  old_name text;
  new_name text;
BEGIN
  SET LOCAL citus.multi_shard_modify_mode TO 'sequential';
  SELECT trend_directory.to_char($1) INTO old_name;
  SELECT $2::text INTO new_name;
  PERFORM trend_directory.rename_trend_store_part($1, $2);
  EXECUTE format(
      'ALTER TABLE %I.%I RENAME TO %I',
      trend_directory.staging_table_schema(),
      old_name || '_staging',
      new_name || '_staging'
  );
  PERFORM trend_directory.rename_partitions($1, $2);
  EXECUTE
      'UPDATE trend_directory.view_materialization '
      'SET src_view = $1 '
      'WHERE src_view = $2'
      USING 'trend."_' || new_name || '"', 'trend."_' || old_name || '"';
  EXECUTE
      'UPDATE trend_directory.function_materialization '
      'SET src_function = $1 '
      'WHERE src_function = $2'
      USING 'trend."' || new_name || '"', 'trend."' || old_name || '"';
  RETURN $1;
END
$$ LANGUAGE plpgsql VOLATILE;


CREATE OR REPLACE FUNCTION "attribute_directory"."init"(attribute_directory.attribute)
    RETURNS attribute_directory.attribute
AS $$
SELECT public.action(
    $1,
    format('SELECT attribute_directory.add_attribute_column(attribute_store, %L, %L) FROM attribute_directory.attribute_store WHERE id = %s',
    $1.name, $1.data_type, $1.attribute_store_id)
)
$$ LANGUAGE sql VOLATILE;
