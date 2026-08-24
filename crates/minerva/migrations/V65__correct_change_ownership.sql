CREATE FUNCTION directory.change_ownership_for_all_schemas(new_owner text)
  RETURNS void AS $$
    SET LOCAL citus.multi_shard_modify_mode TO 'sequential';
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
  