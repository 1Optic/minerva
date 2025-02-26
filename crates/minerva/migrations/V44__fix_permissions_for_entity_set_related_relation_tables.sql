CREATE OR REPLACE FUNCTION "relation_directory"."create_entity_set"("name" text, "group" text, "entity_type_name" text, "owner" text, "description" text)
    RETURNS integer
AS $$
DECLARE
  entity_id integer;
BEGIN
  EXECUTE FORMAT(
    'CREATE TABLE IF NOT EXISTS relation."%s->entity_set"('
    'source_id integer, '
    'target_id integer, '
    'PRIMARY KEY (source_id, target_id));',
    entity_type_name
  );
  EXECUTE FORMAT(
    'ALTER TABLE relation."%s->entity_set" OWNER TO minerva_admin;',
    entity_type_name
  );
  EXECUTE FORMAT(
    'GRANT INSERT,UPDATE,DELETE ON relation."%s->entity_set" TO minerva_writer;',
    entity_type_name
  );
  EXECUTE FORMAT(
    'GRANT SELECT ON relation."%s->entity_set" TO minerva;',
    entity_type_name
  );
  PERFORM relation_directory.name_to_type(entity_type_name || '->entity_set');
  SELECT id FROM entity.to_entity_set(name || '_' || "group" || '_' || owner) INTO entity_id;
  INSERT INTO attribute_staging.minerva_entity_set(
      entity_id, timestamp, name, fullname, "group", source_entity_type, owner, description, last_update
    ) VALUES (
      entity_id,
      now(),
      name,
      name || '_' || "group" || '_' || owner,
      "group",
      entity_type_name,
      owner,
      description,
      CURRENT_DATE::text
    );
  PERFORM attribute_directory.transfer_staged(attribute_directory.get_attribute_store('minerva', 'entity_set'));
  PERFORM attribute_directory.materialize_curr_ptr(attribute_directory.get_attribute_store('minerva', 'entity_set'));
  RETURN entity_id;
END;
$$ LANGUAGE plpgsql VOLATILE;
