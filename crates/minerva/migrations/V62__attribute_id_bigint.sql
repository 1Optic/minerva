-- 2,+25 replaces V10__main.sql 5528,+25
CREATE OR REPLACE FUNCTION attribute_directory.create_at_func_ptr_sql(attribute_directory.attribute_store) RETURNS text[]
    LANGUAGE sql STABLE
    AS $function$
SELECT ARRAY[
        format(
            'CREATE FUNCTION attribute_history.%I(timestamp with time zone)
RETURNS TABLE(id bigint)
AS $$
    BEGIN
        RETURN QUERY SELECT DISTINCT ON (entity_id) s.id
            FROM attribute_history.%I s
            WHERE timestamp <= $1
            ORDER BY entity_id, timestamp DESC;
    END;
$$ LANGUAGE plpgsql STABLE',
            attribute_directory.at_ptr_function_name($1),
            attribute_directory.to_table_name($1)
        ),
        format(
            'ALTER FUNCTION attribute_history.%I(timestamp with time zone) '
            'OWNER TO minerva_writer',
            attribute_directory.at_ptr_function_name($1)
        )
    ];
$function$;

-- 29,+18 replaces V10__main.sql 4483,+18
CREATE OR REPLACE FUNCTION attribute_directory.create_curr_ptr_table_sql(attribute_directory.attribute_store) RETURNS text[]
    LANGUAGE sql STABLE
    AS $_$
SELECT ARRAY[
    format('CREATE TABLE attribute_history.%I (
        id bigint,
        PRIMARY KEY (id))',
        attribute_directory.curr_ptr_table_name($1)
    ),
    format(
        'CREATE INDEX ON attribute_history.%I (id)',
        attribute_directory.curr_ptr_table_name($1)
    ),
    format(
        'ALTER TABLE attribute_history.%I OWNER TO minerva_writer',
        attribute_directory.curr_ptr_table_name($1)
    )
];
$_$;

-- 50,+24 replaces V10__main.sql 5587,+24
CREATE OR REPLACE FUNCTION attribute_directory.create_entity_at_func_ptr_sql(attribute_directory.attribute_store) RETURNS text[]
    AS $_$
SELECT ARRAY[
    format(
        'CREATE FUNCTION attribute_history.%I(entity_id integer, timestamp with time zone)
RETURNS bigint
AS $$
  BEGIN
    RETURN a.id
    FROM
        attribute_history.%I a
    WHERE a.timestamp <= $2 AND a.entity_id = $1
    ORDER BY a.timestamp DESC LIMIT 1;
  END;
$$ LANGUAGE plpgsql STABLE',
        attribute_directory.at_ptr_function_name($1),
        attribute_directory.to_table_name($1)
    ),
    format(
        'ALTER FUNCTION attribute_history.%I(entity_id integer, timestamp with time zone) '
        'OWNER TO minerva_writer',
        attribute_directory.at_ptr_function_name($1)
    )
];
$_$ LANGUAGE sql STABLE;

-- 77,+38 replaces V10__main.sql 4674,+38
CREATE OR REPLACE FUNCTION attribute_directory.create_history_table_sql(attribute_directory.attribute_store) RETURNS text[]
    LANGUAGE sql STABLE
    AS $_$
SELECT ARRAY[
    format(
        'CREATE TABLE attribute_history.%I (
        id bigserial,
        first_appearance timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
        modified timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
        hash character varying GENERATED ALWAYS AS (%s) STORED,
        %s,
        PRIMARY KEY (id, entity_id)
        )',
        attribute_directory.to_table_name($1),
        attribute_directory.hash_query($1),
        array_to_string(attribute_directory.column_specs($1), ',')
    ),
    format(
        'CREATE INDEX ON attribute_history.%I (id)',
        attribute_directory.to_table_name($1)
    ),
    format(
        'CREATE INDEX ON attribute_history.%I (first_appearance)',
        attribute_directory.to_table_name($1)
    ),
    format(
        'CREATE INDEX ON attribute_history.%I (modified)',
        attribute_directory.to_table_name($1)
    ),
    format(
        'ALTER TABLE attribute_history.%I OWNER TO minerva_writer',
        attribute_directory.to_table_name($1)
    ),
    format(
        'SELECT create_distributed_table(''attribute_history.%I'', ''entity_id'')',
        attribute_directory.to_table_name($1)
    )
];
$_$;

-- 118,+13 replaces V10__main.sql 4935,+10
DROP FUNCTION IF EXISTS attribute_directory.last_history_id(integer);

CREATE FUNCTION attribute_directory.last_history_id(attribute_store_id integer) RETURNS bigint
    AS $_$
DECLARE
  result bigint;
BEGIN
  EXECUTE FORMAT(
    'SELECT COALESCE(MAX(id), 0) FROM attribute_history.%I',
    attribute_directory.to_table_name(attribute_directory.get_attribute_store($1))
  ) INTO result;
  RETURN result;
END;
$_$LANGUAGE plpgsql STABLE;
