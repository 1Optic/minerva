CREATE TABLE "trend_directory"."table_trend_statistics"
(
   table_trend_id integer NOT NULL,
   min numeric DEFAULT NULL,
   max numeric DEFAULT NULL
);

ALTER TABLE "trend_directory"."table_trend_statistics"
  ADD CONSTRAINT "table_trend_statistics_table_trend_id_fkey"
  FOREIGN KEY (table_trend_id)
  REFERENCES "trend_directory"."table_trend" (id) ON DELETE CASCADE;

SELECT citus_add_local_table_to_metadata($$trend_directory.table_trend_statistics$$, cascade_via_foreign_keys=>true);

INSERT INTO trend_directory.table_trend_statistics (table_trend_id, min, max)
  SELECT id, NULL, NULL
  FROM trend_directory.table_trend;

CREATE OR REPLACE FUNCTION trend_directory.get_lowest_trend_value(trend_store_part text, trend text)
  RETURNS numeric
  LANGUAGE plpgsql STABLE
AS $function$
DECLARE
  result numeric;
BEGIN
  EXECUTE FORMAT(
    'WITH x AS ('
      'SELECT unnest(stavalues1::text::numeric[] || stavalues2::text::numeric[] || stavalues3::text::numeric[] || stavalues4::text::numeric[] || stavalues5::text::numeric[]) AS value '
      'FROM pg_statistic s '
      'JOIN pg_class c ON s.starelid = c.oid '
      'JOIN pg_namespace ns ON c.relnamespace = ns.oid '
      'JOIN trend_directory.partition p ON c.relname LIKE p.name || ''%%'' '
      'JOIN trend_directory.trend_store_part tsp ON p.trend_store_part_id = tsp.id '
      'JOIN pg_attribute a ON a.attrelid = s.starelid and a.attnum = s.staattnum '
      'WHERE ns.nspname = ''trend_partition'' AND tsp.name = ''%s'' AND a.attname = ''%s'')'
    'SELECT FLOOR(MIN(VALUE)) FROM x',
    $1, $2
  ) INTO result;
  RETURN result;
END;
$function$;

CREATE OR REPLACE FUNCTION trend_directory.get_highest_trend_value(trend_store_part text, trend text)
  RETURNS numeric
  LANGUAGE plpgsql STABLE
AS $function$
DECLARE
  result numeric;
BEGIN
  EXECUTE FORMAT(
    'WITH x AS ('
      'SELECT unnest(stavalues1::text::numeric[] || stavalues2::text::numeric[] || stavalues3::text::numeric[] || stavalues4::text::numeric[] || stavalues5::text::numeric[]) AS value '
      'FROM pg_statistic s '
      'JOIN pg_class c ON s.starelid = c.oid '
      'JOIN pg_namespace ns ON c.relnamespace = ns.oid '
      'JOIN trend_directory.partition p ON c.relname LIKE p.name || ''%%'' '
      'JOIN trend_directory.trend_store_part tsp ON p.trend_store_part_id = tsp.id '
      'JOIN pg_attribute a ON a.attrelid = s.starelid and a.attnum = s.staattnum '
      'WHERE ns.nspname = ''trend_partition'' AND tsp.name = ''%s'' AND a.attname = ''%s'')'
   'SELECT CEIL(MAX(VALUE)) FROM x',
    $1, $2
  ) INTO result;
  RETURN result;
END;
$function$;

CREATE OR REPLACE FUNCTION trend_directory.update_statistics(trend_directory.table_trend)
  RETURNS record
  LANGUAGE plpgsql VOLATILE
AS $function$
  DECLARE
    tspname text;
    min numeric;
    max numeric;
    fresult record;
  BEGIN
    SELECT tsp.name FROM trend_directory.trend_store_part tsp JOIN trend_directory.table_trend tt ON tt.trend_store_part_id = tsp.id WHERE tt.id = $1.id INTO tspname;
    IF $1.data_type <> 'boolean' THEN
      IF trend_directory.greatest_data_type($1.data_type, 'numeric') = 'numeric' THEN
        EXECUTE FORMAT(
          'SELECT min(nullif(result, '''')) FROM run_command_on_shards('
          '''trend."%s"'', '
          '$cmd$ SELECT trend_directory.get_lowest_trend_value(''%s'', ''%s'') $cmd$)',
          tspname, tspname, $1.name) INTO min;
        EXECUTE FORMAT(
          'SELECT max(nullif(result, '''')) FROM run_command_on_shards('
          '''trend."%s"'', '
          '$cmd$ SELECT trend_directory.get_highest_trend_value(''%s'', ''%s'') $cmd$)',
          tspname, tspname, $1.name) INTO max;
        IF min IS NOT NULL THEN
          IF min >=0 AND max <= 1 THEN
            min = 0;
            max = 1;
          ELSE
            IF MIN <=0 THEN
              min = FLOOR(1.1 * min);
	    ELSE
              min = FLOOR(0.9 * min);
	    END IF;
	    IF MAX <=0 THEN
	      max = CEIL (0.9 * max);
	    ELSE
	      max = CEIL (1.1 * max);
	    END IF;
          END IF;
          IF min > 0 AND min*4 < max THEN min = 0; END IF;
          IF max < 0 AND max*4 > min THEN max = 0; END IF;
          EXECUTE FORMAT('UPDATE trend_directory.table_trend_statistics SET min = %s, max = %s WHERE table_trend_id = %s', min, max, $1.id);
        END IF;
        SELECT $1.name, min, max INTO fresult;
      END IF;
    END IF;
    RETURN fresult;
  END;
$function$;

CREATE FUNCTION trend_directory.create_statistics_for_table_trend()
  RETURNS TRIGGER
AS $$
BEGIN
  INSERT INTO trend_directory.table_trend_statistics(table_trend_id, min, max)
  VALUES (NEW.id, NULL, NULL);
  RETURN NEW;
END;
$$ LANGUAGE plpgsql VOLATILE;

CREATE TRIGGER create_statistics_on_new_table_trend
  AFTER INSERT ON trend_directory.table_trend
  FOR EACH ROW
  EXECUTE PROCEDURE trend_directory.create_statistics_for_table_trend();
