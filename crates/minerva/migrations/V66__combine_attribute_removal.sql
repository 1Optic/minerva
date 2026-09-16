CREATE FUNCTION attribute_directory.remove_attribute_columns(attribute_directory.attribute_store, attributes name[]) RETURNS attribute_directory.attribute_store  
AS $$
    DECLARE
        drop_column_parts text[];
        drop_column_text text;
        attribute_name name;
    BEGIN
        FOREACH attribute_name IN ARRAY attributes LOOP
            drop_column_parts := array_append(drop_column_parts, format('DROP COLUMN %I CASCADE', attribute_name));
        END LOOP;
        drop_column_text := array_to_string(drop_column_parts, ', ');
        EXECUTE format('SELECT attribute_directory.drop_hash(%s::attribute_directory.attribute_store)', $1);
        EXECUTE format('ALTER TABLE attribute_base.%I %s', attribute_directory.to_char($1), drop_column_text);
        EXECUTE format('ALTER TABLE attribute_history.%I %s', attribute_directory.to_char($1), drop_column_text);
        EXECUTE format('SELECT attribute_directory.add_hash(%s::attribute_directory.attribute_store)', $1);
        EXECUTE format('SELECT attribute_directory.drop_staging_dependees(%s)', $1);
        EXECUTE format('ALTER TABLE attribute_staging.%I %s', attribute_directory.to_char($1), drop_column_text);
        EXECUTE format('SELECT attribute_directory.add_staging_dependees(%s)', $1);
    RETURN $1;
    END;
$$ LANGUAGE plpgsql VOLATILE;

CREATE FUNCTION attribute_directory.drop_attributes(attribute_directory.attribute_store, attributes name[]) RETURNS attribute_directory.attribute_store
AS $$
    DELETE FROM attribute_directory.attribute
        WHERE attribute_store_id = $1.id AND name = ANY(attributes);
    SELECT attribute_directory.remove_attribute_columns($1, attributes);
$$ LANGUAGE sql VOLATILE;
