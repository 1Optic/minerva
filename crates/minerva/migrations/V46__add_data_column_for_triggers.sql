CREATE OR REPLACE FUNCTION "trigger"."create_trigger_notification_store"(name)
    RETURNS notification_directory.notification_store
AS $$
SELECT trigger.add_insert_trigger(
        notification_directory.create_notification_store($1, ARRAY[
            ('created', 'timestamp with time zone', 'time of notification creation'),
            ('rule_id', 'integer', 'source rule for this notification'),
            ('weight', 'integer', 'weight/importance of the notification'),
            ('details', 'text', 'extra information'),
            ('data', 'json', 'trigger specific data for notification')
        ]::notification_directory.attr_def[])
);
$$ LANGUAGE sql VOLATILE;

INSERT INTO notification_directory.attribute (notification_store_id, name, data_type, description) SELECT a1.notification_store_id, 'data', 'json', 'trigger specific data for notification' FROM notification_directory.attribute a1 LEFT JOIN notification_directory.attribute a2 ON a1.notification_store_id = a2.notification_store_id AND a2.name = 'data' WHERE a1.name = 'details' AND a2 IS NULL;
