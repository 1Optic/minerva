CREATE VIEW attribute_directory.attribute_context AS
SELECT
    a.id          AS attribute_id,
    a.name        AS name,
    ast.id        AS attribute_store_id,
    ds.name       AS data_source,
    et.name       AS entity_type,
    a.data_type   AS data_type,
    jsonb_build_object(
        'data_source',       ds.name,
        'entity_type',       et.name,
        'attribute_store_id', ast.id,
        'data_type',         a.data_type,
        'description',       a.description,
        'extra_data',        a.extra_data
    )             AS context
FROM attribute_directory.attribute a
JOIN attribute_directory.attribute_store ast ON ast.id = a.attribute_store_id
LEFT JOIN directory.data_source ds           ON ds.id  = ast.data_source_id
LEFT JOIN directory.entity_type et           ON et.id  = ast.entity_type_id;


CREATE VIEW trend_directory.trend_context AS
SELECT
    tt.id                         AS trend_id,
    tt.name                       AS name,
    tsp.id                        AS trend_store_part_id,
    tsp.name                      AS trend_store_part_name,
    ts.id                         AS trend_store_id,
    ds.name                       AS data_source,
    et.name                       AS entity_type,
    ts.granularity                AS granularity,
    ts.partition_size             AS partition_size,
    ts.retention_period           AS retention_period,
    tt.data_type                  AS data_type,
    jsonb_build_object(
        'data_source',        ds.name,
        'entity_type',        et.name,
        'granularity',        ts.granularity::text,
        'partition_size',     ts.partition_size::text,
        'retention_period',   ts.retention_period::text,
        'trend_store_id',     ts.id,
        'trend_store_part',   tsp.name,
        'data_type',          tt.data_type,
        'time_aggregation',   tt.time_aggregation,
        'entity_aggregation', tt.entity_aggregation,
        'description',        tt.description,
        'extra_data',         tt.extra_data
    )                             AS context
FROM trend_directory.table_trend tt
JOIN trend_directory.trend_store_part tsp ON tsp.id = tt.trend_store_part_id
JOIN trend_directory.trend_store ts        ON ts.id  = tsp.trend_store_id
LEFT JOIN directory.data_source ds         ON ds.id  = ts.data_source_id
LEFT JOIN directory.entity_type et         ON et.id  = ts.entity_type_id;

COMMENT ON VIEW trend_directory.trend_context IS
'Convenience view exposing full context (store, part, granularity, '
'partition size, aggregation) per trend as a flat row plus a '
'jsonb blob.';