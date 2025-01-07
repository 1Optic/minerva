SET citus.multi_shard_modify_mode TO 'sequential';

DO
$do$
BEGIN
   IF EXISTS (
      SELECT FROM pg_catalog.pg_roles
      WHERE rolname = 'minerva') THEN

      RAISE NOTICE 'Role "minerva" already exists. Skipping.';
   ELSE
      BEGIN   -- nested block
        CREATE ROLE minerva
          NOSUPERUSER INHERIT NOCREATEDB NOCREATEROLE;
      EXCEPTION
         WHEN duplicate_object THEN
            RAISE NOTICE 'Role "minerva" was just created by a concurrent transaction. Skipping.';
      END;
   END IF;
END
$do$;

DO
$do$
BEGIN
   IF EXISTS (
      SELECT FROM pg_catalog.pg_roles
      WHERE rolname = 'minerva_writer') THEN

      RAISE NOTICE 'Role "minerva_writer" already exists. Skipping.';
   ELSE
      BEGIN   -- nested block
        CREATE ROLE minerva_writer
          NOSUPERUSER INHERIT NOCREATEDB NOCREATEROLE;
      EXCEPTION
         WHEN duplicate_object THEN
            RAISE NOTICE 'Role "minerva_writer" was just created by a concurrent transaction. Skipping.';
      END;
   END IF;
END
$do$;

GRANT minerva TO minerva_writer;


DO
$do$
BEGIN
   IF EXISTS (
      SELECT FROM pg_catalog.pg_roles
      WHERE rolname = 'minerva_admin') THEN

      RAISE NOTICE 'Role "minerva_admin" already exists. Skipping.';
   ELSE
      BEGIN   -- nested block
        CREATE ROLE minerva_admin
          LOGIN NOSUPERUSER INHERIT NOCREATEDB NOCREATEROLE;
      EXCEPTION
         WHEN duplicate_object THEN
            RAISE NOTICE 'Role "minerva_admin" was just created by a concurrent transaction. Skipping.';
      END;
   END IF;
END
$do$;

GRANT minerva TO minerva_admin;

GRANT minerva_writer TO minerva_admin;

GRANT postgres TO minerva_admin;

CREATE SCHEMA IF NOT EXISTS "system";


CREATE SCHEMA IF NOT EXISTS "directory";
COMMENT ON SCHEMA "directory" IS 'Stores contextual information for the data. This includes the entities, entity_types, data_sources, etc. It is the entrypoint when looking for data.';
GRANT USAGE ON SCHEMA "directory" TO "minerva";
ALTER DEFAULT PRIVILEGES IN SCHEMA "directory" GRANT USAGE,SELECT ON sequences TO "minerva_writer";

CREATE SCHEMA IF NOT EXISTS "entity";
GRANT USAGE,CREATE ON SCHEMA "entity" TO "minerva_writer";
GRANT USAGE ON SCHEMA "entity" TO "minerva";
ALTER DEFAULT PRIVILEGES IN SCHEMA "entity" GRANT SELECT,INSERT,UPDATE,DELETE ON tables TO "minerva_writer";

ALTER DEFAULT PRIVILEGES IN SCHEMA "entity" GRANT SELECT ON tables TO "minerva";

ALTER DEFAULT PRIVILEGES IN SCHEMA "entity" GRANT USAGE,SELECT ON sequences TO "minerva_writer";


CREATE SCHEMA IF NOT EXISTS "alias";
GRANT USAGE,CREATE ON SCHEMA "alias" TO "minerva_writer";
GRANT USAGE ON SCHEMA "alias" TO "minerva";
ALTER DEFAULT PRIVILEGES IN SCHEMA "alias" GRANT SELECT ON tables TO "minerva";

ALTER DEFAULT PRIVILEGES IN SCHEMA "alias" GRANT SELECT,UPDATE,DELETE ON tables TO "minerva_writer";

ALTER DEFAULT PRIVILEGES IN SCHEMA "alias" GRANT USAGE,SELECT ON sequences TO "minerva_writer";


CREATE SCHEMA IF NOT EXISTS "alias_def";
GRANT USAGE,CREATE ON SCHEMA "alias_def" TO "minerva_writer";
GRANT USAGE ON SCHEMA "alias_def" TO "minerva";
ALTER DEFAULT PRIVILEGES IN SCHEMA "alias_def" GRANT SELECT ON tables TO "minerva";

ALTER DEFAULT PRIVILEGES IN SCHEMA "alias_def" GRANT SELECT,UPDATE,DELETE ON tables TO "minerva_writer";

ALTER DEFAULT PRIVILEGES IN SCHEMA "alias_def" GRANT USAGE,SELECT ON sequences TO "minerva_writer";


CREATE SCHEMA IF NOT EXISTS "alias_directory";
GRANT USAGE,CREATE ON SCHEMA "alias_directory" TO "minerva_writer";
GRANT USAGE ON SCHEMA "alias_directory" TO "minerva";
ALTER DEFAULT PRIVILEGES IN SCHEMA "alias_directory" GRANT USAGE,SELECT ON sequences TO "minerva_writer";


CREATE SCHEMA IF NOT EXISTS "relation";
COMMENT ON SCHEMA "relation" IS 'Stores the actual relations between entities in tables.
';
GRANT USAGE,CREATE ON SCHEMA "relation" TO "minerva_writer";
GRANT USAGE ON SCHEMA "relation" TO "minerva";
ALTER DEFAULT PRIVILEGES IN SCHEMA "relation" GRANT SELECT,INSERT,UPDATE,DELETE ON tables TO "minerva_writer";

ALTER DEFAULT PRIVILEGES IN SCHEMA "relation" GRANT SELECT ON tables TO "minerva";


CREATE SCHEMA IF NOT EXISTS "relation_def";
COMMENT ON SCHEMA "relation_def" IS 'Stores the views that define the contents of the relation tables.
';
GRANT USAGE,CREATE ON SCHEMA "relation_def" TO "minerva_writer";
GRANT USAGE ON SCHEMA "relation_def" TO "minerva";
ALTER DEFAULT PRIVILEGES IN SCHEMA "relation_def" GRANT SELECT ON tables TO "minerva";

ALTER DEFAULT PRIVILEGES IN SCHEMA "relation_def" GRANT SELECT,INSERT,UPDATE,DELETE ON tables TO "minerva_writer";

ALTER DEFAULT PRIVILEGES IN SCHEMA "relation_def" GRANT USAGE,SELECT ON sequences TO "minerva_writer";


CREATE SCHEMA IF NOT EXISTS "relation_directory";
ALTER DEFAULT PRIVILEGES IN SCHEMA "relation_directory" GRANT USAGE,SELECT ON sequences TO "minerva_writer";



CREATE SCHEMA IF NOT EXISTS "trend";
COMMENT ON SCHEMA "trend" IS 'Stores information with fixed interval and format, like periodic measurements.';
GRANT USAGE,CREATE ON SCHEMA "trend" TO "minerva_writer";
GRANT USAGE ON SCHEMA "trend" TO "minerva";
ALTER DEFAULT PRIVILEGES IN SCHEMA "trend" GRANT SELECT,INSERT,UPDATE,DELETE ON tables TO "minerva_writer";

ALTER DEFAULT PRIVILEGES IN SCHEMA "trend" GRANT SELECT ON tables TO "minerva";


CREATE SCHEMA IF NOT EXISTS "trend_partition";
COMMENT ON SCHEMA "trend_partition" IS 'Holds partitions of the trend store tables in the trend schema.';
GRANT USAGE,CREATE ON SCHEMA "trend_partition" TO "minerva_writer";
GRANT USAGE ON SCHEMA "trend_partition" TO "minerva";
ALTER DEFAULT PRIVILEGES IN SCHEMA "trend_partition" GRANT SELECT,INSERT,UPDATE,DELETE ON tables TO "minerva_writer";

ALTER DEFAULT PRIVILEGES IN SCHEMA "trend_partition" GRANT SELECT ON tables TO "minerva";


CREATE SCHEMA IF NOT EXISTS "trend_directory";
GRANT USAGE ON SCHEMA "trend_directory" TO "minerva_writer";
GRANT USAGE ON SCHEMA "trend_directory" TO "minerva";
ALTER DEFAULT PRIVILEGES IN SCHEMA "trend_directory" GRANT USAGE,SELECT ON sequences TO "minerva_writer";



CREATE SCHEMA IF NOT EXISTS "attribute";
GRANT USAGE,CREATE ON SCHEMA "attribute" TO "minerva_writer";
GRANT USAGE ON SCHEMA "attribute" TO "minerva";
ALTER DEFAULT PRIVILEGES IN SCHEMA "attribute" GRANT ALL ON tables TO "minerva_writer";

ALTER DEFAULT PRIVILEGES IN SCHEMA "attribute" GRANT SELECT ON tables TO "minerva";


CREATE SCHEMA IF NOT EXISTS "attribute_base";
GRANT USAGE,CREATE ON SCHEMA "attribute_base" TO "minerva_writer";
GRANT USAGE ON SCHEMA "attribute_base" TO "minerva";
ALTER DEFAULT PRIVILEGES IN SCHEMA "attribute_base" GRANT ALL ON tables TO "minerva_writer";

ALTER DEFAULT PRIVILEGES IN SCHEMA "attribute_base" GRANT SELECT ON tables TO "minerva";


CREATE SCHEMA IF NOT EXISTS "attribute_directory";
GRANT USAGE,CREATE ON SCHEMA "attribute_directory" TO "minerva_writer";
GRANT USAGE ON SCHEMA "attribute_directory" TO "minerva";
ALTER DEFAULT PRIVILEGES IN SCHEMA "attribute_directory" GRANT USAGE,SELECT ON sequences TO "minerva_writer";


CREATE SCHEMA IF NOT EXISTS "attribute_history";
GRANT USAGE,CREATE ON SCHEMA "attribute_history" TO "minerva_writer";
GRANT USAGE ON SCHEMA "attribute_history" TO "minerva";
ALTER DEFAULT PRIVILEGES IN SCHEMA "attribute_history" GRANT ALL ON tables TO "minerva_writer";

ALTER DEFAULT PRIVILEGES IN SCHEMA "attribute_history" GRANT SELECT ON tables TO "minerva";

ALTER DEFAULT PRIVILEGES IN SCHEMA "attribute_history" GRANT USAGE,SELECT ON sequences TO "minerva_writer";


CREATE SCHEMA IF NOT EXISTS "attribute_staging";
GRANT USAGE,CREATE ON SCHEMA "attribute_staging" TO "minerva_writer";
GRANT USAGE ON SCHEMA "attribute_staging" TO "minerva";
ALTER DEFAULT PRIVILEGES IN SCHEMA "attribute_staging" GRANT SELECT,INSERT,UPDATE,DELETE ON tables TO "minerva_writer";

ALTER DEFAULT PRIVILEGES IN SCHEMA "attribute_staging" GRANT SELECT ON tables TO "minerva";


CREATE SCHEMA IF NOT EXISTS "notification";
COMMENT ON SCHEMA "notification" IS 'Stores information of events that can occur at irregular intervals,
but still have a fixed, known format. This schema is dynamically populated.';
GRANT USAGE,CREATE ON SCHEMA "notification" TO "minerva_writer";
GRANT USAGE ON SCHEMA "notification" TO "minerva";
ALTER DEFAULT PRIVILEGES IN SCHEMA "notification" GRANT ALL ON tables TO "minerva_writer";

ALTER DEFAULT PRIVILEGES IN SCHEMA "notification" GRANT SELECT ON tables TO "minerva";

ALTER DEFAULT PRIVILEGES IN SCHEMA "notification" GRANT USAGE,SELECT ON sequences TO "minerva_writer";


CREATE SCHEMA IF NOT EXISTS "notification_directory";
COMMENT ON SCHEMA "notification_directory" IS 'Stores meta-data about notification data in the notification schema.';
GRANT USAGE,CREATE ON SCHEMA "notification_directory" TO "minerva_writer";
GRANT USAGE ON SCHEMA "notification_directory" TO "minerva";
ALTER DEFAULT PRIVILEGES IN SCHEMA "notification_directory" GRANT USAGE,SELECT ON sequences TO "minerva_writer";


CREATE SCHEMA IF NOT EXISTS "metric";
GRANT USAGE ON SCHEMA "metric" TO "minerva";


CREATE SCHEMA IF NOT EXISTS "virtual_entity";
GRANT USAGE ON SCHEMA "virtual_entity" TO "minerva";


CREATE SCHEMA IF NOT EXISTS "olap";
GRANT USAGE,CREATE ON SCHEMA "olap" TO "minerva_writer";
GRANT USAGE ON SCHEMA "olap" TO "minerva";


CREATE SCHEMA IF NOT EXISTS "trigger";
GRANT USAGE,CREATE ON SCHEMA "trigger" TO "minerva_writer";
GRANT USAGE ON SCHEMA "trigger" TO "minerva";
ALTER DEFAULT PRIVILEGES IN SCHEMA "trigger" GRANT USAGE,SELECT ON sequences TO "minerva_writer";


CREATE SCHEMA IF NOT EXISTS "trigger_rule";
GRANT USAGE,CREATE ON SCHEMA "trigger_rule" TO "minerva_writer";
GRANT USAGE ON SCHEMA "trigger_rule" TO "minerva";
ALTER DEFAULT PRIVILEGES IN SCHEMA "trigger_rule" GRANT SELECT ON tables TO "minerva";

ALTER DEFAULT PRIVILEGES IN SCHEMA "trigger_rule" GRANT ALL ON tables TO "minerva_admin";

ALTER DEFAULT PRIVILEGES IN SCHEMA "trigger_rule" GRANT SELECT,INSERT,UPDATE,DELETE ON tables TO "minerva_writer";

ALTER DEFAULT PRIVILEGES IN SCHEMA "trigger_rule" GRANT USAGE,SELECT ON sequences TO "minerva_writer";

ALTER DEFAULT PRIVILEGES IN SCHEMA "trigger_rule" GRANT EXECUTE ON functions TO "minerva";



CREATE SCHEMA IF NOT EXISTS "logging";
GRANT USAGE ON SCHEMA "logging" TO "minerva";
ALTER DEFAULT PRIVILEGES IN SCHEMA "logging" GRANT USAGE,SELECT ON sequences TO "minerva_writer";



