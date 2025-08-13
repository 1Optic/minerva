
CREATE TRIGGER create_statistics_on_new_table_trend
  AFTER INSERT ON trend_directory.table_trend
  FOR EACH ROW
  EXECUTE PROCEDURE trend_directory.create_statistics_for_table_trend();

INSERT INTO trend_directory.table_trend_statistics
  SELECT id AS table_trend_id, NULL AS min, NULL AS max
  FROM trend_directory.table_trend;
