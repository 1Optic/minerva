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
database by the Minerva CLI interface. The exact SQL commands issued and a
timestamp of when they are applied need to be logged.

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
