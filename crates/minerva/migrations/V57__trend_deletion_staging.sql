ALTER TABLE trend_directory.table_trend ADD COLUMN deleted timestamp with time zone DEFAULT NULL;
ALTER TABLE trend_directory.table_trend ADD COLUMN staged_for_deletion timestamp with time zone DEFAULT NULL;
ALTER TABLE trend_directory.table_trend ADD COLUMN deletion_staging_column text DEFAULT NULL;
