ALTER TABLE directory.tag_group ALTER COLUMN complementary SET DEFAULT false;

CREATE FUNCTION directory.create_tag_group(name varchar) RETURNS integer
    LANGUAGE sql
AS $$
    INSERT INTO directory.tag_group (name) VALUES ($1) RETURNING id;
$$;

CREATE FUNCTION directory.get_or_create_tag_group(name varchar) RETURNS integer
    LANGUAGE sql
AS $$
    SELECT COALESCE(
        (SELECT id FROM directory.tag_group WHERE name = $1),
        (SELECT directory.create_tag_group($1))
    );
$$;

CREATE FUNCTION directory.create_tag(name varchar, default_tag_group_name varchar) RETURNS integer
    LANGUAGE sql
AS $$
    INSERT INTO directory.tag (name, tag_group_id)
        VALUES ($1, directory.get_or_create_tag_group($2))
    RETURNING id;
$$;

CREATE FUNCTION directory.get_or_create_tag(name varchar, default_tag_group_name varchar) RETURNS integer
    LANGUAGE sql
AS $$
    SELECT COALESCE(
        (SELECT id FROM directory.tag WHERE name = $1),
        (SELECT directory.create_tag($1, $2))
    );
$$;

CREATE FUNCTION trigger.tag(tag_name varchar, rule_id integer, default_tag_group_name varchar) RETURNS trigger.rule_tag_link
    LANGUAGE sql
AS $$
    INSERT INTO trigger.rule_tag_link (rule_id, tag_id)
        VALUES ($2, directory.get_or_create_tag($1, $3))
    RETURNING *;
$$;

CREATE FUNCTION trigger.set_tags(rule_id integer, tag_names text[], default_tag_group_name text) RETURNS trigger.rule_tag_link
    LANGUAGE sql
AS $$
    DELETE FROM trigger.rule_tag_link WHERE rule_id = $1;
    SELECT trigger.tag(tag_name::varchar, rule_id, default_tag_group_name::varchar)
        FROM unnest($2) AS tag_name;
$$;

CREATE FUNCTION trigger.set_fingerprint_fn(trigger.rule, fn_sql text) RETURNS trigger.rule
    LANGUAGE sql
AS $$
    SELECT public.action($1, trigger.create_fingerprint_fn_sql($1, $2));
$$;
