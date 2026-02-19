-- Note: This migration can only be applied by a superuser.

CREATE FUNCTION system.set_owner_to_postgres() RETURNS event_trigger
AS $$
DECLARE
  command_tag text;
  object_type text;
  schema_name text;
  object_id integer;
  object_name text;
BEGIN
  SELECT c.command_tag, c.object_type, c.schema_name, c.objid FROM pg_event_trigger_ddl_commands() c INTO command_tag, object_type, schema_name, object_id;
  IF command_tag IN ('CREATE TABLE', 'CREATE VIEW') AND schema_name IN ('alias_directory', 'attribute_directory', 'directory', 'entity', 'logging', 'notification', 'notification_directory', 'public', 'relation_directory', 'system', 'trend_directory', 'trigger', 'virtual_entity') THEN
    SELECT relname FROM pg_class WHERE oid = object_id INTO object_name;
    EXECUTE format('ALTER %s %I.%I OWNER TO postgres', object_type, schema_name, object_name);
  ELSIF command_tag IN ('CREATE FUNCTION', 'CREATE PROCEDURE') AND schema_name IN ('alias_directory', 'attribute_directory', 'directory', 'entity', 'logging', 'notification', 'notification_directory', 'public', 'relation_directory', 'system', 'trend_directory', 'trigger', 'virtual_entity') THEN
    SELECT proname FROM pg_proc WHERE oid = object_id INTO object_name;
    EXECUTE format('ALTER %s %I.%I(%s) OWNER TO postgres',
      object_type, schema_name, object_name,
      (SELECT string_agg(format_type(oid, NULL), ',') FROM unnest((SELECT proargtypes FROM pg_proc WHERE proname = object_name AND pronamespace = (SELECT oid FROM pg_namespace WHERE nspname = schema_name))::oid[]) AS oid)
    );
  END IF;
END;
$$ LANGUAGE plpgsql VOLATILE;

CREATE EVENT TRIGGER set_owner_to_postgres ON ddl_command_end EXECUTE FUNCTION system.set_owner_to_postgres();

ALTER FUNCTION system.set_owner_to_postgres() OWNER TO postgres;
