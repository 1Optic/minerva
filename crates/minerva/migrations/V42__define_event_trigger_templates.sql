SELECT "trigger"."create_trigger_notification_store"('template-trigger-notification');

CREATE TABLE "trigger"."template"
(
    "id" serial NOT NULL,
    "name" text NOT NULL,
    "description_body" text,
    "sql_body" text NOT NULL DEFAULT 'FALSE',
    PRIMARY KEY (id)
);

CREATE UNIQUE INDEX "ix_template_name" ON "trigger"."template" USING btree(name);

CREATE TABLE "trigger"."template_parameter"
(
    "template_id" integer NOT NULL,
    "name" text NOT NULL,
    "is_variable" boolean DEFAULT false,
    "is_source_name" boolean DEFAULT false,
    PRIMARY KEY (template_id, "name")
);
