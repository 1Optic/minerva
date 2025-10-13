CREATE FUNCTION "entity"."get_alias_if_any"(directory.entity_type, TEXT)
    RETURNS TEXT
AS $$
BEGIN
    IF $1.primary_alias IS NULL THEN
        RETURN NULL;
    ELSE
        RETURN FORMAT(
            'SELECT primary_alias FROM entity.%I WHERE name = %L',
            $1.name, $2
        );
    END IF;
END;
$$ LANGUAGE plpgsql STABLE;

CREATE TYPE "entity"."generic_entity" AS (
    id TEXT,
    "name" TEXT,
    alias TEXT
);

CREATE FUNCTION "entity"."get_entity"(directory.entity_type, TEXT)
    RETURNS entity.generic_entity
AS $$
DECLARE
    result entity.generic_entity;
BEGIN
    IF $1.primary_alias IS NULL THEN
        EXECUTE FORMAT(
            'SELECT id, name, NULL AS alias
            FROM entity."to_%s"(%L)',
        $1.name, $2) INTO result;
    ELSE
        EXECUTE FORMAT(
            'SELECT id, name, primary_alias AS alias
            FROM entity."to_%s"(%L)',
        $1.name, $2) INTO result;
    END IF;
    RETURN result;
END;
$$ LANGUAGE plpgsql VOLATILE;

CREATE FUNCTION "entity"."get_existing_entities"(directory.entity_type, TEXT[])
    RETURNS TABLE(id integer, "name" TEXT, alias TEXT)
AS $$
BEGIN
    IF $1.primary_alias IS NULL THEN
        RETURN QUERY EXECUTE FORMAT(
            'WITH data AS (SELECT e.id, n.name::text, NULL AS alias
            FROM UNNEST(%L::TEXT[]) n JOIN entity.%I e ON n.name = e.name
            )
            SELECT * FROM data',
        $2, $1.name);
    ELSE
        RETURN QUERY EXECUTE FORMAT(
            'WITH data AS (SELECT e.id, n.name::text, e.primary_alias AS alias
            FROM UNNEST(%L::TEXT[]) n JOIN entity.%I e ON n.name = e.name
            )
            SELECT * FROM data',
        $2, $1.name);
    END IF;
END;
$$ LANGUAGE plpgsql STABLE;
