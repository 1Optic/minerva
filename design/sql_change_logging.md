# SQL Change Logging

Databases managed by the Minerva toolset can be changed relatively often during
their lifetime. These changes can have a big impact on the functioning of the
database and mistakes can be made during maintenance. To be able to
trouble-shoot issues when they arise, you need to be able to see what changes
were made and when. For this purpose, the administration tooling needs to log
these changes in detail. The 'SQL Change Logging' feature provides these
details specifically for those situations where an engineer needs to be able to
see precisely what happened and when.

## Design

The change logging will be done for every change performed on a Minerva
database by the Minerva CLI interface.

### What

The following shall be logged:

1. The original Minerva administrative command (with arguments) as issued from
   the command line (or a script).
2. The exact SQL commands issued to the database after they succeed (commit).
3. The timestamp of when they are applied (commit).
4. The username of the current user.

### How

The logging shall be done in files in a configurable directory. Each
administrative command shall create 1 file with the timestamp of the start of
the command in the file name. The timestamp uses the following [ISO
8601](https://en.wikipedia.org/wiki/ISO_8601) format:

```
20241017T193114Z
```

The resulting file name will be:

```
minerva_change_20241017T193114Z.json
```

The format of the records in the log will be:

```
{
    "command": ["minerva", "update", "/usr/share/minerva_4g"],
    "user": "Steven Ops",
    "actions": [
        {
            "timestamp": "2024-10-17T19:31:14Z",
            "sql": "INSERT INTO directory.entity_type(name, description) VALUES ('v-cell', 'Vendor agnostic cell representation')"
        },
        {
            "timestamp": "2024-10-17T19:31:14Z",
            "sql": "INSERT INTO directory.entity_type(name, description) VALUES ('v-site', 'Vendor agnostic site representation')"
        },
    ]
}
```

## Follow-up Changes

Currently, a lot of change actions on Minerva databases are implemented as SQL
functions, which makes it much harder to implement comprehensive logging of the
changes. E.g. altering an attribute store by adding an attribute currently
results in one call to the attribute_directory.create_attribute SQL function,
but that function:

1. Creates a record in the attribute_directory.attribute table
2. Creates a new column in the attribute store base table
3. Drops the hash function for the attribute store
4. Adds a new column in the attribute store history tabe
5. Creates the hash function for the attribute store
6. Adds a new column in the attribute store compacted temp table
7. Drops the dependees of the staging table, meaning:
   a. Drops the staging modified view
   b. Drops the staging new view
8. Adds a new column in the attribute store staging table
9. Creates the dependees of the staging table
   a. Creates the staging modified view
   b. Creates the staging new view

This logic can better be placed in the administrative tooling so that each step
can be logged in a similar manner.

## Rationale

Here are descriptions of choices that are made with alternatives and reasoning.

### Standard PostgreSQL Statement Logging

The use of standard PostgreSQL statement logging was considered as an
alternative to logging of statements in the Minerva administrative tool. This
is an option and can be configured per user. We choose to still log using the
Minerva administrative tool because it allows us to provide more context and
have the statements grouped together more logically.
