# Multi-Stage Relation Materialization

In some situations, the logic for generating relations is too complex to be performed in one query. A solution is then to cut the problem in stages, generate intermediate results and store them in one or more tables. By also keeping the stages in separate transactions, you can prevent excessive locking.

Currently, a relation materialization is always defined by one backing view that is then queried and its results stored in the relation table. For the multi-stage relation materialization solution, we also allow a relation to be populated by a stored procedure.

So with this feature, the relation is defined by:

1. A view `relation_def.<RELATION_TYPE_NAME>`

Or:

2. A stored procedure `relation_def.<RELATION_TYPE_NAME>()` with no arguments

The `minerva relation materialize` command will first look for a view, and if it does not exist, it will look for a stored procedure. If there is only a stored procedure, it will be executed and expected to populate the relation table `relation.<RELATION_TYPE_NAME>`. Otherwise, when there is a view, it will be used as before.


Example relation definition with a stored procedure:

```
CREATE OR REPLACE PROCEDURE relation.test() language plpgsql AS $$
BEGIN
    CREATE TABLE IF NOT EXISTS relation_def.test_1(
        source_id integer,
        target_id integer
    );

    TRUNCATE relation_def.test_1;

    INSERT INTO relation_def.test_1(source_id, target_id)
    SELECT a.id, b.id
    FROM attribute.a
    JOIN attribute.b on a.cell_id = b.cell_id
        AND b.ac_type = 'blue';

    COMMIT;

    INSERT INTO relation.test(source_id, target_id)
    SELECT source_id, target_id
    FROM relation_def.test_1;

    DROP TABLE relation_def.test_1;

    COMMIT;
END; $$;
```

More intermediate stages can be added if needed.
