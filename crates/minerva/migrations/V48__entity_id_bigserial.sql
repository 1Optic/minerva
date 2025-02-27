CREATE OR REPLACE FUNCTION "entity"."create_entity_table_sql"(directory.entity_type)
    RETURNS text[]
AS $$
SELECT ARRAY[
    format(
      'CREATE TABLE IF NOT EXISTS entity.%I('
      'id bigserial,'
      'name text UNIQUE,'
      'created timestamp with time zone default now()'
      ');',
      $1.name
    ),
    format(
       'SELECT create_reference_table(''entity.%I'');',
       $1.name
    )
];
$$ LANGUAGE sql VOLATILE;
