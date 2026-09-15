CREATE FUNCTION attribute_directory.remove_attribute_columns(attribute_directory.attribute_store, name[]) RETURNS attribute_directory.attribute_store  
AS $$
    BEGIN
        EXECUTE format('SELECT attribute_directory.drop_hash(%s::attribute_directory.attribute_store)', $1);
        EXECUTE format('ALTER TABLE attribute_base.%I DROP COLUMN "%s" CASCADE', attribute_directory.to_char($1), array_to_string($2, '" CASCADE, DROP COLUMN "'));
        EXECUTE format('ALTER TABLE attribute_history.%I DROP COLUMN "%s" CASCADE', attribute_directory.to_char($1), array_to_string($2, '" CASCADE, DROP COLUMN "'));
        EXECUTE format('SELECT attribute_directory.add_hash(%s::attribute_directory.attribute_store)', $1);
        EXECUTE format('SELECT attribute_directory.drop_staging_dependees(%s)', $1);
        EXECUTE format('ALTER TABLE attribute_staging.%I DROP COLUMN "%s" CASCADE', attribute_directory.to_char($1), array_to_string($2, '" CASCADE, DROP COLUMN "'));
        EXECUTE format('SELECT attribute_directory.add_staging_dependees(%s)', $1);
    RETURN $1;
    END;
$$ LANGUAGE plpgsql VOLATILE;

CREATE FUNCTION attribute_directory.drop_attributes(attribute_directory.attribute_store, name[]) RETURNS attribute_directory.attribute_store
AS $$
    DELETE FROM attribute_directory.attribute
        WHERE attribute_store_id = $1.id AND name = ANY($2);
    SELECT attribute_directory.remove_attribute_columns($1, $2);
$$ LANGUAGE sql VOLATILE;
