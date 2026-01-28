CREATE OR REPLACE FUNCTION "trigger"."drop_exception_weight_table_sql"(trigger.rule)
    RETURNS text
AS $$
SELECT format('DROP TABLE IF EXISTS trigger_rule.%I CASCADE', trigger.exception_weight_table_name($1));
$$ LANGUAGE sql IMMUTABLE;

CREATE OR REPLACE FUNCTION "trigger"."drop_exception_threshold_table_sql"(trigger.rule)
    RETURNS text
AS $$
SELECT format('DROP TABLE IF EXISTS trigger_rule.%I CASCADE', trigger.exception_threshold_table_name($1))
$$ LANGUAGE sql IMMUTABLE;
