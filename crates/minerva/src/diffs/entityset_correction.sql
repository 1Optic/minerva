

CREATE OR REPLACE FUNCTION "relation_directory"."add_entity_to_set"("minerva_entity_set_id" integer, "entity" text)
    RETURNS attribute.minerva_entity_set
AS $$
DECLARE
  set attribute.minerva_entity_set;
BEGIN
  SELECT * FROM attribute.minerva_entity_set WHERE entity_id = $1 INTO set;
  PERFORM relation_directory.update_entity_set_attributes($1);
  PERFORM action(FORMAT(
    'INSERT INTO relation."%s->entity_set" (source_id, target_id) '
    'SELECT source.id AS source_id, %s AS target '
    'FROM entity.%I source '
    'WHERE source.name = ''%s'''
    'ON CONFLICT DO NOTHING;',
    set.source_entity_type,
    set.entity_id,
    set.source_entity_type,
    $2
  ));
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
  PERFORM action(FORMAT(
    'DELETE es FROM relation."%s->entity_set" es '
    'JOIN entity.%I source ON es.source_id = source.id '
    'WHERE source.name = ''%s'' AND target_id = %s;',
    set.source_entity_type,
    set.source_entity_type,
    $2,
    set.entity_id
  ));
END;
$$ LANGUAGE plpgsql VOLATILE;


CREATE OR REPLACE FUNCTION "relation_directory"."change_set_entities"("minerva_entity_set_id" integer, "entities" text[])
    RETURNS void
AS $$
DECLARE
  set attribute.minerva_entity_set;
BEGIN
  SELECT * FROM attribute.minerva_entity_set WHERE entity_id = $1 INTO set;
  PERFORM action(FORMAT(
    'DELETE FROM relation."%s->entity_set" '
    'WHERE target_id = %s;',
    set.source_entity_type,
    set.entity_id
  ));
  PERFORM relation_directory.add_entities_to_set($1, $2);
END;
$$ LANGUAGE plpgsql VOLATILE;

COMMENT ON FUNCTION "relation_directory"."change_set_entities"("minerva_entity_set_id" integer, "entities" text[]) IS 'Set the entities in the set to exactly the specified entities';


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
      'SELECT name FROM entity.%I WHERE name = ''%s'';',
      set.source_entity_type,
      entity
    ) INTO real_entity;
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


CREATE OR REPLACE FUNCTION "relation_directory"."get_entity_set_members"("minerva_entity_set_id" integer)
    RETURNS text[]
AS $$
DECLARE
  set attribute.minerva_entity_set;
  result text[];
BEGIN
  SELECT * FROM attribute.minerva_entity_set WHERE entity_id = $1 INTO set;
  EXECUTE FORMAT(
    'SELECT array_agg(e.name) '
    'FROM relation."%s->entity_set" es JOIN entity.%I e ON es.source_id = e.id '
    'WHERE es.target_id = %s',
    set.source_entity_type,
    set.source_entity_type,
    set.entity_id
  ) INTO result;
  RETURN result;
END;
$$ LANGUAGE plpgsql STABLE;


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
      'SELECT name FROM entity.%I WHERE name = ''%s'';',
      $3,
      entity
    ) INTO real_entity;
    SELECT array_remove(result, real_entity) INTO result;
  END LOOP;
  IF ARRAY_LENGTH(result, 1) IS NULL THEN
    SELECT entity_id FROM relation_directory.create_entity_set($1, $2, $3, $4, $5) INTO entityset;
    PERFORM relation_directory.change_set_entities(entityset, $6);
  END IF;
  RETURN result;
END;
$$ LANGUAGE plpgsql VOLATILE;


CREATE OR REPLACE FUNCTION "relation_directory"."get_entity_set_members"("name" text, "owner" text)
    RETURNS text[]
AS $$
SELECT relation_directory.get_entity_set_members(es.entity_id)
  FROM attribute.minerva_entity_set es
  WHERE owner = $2 AND name = $1;
$$ LANGUAGE sql STABLE;
