ALTER TYPE "notification"."generic_notification" DROP ATTRIBUTE "details";
ALTER TYPE "trigger"."notification" DROP ATTRIBUTE "details";


CREATE OR REPLACE FUNCTION "notification_directory"."get_next_notifications"("notification_store" text, "last_notification_seen" integer, "max_notifications" integer)
    RETURNS SETOF notification.generic_notification
AS $$
DECLARE
  entity_type text;
BEGIN
  SELECT et.name FROM notification_directory.notification_store ns
    JOIN directory.data_source ds ON ds.id = ns.data_source_id
    JOIN directory.entity_type et ON et.id = ns.entity_type_id
    WHERE ds.name = $1
    INTO entity_type;
  RETURN QUERY EXECUTE(FORMAT(
    'SELECT n.id as id, timestamp, r.name::text as rule, e.name::text as entity, weight, data '
    'FROM notification.%I n '
    'JOIN trigger.rule r ON n.rule_id = r.id '
    'JOIN entity.%I e on n.entity_id = e.id '
    'WHERE n.id > %s ORDER BY n.id  LIMIT %s',
    $1,
    entity_type,
    $2,
    $3
  ));
END;
$$ LANGUAGE plpgsql STABLE;


CREATE OR REPLACE FUNCTION "notification_directory"."get_last_notifications"("notification_store" text, "max_notifications" integer)
    RETURNS SETOF notification.generic_notification
AS $$
DECLARE
  entity_type text;
BEGIN
  SELECT et.name FROM notification_directory.notification_store ns
    JOIN directory.data_source ds ON ds.id = ns.data_source_id
    JOIN directory.entity_type et ON et.id = ns.entity_type_id
    WHERE ds.name = $1
    INTO entity_type;
  RETURN QUERY EXECUTE(FORMAT(
    'SELECT n.id as id, timestamp, r.name::text as rule, e.name::text as entity, weight, details, data '
    'FROM notification.%I n '
    'JOIN trigger.rule r ON n.rule_id = r.id '
    'JOIN entity.%I e on n.entity_id = e.id '
    'ORDER BY n.id DESC LIMIT %s',
    $1,
    entity_type,
    $2
  ));
END;
$$ LANGUAGE plpgsql STABLE;


CREATE OR REPLACE FUNCTION "trigger"."cleanup_rule"(trigger.rule)
    RETURNS trigger.rule
AS $$
BEGIN
    EXECUTE trigger.drop_set_thresholds_fn_sql($1);
    EXECUTE trigger.drop_rule_fn_sql($1);
    EXECUTE trigger.drop_kpi_function_sql($1);
    EXECUTE trigger.drop_notification_fn_sql($1);
    EXECUTE trigger.drop_runnable_fn_sql($1);
    EXECUTE trigger.drop_fingerprint_fn_sql($1);
    EXECUTE trigger.drop_with_threshold_fn_sql($1);
    EXECUTE trigger.drop_weight_fn_sql($1);
    EXECUTE trigger.drop_notification_message_fn_sql($1);
    EXECUTE trigger.drop_exception_weight_table_sql($1);
    EXECUTE trigger.drop_thresholds_view_sql($1);
    EXECUTE trigger.drop_exception_threshold_table_sql($1);
    EXECUTE trigger.drop_details_type_sql($1);
    EXECUTE trigger.drop_kpi_type_sql($1);

    RETURN $1;
END;
$$ LANGUAGE plpgsql VOLATILE;


SELECT public.action(trigger.drop_notification_type_sql(r)) FROM trigger.rule r;


DROP FUNCTION "trigger"."notification_test_threshold_fn_sql";
DROP FUNCTION "trigger"."notification_threshold_test_fn_name";

DROP FUNCTION "trigger"."drop_notification_type_sql";
DROP FUNCTION "trigger"."create_notification_type";
DROP FUNCTION "trigger"."create_notification_type_sql";
DROP FUNCTION "trigger"."notification_type_name";


CREATE OR REPLACE FUNCTION "trigger"."create_trigger_notification_store"(name)
    RETURNS notification_directory.notification_store
AS $$
SELECT trigger.add_insert_trigger(
        notification_directory.create_notification_store($1, ARRAY[
            ('created', 'timestamp with time zone', 'time of notification creation'),
            ('rule_id', 'integer', 'source rule for this notification'),
            ('weight', 'integer', 'weight/importance of the notification'),
            ('data', 'json', 'trigger specific data for notification')
        ]::notification_directory.attr_def[])
);
$$ LANGUAGE sql VOLATILE;


CREATE OR REPLACE FUNCTION "trigger"."transfer_notifications_from_staging"(notification_directory.notification_store)
    RETURNS integer
AS $$
DECLARE
    num_rows integer;
BEGIN
    EXECUTE format(
$query$
INSERT INTO notification.%I(entity_id, timestamp, created, rule_id, weight, data)
SELECT staging.entity_id, staging.timestamp, staging.created, staging.rule_id, staging.weight, staging.data
FROM notification.%I staging
LEFT JOIN notification.%I target ON target.entity_id = staging.entity_id AND target.timestamp = staging.timestamp AND target.rule_id = staging.rule_id
WHERE target.entity_id IS NULL;
$query$,
        notification_directory.table_name($1), notification_directory.staging_table_name($1), notification_directory.table_name($1));

    GET DIAGNOSTICS num_rows = ROW_COUNT;

    EXECUTE format('DELETE FROM notification.%I', notification_directory.staging_table_name($1));

    RETURN num_rows;
END;
$$ LANGUAGE plpgsql VOLATILE;


CREATE OR REPLACE FUNCTION "trigger"."create_notifications"(trigger.rule, notification_directory.notification_store, timestamp with time zone)
    RETURNS integer
AS $$
DECLARE
    num_rows integer;
BEGIN
    EXECUTE format(
$query$
INSERT INTO notification.%I(entity_id, timestamp, created, rule_id, weight, data)
(SELECT entity_id, timestamp, now(), $1, weight, data FROM trigger_rule.%I($2) WHERE data IS NOT NULL)
$query$,
        notification_directory.staging_table_name($2), trigger.notification_fn_name($1)
    )
    USING $1.id, $3;

    SELECT trigger.transfer_notifications_from_staging($2) INTO num_rows;

    RETURN num_rows;
END;
$$ LANGUAGE plpgsql VOLATILE;


CREATE OR REPLACE FUNCTION "trigger"."create_notifications"(trigger.rule, notification_directory.notification_store, interval)
    RETURNS integer
AS $$
DECLARE
    num_rows integer;
BEGIN
    EXECUTE format(
$query$
INSERT INTO notification.%I(entity_id, timestamp, created, rule_id, weight, data)
(SELECT entity_id, timestamp, now(), $1, weight, data FROM trigger_rule.%I WHERE timestamp > now() - $2)
$query$,
        notification_directory.staging_table_name($2), trigger.notification_view_name($1)
    )
    USING $1.id, $3;

    SELECT trigger.transfer_notifications_from_staging($2) INTO num_rows;

    RETURN num_rows;
END;
$$ LANGUAGE plpgsql VOLATILE;


CREATE OR REPLACE FUNCTION "trigger"."notification_fn_sql"(trigger.rule)
    RETURNS text
AS $$
SELECT format(
    'CREATE OR REPLACE FUNCTION trigger_rule.%I(timestamp with time zone)
    RETURNS SETOF trigger.notification
AS $fn$
SELECT
    n.entity_id,
    n.timestamp,
    COALESCE(exc.weight, trigger_rule.%I(n)) AS weight,
    trigger_rule.%I(n) AS data
FROM trigger_rule.%I($1) AS n
LEFT JOIN trigger_rule.%I AS exc ON
    exc.entity_id = n.entity_id AND
    exc.start <= n.timestamp AND
    exc.expires > n.timestamp $fn$ LANGUAGE sql STABLE',
    trigger.notification_fn_name($1),
    trigger.weight_fn_name($1),
    trigger.notification_data_fn_name($1),
    $1.name,
    trigger.exception_weight_table_name($1)
);
$$ LANGUAGE sql STABLE;

-- Update all existing 'trigger_rule.<name>_create_notification' functions
SELECT trigger.create_notification_fn(rule) FROM trigger.rule;

INSERT INTO notification_directory.attribute (notification_store_id, name, data_type, description) SELECT a1.notification_store_id, 'data', 'json', 'trigger specific data for notification' FROM notification_directory.attribute a1 LEFT JOIN notification_directory.attribute a2 ON a1.notification_store_id = a2.notification_store_id AND a2.name = 'data' WHERE a1.name = 'details' AND a2 IS NULL;

DELETE FROM notification_directory.attribute WHERE name = 'details';


SELECT public.action(
  FORMAT(
    'ALTER TABLE %I.%I DROP COLUMN IF EXISTS details',
    notification_directory.notification_store_schema(),
    notification_directory.table_name(ns)
 )) FROM notification_directory.notification_store ns;

SELECT public.action(
  FORMAT(
    'ALTER TABLE %I.%I DROP COLUMN IF EXISTS details',
    notification_directory.notification_store_schema(),
    notification_directory.staging_table_name(ns)
 )) FROM notification_directory.notification_store ns;
