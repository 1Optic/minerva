ALTER TABLE alias_directory.alias_type OWNER TO postgres;

ALTER TABLE attribute_directory.attribute OWNER TO postgres;
ALTER TABLE attribute_directory.attribute_store OWNER TO postgres;
ALTER TABLE attribute_directory.attribute_store_compacted OWNER TO postgres;
ALTER TABLE attribute_directory.attribute_store_curr_materialized OWNER TO postgres;
ALTER TABLE attribute_directory.attribute_store_modified OWNER TO postgres;
ALTER TABLE attribute_directory.attribute_tag_link OWNER TO postgres;
ALTER TABLE attribute_directory.sampled_view_materialization OWNER TO postgres;

ALTER TABLE directory.data_source OWNER TO postgres;
ALTER TABLE directory.entity_type OWNER TO postgres;
ALTER TABLE directory.tag OWNER TO postgres;
ALTER TABLE directory.tag_group OWNER TO postgres;
ALTER TABLE entity.entity_set OWNER TO postgres;
ALTER TABLE logging.job OWNER TO postgres;

ALTER TABLE notification_directory.attribute OWNER TO postgres;
ALTER TABLE notification_directory.last_notification OWNER TO postgres;
ALTER TABLE notification_directory.notification_set_store OWNER TO postgres;
ALTER TABLE notification_directory.notification_store OWNER TO postgres;
ALTER TABLE notification_directory.set_attribute OWNER TO postgres;

ALTER TABLE relation_directory.type OWNER TO postgres;
ALTER TABLE system.setting OWNER TO postgres;

ALTER TABLE trend_directory.function_materialization OWNER TO postgres;
ALTER TABLE trend_directory.function_materialization_state OWNER TO postgres;
ALTER TABLE trend_directory.generated_table_trend OWNER TO postgres;
ALTER TABLE trend_directory.materialization OWNER TO postgres;
ALTER TABLE trend_directory.materialization_metrics OWNER TO postgres;
ALTER TABLE trend_directory.materialization_state OWNER TO postgres;
ALTER TABLE trend_directory.materialization_tag_link OWNER TO postgres;
ALTER TABLE trend_directory.materialization_trend_store_link OWNER TO postgres;
ALTER TABLE trend_directory.modified OWNER TO postgres;
ALTER TABLE trend_directory.modified_log OWNER TO postgres;
ALTER TABLE trend_directory.modified_log_processing_state OWNER TO postgres;
ALTER TABLE trend_directory.partition OWNER TO postgres;
ALTER TABLE trend_directory.table_trend OWNER TO postgres;
ALTER TABLE trend_directory.table_trend_statistics OWNER TO postgres;
ALTER TABLE trend_directory.table_trend_tag_link OWNER TO postgres;
ALTER TABLE trend_directory.trend_store OWNER TO postgres;
ALTER TABLE trend_directory.trend_store_part OWNER TO postgres;
ALTER TABLE trend_directory.trend_store_part_stats OWNER TO postgres;
ALTER TABLE trend_directory.trend_view OWNER TO postgres;
ALTER TABLE trend_directory.trend_view_part OWNER TO postgres;
ALTER TABLE trend_directory.view_materialization OWNER TO postgres;
ALTER TABLE trend_directory.view_trend OWNER TO postgres;

ALTER TABLE trigger.exception_base OWNER TO postgres;
ALTER TABLE trigger.rule OWNER TO postgres;
ALTER TABLE trigger.rule_tag_link OWNER TO postgres;
ALTER TABLE trigger.rule_trend_store_link OWNER TO postgres;
ALTER TABLE trigger.template OWNER TO postgres;
ALTER TABLE trigger.template_parameter OWNER TO postgres;

ALTER VIEW trend_directory.trend_store_part_stats_to_update OWNER TO postgres;
ALTER VIEW attribute_directory.dependencies OWNER TO postgres;

CREATE OR REPLACE FUNCTION chown_all_functions(schema text)
  RETURNS void
AS $$
DECLARE
  function_data RECORD;
  argument_types text;
BEGIN
  FOR function_data IN SELECT proname AS name, CASE WHEN prokind = 'p' THEN 'PROCEDURE' ELSE 'FUNCTION' END AS kind, proargtypes AS arguments FROM pg_proc p JOIN pg_namespace ns ON p.pronamespace = ns.oid JOIN pg_roles r ON p.proowner = r.oid WHERE nspname = $1 AND rolname != 'postgres'
  LOOP
    SELECT string_agg(format_type(oid, NULL), ',') INTO argument_types FROM unnest(function_data.arguments) AS oid;
    EXECUTE format(
      'ALTER %s %I.%I(%s) OWNER TO postgres;',
      function_data.kind, $1, function_data.name, argument_types
    );
  END LOOP;
END;
$$ LANGUAGE plpgsql VOLATILE;

SELECT chown_all_functions('alias_directory');
SELECT chown_all_functions('attribute_directory');
SELECT chown_all_functions('directory');
SELECT chown_all_functions('entity');
SELECT chown_all_functions('logging');
SELECT chown_all_functions('notification');
SELECT chown_all_functions('notification_directory');
SELECT chown_all_functions('public');
SELECT chown_all_functions('relation_directory');
SELECT chown_all_functions('system');
SELECT chown_all_functions('trend');
SELECT chown_all_functions('trend_directory');
SELECT chown_all_functions('trigger');
SELECT chown_all_functions('virtual_entity');

DROP FUNCTION chown_all_functions(text);

