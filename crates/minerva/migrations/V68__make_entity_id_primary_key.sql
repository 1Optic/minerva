CREATE OR REPLACE FUNCTION "entity"."create_entity_table_sql"(directory.entity_type, primary_alias text)
    RETURNS text[]
AS $$
SELECT ARRAY[
    format(
      'CREATE TABLE IF NOT EXISTS entity.%I('
      'id serial PRIMARY KEY, '
      'name text UNIQUE, '
      'created timestamp with time zone default now(), '
      'primary_alias text GENERATED ALWAYS AS (%s) STORED'
      ');',
      $1.name,
      $2
    ),
    format(
       'SELECT create_reference_table(''entity.%I'');',
       $1.name
    )
];
$$ LANGUAGE sql VOLATILE STRICT;

