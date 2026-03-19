CREATE FUNCTION directory.change_ownership_for_schema("schema" text, new_owner text)
  RETURNS void AS $$
    DECLARE
      "table" text;
      "view" text;
      "type" text;
    BEGIN
      FOR "table" IN (SELECT tablename FROM pg_tables WHERE schemaname = $1) LOOP
        PERFORM run_command_on_workers(format('ALTER TABLE %I.%I OWNER TO %I', $1, "table", $2));
        EXECUTE format('ALTER TABLE %I.%I OWNER TO %I', $1, "table", $2);
      END LOOP;
      FOR "view" IN (SELECT table_name FROM information_schema.views WHERE table_schema = $1) LOOP
        PERFORM run_command_on_workers(format('ALTER VIEW %I.%I OWNER TO %I', $1, "view", $2));
        EXECUTE format('ALTER VIEW %I.%I OWNER TO %I', $1, "view", $2);
      END LOOP;
      FOR "type" IN (
        SELECT typname FROM pg_type t 
          LEFT JOIN pg_catalog.pg_namespace n ON t.typnamespace = n.oid 
          LEFT JOIN pg_catalog.pg_class c ON c.oid = t.typrelid 
        WHERE c.relkind = 'c' AND n.nspname = $1
      ) LOOP
        PERFORM run_command_on_workers(format('ALTER TYPE %I.%I OWNER TO %I', $1, "type", $2));
        EXECUTE format('ALTER TYPE %I.%I OWNER TO %I', $1, "type", $2);
      END LOOP;
    END;
  $$ LANGUAGE plpgsql VOLATILE;

CREATE FUNCTION directory.change_ownership_for_all_schemas(new_owner text)
  RETURNS void AS $$
    SELECT directory.change_ownership_for_schema('alias', new_owner);
    SELECT directory.change_ownership_for_schema('alias_def', new_owner);
    SELECT directory.change_ownership_for_schema('alias_directory', new_owner);
    SELECT directory.change_ownership_for_schema('attribute', new_owner);
    SELECT directory.change_ownership_for_schema('attribute_base', new_owner);
    SELECT directory.change_ownership_for_schema('attribute_directory', new_owner);
    SELECT directory.change_ownership_for_schema('attribute_history', new_owner);
    SELECT directory.change_ownership_for_schema('attribute_staging', new_owner);
    SELECT directory.change_ownership_for_schema('cached', new_owner);
    SELECT directory.change_ownership_for_schema('cached_def', new_owner);
    SELECT directory.change_ownership_for_schema('directory', new_owner);
    SELECT directory.change_ownership_for_schema('entity', new_owner);
    SELECT directory.change_ownership_for_schema('handover', new_owner);
    SELECT directory.change_ownership_for_schema('handover_directory', new_owner);
    SELECT directory.change_ownership_for_schema('logging', new_owner);
    SELECT directory.change_ownership_for_schema('notification', new_owner);
    SELECT directory.change_ownership_for_schema('notification_directory', new_owner);
    SELECT directory.change_ownership_for_schema('relation', new_owner);
    SELECT directory.change_ownership_for_schema('relation_def', new_owner);
    SELECT directory.change_ownership_for_schema('relation_directory', new_owner);
    SELECT directory.change_ownership_for_schema('staging', new_owner);
    SELECT directory.change_ownership_for_schema('trend', new_owner);
    SELECT directory.change_ownership_for_schema('trend_directory', new_owner);
    SELECT directory.change_ownership_for_schema('trend_partition', new_owner);
    SELECT directory.change_ownership_for_schema('trigger', new_owner);
    SELECT directory.change_ownership_for_schema('trigger_rule', new_owner);
    SELECT directory.change_ownership_for_schema('virtual_entity', new_owner);
  $$ LANGUAGE sql VOLATILE;

SELECT directory.change_ownership_for_all_schemas('postgres');
