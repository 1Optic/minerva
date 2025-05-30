DROP FUNCTION trigger.create_notifications(name);

DROP FUNCTION trigger.create_notifications(trigger.rule);

DROP FUNCTION trigger.create_notifications(name, interval);

DROP FUNCTION trigger.create_notifications(trigger.rule, interval);

DROP FUNCTION trigger.create_notifications(name, name, interval);

DROP FUNCTION trigger.create_notifications(trigger.rule, notification_directory.notification_store, interval);

DROP FUNCTION trigger.notification_view_name(trigger.rule);
