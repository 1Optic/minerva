CREATE FUNCTION change_ownership_for_schema("schema" text, new_owner text)
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
END;
$$ LANGUAGE plpgsql VOLATILE;

SELECT change_ownership_for_schema('alias', 'postgres');
SELECT change_ownership_for_schema('alias_def', 'postgres');
SELECT change_ownership_for_schema('alias_directory', 'postgres');
SELECT change_ownership_for_schema('attribute', 'postgres');
SELECT change_ownership_for_schema('attribute_base', 'postgres');
SELECT change_ownership_for_schema('attribute_directory', 'postgres');
SELECT change_ownership_for_schema('attribute_history', 'postgres');
SELECT change_ownership_for_schema('attribute_staging', 'postgres');
SELECT change_ownership_for_schema('cached', 'postgres');
SELECT change_ownership_for_schema('cached_def', 'postgres');
SELECT change_ownership_for_schema('directory', 'postgres');
SELECT change_ownership_for_schema('entity', 'postgres');
SELECT change_ownership_for_schema('handover', 'postgres');
SELECT change_ownership_for_schema('handover_directory', 'postgres');
SELECT change_ownership_for_schema('logging', 'postgres');
SELECT change_ownership_for_schema('notification', 'postgres');
SELECT change_ownership_for_schema('notification_directory', 'postgres');
SELECT change_ownership_for_schema('relation', 'postgres');
SELECT change_ownership_for_schema('relation_def', 'postgres');
SELECT change_ownership_for_schema('relation_directory', 'postgres');
SELECT change_ownership_for_schema('staging', 'postgres');
SELECT change_ownership_for_schema('trend', 'postgres');
SELECT change_ownership_for_schema('trend_directory', 'postgres');
SELECT change_ownership_for_schema('trend_partition', 'postgres');
SELECT change_ownership_for_schema('trigger', 'postgres');
SELECT change_ownership_for_schema('trigger_rule', 'postgres');
SELECT change_ownership_for_schema('virtual_entity', 'postgres');

DROP FUNCTION change_ownership_for_schema(text, text);
