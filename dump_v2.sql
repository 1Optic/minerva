--
-- PostgreSQL database dump
--

-- Dumped from database version 15.3 (Debian 15.3-1.pgdg120+1)
-- Dumped by pg_dump version 16.4 (Ubuntu 16.4-0ubuntu0.24.04.2)

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

--
-- Name: alias; Type: SCHEMA; Schema: -; Owner: postgres
--

CREATE SCHEMA alias;


ALTER SCHEMA alias OWNER TO postgres;

--
-- Name: alias_def; Type: SCHEMA; Schema: -; Owner: postgres
--

CREATE SCHEMA alias_def;


ALTER SCHEMA alias_def OWNER TO postgres;

--
-- Name: alias_directory; Type: SCHEMA; Schema: -; Owner: postgres
--

CREATE SCHEMA alias_directory;


ALTER SCHEMA alias_directory OWNER TO postgres;

--
-- Name: attribute; Type: SCHEMA; Schema: -; Owner: postgres
--

CREATE SCHEMA attribute;


ALTER SCHEMA attribute OWNER TO postgres;

--
-- Name: attribute_base; Type: SCHEMA; Schema: -; Owner: postgres
--

CREATE SCHEMA attribute_base;


ALTER SCHEMA attribute_base OWNER TO postgres;

--
-- Name: attribute_directory; Type: SCHEMA; Schema: -; Owner: postgres
--

CREATE SCHEMA attribute_directory;


ALTER SCHEMA attribute_directory OWNER TO postgres;

--
-- Name: attribute_history; Type: SCHEMA; Schema: -; Owner: postgres
--

CREATE SCHEMA attribute_history;


ALTER SCHEMA attribute_history OWNER TO postgres;

--
-- Name: attribute_staging; Type: SCHEMA; Schema: -; Owner: postgres
--

CREATE SCHEMA attribute_staging;


ALTER SCHEMA attribute_staging OWNER TO postgres;

--
-- Name: citus; Type: EXTENSION; Schema: -; Owner: -
--

CREATE EXTENSION IF NOT EXISTS citus WITH SCHEMA pg_catalog;


--
-- Name: EXTENSION citus; Type: COMMENT; Schema: -; Owner: 
--

COMMENT ON EXTENSION citus IS 'Citus distributed database';


--
-- Name: citus_columnar; Type: EXTENSION; Schema: -; Owner: -
--

CREATE EXTENSION IF NOT EXISTS citus_columnar WITH SCHEMA pg_catalog;


--
-- Name: EXTENSION citus_columnar; Type: COMMENT; Schema: -; Owner: 
--

COMMENT ON EXTENSION citus_columnar IS 'Citus Columnar extension';


--
-- Name: directory; Type: SCHEMA; Schema: -; Owner: postgres
--

CREATE SCHEMA directory;


ALTER SCHEMA directory OWNER TO postgres;

--
-- Name: SCHEMA directory; Type: COMMENT; Schema: -; Owner: postgres
--

COMMENT ON SCHEMA directory IS 'Stores contextual information for the data. This includes the entities, entity_types, data_sources, etc. It is the entrypoint when looking for data.';


--
-- Name: entity; Type: SCHEMA; Schema: -; Owner: postgres
--

CREATE SCHEMA entity;


ALTER SCHEMA entity OWNER TO postgres;

--
-- Name: logging; Type: SCHEMA; Schema: -; Owner: postgres
--

CREATE SCHEMA logging;


ALTER SCHEMA logging OWNER TO postgres;

--
-- Name: metric; Type: SCHEMA; Schema: -; Owner: postgres
--

CREATE SCHEMA metric;


ALTER SCHEMA metric OWNER TO postgres;

--
-- Name: notification; Type: SCHEMA; Schema: -; Owner: postgres
--

CREATE SCHEMA notification;


ALTER SCHEMA notification OWNER TO postgres;

--
-- Name: SCHEMA notification; Type: COMMENT; Schema: -; Owner: postgres
--

COMMENT ON SCHEMA notification IS 'Stores information of events that can occur at irregular intervals,
but still have a fixed, known format. This schema is dynamically populated.';


--
-- Name: notification_directory; Type: SCHEMA; Schema: -; Owner: postgres
--

CREATE SCHEMA notification_directory;


ALTER SCHEMA notification_directory OWNER TO postgres;

--
-- Name: SCHEMA notification_directory; Type: COMMENT; Schema: -; Owner: postgres
--

COMMENT ON SCHEMA notification_directory IS 'Stores meta-data about notification data in the notification schema.';


--
-- Name: olap; Type: SCHEMA; Schema: -; Owner: postgres
--

CREATE SCHEMA olap;


ALTER SCHEMA olap OWNER TO postgres;

--
-- Name: relation; Type: SCHEMA; Schema: -; Owner: postgres
--

CREATE SCHEMA relation;


ALTER SCHEMA relation OWNER TO postgres;

--
-- Name: SCHEMA relation; Type: COMMENT; Schema: -; Owner: postgres
--

COMMENT ON SCHEMA relation IS 'Stores the actual relations between entities in tables.
';


--
-- Name: relation_def; Type: SCHEMA; Schema: -; Owner: postgres
--

CREATE SCHEMA relation_def;


ALTER SCHEMA relation_def OWNER TO postgres;

--
-- Name: SCHEMA relation_def; Type: COMMENT; Schema: -; Owner: postgres
--

COMMENT ON SCHEMA relation_def IS 'Stores the views that define the contents of the relation tables.
';


--
-- Name: relation_directory; Type: SCHEMA; Schema: -; Owner: postgres
--

CREATE SCHEMA relation_directory;


ALTER SCHEMA relation_directory OWNER TO postgres;

--
-- Name: system; Type: SCHEMA; Schema: -; Owner: postgres
--

CREATE SCHEMA system;


ALTER SCHEMA system OWNER TO postgres;

--
-- Name: trend; Type: SCHEMA; Schema: -; Owner: postgres
--

CREATE SCHEMA trend;


ALTER SCHEMA trend OWNER TO postgres;

--
-- Name: SCHEMA trend; Type: COMMENT; Schema: -; Owner: postgres
--

COMMENT ON SCHEMA trend IS 'Stores information with fixed interval and format, like periodic measurements.';


--
-- Name: trend_directory; Type: SCHEMA; Schema: -; Owner: postgres
--

CREATE SCHEMA trend_directory;


ALTER SCHEMA trend_directory OWNER TO postgres;

--
-- Name: trend_partition; Type: SCHEMA; Schema: -; Owner: postgres
--

CREATE SCHEMA trend_partition;


ALTER SCHEMA trend_partition OWNER TO postgres;

--
-- Name: SCHEMA trend_partition; Type: COMMENT; Schema: -; Owner: postgres
--

COMMENT ON SCHEMA trend_partition IS 'Holds partitions of the trend store tables in the trend schema.';


--
-- Name: trigger; Type: SCHEMA; Schema: -; Owner: postgres
--

CREATE SCHEMA trigger;


ALTER SCHEMA trigger OWNER TO postgres;

--
-- Name: trigger_rule; Type: SCHEMA; Schema: -; Owner: postgres
--

CREATE SCHEMA trigger_rule;


ALTER SCHEMA trigger_rule OWNER TO postgres;

--
-- Name: virtual_entity; Type: SCHEMA; Schema: -; Owner: postgres
--

CREATE SCHEMA virtual_entity;


ALTER SCHEMA virtual_entity OWNER TO postgres;

--
-- Name: attribute_descr; Type: TYPE; Schema: attribute_directory; Owner: postgres
--

CREATE TYPE attribute_directory.attribute_descr AS (
	name name,
	data_type text,
	description text
);


ALTER TYPE attribute_directory.attribute_descr OWNER TO postgres;

--
-- Name: attribute_info; Type: TYPE; Schema: attribute_directory; Owner: postgres
--

CREATE TYPE attribute_directory.attribute_info AS (
	name name,
	data_type character varying
);


ALTER TYPE attribute_directory.attribute_info OWNER TO postgres;

--
-- Name: generic_notification; Type: TYPE; Schema: notification; Owner: postgres
--

CREATE TYPE notification.generic_notification AS (
	id integer,
	"timestamp" timestamp with time zone,
	rule text,
	entity text,
	weight integer,
	details text,
	data json
);


ALTER TYPE notification.generic_notification OWNER TO postgres;

--
-- Name: attr_def; Type: TYPE; Schema: notification_directory; Owner: postgres
--

CREATE TYPE notification_directory.attr_def AS (
	name name,
	data_type name,
	description text
);


ALTER TYPE notification_directory.attr_def OWNER TO postgres;

--
-- Name: type_cardinality_enum; Type: TYPE; Schema: relation_directory; Owner: postgres
--

CREATE TYPE relation_directory.type_cardinality_enum AS ENUM (
    'one-to-one',
    'one-to-many',
    'many-to-one'
);


ALTER TYPE relation_directory.type_cardinality_enum OWNER TO postgres;

--
-- Name: version_tuple; Type: TYPE; Schema: system; Owner: postgres
--

CREATE TYPE system.version_tuple AS (
	major smallint,
	minor smallint,
	patch smallint
);


ALTER TYPE system.version_tuple OWNER TO postgres;

--
-- Name: trend_data; Type: TYPE; Schema: trend; Owner: postgres
--

CREATE TYPE trend.trend_data AS (
	"timestamp" timestamp with time zone,
	entity_id integer,
	counters numeric[]
);


ALTER TYPE trend.trend_data OWNER TO postgres;

--
-- Name: change_trend_store_part_result; Type: TYPE; Schema: trend_directory; Owner: postgres
--

CREATE TYPE trend_directory.change_trend_store_part_result AS (
	added_trends text[],
	removed_trends text[],
	changed_trends text[]
);


ALTER TYPE trend_directory.change_trend_store_part_result OWNER TO postgres;

--
-- Name: column_info; Type: TYPE; Schema: trend_directory; Owner: postgres
--

CREATE TYPE trend_directory.column_info AS (
	name name,
	data_type text
);


ALTER TYPE trend_directory.column_info OWNER TO postgres;

--
-- Name: fingerprint; Type: TYPE; Schema: trend_directory; Owner: postgres
--

CREATE TYPE trend_directory.fingerprint AS (
	modified timestamp with time zone,
	body jsonb
);


ALTER TYPE trend_directory.fingerprint OWNER TO postgres;

--
-- Name: generated_trend_descr; Type: TYPE; Schema: trend_directory; Owner: postgres
--

CREATE TYPE trend_directory.generated_trend_descr AS (
	name name,
	data_type text,
	description text,
	expression text,
	extra_data jsonb
);


ALTER TYPE trend_directory.generated_trend_descr OWNER TO postgres;

--
-- Name: transfer_result; Type: TYPE; Schema: trend_directory; Owner: postgres
--

CREATE TYPE trend_directory.transfer_result AS (
	row_count integer,
	max_modified timestamp with time zone
);


ALTER TYPE trend_directory.transfer_result OWNER TO postgres;

--
-- Name: trend_descr; Type: TYPE; Schema: trend_directory; Owner: postgres
--

CREATE TYPE trend_directory.trend_descr AS (
	name name,
	data_type text,
	description text,
	time_aggregation text,
	entity_aggregation text,
	extra_data jsonb
);


ALTER TYPE trend_directory.trend_descr OWNER TO postgres;

--
-- Name: trend_store_part_descr; Type: TYPE; Schema: trend_directory; Owner: postgres
--

CREATE TYPE trend_directory.trend_store_part_descr AS (
	name name,
	trends trend_directory.trend_descr[],
	generated_trends trend_directory.generated_trend_descr[]
);


ALTER TYPE trend_directory.trend_store_part_descr OWNER TO postgres;

--
-- Name: trend_view_part_descr; Type: TYPE; Schema: trend_directory; Owner: postgres
--

CREATE TYPE trend_directory.trend_view_part_descr AS (
	name name,
	query text
);


ALTER TYPE trend_directory.trend_view_part_descr OWNER TO postgres;

--
-- Name: upgrade_record; Type: TYPE; Schema: trend_directory; Owner: postgres
--

CREATE TYPE trend_directory.upgrade_record AS (
	"timestamp" timestamp with time zone,
	number_of_rows integer
);


ALTER TYPE trend_directory.upgrade_record OWNER TO postgres;

--
-- Name: kpi_def; Type: TYPE; Schema: trigger; Owner: postgres
--

CREATE TYPE trigger.kpi_def AS (
	name name,
	data_type name
);


ALTER TYPE trigger.kpi_def OWNER TO postgres;

--
-- Name: notification; Type: TYPE; Schema: trigger; Owner: postgres
--

CREATE TYPE trigger.notification AS (
	entity_id integer,
	"timestamp" timestamp with time zone,
	weight integer,
	details text,
	data json
);


ALTER TYPE trigger.notification OWNER TO postgres;

--
-- Name: threshold_def; Type: TYPE; Schema: trigger; Owner: postgres
--

CREATE TYPE trigger.threshold_def AS (
	name name,
	data_type name
);


ALTER TYPE trigger.threshold_def OWNER TO postgres;

--
-- Name: node/15m/highpowerusage_details; Type: TYPE; Schema: trigger_rule; Owner: postgres
--

CREATE TYPE trigger_rule."node/15m/highpowerusage_details" AS (
	entity_id integer,
	"timestamp" timestamp with time zone,
	power_kwh numeric,
	max_power numeric
);


ALTER TYPE trigger_rule."node/15m/highpowerusage_details" OWNER TO postgres;

--
-- Name: node/15m/highpowerusage_kpi; Type: TYPE; Schema: trigger_rule; Owner: postgres
--

CREATE TYPE trigger_rule."node/15m/highpowerusage_kpi" AS (
	entity_id integer,
	"timestamp" timestamp with time zone,
	power_kwh numeric
);


ALTER TYPE trigger_rule."node/15m/highpowerusage_kpi" OWNER TO postgres;

--
-- Name: node/1d/highpowerusage_details; Type: TYPE; Schema: trigger_rule; Owner: postgres
--

CREATE TYPE trigger_rule."node/1d/highpowerusage_details" AS (
	entity_id integer,
	"timestamp" timestamp with time zone,
	power_kwh numeric,
	max_power numeric
);


ALTER TYPE trigger_rule."node/1d/highpowerusage_details" OWNER TO postgres;

--
-- Name: node/1d/highpowerusage_kpi; Type: TYPE; Schema: trigger_rule; Owner: postgres
--

CREATE TYPE trigger_rule."node/1d/highpowerusage_kpi" AS (
	entity_id integer,
	"timestamp" timestamp with time zone,
	power_kwh numeric
);


ALTER TYPE trigger_rule."node/1d/highpowerusage_kpi" OWNER TO postgres;

--
-- Name: node/1h/highpowerusage_details; Type: TYPE; Schema: trigger_rule; Owner: postgres
--

CREATE TYPE trigger_rule."node/1h/highpowerusage_details" AS (
	entity_id integer,
	"timestamp" timestamp with time zone,
	power_kwh numeric,
	max_power numeric
);


ALTER TYPE trigger_rule."node/1h/highpowerusage_details" OWNER TO postgres;

--
-- Name: node/1h/highpowerusage_kpi; Type: TYPE; Schema: trigger_rule; Owner: postgres
--

CREATE TYPE trigger_rule."node/1h/highpowerusage_kpi" AS (
	entity_id integer,
	"timestamp" timestamp with time zone,
	power_kwh numeric
);


ALTER TYPE trigger_rule."node/1h/highpowerusage_kpi" OWNER TO postgres;

--
-- Name: node/1w/highpowerusage_details; Type: TYPE; Schema: trigger_rule; Owner: postgres
--

CREATE TYPE trigger_rule."node/1w/highpowerusage_details" AS (
	entity_id integer,
	"timestamp" timestamp with time zone,
	power_kwh numeric,
	max_power numeric
);


ALTER TYPE trigger_rule."node/1w/highpowerusage_details" OWNER TO postgres;

--
-- Name: node/1w/highpowerusage_kpi; Type: TYPE; Schema: trigger_rule; Owner: postgres
--

CREATE TYPE trigger_rule."node/1w/highpowerusage_kpi" AS (
	entity_id integer,
	"timestamp" timestamp with time zone,
	power_kwh numeric
);


ALTER TYPE trigger_rule."node/1w/highpowerusage_kpi" OWNER TO postgres;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: attribute_store; Type: TABLE; Schema: attribute_directory; Owner: postgres
--

CREATE TABLE attribute_directory.attribute_store (
    id integer NOT NULL,
    data_source_id integer NOT NULL,
    entity_type_id integer NOT NULL
);


ALTER TABLE attribute_directory.attribute_store OWNER TO postgres;

--
-- Name: to_char(attribute_directory.attribute_store); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.to_char(attribute_directory.attribute_store) RETURNS text
    LANGUAGE sql STABLE STRICT
    AS $_$
SELECT data_source.name || '_' || entity_type.name
  FROM directory.data_source, directory.entity_type
  WHERE data_source.id = $1.data_source_id AND entity_type.id = $1.entity_type_id;
$_$;


ALTER FUNCTION attribute_directory.to_char(attribute_directory.attribute_store) OWNER TO postgres;

--
-- Name: CAST (attribute_directory.attribute_store AS text); Type: CAST; Schema: -; Owner: -
--

CREATE CAST (attribute_directory.attribute_store AS text) WITH FUNCTION attribute_directory.to_char(attribute_directory.attribute_store);


--
-- Name: materialization; Type: TABLE; Schema: trend_directory; Owner: postgres
--

CREATE TABLE trend_directory.materialization (
    id integer NOT NULL,
    dst_trend_store_part_id integer NOT NULL,
    processing_delay interval NOT NULL,
    stability_delay interval NOT NULL,
    reprocessing_period interval NOT NULL,
    enabled boolean DEFAULT false NOT NULL,
    description jsonb DEFAULT '{}'::jsonb NOT NULL
);


ALTER TABLE trend_directory.materialization OWNER TO postgres;

--
-- Name: TABLE materialization; Type: COMMENT; Schema: trend_directory; Owner: postgres
--

COMMENT ON TABLE trend_directory.materialization IS 'A ``materialization`` is a materialization that uses the data from
the view registered in the ``src_view`` column to populate the target trend
store.';


--
-- Name: COLUMN materialization.id; Type: COMMENT; Schema: trend_directory; Owner: postgres
--

COMMENT ON COLUMN trend_directory.materialization.id IS 'The unique identifier of this materialization';


--
-- Name: COLUMN materialization.dst_trend_store_part_id; Type: COMMENT; Schema: trend_directory; Owner: postgres
--

COMMENT ON COLUMN trend_directory.materialization.dst_trend_store_part_id IS 'The ID of the destination trend_store_part';


--
-- Name: COLUMN materialization.processing_delay; Type: COMMENT; Schema: trend_directory; Owner: postgres
--

COMMENT ON COLUMN trend_directory.materialization.processing_delay IS 'The time after the destination timestamp before this materialization can be executed
';


--
-- Name: COLUMN materialization.stability_delay; Type: COMMENT; Schema: trend_directory; Owner: postgres
--

COMMENT ON COLUMN trend_directory.materialization.stability_delay IS 'The time to wait after the most recent modified timestamp before the source data is considered ''stable''
';


--
-- Name: COLUMN materialization.reprocessing_period; Type: COMMENT; Schema: trend_directory; Owner: postgres
--

COMMENT ON COLUMN trend_directory.materialization.reprocessing_period IS 'The maximum time after the destination timestamp that the materialization is allowed to be executed
';


--
-- Name: COLUMN materialization.enabled; Type: COMMENT; Schema: trend_directory; Owner: postgres
--

COMMENT ON COLUMN trend_directory.materialization.enabled IS 'Indicates if jobs should be created for this materialization (manual execution is always possible)
';


--
-- Name: COLUMN materialization.description; Type: COMMENT; Schema: trend_directory; Owner: postgres
--

COMMENT ON COLUMN trend_directory.materialization.description IS 'Gives a description of the function used for the materialization in json format
';


--
-- Name: to_char(trend_directory.materialization); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.to_char(trend_directory.materialization) RETURNS text
    LANGUAGE sql STABLE STRICT
    AS $_$
SELECT trend_store_part.name::text
FROM trend_directory.trend_store_part
WHERE trend_store_part.id = $1.dst_trend_store_part_id
$_$;


ALTER FUNCTION trend_directory.to_char(trend_directory.materialization) OWNER TO postgres;

--
-- Name: CAST (trend_directory.materialization AS text); Type: CAST; Schema: -; Owner: -
--

CREATE CAST (trend_directory.materialization AS text) WITH FUNCTION trend_directory.to_char(trend_directory.materialization);


--
-- Name: notification_store; Type: TABLE; Schema: notification_directory; Owner: postgres
--

CREATE TABLE notification_directory.notification_store (
    id integer NOT NULL,
    data_source_id integer,
    entity_type_id integer
);


ALTER TABLE notification_directory.notification_store OWNER TO postgres;

--
-- Name: TABLE notification_store; Type: COMMENT; Schema: notification_directory; Owner: postgres
--

COMMENT ON TABLE notification_directory.notification_store IS 'Describes notification_stores. Each notification_store maps to a set of
tables and functions that can store and manage notifications of a certain
type. These corresponding tables and functions are created automatically
for each notification_store. Because each notification_store maps
one-on-one to a data_source, the name of the notification_store is the
same as that of the data_source. Use the create_notification_store
function to create new notification_stores.';


--
-- Name: to_char(notification_directory.notification_store); Type: FUNCTION; Schema: notification_directory; Owner: postgres
--

CREATE FUNCTION notification_directory.to_char(notification_directory.notification_store) RETURNS text
    LANGUAGE sql STABLE STRICT
    AS $_$
SELECT data_source.name
FROM directory.data_source
WHERE data_source.id = $1.data_source_id;
$_$;


ALTER FUNCTION notification_directory.to_char(notification_directory.notification_store) OWNER TO postgres;

--
-- Name: CAST (notification_directory.notification_store AS text); Type: CAST; Schema: -; Owner: -
--

CREATE CAST (notification_directory.notification_store AS text) WITH FUNCTION notification_directory.to_char(notification_directory.notification_store);


--
-- Name: trend_store_part; Type: TABLE; Schema: trend_directory; Owner: postgres
--

CREATE TABLE trend_directory.trend_store_part (
    id integer NOT NULL,
    name name NOT NULL,
    trend_store_id integer NOT NULL
);


ALTER TABLE trend_directory.trend_store_part OWNER TO postgres;

--
-- Name: TABLE trend_store_part; Type: COMMENT; Schema: trend_directory; Owner: postgres
--

COMMENT ON TABLE trend_directory.trend_store_part IS 'The parts of a horizontally partitioned table trend store. Each table trend store has at least 1 part.';


--
-- Name: base_table_name(trend_directory.trend_store_part); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.base_table_name(trend_directory.trend_store_part) RETURNS name
    LANGUAGE sql IMMUTABLE STRICT
    AS $_$
SELECT $1.name;
$_$;


ALTER FUNCTION trend_directory.base_table_name(trend_directory.trend_store_part) OWNER TO postgres;

--
-- Name: CAST (trend_directory.trend_store_part AS name); Type: CAST; Schema: -; Owner: -
--

CREATE CAST (trend_directory.trend_store_part AS name) WITH FUNCTION trend_directory.base_table_name(trend_directory.trend_store_part) AS IMPLICIT;


--
-- Name: to_char(trend_directory.trend_store_part); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.to_char(trend_directory.trend_store_part) RETURNS text
    LANGUAGE sql STABLE STRICT
    AS $_$
SELECT $1.name::text;
$_$;


ALTER FUNCTION trend_directory.to_char(trend_directory.trend_store_part) OWNER TO postgres;

--
-- Name: CAST (trend_directory.trend_store_part AS text); Type: CAST; Schema: -; Owner: -
--

CREATE CAST (trend_directory.trend_store_part AS text) WITH FUNCTION trend_directory.to_char(trend_directory.trend_store_part) AS IMPLICIT;


--
-- Name: trend_view_part; Type: TABLE; Schema: trend_directory; Owner: postgres
--

CREATE TABLE trend_directory.trend_view_part (
    id integer NOT NULL,
    name name NOT NULL,
    trend_view_id integer NOT NULL
);


ALTER TABLE trend_directory.trend_view_part OWNER TO postgres;

--
-- Name: view_name(trend_directory.trend_view_part); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.view_name(trend_directory.trend_view_part) RETURNS name
    LANGUAGE sql
    AS $_$
SELECT $1.name;
$_$;


ALTER FUNCTION trend_directory.view_name(trend_directory.trend_view_part) OWNER TO postgres;

--
-- Name: CAST (trend_directory.trend_view_part AS name); Type: CAST; Schema: -; Owner: -
--

CREATE CAST (trend_directory.trend_view_part AS name) WITH FUNCTION trend_directory.view_name(trend_directory.trend_view_part) AS IMPLICIT;


--
-- Name: to_char(trend_directory.trend_view_part); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.to_char(trend_directory.trend_view_part) RETURNS text
    LANGUAGE sql STABLE STRICT
    AS $_$
SELECT $1.name::text;
$_$;


ALTER FUNCTION trend_directory.to_char(trend_directory.trend_view_part) OWNER TO postgres;

--
-- Name: CAST (trend_directory.trend_view_part AS text); Type: CAST; Schema: -; Owner: -
--

CREATE CAST (trend_directory.trend_view_part AS text) WITH FUNCTION trend_directory.to_char(trend_directory.trend_view_part) AS IMPLICIT;


--
-- Name: alias_schema(); Type: FUNCTION; Schema: alias_directory; Owner: postgres
--

CREATE FUNCTION alias_directory.alias_schema() RETURNS name
    LANGUAGE sql STABLE
    AS $$
SELECT 'alias'::name;
$$;


ALTER FUNCTION alias_directory.alias_schema() OWNER TO postgres;

--
-- Name: alias_type; Type: TABLE; Schema: alias_directory; Owner: postgres
--

CREATE TABLE alias_directory.alias_type (
    id integer NOT NULL,
    name character varying NOT NULL
);


ALTER TABLE alias_directory.alias_type OWNER TO postgres;

--
-- Name: create_alias_type(name); Type: FUNCTION; Schema: alias_directory; Owner: postgres
--

CREATE FUNCTION alias_directory.create_alias_type(name name) RETURNS alias_directory.alias_type
    LANGUAGE sql
    AS $_$
SELECT alias_directory.initialize_alias_type(
    alias_directory.define_alias_type($1)
);
$_$;


ALTER FUNCTION alias_directory.create_alias_type(name name) OWNER TO postgres;

--
-- Name: FUNCTION create_alias_type(name name); Type: COMMENT; Schema: alias_directory; Owner: postgres
--

COMMENT ON FUNCTION alias_directory.create_alias_type(name name) IS 'Define a new alias type and created the table for storing the aliases.';


--
-- Name: define_alias_type(name); Type: FUNCTION; Schema: alias_directory; Owner: postgres
--

CREATE FUNCTION alias_directory.define_alias_type(name name) RETURNS alias_directory.alias_type
    LANGUAGE sql
    AS $_$
INSERT INTO alias_directory.alias_type(name) VALUES ($1) RETURNING *;
$_$;


ALTER FUNCTION alias_directory.define_alias_type(name name) OWNER TO postgres;

--
-- Name: FUNCTION define_alias_type(name name); Type: COMMENT; Schema: alias_directory; Owner: postgres
--

COMMENT ON FUNCTION alias_directory.define_alias_type(name name) IS 'Define a new alias type, but do not create a table for it.';


--
-- Name: delete_alias_type(alias_directory.alias_type); Type: FUNCTION; Schema: alias_directory; Owner: postgres
--

CREATE FUNCTION alias_directory.delete_alias_type(alias_directory.alias_type) RETURNS alias_directory.alias_type
    LANGUAGE sql
    AS $_$
DELETE FROM alias_directory.alias_type WHERE id = $1.id;
SELECT public.action($1, alias_directory.drop_alias_type_sql($1));
$_$;


ALTER FUNCTION alias_directory.delete_alias_type(alias_directory.alias_type) OWNER TO postgres;

--
-- Name: drop_alias_type_sql(alias_directory.alias_type); Type: FUNCTION; Schema: alias_directory; Owner: postgres
--

CREATE FUNCTION alias_directory.drop_alias_type_sql(alias_directory.alias_type) RETURNS text
    LANGUAGE sql STABLE
    AS $_$
SELECT format(
    'DROP TABLE %I.%I;',
    alias_directory.alias_schema(),
    $1.name
);
$_$;


ALTER FUNCTION alias_directory.drop_alias_type_sql(alias_directory.alias_type) OWNER TO postgres;

--
-- Name: get_alias(integer, text); Type: FUNCTION; Schema: alias_directory; Owner: postgres
--

CREATE FUNCTION alias_directory.get_alias(entity_id integer, alias_type_name text) RETURNS text
    LANGUAGE plpgsql STABLE
    AS $_$
DECLARE
    result text;
BEGIN
    EXECUTE format(
        'SELECT alias FROM alias.%I WHERE entity_id = %s',
        $2, $1
    ) INTO result;
    RETURN result;
END;
$_$;


ALTER FUNCTION alias_directory.get_alias(entity_id integer, alias_type_name text) OWNER TO postgres;

--
-- Name: get_alias_type(name); Type: FUNCTION; Schema: alias_directory; Owner: postgres
--

CREATE FUNCTION alias_directory.get_alias_type(name name) RETURNS alias_directory.alias_type
    LANGUAGE sql STABLE
    AS $_$
SELECT alias_type FROM alias_directory.alias_type WHERE name = $1;
$_$;


ALTER FUNCTION alias_directory.get_alias_type(name name) OWNER TO postgres;

--
-- Name: get_or_create_alias_type(name); Type: FUNCTION; Schema: alias_directory; Owner: postgres
--

CREATE FUNCTION alias_directory.get_or_create_alias_type(name name) RETURNS alias_directory.alias_type
    LANGUAGE sql
    AS $_$
SELECT COALESCE(
  alias_directory.get_alias_type($1),
  alias_directory.create_alias_type($1)
);
$_$;


ALTER FUNCTION alias_directory.get_or_create_alias_type(name name) OWNER TO postgres;

--
-- Name: initialize_alias_type(alias_directory.alias_type); Type: FUNCTION; Schema: alias_directory; Owner: postgres
--

CREATE FUNCTION alias_directory.initialize_alias_type(alias_directory.alias_type) RETURNS alias_directory.alias_type
    LANGUAGE sql
    AS $_$
SELECT public.action($1, alias_directory.initialize_alias_type_sql($1));
$_$;


ALTER FUNCTION alias_directory.initialize_alias_type(alias_directory.alias_type) OWNER TO postgres;

--
-- Name: initialize_alias_type_sql(alias_directory.alias_type); Type: FUNCTION; Schema: alias_directory; Owner: postgres
--

CREATE FUNCTION alias_directory.initialize_alias_type_sql(alias_directory.alias_type) RETURNS text[]
    LANGUAGE sql STABLE
    AS $_$
SELECT ARRAY[
    format(
        'CREATE TABLE %I.%I ('
        '  entity_id serial PRIMARY KEY,'
        '  alias text NOT NULL,'
        ');',
        alias_directory.alias_schema(),
        $1.name, $1.name
    ),
    format(
        'CREATE INDEX ON %I.%I USING btree(alias);',
        alias_directory.alias_schema(),
        $1.name
    )
];
$_$;


ALTER FUNCTION alias_directory.initialize_alias_type_sql(alias_directory.alias_type) OWNER TO postgres;

--
-- Name: update_alias(alias_directory.alias_type); Type: FUNCTION; Schema: alias_directory; Owner: postgres
--

CREATE FUNCTION alias_directory.update_alias(alias_directory.alias_type) RETURNS alias_directory.alias_type
    LANGUAGE sql
    AS $_$
SELECT public.action($1, alias_directory.update_alias_sql($1));
$_$;


ALTER FUNCTION alias_directory.update_alias(alias_directory.alias_type) OWNER TO postgres;

--
-- Name: update_alias_sql(alias_directory.alias_type); Type: FUNCTION; Schema: alias_directory; Owner: postgres
--

CREATE FUNCTION alias_directory.update_alias_sql(alias_directory.alias_type) RETURNS text[]
    LANGUAGE sql STABLE
    AS $_$
SELECT ARRAY[
    format(
        'DELETE FROM %I.%I',
        alias_directory.alias_schema(),
        $1.name
    ),
    format(
        'INSERT INTO %I.%I(entity_id, alias) SELECT entity_id, alias FROM alias_def.%I'
        ');',
        alias_directory.alias_schema(),
        $1.name, $1.name
    )
];
$_$;


ALTER FUNCTION alias_directory.update_alias_sql(alias_directory.alias_type) OWNER TO postgres;

--
-- Name: add_attribute_column(attribute_directory.attribute_store, name, text); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.add_attribute_column(attribute_directory.attribute_store, name, text) RETURNS attribute_directory.attribute_store
    LANGUAGE sql
    AS $_$
SELECT public.action(
    $1,
    ARRAY[
        format('ALTER TABLE attribute_base.%I ADD COLUMN %I %s', attribute_directory.to_char($1), $2, $3),
        format('SELECT attribute_directory.drop_hash(%s::attribute_directory.attribute_store)', $1),
        format('ALTER TABLE attribute_history.%I ADD COLUMN %I %s', attribute_directory.to_char($1), $2, $3),
        format('SELECT attribute_directory.add_hash(%s::attribute_directory.attribute_store)', $1),
        format('SELECT attribute_directory.drop_staging_dependees(%s)', $1),
        format('ALTER TABLE attribute_staging.%I ADD COLUMN %I %s', attribute_directory.to_char($1), $2, $3),
        format('SELECT attribute_directory.add_staging_dependees(%s)', $1)
    ]
);
$_$;


ALTER FUNCTION attribute_directory.add_attribute_column(attribute_directory.attribute_store, name, text) OWNER TO postgres;

--
-- Name: attribute; Type: TABLE; Schema: attribute_directory; Owner: postgres
--

CREATE TABLE attribute_directory.attribute (
    id integer NOT NULL,
    attribute_store_id integer NOT NULL,
    description text,
    name name NOT NULL,
    data_type text NOT NULL,
    extra_data jsonb DEFAULT '{}'::jsonb NOT NULL
);


ALTER TABLE attribute_directory.attribute OWNER TO postgres;

--
-- Name: add_attribute_columns(attribute_directory.attribute_store, attribute_directory.attribute[]); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.add_attribute_columns(attribute_directory.attribute_store, attribute_directory.attribute[]) RETURNS attribute_directory.attribute_store
    LANGUAGE sql
    AS $_$
SELECT public.action(
    $1,
    ARRAY[
        format('SELECT attribute_directory.drop_dependees(attribute_store) FROM attribute_directory.attribute_store WHERE id = %s', $1.id),
        format(
            'ALTER TABLE attribute_base.%I %s',
            attribute_directory.to_char($1),
            (SELECT array_to_string(array_agg(format('ADD COLUMN %I %s', attribute.name, attribute.data_type)), ',') FROM unnest($2) AS attribute)
        ),
        format('SELECT attribute_directory.create_dependees(attribute_store) FROM attribute_directory.attribute_store WHERE id = %s', $1.id)
    ]
);
$_$;


ALTER FUNCTION attribute_directory.add_attribute_columns(attribute_directory.attribute_store, attribute_directory.attribute[]) OWNER TO postgres;

--
-- Name: add_attributes(attribute_directory.attribute_store, attribute_directory.attribute_descr[]); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.add_attributes(attribute_directory.attribute_store, attributes attribute_directory.attribute_descr[]) RETURNS attribute_directory.attribute_store
    LANGUAGE plpgsql
    AS $_$
BEGIN
  INSERT INTO attribute_directory.attribute(attribute_store_id, name, data_type, description) (
      SELECT $1.id, name, data_type, description
      FROM unnest($2) atts
  );
  RETURN $1;
END;
$_$;


ALTER FUNCTION attribute_directory.add_attributes(attribute_directory.attribute_store, attributes attribute_directory.attribute_descr[]) OWNER TO postgres;

--
-- Name: add_first_appearance_to_attribute_table(attribute_directory.attribute_store); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.add_first_appearance_to_attribute_table(attribute_directory.attribute_store) RETURNS attribute_directory.attribute_store
    LANGUAGE plpgsql
    AS $_$
DECLARE
    table_name name;
BEGIN
    table_name = attribute_directory.to_table_name($1);

    EXECUTE format('ALTER TABLE attribute_base.%I ADD COLUMN
        first_appearance timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP', table_name);

    EXECUTE format('UPDATE attribute_history.%I SET first_appearance = modified', table_name);

    EXECUTE format('CREATE INDEX ON attribute_history.%I (first_appearance)', table_name);

    RETURN $1;
END;
$_$;


ALTER FUNCTION attribute_directory.add_first_appearance_to_attribute_table(attribute_directory.attribute_store) OWNER TO postgres;

--
-- Name: add_hash(attribute_directory.attribute_store); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.add_hash(attribute_directory.attribute_store) RETURNS attribute_directory.attribute_store
    LANGUAGE sql
    AS $_$
SELECT public.action(
    $1,
    ARRAY[
        format(
            'ALTER TABLE attribute_history.%I ADD COLUMN hash character varying GENERATED ALWAYS AS (%s) STORED',
            attribute_directory.attribute_store_to_char($1.id),
            attribute_directory.hash_query($1)
        ),
        format('SELECT attribute_directory.create_curr_view(%s)', $1)
    ]
);
$_$;


ALTER FUNCTION attribute_directory.add_hash(attribute_directory.attribute_store) OWNER TO postgres;

--
-- Name: add_staging_dependees(attribute_directory.attribute_store); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.add_staging_dependees(attribute_directory.attribute_store) RETURNS attribute_directory.attribute_store
    LANGUAGE sql
    AS $_$
SELECT public.action(
    $1,
    ARRAY[
        format('SELECT attribute_directory.create_staging_new_view(%s)', $1),
        format('SELECT attribute_directory.create_staging_modified_view(%s)', $1)
    ]
);
$_$;


ALTER FUNCTION attribute_directory.add_staging_dependees(attribute_directory.attribute_store) OWNER TO postgres;

--
-- Name: at_function_name(attribute_directory.attribute_store); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.at_function_name(attribute_directory.attribute_store) RETURNS name
    LANGUAGE sql IMMUTABLE
    AS $_$
SELECT (attribute_directory.to_table_name($1) || '_at')::name;
$_$;


ALTER FUNCTION attribute_directory.at_function_name(attribute_directory.attribute_store) OWNER TO postgres;

--
-- Name: at_ptr_function_name(attribute_directory.attribute_store); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.at_ptr_function_name(attribute_directory.attribute_store) RETURNS name
    LANGUAGE sql IMMUTABLE
    AS $_$
SELECT (attribute_directory.to_table_name($1) || '_at_ptr')::name;
$_$;


ALTER FUNCTION attribute_directory.at_ptr_function_name(attribute_directory.attribute_store) OWNER TO postgres;

--
-- Name: attribute_name(integer); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.attribute_name(attribute_store_id integer) RETURNS text
    LANGUAGE plpgsql STABLE STRICT
    AS $_$
DECLARE
  attribute attribute_directory.attribute_store;
BEGIN
  SELECT * FROM attribute_directory.attribute_store WHERE id = $1 INTO attribute;
  RETURN attribute_directory.to_char(attribute);
END;
$_$;


ALTER FUNCTION attribute_directory.attribute_name(attribute_store_id integer) OWNER TO postgres;

--
-- Name: attribute_store_to_char(integer); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.attribute_store_to_char(attribute_store_id integer) RETURNS text
    LANGUAGE sql STABLE STRICT
    AS $_$
SELECT data_source.name || '_' || entity_type.name
  FROM attribute_directory.attribute_store
    JOIN directory.data_source ON data_source.id = attribute_store.data_source_id
    JOIN directory.entity_type ON entity_type.id = attribute_store.entity_type_id
  WHERE attribute_store.id = $1;
$_$;


ALTER FUNCTION attribute_directory.attribute_store_to_char(attribute_store_id integer) OWNER TO postgres;

--
-- Name: base_columns(); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.base_columns() RETURNS text[]
    LANGUAGE sql IMMUTABLE
    AS $$
SELECT ARRAY[
    'entity_id integer NOT NULL',
    '"timestamp" timestamp with time zone NOT NULL',
    '"end" timestamp with time zone DEFAULT NULL'
];
$$;


ALTER FUNCTION attribute_directory.base_columns() OWNER TO postgres;

--
-- Name: check_attribute_types(attribute_directory.attribute_store, attribute_directory.attribute_descr[]); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.check_attribute_types(attribute_directory.attribute_store, attribute_directory.attribute_descr[]) RETURNS SETOF attribute_directory.attribute
    LANGUAGE sql
    AS $_$
SELECT attribute_directory.update_data_type(attribute, n.data_type)
  FROM unnest($2) n
  JOIN attribute_directory.attribute
    ON attribute.name = n.name
  WHERE attribute.attribute_store_id = $1.id
    AND attribute.data_type <> trend_directory.greatest_data_type(n.data_type, attribute.data_type);
$_$;


ALTER FUNCTION attribute_directory.check_attribute_types(attribute_directory.attribute_store, attribute_directory.attribute_descr[]) OWNER TO postgres;

--
-- Name: check_attributes_exist(attribute_directory.attribute_store, attribute_directory.attribute_descr[]); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.check_attributes_exist(attribute_directory.attribute_store, attribute_directory.attribute_descr[]) RETURNS SETOF attribute_directory.attribute
    LANGUAGE sql
    AS $_$
SELECT attribute_directory.to_attribute($1, n.name, n.data_type, n.description)
    FROM unnest($2) n
    LEFT JOIN attribute_directory.attribute
    ON attribute.attribute_store_id = $1.id AND n.name = attribute.name
    WHERE attribute.name IS NULL;
$_$;


ALTER FUNCTION attribute_directory.check_attributes_exist(attribute_directory.attribute_store, attribute_directory.attribute_descr[]) OWNER TO postgres;

--
-- Name: column_specs(attribute_directory.attribute_store); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.column_specs(attribute_directory.attribute_store) RETURNS text[]
    LANGUAGE sql STABLE
    AS $_$
SELECT attribute_directory.base_columns() || array_agg(format('%I %s', name, data_type))
FROM attribute_directory.attribute
WHERE attribute_store_id = $1.id;
$_$;


ALTER FUNCTION attribute_directory.column_specs(attribute_directory.attribute_store) OWNER TO postgres;

--
-- Name: create_at_func(attribute_directory.attribute_store); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.create_at_func(attribute_directory.attribute_store) RETURNS attribute_directory.attribute_store
    LANGUAGE sql
    AS $_$
SELECT public.action(
    $1,
    format(
        'CREATE FUNCTION attribute_history.%I(timestamp with time zone)
        RETURNS SETOF attribute_history.%I
        AS $$
          BEGIN
            RETURN QUERY SELECT a.*
            FROM
                attribute_history.%I a
            JOIN
                attribute_HISTORY.%I($1) at
            ON at.id = a.id;
          END;
        $$ LANGUAGE plpgsql STABLE;',
        attribute_directory.at_function_name($1),
        attribute_directory.to_table_name($1),
        attribute_directory.to_table_name($1),
        attribute_directory.at_ptr_function_name($1)
    )
);

SELECT public.action(
    $1,
    format(
        'ALTER FUNCTION attribute_history.%I(timestamp with time zone) '
        'OWNER TO minerva_writer',
        attribute_directory.at_function_name($1)
    )
);
$_$;


ALTER FUNCTION attribute_directory.create_at_func(attribute_directory.attribute_store) OWNER TO postgres;

--
-- Name: create_at_func_ptr(attribute_directory.attribute_store); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.create_at_func_ptr(attribute_directory.attribute_store) RETURNS attribute_directory.attribute_store
    LANGUAGE plpgsql
    AS $_$
BEGIN
  RETURN public.action(
    $1,
    attribute_directory.create_at_func_ptr_sql($1)
  );
END;
$_$;


ALTER FUNCTION attribute_directory.create_at_func_ptr(attribute_directory.attribute_store) OWNER TO postgres;

--
-- Name: create_at_func_ptr_sql(attribute_directory.attribute_store); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.create_at_func_ptr_sql(attribute_directory.attribute_store) RETURNS text[]
    LANGUAGE sql STABLE
    AS $_$
SELECT ARRAY[
        format(
            'CREATE FUNCTION attribute_history.%I(timestamp with time zone)
RETURNS TABLE(id integer)
AS $$
    BEGIN
        RETURN QUERY SELECT DISTINCT ON (entity_id) s.id
            FROM attribute_history.%I s
            WHERE timestamp <= $1
            ORDER BY entity_id, timestamp DESC;
    END;
$$ LANGUAGE plpgsql STABLE',
            attribute_directory.at_ptr_function_name($1),
            attribute_directory.to_table_name($1)
        ),
        format(
            'ALTER FUNCTION attribute_history.%I(timestamp with time zone) '
            'OWNER TO minerva_writer',
            attribute_directory.at_ptr_function_name($1)
        )
    ];
$_$;


ALTER FUNCTION attribute_directory.create_at_func_ptr_sql(attribute_directory.attribute_store) OWNER TO postgres;

--
-- Name: create_attribute(attribute_directory.attribute_store, name, text, text); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.create_attribute(attribute_directory.attribute_store, name, text, text) RETURNS attribute_directory.attribute
    LANGUAGE sql
    AS $_$
INSERT INTO attribute_directory.attribute(attribute_store_id, name, data_type, description)
VALUES ($1.id, $2, $3, $4);

SELECT attribute_directory.add_attribute_column($1, $2, $3);

SELECT attribute FROM attribute_directory.attribute WHERE attribute_store_id = $1.id AND name = $2;
$_$;


ALTER FUNCTION attribute_directory.create_attribute(attribute_directory.attribute_store, name, text, text) OWNER TO postgres;

--
-- Name: create_attribute_store(text, text); Type: PROCEDURE; Schema: attribute_directory; Owner: postgres
--

CREATE PROCEDURE attribute_directory.create_attribute_store(IN data_source_name text, IN entity_type_name text)
    LANGUAGE plpgsql
    AS $_$
BEGIN
  CALL attribute_directory.init(attribute_directory.define_attribute_store($1, $2));
END;
$_$;


ALTER PROCEDURE attribute_directory.create_attribute_store(IN data_source_name text, IN entity_type_name text) OWNER TO postgres;

--
-- Name: create_attribute_store(integer, integer, attribute_directory.attribute_descr[]); Type: PROCEDURE; Schema: attribute_directory; Owner: postgres
--

CREATE PROCEDURE attribute_directory.create_attribute_store(IN data_source_id integer, IN entity_type_id integer, IN attributes attribute_directory.attribute_descr[])
    LANGUAGE sql
    AS $_$
CALL attribute_directory.init(
    attribute_directory.add_attributes(attribute_directory.define_attribute_store($1, $2), $3)
);
$_$;


ALTER PROCEDURE attribute_directory.create_attribute_store(IN data_source_id integer, IN entity_type_id integer, IN attributes attribute_directory.attribute_descr[]) OWNER TO postgres;

--
-- Name: create_attribute_store(text, text, attribute_directory.attribute_descr[]); Type: PROCEDURE; Schema: attribute_directory; Owner: postgres
--

CREATE PROCEDURE attribute_directory.create_attribute_store(IN data_source_name text, IN entity_type_name text, IN attributes attribute_directory.attribute_descr[])
    LANGUAGE plpgsql
    AS $_$
DECLARE
  store attribute_directory.attribute_store;
BEGIN
  SELECT * FROM attribute_directory.define_attribute_store($1, $2) INTO store;
  PERFORM attribute_directory.add_attributes(store, $3);
  CALL attribute_directory.init(store);
END;
$_$;


ALTER PROCEDURE attribute_directory.create_attribute_store(IN data_source_name text, IN entity_type_name text, IN attributes attribute_directory.attribute_descr[]) OWNER TO postgres;

--
-- Name: create_base_table(attribute_directory.attribute_store); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.create_base_table(attribute_directory.attribute_store) RETURNS attribute_directory.attribute_store
    LANGUAGE sql
    AS $_$
SELECT public.action($1, attribute_directory.create_base_table_sql($1));
$_$;


ALTER FUNCTION attribute_directory.create_base_table(attribute_directory.attribute_store) OWNER TO postgres;

--
-- Name: create_base_table_sql(attribute_directory.attribute_store); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.create_base_table_sql(attribute_directory.attribute_store) RETURNS text[]
    LANGUAGE sql STABLE
    AS $_$
SELECT ARRAY[
    format(
        'CREATE TABLE attribute_base.%I (%s)',
        attribute_directory.to_table_name($1),
        array_to_string(attribute_directory.column_specs($1), ',')
    ),
    format(
        'ALTER TABLE attribute_base.%I OWNER TO minerva_writer',
        attribute_directory.to_table_name($1)
    ),
    format(
        'SELECT create_distributed_table(''attribute_base.%I'', ''entity_id'')',
        attribute_directory.to_table_name($1)
    )
]
$_$;


ALTER FUNCTION attribute_directory.create_base_table_sql(attribute_directory.attribute_store) OWNER TO postgres;

--
-- Name: create_curr_ptr_table(attribute_directory.attribute_store); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.create_curr_ptr_table(attribute_directory.attribute_store) RETURNS attribute_directory.attribute_store
    LANGUAGE sql
    AS $_$
SELECT public.action(
    $1,
    attribute_directory.create_curr_ptr_table_sql($1)
);
$_$;


ALTER FUNCTION attribute_directory.create_curr_ptr_table(attribute_directory.attribute_store) OWNER TO postgres;

--
-- Name: create_curr_ptr_table_sql(attribute_directory.attribute_store); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.create_curr_ptr_table_sql(attribute_directory.attribute_store) RETURNS text[]
    LANGUAGE sql STABLE
    AS $_$
SELECT ARRAY[
    format('CREATE TABLE attribute_history.%I (
        id integer,
        PRIMARY KEY (id))',
        attribute_directory.curr_ptr_table_name($1)
    ),
    format(
        'CREATE INDEX ON attribute_history.%I (id)',
        attribute_directory.curr_ptr_table_name($1)
    ),
    format(
        'ALTER TABLE attribute_history.%I OWNER TO minerva_writer',
        attribute_directory.curr_ptr_table_name($1)
    )
];
$_$;


ALTER FUNCTION attribute_directory.create_curr_ptr_table_sql(attribute_directory.attribute_store) OWNER TO postgres;

--
-- Name: create_curr_ptr_view(attribute_directory.attribute_store); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.create_curr_ptr_view(attribute_directory.attribute_store) RETURNS attribute_directory.attribute_store
    LANGUAGE sql
    AS $_$
SELECT public.action(
    $1,
    attribute_directory.create_curr_ptr_view_sql($1)
);
$_$;


ALTER FUNCTION attribute_directory.create_curr_ptr_view(attribute_directory.attribute_store) OWNER TO postgres;

--
-- Name: create_curr_ptr_view_sql(attribute_directory.attribute_store); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.create_curr_ptr_view_sql(attribute_directory.attribute_store) RETURNS text[]
    LANGUAGE plpgsql
    AS $_$
DECLARE
    table_name name := attribute_directory.to_table_name($1);
    view_name name := attribute_directory.curr_ptr_view_name($1);
    view_sql text;
BEGIN
    view_sql = format(
        'SELECT DISTINCT ON (entity_id) '
        'a.id '
        'FROM attribute_history.%I a '
        'ORDER BY entity_id, timestamp DESC',
        table_name
    );

    RETURN ARRAY[
        format('CREATE VIEW attribute_history.%I AS %s', view_name, view_sql),
        format(
            'ALTER TABLE attribute_history.%I '
            'OWNER TO minerva_writer',
            view_name
        )
    ];
END;
$_$;


ALTER FUNCTION attribute_directory.create_curr_ptr_view_sql(attribute_directory.attribute_store) OWNER TO postgres;

--
-- Name: create_curr_view(attribute_directory.attribute_store); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.create_curr_view(attribute_directory.attribute_store) RETURNS attribute_directory.attribute_store
    LANGUAGE sql
    AS $_$
SELECT public.action(
    $1,
    attribute_directory.create_curr_view_sql($1)
);
$_$;


ALTER FUNCTION attribute_directory.create_curr_view(attribute_directory.attribute_store) OWNER TO postgres;

--
-- Name: create_curr_view_sql(attribute_directory.attribute_store); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.create_curr_view_sql(attribute_directory.attribute_store) RETURNS text[]
    LANGUAGE sql
    AS $_$
SELECT ARRAY[
    format(
        'CREATE VIEW attribute.%I AS %s',
        attribute_directory.curr_view_name($1),
        attribute_directory.curr_view_query($1)
    ),
    format(
        'ALTER TABLE attribute.%I OWNER TO minerva_writer',
        attribute_directory.curr_view_name($1)
    )
];
$_$;


ALTER FUNCTION attribute_directory.create_curr_view_sql(attribute_directory.attribute_store) OWNER TO postgres;

--
-- Name: create_dependees(attribute_directory.attribute_store); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.create_dependees(attribute_directory.attribute_store) RETURNS attribute_directory.attribute_store
    LANGUAGE sql
    AS $_$
SELECT attribute_directory.create_staging_new_view($1);
SELECT attribute_directory.create_staging_modified_view($1);
SELECT attribute_directory.create_curr_ptr_view($1);
SELECT attribute_directory.create_curr_view($1);
SELECT $1;
$_$;


ALTER FUNCTION attribute_directory.create_dependees(attribute_directory.attribute_store) OWNER TO postgres;

--
-- Name: create_entity_at_func(attribute_directory.attribute_store); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.create_entity_at_func(attribute_directory.attribute_store) RETURNS attribute_directory.attribute_store
    LANGUAGE plpgsql
    AS $_$
BEGIN
  RETURN public.action(
    $1,
    attribute_directory.create_entity_at_func_sql($1)
  );
END;
$_$;


ALTER FUNCTION attribute_directory.create_entity_at_func(attribute_directory.attribute_store) OWNER TO postgres;

--
-- Name: create_entity_at_func_ptr(attribute_directory.attribute_store); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.create_entity_at_func_ptr(attribute_directory.attribute_store) RETURNS attribute_directory.attribute_store
    LANGUAGE plpgsql
    AS $_$
BEGIN
  RETURN public.action(
    $1,
    attribute_directory.create_entity_at_func_ptr_sql($1)
  );
END;
$_$;


ALTER FUNCTION attribute_directory.create_entity_at_func_ptr(attribute_directory.attribute_store) OWNER TO postgres;

--
-- Name: create_entity_at_func_ptr_sql(attribute_directory.attribute_store); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.create_entity_at_func_ptr_sql(attribute_directory.attribute_store) RETURNS text[]
    LANGUAGE sql
    AS $_$
SELECT ARRAY[
    format(
        'CREATE FUNCTION attribute_history.%I(entity_id integer, timestamp with time zone)
RETURNS integer
AS $$
  BEGIN
    RETURN a.id
    FROM
        attribute_history.%I a
    WHERE a.timestamp <= $2 AND a.entity_id = $1
    ORDER BY a.timestamp DESC LIMIT 1;
  END;
$$ LANGUAGE plpgsql STABLE',
        attribute_directory.at_ptr_function_name($1),
        attribute_directory.to_table_name($1)
    ),
    format(
        'ALTER FUNCTION attribute_history.%I(entity_id integer, timestamp with time zone) '
        'OWNER TO minerva_writer',
        attribute_directory.at_ptr_function_name($1)
    )
];
$_$;


ALTER FUNCTION attribute_directory.create_entity_at_func_ptr_sql(attribute_directory.attribute_store) OWNER TO postgres;

--
-- Name: create_entity_at_func_sql(attribute_directory.attribute_store); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.create_entity_at_func_sql(attribute_directory.attribute_store) RETURNS text[]
    LANGUAGE sql STABLE
    AS $_$
SELECT ARRAY[
        format(
            'CREATE FUNCTION attribute_history.%I(entity_id integer, timestamp with time zone)
    RETURNS attribute_history.%I
AS $$
DECLARE
  result attribute_history.%I;
BEGIN
  SELECT *
    FROM attribute_history.%I
    WHERE id = attribute_history.%I($1, $2)
  INTO result;
  RETURN result;
END;
$$ LANGUAGE plpgsql STABLE;',
            attribute_directory.at_function_name($1),
            attribute_directory.to_table_name($1),
            attribute_directory.to_table_name($1),
            attribute_directory.to_table_name($1),
            attribute_directory.at_ptr_function_name($1)
        ),
        format(
            'ALTER FUNCTION attribute_history.%I(entity_id integer, timestamp with time zone) '
            'OWNER TO minerva_writer',
            attribute_directory.at_function_name($1)
        )
    ];
$_$;


ALTER FUNCTION attribute_directory.create_entity_at_func_sql(attribute_directory.attribute_store) OWNER TO postgres;

--
-- Name: create_history_table(attribute_directory.attribute_store); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.create_history_table(attribute_directory.attribute_store) RETURNS attribute_directory.attribute_store
    LANGUAGE sql
    AS $_$
SELECT public.action($1, attribute_directory.create_history_table_sql($1));
$_$;


ALTER FUNCTION attribute_directory.create_history_table(attribute_directory.attribute_store) OWNER TO postgres;

--
-- Name: create_history_table_sql(attribute_directory.attribute_store); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.create_history_table_sql(attribute_directory.attribute_store) RETURNS text[]
    LANGUAGE sql STABLE
    AS $_$
SELECT ARRAY[
    format(
        'CREATE TABLE attribute_history.%I (
        id serial,
        first_appearance timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
        modified timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
        hash character varying GENERATED ALWAYS AS (%s) STORED,
        %s,
        PRIMARY KEY (id, entity_id)
        )',
        attribute_directory.to_table_name($1),
        attribute_directory.hash_query($1),
        array_to_string(attribute_directory.column_specs($1), ',')
    ),
    format(
        'CREATE INDEX ON attribute_history.%I (id)',
        attribute_directory.to_table_name($1)
    ),
    format(
        'CREATE INDEX ON attribute_history.%I (first_appearance)',
        attribute_directory.to_table_name($1)
    ),
    format(
        'CREATE INDEX ON attribute_history.%I (modified)',
        attribute_directory.to_table_name($1)
    ),
    format(
        'ALTER TABLE attribute_history.%I OWNER TO minerva_writer',
        attribute_directory.to_table_name($1)
    ),
    format(
        'SELECT create_distributed_table(''attribute_history.%I'', ''entity_id'')',
        attribute_directory.to_table_name($1)
    )
];
$_$;


ALTER FUNCTION attribute_directory.create_history_table_sql(attribute_directory.attribute_store) OWNER TO postgres;

--
-- Name: create_staging_modified_view(attribute_directory.attribute_store); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.create_staging_modified_view(attribute_directory.attribute_store) RETURNS attribute_directory.attribute_store
    LANGUAGE sql
    AS $_$
SELECT public.action(
    $1,
    attribute_directory.create_staging_modified_view_sql($1)
);
$_$;


ALTER FUNCTION attribute_directory.create_staging_modified_view(attribute_directory.attribute_store) OWNER TO postgres;

--
-- Name: create_staging_modified_view_sql(attribute_directory.attribute_store); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.create_staging_modified_view_sql(attribute_directory.attribute_store) RETURNS text[]
    LANGUAGE plpgsql
    AS $_$
DECLARE
    table_name name;
    view_name name;
BEGIN
    table_name = attribute_directory.to_table_name($1);
    view_name = attribute_directory.staging_modified_view_name($1);

    RETURN ARRAY[
        format('CREATE VIEW attribute_staging.%I
AS SELECT s.* FROM attribute_staging.%I s
JOIN attribute_history.%I a ON a.entity_id = s.entity_id AND a.timestamp = s.timestamp', view_name, table_name, table_name),
        format('ALTER TABLE attribute_staging.%I
        OWNER TO minerva_writer', view_name)
    ];
END;
$_$;


ALTER FUNCTION attribute_directory.create_staging_modified_view_sql(attribute_directory.attribute_store) OWNER TO postgres;

--
-- Name: create_staging_new_view(attribute_directory.attribute_store); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.create_staging_new_view(attribute_directory.attribute_store) RETURNS attribute_directory.attribute_store
    LANGUAGE sql
    AS $_$
SELECT public.action(
    $1,
    attribute_directory.create_staging_new_view_sql($1)
);
$_$;


ALTER FUNCTION attribute_directory.create_staging_new_view(attribute_directory.attribute_store) OWNER TO postgres;

--
-- Name: create_staging_new_view_sql(attribute_directory.attribute_store); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.create_staging_new_view_sql(attribute_directory.attribute_store) RETURNS text[]
    LANGUAGE plpgsql STABLE
    AS $_$
DECLARE
    table_name name;
    view_name name;
    column_expressions text[];
    columns_part character varying;
BEGIN
    table_name = attribute_directory.to_table_name($1);
    view_name = attribute_directory.staging_new_view_name($1);

    SELECT
        array_agg(format('public.last(s.%I) AS %I', name, name)) INTO column_expressions
    FROM
        public.column_names('attribute_staging', table_name) name
    WHERE name NOT in ('entity_id', 'timestamp');

    SELECT array_to_string(
        ARRAY['s.entity_id', 's.timestamp'] || column_expressions,
        ', ')
    INTO columns_part;

    RETURN ARRAY[
        format('CREATE VIEW attribute_staging.%I
AS SELECT %s FROM attribute_staging.%I s
LEFT JOIN attribute_history.%I a
    ON a.entity_id = s.entity_id
    AND a.timestamp = s.timestamp
WHERE a.entity_id IS NULL
GROUP BY s.entity_id, s.timestamp', view_name, columns_part, table_name, table_name),
        format('ALTER TABLE attribute_staging.%I OWNER TO minerva_writer', view_name)
    ];
END;
$_$;


ALTER FUNCTION attribute_directory.create_staging_new_view_sql(attribute_directory.attribute_store) OWNER TO postgres;

--
-- Name: create_staging_table(attribute_directory.attribute_store); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.create_staging_table(attribute_directory.attribute_store) RETURNS attribute_directory.attribute_store
    LANGUAGE sql
    AS $_$
SELECT public.action(
    $1,
    attribute_directory.create_staging_table_sql($1)
);
$_$;


ALTER FUNCTION attribute_directory.create_staging_table(attribute_directory.attribute_store) OWNER TO postgres;

--
-- Name: create_staging_table_sql(attribute_directory.attribute_store); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.create_staging_table_sql(attribute_directory.attribute_store) RETURNS text[]
    LANGUAGE sql STABLE
    AS $_$
SELECT ARRAY[
    format(
        'CREATE UNLOGGED TABLE attribute_staging.%I (%s)',
        attribute_directory.to_table_name($1),
        array_to_string(attribute_directory.column_specs($1), ',')
    ),
    format(
        'CREATE INDEX ON attribute_staging.%I USING btree (entity_id, timestamp)',
        attribute_directory.to_table_name($1)
    ),
    format(
        'ALTER TABLE attribute_staging.%I OWNER TO minerva_writer',
        attribute_directory.to_table_name($1)
    ),
    format(
        'SELECT create_distributed_table(''attribute_staging.%I'', ''entity_id'')',
        attribute_directory.to_table_name($1)
    )
];
$_$;


ALTER FUNCTION attribute_directory.create_staging_table_sql(attribute_directory.attribute_store) OWNER TO postgres;

--
-- Name: curr_ptr_table_name(attribute_directory.attribute_store); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.curr_ptr_table_name(attribute_directory.attribute_store) RETURNS name
    LANGUAGE sql STABLE
    AS $_$
SELECT (attribute_directory.to_table_name($1) || '_curr_ptr')::name;
$_$;


ALTER FUNCTION attribute_directory.curr_ptr_table_name(attribute_directory.attribute_store) OWNER TO postgres;

--
-- Name: curr_ptr_view_name(attribute_directory.attribute_store); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.curr_ptr_view_name(attribute_directory.attribute_store) RETURNS name
    LANGUAGE sql STABLE
    AS $_$
SELECT (attribute_directory.to_table_name($1) || '_curr_selection')::name;
$_$;


ALTER FUNCTION attribute_directory.curr_ptr_view_name(attribute_directory.attribute_store) OWNER TO postgres;

--
-- Name: curr_view_name(attribute_directory.attribute_store); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.curr_view_name(attribute_directory.attribute_store) RETURNS name
    LANGUAGE sql STABLE
    AS $_$
SELECT attribute_directory.to_table_name($1);
$_$;


ALTER FUNCTION attribute_directory.curr_view_name(attribute_directory.attribute_store) OWNER TO postgres;

--
-- Name: curr_view_query(attribute_directory.attribute_store); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.curr_view_query(attribute_directory.attribute_store) RETURNS text
    LANGUAGE sql STABLE
    AS $_$
SELECT format(
    'SELECT h.* FROM attribute_history.%I h JOIN attribute_history.%I c ON h.id = c.id',
    attribute_directory.to_table_name($1),
    attribute_directory.curr_ptr_table_name($1)
);
$_$;


ALTER FUNCTION attribute_directory.curr_view_query(attribute_directory.attribute_store) OWNER TO postgres;

--
-- Name: define_attribute(attribute_directory.attribute_store, name, text, text); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.define_attribute(attribute_directory.attribute_store, name name, data_type text, description text) RETURNS attribute_directory.attribute
    LANGUAGE sql
    AS $_$
INSERT INTO attribute_directory.attribute(attribute_store_id, name, data_type, description)
VALUES ($1.id, $2, $3, $4)
RETURNING attribute;
$_$;


ALTER FUNCTION attribute_directory.define_attribute(attribute_directory.attribute_store, name name, data_type text, description text) OWNER TO postgres;

--
-- Name: define_attribute_store(integer, integer); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.define_attribute_store(data_source_id integer, entity_type_id integer) RETURNS attribute_directory.attribute_store
    LANGUAGE sql
    AS $_$
INSERT INTO attribute_directory.attribute_store(data_source_id, entity_type_id)
VALUES ($1, $2) RETURNING attribute_store;
$_$;


ALTER FUNCTION attribute_directory.define_attribute_store(data_source_id integer, entity_type_id integer) OWNER TO postgres;

--
-- Name: define_attribute_store(text, text); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.define_attribute_store(data_source_name text, entity_type_name text) RETURNS attribute_directory.attribute_store
    LANGUAGE sql
    AS $_$
INSERT INTO attribute_directory.attribute_store(data_source_id, entity_type_id)
VALUES ((directory.name_to_data_source($1)).id, (directory.name_to_entity_type($2)).id);
SELECT * FROM attribute_directory.attribute_store WHERE data_source_id = (directory.name_to_data_source($1)).id
  AND entity_type_id = (directory.name_to_entity_type($2)).id;
$_$;


ALTER FUNCTION attribute_directory.define_attribute_store(data_source_name text, entity_type_name text) OWNER TO postgres;

--
-- Name: deinit(attribute_directory.attribute_store); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.deinit(attribute_directory.attribute_store) RETURNS void
    LANGUAGE sql
    AS $_$
-- Other
SELECT attribute_directory.drop_dependees($1);

SELECT attribute_directory.drop_entity_at_func($1);
SELECT attribute_directory.drop_entity_at_func_ptr($1);

SELECT attribute_directory.drop_at_func($1);
SELECT attribute_directory.drop_at_func_ptr($1);

SELECT attribute_directory.drop_curr_ptr_table($1);

-- Dependent tables
SELECT attribute_directory.drop_staging_table($1);
SELECT attribute_directory.drop_history_table($1);

-- Base/parent table
SELECT attribute_directory.drop_base_table($1);
$_$;


ALTER FUNCTION attribute_directory.deinit(attribute_directory.attribute_store) OWNER TO postgres;

--
-- Name: delete_attribute_store(attribute_directory.attribute_store); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.delete_attribute_store(attribute_store attribute_directory.attribute_store) RETURNS void
    LANGUAGE sql
    AS $_$
SELECT attribute_directory.deinit($1);
DELETE FROM attribute_directory.attribute_store WHERE id = $1.id;
$_$;


ALTER FUNCTION attribute_directory.delete_attribute_store(attribute_store attribute_directory.attribute_store) OWNER TO postgres;

--
-- Name: delete_attribute_store(integer); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.delete_attribute_store(attribute_store_id integer) RETURNS void
    LANGUAGE plpgsql
    AS $_$
DECLARE
  store attribute_directory.attribute_store;
BEGIN
  SELECT * FROM attribute_directory.attribute_store WHERE id = $1 INTO store;
  PERFORM attribute_directory.delete_attribute_store(store);
END;
$_$;


ALTER FUNCTION attribute_directory.delete_attribute_store(attribute_store_id integer) OWNER TO postgres;

--
-- Name: delete_attribute_store(text); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.delete_attribute_store(name text) RETURNS void
    LANGUAGE plpgsql
    AS $_$
DECLARE
  store attribute_directory.attribute_store;
BEGIN
  SELECT * FROM attribute_directory.attribute_store WHERE attribute_directory.attribute_store_to_char(attribute_store.id) = $1 INTO store;
  PERFORM attribute_directory.delete_attribute_store(store);
END;
$_$;


ALTER FUNCTION attribute_directory.delete_attribute_store(name text) OWNER TO postgres;

--
-- Name: dependers(name); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.dependers(name name) RETURNS TABLE(name name, level integer)
    LANGUAGE sql STABLE
    AS $_$
SELECT * FROM attribute_directory.dependers($1, 1);
$_$;


ALTER FUNCTION attribute_directory.dependers(name name) OWNER TO postgres;

--
-- Name: dependers(name, integer); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.dependers(name name, level integer) RETURNS TABLE(name name, level integer)
    LANGUAGE sql STABLE
    AS $_$
SELECT (d.dependers).* FROM (
    SELECT attribute_directory.dependers(depender, $2 + 1)
    FROM attribute_directory.direct_dependers($1) depender
) d
UNION ALL
SELECT depender, $2
FROM attribute_directory.direct_dependers($1) depender;
$_$;


ALTER FUNCTION attribute_directory.dependers(name name, level integer) OWNER TO postgres;

--
-- Name: direct_dependers(text); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.direct_dependers(name text) RETURNS SETOF name
    LANGUAGE sql STABLE
    AS $_$
SELECT dependee.relname AS name
FROM pg_depend
JOIN pg_rewrite ON pg_depend.objid = pg_rewrite.oid
JOIN pg_class as dependee ON pg_rewrite.ev_class = dependee.oid
JOIN pg_class as dependent ON pg_depend.refobjid = dependent.oid
JOIN pg_namespace as n ON dependent.relnamespace = n.oid
JOIN pg_attribute ON
        pg_depend.refobjid = pg_attribute.attrelid
        AND
        pg_depend.refobjsubid = pg_attribute.attnum
WHERE pg_attribute.attnum > 0 AND dependent.relname = $1;
$_$;


ALTER FUNCTION attribute_directory.direct_dependers(name text) OWNER TO postgres;

--
-- Name: drop_at_func(attribute_directory.attribute_store); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.drop_at_func(attribute_directory.attribute_store) RETURNS attribute_directory.attribute_store
    LANGUAGE sql
    AS $_$
SELECT public.action(
    $1,
    format(
        'DROP FUNCTION attribute_history.%I(timestamp with time zone)',
        attribute_directory.at_function_name($1)
    )
);
$_$;


ALTER FUNCTION attribute_directory.drop_at_func(attribute_directory.attribute_store) OWNER TO postgres;

--
-- Name: drop_at_func_ptr(attribute_directory.attribute_store); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.drop_at_func_ptr(attribute_directory.attribute_store) RETURNS attribute_directory.attribute_store
    LANGUAGE sql
    AS $_$
SELECT public.action(
    $1,
    attribute_directory.drop_at_func_ptr_sql($1)
);
$_$;


ALTER FUNCTION attribute_directory.drop_at_func_ptr(attribute_directory.attribute_store) OWNER TO postgres;

--
-- Name: drop_at_func_ptr_sql(attribute_directory.attribute_store); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.drop_at_func_ptr_sql(attribute_directory.attribute_store) RETURNS text
    LANGUAGE sql STABLE
    AS $_$
SELECT format(
    'DROP FUNCTION attribute_history.%I(timestamp with time zone)',
    attribute_directory.at_ptr_function_name($1)
)
$_$;


ALTER FUNCTION attribute_directory.drop_at_func_ptr_sql(attribute_directory.attribute_store) OWNER TO postgres;

--
-- Name: drop_attribute(attribute_directory.attribute_store, name); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.drop_attribute(attribute_directory.attribute_store, name) RETURNS attribute_directory.attribute_store
    LANGUAGE sql
    AS $_$
DELETE FROM attribute_directory.attribute
WHERE attribute_store_id = $1.id AND name = $2;

SELECT attribute_directory.remove_attribute_column($1, $2);
$_$;


ALTER FUNCTION attribute_directory.drop_attribute(attribute_directory.attribute_store, name) OWNER TO postgres;

--
-- Name: drop_base_table(attribute_directory.attribute_store); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.drop_base_table(attribute_directory.attribute_store) RETURNS attribute_directory.attribute_store
    LANGUAGE sql
    AS $_$
SELECT public.action($1, attribute_directory.drop_base_table_sql($1));
$_$;


ALTER FUNCTION attribute_directory.drop_base_table(attribute_directory.attribute_store) OWNER TO postgres;

--
-- Name: drop_base_table_sql(attribute_directory.attribute_store); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.drop_base_table_sql(attribute_directory.attribute_store) RETURNS text
    LANGUAGE sql STABLE
    AS $_$
SELECT format(
    'DROP TABLE attribute_base.%I',
    attribute_directory.to_table_name($1)
);
$_$;


ALTER FUNCTION attribute_directory.drop_base_table_sql(attribute_directory.attribute_store) OWNER TO postgres;

--
-- Name: drop_curr_ptr_table(attribute_directory.attribute_store); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.drop_curr_ptr_table(attribute_directory.attribute_store) RETURNS attribute_directory.attribute_store
    LANGUAGE sql
    AS $_$
SELECT public.action(
    $1,
    attribute_directory.drop_curr_ptr_table_sql($1)
);
$_$;


ALTER FUNCTION attribute_directory.drop_curr_ptr_table(attribute_directory.attribute_store) OWNER TO postgres;

--
-- Name: drop_curr_ptr_table_sql(attribute_directory.attribute_store); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.drop_curr_ptr_table_sql(attribute_directory.attribute_store) RETURNS text
    LANGUAGE sql STABLE
    AS $_$
SELECT format(
    'DROP TABLE attribute_history.%I',
    attribute_directory.curr_ptr_table_name($1)
);
$_$;


ALTER FUNCTION attribute_directory.drop_curr_ptr_table_sql(attribute_directory.attribute_store) OWNER TO postgres;

--
-- Name: drop_curr_ptr_view(attribute_directory.attribute_store); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.drop_curr_ptr_view(attribute_directory.attribute_store) RETURNS attribute_directory.attribute_store
    LANGUAGE sql
    AS $_$
SELECT public.action(
    $1,
    attribute_directory.drop_curr_ptr_view_sql($1)
);
$_$;


ALTER FUNCTION attribute_directory.drop_curr_ptr_view(attribute_directory.attribute_store) OWNER TO postgres;

--
-- Name: drop_curr_ptr_view_sql(attribute_directory.attribute_store); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.drop_curr_ptr_view_sql(attribute_directory.attribute_store) RETURNS character varying
    LANGUAGE sql
    AS $_$
SELECT format('DROP VIEW attribute_history.%I', attribute_directory.curr_ptr_view_name($1));
$_$;


ALTER FUNCTION attribute_directory.drop_curr_ptr_view_sql(attribute_directory.attribute_store) OWNER TO postgres;

--
-- Name: drop_curr_view(attribute_directory.attribute_store); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.drop_curr_view(attribute_directory.attribute_store) RETURNS attribute_directory.attribute_store
    LANGUAGE sql
    AS $_$
SELECT public.action(
    $1,
    attribute_directory.drop_curr_view_sql($1)
);
$_$;


ALTER FUNCTION attribute_directory.drop_curr_view(attribute_directory.attribute_store) OWNER TO postgres;

--
-- Name: drop_curr_view_sql(attribute_directory.attribute_store); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.drop_curr_view_sql(attribute_directory.attribute_store) RETURNS character varying
    LANGUAGE sql STABLE
    AS $_$
SELECT format('DROP VIEW attribute.%I CASCADE', attribute_directory.to_table_name($1));
$_$;


ALTER FUNCTION attribute_directory.drop_curr_view_sql(attribute_directory.attribute_store) OWNER TO postgres;

--
-- Name: drop_dependees(attribute_directory.attribute_store); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.drop_dependees(attribute_directory.attribute_store) RETURNS attribute_directory.attribute_store
    LANGUAGE sql
    AS $_$
SELECT attribute_directory.drop_curr_view($1);
SELECT attribute_directory.drop_curr_ptr_view($1);
SELECT attribute_directory.drop_staging_modified_view($1);
SELECT attribute_directory.drop_staging_new_view($1);
SELECT $1;
$_$;


ALTER FUNCTION attribute_directory.drop_dependees(attribute_directory.attribute_store) OWNER TO postgres;

--
-- Name: drop_entity_at_func(attribute_directory.attribute_store); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.drop_entity_at_func(attribute_directory.attribute_store) RETURNS attribute_directory.attribute_store
    LANGUAGE sql
    AS $_$
SELECT public.action(
    $1,
    attribute_directory.drop_entity_at_func_sql($1)
);
$_$;


ALTER FUNCTION attribute_directory.drop_entity_at_func(attribute_directory.attribute_store) OWNER TO postgres;

--
-- Name: drop_entity_at_func_ptr(attribute_directory.attribute_store); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.drop_entity_at_func_ptr(attribute_directory.attribute_store) RETURNS attribute_directory.attribute_store
    LANGUAGE sql
    AS $_$
SELECT public.action(
    $1,
    attribute_directory.drop_entity_at_func_ptr_sql($1)
);
$_$;


ALTER FUNCTION attribute_directory.drop_entity_at_func_ptr(attribute_directory.attribute_store) OWNER TO postgres;

--
-- Name: drop_entity_at_func_ptr_sql(attribute_directory.attribute_store); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.drop_entity_at_func_ptr_sql(attribute_directory.attribute_store) RETURNS text
    LANGUAGE sql
    AS $_$
SELECT format(
    'DROP FUNCTION attribute_history.%I(entity_id integer, timestamp with time zone)',
    attribute_directory.at_ptr_function_name($1)
)
$_$;


ALTER FUNCTION attribute_directory.drop_entity_at_func_ptr_sql(attribute_directory.attribute_store) OWNER TO postgres;

--
-- Name: drop_entity_at_func_sql(attribute_directory.attribute_store); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.drop_entity_at_func_sql(attribute_directory.attribute_store) RETURNS text
    LANGUAGE sql STABLE
    AS $_$
SELECT format(
    'DROP FUNCTION attribute_history.%I(integer, timestamp with time zone)',
    attribute_directory.at_function_name($1)
);
$_$;


ALTER FUNCTION attribute_directory.drop_entity_at_func_sql(attribute_directory.attribute_store) OWNER TO postgres;

--
-- Name: drop_hash(attribute_directory.attribute_store); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.drop_hash(attribute_directory.attribute_store) RETURNS attribute_directory.attribute_store
    LANGUAGE sql
    AS $_$
SELECT public.action(
    $1,
    ARRAY[
        format('SELECT attribute_directory.drop_curr_view(%s)', $1),
        format('ALTER TABLE attribute_history.%I DROP COLUMN hash CASCADE', attribute_directory.attribute_store_to_char($1.id))
    ]
);
$_$;


ALTER FUNCTION attribute_directory.drop_hash(attribute_directory.attribute_store) OWNER TO postgres;

--
-- Name: drop_hash_function(attribute_directory.attribute_store); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.drop_hash_function(attribute_directory.attribute_store) RETURNS attribute_directory.attribute_store
    LANGUAGE plpgsql
    AS $_$
BEGIN
    EXECUTE format('DROP FUNCTION attribute_history.values_hash(attribute_history.%I)', attribute_directory.to_table_name($1));

    RETURN $1;
END;
$_$;


ALTER FUNCTION attribute_directory.drop_hash_function(attribute_directory.attribute_store) OWNER TO postgres;

--
-- Name: drop_history_table(attribute_directory.attribute_store); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.drop_history_table(attribute_directory.attribute_store) RETURNS attribute_directory.attribute_store
    LANGUAGE sql
    AS $_$
SELECT public.action($1, attribute_directory.drop_history_table_sql($1));
$_$;


ALTER FUNCTION attribute_directory.drop_history_table(attribute_directory.attribute_store) OWNER TO postgres;

--
-- Name: drop_history_table_sql(attribute_directory.attribute_store); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.drop_history_table_sql(attribute_directory.attribute_store) RETURNS text
    LANGUAGE sql STABLE
    AS $_$
SELECT format(
    'DROP TABLE attribute_history.%I',
    attribute_directory.to_table_name($1)
);
$_$;


ALTER FUNCTION attribute_directory.drop_history_table_sql(attribute_directory.attribute_store) OWNER TO postgres;

--
-- Name: drop_staging_dependees(attribute_directory.attribute_store); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.drop_staging_dependees(attribute_directory.attribute_store) RETURNS attribute_directory.attribute_store
    LANGUAGE sql
    AS $_$
SELECT public.action(
    $1,
    ARRAY[
        format('SELECT attribute_directory.drop_staging_modified_view(%s)', $1),
        format('SELECT attribute_directory.drop_staging_new_view(%s)', $1)
    ]
);
$_$;


ALTER FUNCTION attribute_directory.drop_staging_dependees(attribute_directory.attribute_store) OWNER TO postgres;

--
-- Name: drop_staging_modified_view(attribute_directory.attribute_store); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.drop_staging_modified_view(attribute_directory.attribute_store) RETURNS attribute_directory.attribute_store
    LANGUAGE sql
    AS $_$
SELECT public.action(
    $1,
    attribute_directory.drop_staging_modified_view_sql($1)
);
$_$;


ALTER FUNCTION attribute_directory.drop_staging_modified_view(attribute_directory.attribute_store) OWNER TO postgres;

--
-- Name: drop_staging_modified_view_sql(attribute_directory.attribute_store); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.drop_staging_modified_view_sql(attribute_directory.attribute_store) RETURNS character varying
    LANGUAGE sql
    AS $_$
SELECT format('DROP VIEW attribute_staging.%I', attribute_directory.staging_modified_view_name($1));
$_$;


ALTER FUNCTION attribute_directory.drop_staging_modified_view_sql(attribute_directory.attribute_store) OWNER TO postgres;

--
-- Name: drop_staging_new_view(attribute_directory.attribute_store); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.drop_staging_new_view(attribute_directory.attribute_store) RETURNS attribute_directory.attribute_store
    LANGUAGE plpgsql
    AS $_$
BEGIN
    EXECUTE format('DROP VIEW attribute_staging.%I', attribute_directory.to_table_name($1) || '_new');

    RETURN $1;
END;
$_$;


ALTER FUNCTION attribute_directory.drop_staging_new_view(attribute_directory.attribute_store) OWNER TO postgres;

--
-- Name: drop_staging_table(attribute_directory.attribute_store); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.drop_staging_table(attribute_directory.attribute_store) RETURNS attribute_directory.attribute_store
    LANGUAGE sql
    AS $_$
SELECT public.action(
    $1,
    attribute_directory.drop_staging_table_sql($1)
);
$_$;


ALTER FUNCTION attribute_directory.drop_staging_table(attribute_directory.attribute_store) OWNER TO postgres;

--
-- Name: drop_staging_table_sql(attribute_directory.attribute_store); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.drop_staging_table_sql(attribute_directory.attribute_store) RETURNS text
    LANGUAGE sql STABLE
    AS $_$
SELECT format(
    'DROP TABLE attribute_staging.%I',
    attribute_directory.to_table_name($1)
);
$_$;


ALTER FUNCTION attribute_directory.drop_staging_table_sql(attribute_directory.attribute_store) OWNER TO postgres;

--
-- Name: get_attribute(attribute_directory.attribute_store, name); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.get_attribute(attribute_directory.attribute_store, name) RETURNS attribute_directory.attribute
    LANGUAGE sql STABLE
    AS $_$
SELECT attribute
FROM attribute_directory.attribute
WHERE attribute_store_id = $1.id AND name = $2;
$_$;


ALTER FUNCTION attribute_directory.get_attribute(attribute_directory.attribute_store, name) OWNER TO postgres;

--
-- Name: get_attribute_store(integer); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.get_attribute_store(attribute_store_id integer) RETURNS attribute_directory.attribute_store
    LANGUAGE sql STABLE
    AS $_$
SELECT attribute_store
FROM attribute_directory.attribute_store
WHERE id = $1

$_$;


ALTER FUNCTION attribute_directory.get_attribute_store(attribute_store_id integer) OWNER TO postgres;

--
-- Name: get_attribute_store(integer, integer); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.get_attribute_store(data_source_id integer, entity_type_id integer) RETURNS attribute_directory.attribute_store
    LANGUAGE sql STABLE
    AS $_$
SELECT attribute_store
FROM attribute_directory.attribute_store
WHERE data_source_id = $1 AND entity_type_id = $2;
$_$;


ALTER FUNCTION attribute_directory.get_attribute_store(data_source_id integer, entity_type_id integer) OWNER TO postgres;

--
-- Name: get_attribute_store(text, text); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.get_attribute_store(data_source text, entity_type text) RETURNS attribute_directory.attribute_store
    LANGUAGE sql
    AS $_$
SELECT attribute_store
FROM attribute_directory.attribute_store
LEFT JOIN directory.data_source
  ON data_source_id = data_source.id
LEFT JOIN directory.entity_type
  ON entity_type_id = entity_type.id
WHERE data_source.name = $1 AND lower(entity_type.name) = lower($2);
$_$;


ALTER FUNCTION attribute_directory.get_attribute_store(data_source text, entity_type text) OWNER TO postgres;

--
-- Name: hash_query(attribute_directory.attribute_store); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.hash_query(attribute_directory.attribute_store) RETURNS text
    LANGUAGE plpgsql STABLE
    AS $_$
BEGIN
  IF action_count(format('SELECT 1 FROM attribute_directory.attribute WHERE attribute_store_id = %s', $1.id)) = 0
    THEN RETURN '''Q''';
  ELSE 
    RETURN 'md5(' ||
      array_to_string(array_agg(format('(CASE WHEN %I IS NULL THEN '''' ELSE %s END)', name, attribute_directory.hash_query_part(a))), ' || ') ||
      ')' FROM attribute_directory.attribute a WHERE attribute_store_id = $1.id;
  END IF;
END;
$_$;


ALTER FUNCTION attribute_directory.hash_query(attribute_directory.attribute_store) OWNER TO postgres;

--
-- Name: hash_query_part(attribute_directory.attribute); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.hash_query_part(attribute_directory.attribute) RETURNS text
    LANGUAGE sql STABLE
    AS $_$
SELECT CASE
  WHEN $1.data_type LIKE '%[]'
  THEN format('array_to_text(%I)', $1.name)
  ELSE format('%I::text', $1.name)
END;
$_$;


ALTER FUNCTION attribute_directory.hash_query_part(attribute_directory.attribute) OWNER TO postgres;

--
-- Name: init(attribute_directory.attribute); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.init(attribute_directory.attribute) RETURNS attribute_directory.attribute
    LANGUAGE sql
    AS $_$
SELECT public.action(
    $1,
    format('SELECT attribute_directory.add_attribute_column(attribute_store, %L, %L) FROM attribute_directory.attribute_store WHERE id = %s',
    $1.name, $1.data_type, $1.attribute_store_id)
)
$_$;


ALTER FUNCTION attribute_directory.init(attribute_directory.attribute) OWNER TO postgres;

--
-- Name: init(attribute_directory.attribute_store); Type: PROCEDURE; Schema: attribute_directory; Owner: postgres
--

CREATE PROCEDURE attribute_directory.init(IN attribute_directory.attribute_store)
    LANGUAGE plpgsql
    AS $_$
BEGIN
  -- Base table
  PERFORM attribute_directory.create_base_table($1);

  -- Dependent tables
  PERFORM attribute_directory.create_history_table($1);
  PERFORM attribute_directory.create_staging_table($1);

  -- Separate table
  PERFORM attribute_directory.create_curr_ptr_table($1);

  -- Other
  PERFORM attribute_directory.create_at_func_ptr($1);
  PERFORM attribute_directory.create_at_func($1);

  PERFORM attribute_directory.create_entity_at_func_ptr($1);
  PERFORM attribute_directory.create_entity_at_func($1);

  PERFORM attribute_directory.create_dependees($1);

END;
$_$;


ALTER PROCEDURE attribute_directory.init(IN attribute_directory.attribute_store) OWNER TO postgres;

--
-- Name: last_history_id(integer); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.last_history_id(attribute_store_id integer) RETURNS integer
    LANGUAGE plpgsql STABLE
    AS $_$
DECLARE
  result integer;
BEGIN
  EXECUTE FORMAT(
    'SELECT COALESCE(MAX(id), 0) FROM attribute_history.%I', 
    attribute_directory.to_table_name(attribute_directory.get_attribute_store($1))
  ) INTO result;
  RETURN result;
END;
$_$;


ALTER FUNCTION attribute_directory.last_history_id(attribute_store_id integer) OWNER TO postgres;

--
-- Name: attribute_store_curr_materialized; Type: TABLE; Schema: attribute_directory; Owner: postgres
--

CREATE TABLE attribute_directory.attribute_store_curr_materialized (
    attribute_store_id integer NOT NULL,
    materialized timestamp with time zone NOT NULL
);


ALTER TABLE attribute_directory.attribute_store_curr_materialized OWNER TO postgres;

--
-- Name: mark_curr_materialized(integer); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.mark_curr_materialized(attribute_store_id integer) RETURNS attribute_directory.attribute_store_curr_materialized
    LANGUAGE sql
    AS $_$
SELECT attribute_directory.mark_curr_materialized(attribute_store_id, modified)
FROM attribute_directory.attribute_store_modified
WHERE attribute_store_id = $1;
$_$;


ALTER FUNCTION attribute_directory.mark_curr_materialized(attribute_store_id integer) OWNER TO postgres;

--
-- Name: mark_curr_materialized(integer, timestamp with time zone); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.mark_curr_materialized(attribute_store_id integer, materialized timestamp with time zone) RETURNS attribute_directory.attribute_store_curr_materialized
    LANGUAGE sql
    AS $_$
SELECT COALESCE(attribute_directory.update_curr_materialized($1, $2), attribute_directory.store_curr_materialized($1, $2));
$_$;


ALTER FUNCTION attribute_directory.mark_curr_materialized(attribute_store_id integer, materialized timestamp with time zone) OWNER TO postgres;

--
-- Name: attribute_store_modified; Type: TABLE; Schema: attribute_directory; Owner: postgres
--

CREATE TABLE attribute_directory.attribute_store_modified (
    attribute_store_id integer NOT NULL,
    modified timestamp with time zone NOT NULL
);


ALTER TABLE attribute_directory.attribute_store_modified OWNER TO postgres;

--
-- Name: mark_modified(integer); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.mark_modified(attribute_store_id integer) RETURNS attribute_directory.attribute_store_modified
    LANGUAGE sql
    AS $_$
SELECT attribute_directory.mark_modified($1, now())
$_$;


ALTER FUNCTION attribute_directory.mark_modified(attribute_store_id integer) OWNER TO postgres;

--
-- Name: mark_modified(integer, timestamp with time zone); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.mark_modified(attribute_store_id integer, modified timestamp with time zone) RETURNS attribute_directory.attribute_store_modified
    LANGUAGE sql
    AS $_$
INSERT INTO attribute_directory.attribute_store_modified (attribute_store_id, modified)
VALUES ($1, $2)
ON CONFLICT (attribute_store_id) DO UPDATE
SET modified = EXCLUDED.modified
RETURNING attribute_store_modified;
$_$;


ALTER FUNCTION attribute_directory.mark_modified(attribute_store_id integer, modified timestamp with time zone) OWNER TO postgres;

--
-- Name: sampled_view_materialization; Type: TABLE; Schema: attribute_directory; Owner: postgres
--

CREATE TABLE attribute_directory.sampled_view_materialization (
    id integer NOT NULL,
    attribute_store_id integer NOT NULL,
    src_view text NOT NULL
);


ALTER TABLE attribute_directory.sampled_view_materialization OWNER TO postgres;

--
-- Name: materialize(attribute_directory.sampled_view_materialization); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.materialize(attribute_directory.sampled_view_materialization) RETURNS integer
    LANGUAGE sql
    AS $_$
SELECT attribute_directory.stage_sample($1);

SELECT attribute_directory.transfer_staged(attribute_store)
FROM attribute_directory.attribute_store
WHERE id = $1.attribute_store_id;
$_$;


ALTER FUNCTION attribute_directory.materialize(attribute_directory.sampled_view_materialization) OWNER TO postgres;

--
-- Name: materialize_curr_ptr(attribute_directory.attribute_store); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.materialize_curr_ptr(attribute_directory.attribute_store) RETURNS integer
    LANGUAGE plpgsql
    AS $_$
DECLARE
    table_name name := attribute_directory.curr_ptr_table_name($1);
    view_name name := attribute_directory.curr_ptr_view_name($1);
    row_count integer;
BEGIN
    EXECUTE format('TRUNCATE attribute_history.%I', table_name);
    EXECUTE format(
        'INSERT INTO attribute_history.%I (id) '
        'SELECT id '
        'FROM attribute_history.%I', table_name, view_name
    );

    GET DIAGNOSTICS row_count = ROW_COUNT;

    PERFORM attribute_directory.mark_curr_materialized($1.id);

    RETURN row_count;
END;
$_$;


ALTER FUNCTION attribute_directory.materialize_curr_ptr(attribute_directory.attribute_store) OWNER TO postgres;

--
-- Name: modify_column_type(attribute_directory.attribute_store, name, text); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.modify_column_type(attribute_directory.attribute_store, column_name name, data_type text) RETURNS attribute_directory.attribute_store
    LANGUAGE sql
    AS $_$
SELECT attribute_directory.modify_column_type(
    attribute_directory.to_table_name($1), $2, $3
);

SELECT $1;
$_$;


ALTER FUNCTION attribute_directory.modify_column_type(attribute_directory.attribute_store, column_name name, data_type text) OWNER TO postgres;

--
-- Name: modify_column_type(name, name, text); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.modify_column_type(table_name name, column_name name, data_type text) RETURNS void
    LANGUAGE plpgsql
    AS $$
BEGIN
    EXECUTE format('ALTER TABLE attribute_base.%I ALTER %I TYPE %s USING CAST(%I AS %s)', table_name, column_name, data_type, column_name, data_type);
END;
$$;


ALTER FUNCTION attribute_directory.modify_column_type(table_name name, column_name name, data_type text) OWNER TO postgres;

--
-- Name: modify_data_type(attribute_directory.attribute); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.modify_data_type(attribute_directory.attribute) RETURNS attribute_directory.attribute
    LANGUAGE plpgsql
    AS $_$
DECLARE
  store attribute_directory.attribute_store;
BEGIN
  SELECT * FROM attribute_directory.attribute_store WHERE id = $1.attribute_store_id INTO store;
  RETURN public.action(
      $1,
      ARRAY[
          format('ALTER TABLE attribute_base.%I ALTER %I TYPE %s', attribute_directory.to_char(store), $1.name, $1.data_type),
          format('SELECT attribute_directory.drop_hash(%s::attribute_directory.attribute_store)', store),
          format('ALTER TABLE attribute_history.%I ALTER %I TYPE %s', attribute_directory.to_char(store), $1.name, $1.data_type),
          format('SELECT attribute_directory.add_hash(%s::attribute_directory.attribute_store)', store),
          format('SELECT attribute_directory.drop_staging_dependees(%s)', store),
          format('ALTER TABLE attribute_staging.%I ALTER %I TYPE %s', attribute_directory.to_char(store), $1.name, $1.data_type),
          format('SELECT attribute_directory.add_staging_dependees(%s)', store)
      ]
  );
END;
$_$;


ALTER FUNCTION attribute_directory.modify_data_type(attribute_directory.attribute) OWNER TO postgres;

--
-- Name: remove_attribute_column(attribute_directory.attribute_store, name); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.remove_attribute_column(attribute_directory.attribute_store, name) RETURNS attribute_directory.attribute_store
    LANGUAGE sql
    AS $_$
SELECT public.action(
    $1,
    ARRAY[
        format('SELECT attribute_directory.drop_hash(%s::attribute_directory.attribute_store)', $1),
        format('ALTER TABLE attribute_base.%I DROP COLUMN %I CASCADE', attribute_directory.to_char($1), $2),
        format('ALTER TABLE attribute_history.%I DROP COLUMN %I CASCADE', attribute_directory.to_char($1), $2),
        format('SELECT attribute_directory.add_hash(%s::attribute_directory.attribute_store)', $1),
        format('SELECT attribute_directory.drop_staging_dependees(%s)', $1),
        format('ALTER TABLE attribute_staging.%I DROP COLUMN %I CASCADE', attribute_directory.to_char($1), $2),
        format('SELECT attribute_directory.add_staging_dependees(%s)', $1)
    ]
);
$_$;


ALTER FUNCTION attribute_directory.remove_attribute_column(attribute_directory.attribute_store, name) OWNER TO postgres;

--
-- Name: stage_sample(attribute_directory.sampled_view_materialization); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.stage_sample(attribute_directory.sampled_view_materialization) RETURNS integer
    LANGUAGE sql
    AS $_$
SELECT public.action_count(
    attribute_directory.view_to_attribute_staging_sql($1.src_view, attribute_store)
)
FROM attribute_directory.attribute_store
WHERE id = $1.attribute_store_id
$_$;


ALTER FUNCTION attribute_directory.stage_sample(attribute_directory.sampled_view_materialization) OWNER TO postgres;

--
-- Name: staging_modified_view_name(attribute_directory.attribute_store); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.staging_modified_view_name(attribute_directory.attribute_store) RETURNS name
    LANGUAGE sql STABLE
    AS $_$
SELECT (attribute_directory.to_table_name($1) || '_modified')::name;
$_$;


ALTER FUNCTION attribute_directory.staging_modified_view_name(attribute_directory.attribute_store) OWNER TO postgres;

--
-- Name: staging_new_view_name(attribute_directory.attribute_store); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.staging_new_view_name(attribute_directory.attribute_store) RETURNS name
    LANGUAGE sql STABLE
    AS $_$
SELECT (attribute_directory.to_table_name($1) || '_new')::name;
$_$;


ALTER FUNCTION attribute_directory.staging_new_view_name(attribute_directory.attribute_store) OWNER TO postgres;

--
-- Name: store_curr_materialized(integer, timestamp with time zone); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.store_curr_materialized(attribute_store_id integer, materialized timestamp with time zone) RETURNS attribute_directory.attribute_store_curr_materialized
    LANGUAGE sql
    AS $_$
INSERT INTO attribute_directory.attribute_store_curr_materialized (attribute_store_id, materialized)
VALUES ($1, $2)
RETURNING attribute_store_curr_materialized;
$_$;


ALTER FUNCTION attribute_directory.store_curr_materialized(attribute_store_id integer, materialized timestamp with time zone) OWNER TO postgres;

--
-- Name: to_attribute(attribute_directory.attribute_store, name, text, text); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.to_attribute(attribute_directory.attribute_store, name name, data_type text, description text) RETURNS attribute_directory.attribute
    LANGUAGE sql
    AS $_$
SELECT COALESCE(
        attribute_directory.get_attribute($1, $2),
        attribute_directory.init(attribute_directory.define_attribute($1, $2, $3, $4))
    );
$_$;


ALTER FUNCTION attribute_directory.to_attribute(attribute_directory.attribute_store, name name, data_type text, description text) OWNER TO postgres;

--
-- Name: to_table_name(attribute_directory.attribute_store); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.to_table_name(attribute_directory.attribute_store) RETURNS name
    LANGUAGE sql STABLE STRICT
    AS $_$
SELECT (attribute_directory.to_char($1))::name;
$_$;


ALTER FUNCTION attribute_directory.to_table_name(attribute_directory.attribute_store) OWNER TO postgres;

--
-- Name: transfer_staged(attribute_directory.attribute_store); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.transfer_staged(attribute_directory.attribute_store) RETURNS integer
    LANGUAGE plpgsql
    AS $_$
DECLARE
    table_name name;
    columns_part text;
    set_columns_part text;
    default_columns text[];
    row_count integer;
BEGIN
    table_name = attribute_directory.to_table_name($1);

    default_columns = ARRAY[
        'entity_id',
        '"timestamp"'
    ];

    SELECT array_to_string(default_columns || array_agg(format('%I', name)), ', ') INTO columns_part
    FROM attribute_directory.attribute
    WHERE attribute_store_id = $1.id;

    EXECUTE format(
        'INSERT INTO attribute_history.%I(%s) SELECT %s FROM attribute_staging.%I',
        table_name, columns_part, columns_part, table_name || '_new'
    );

    GET DIAGNOSTICS row_count = ROW_COUNT;

    PERFORM attribute_directory.mark_modified($1.id);

    SELECT array_to_string(array_agg(format('%I = m.%I', name, name)), ', ') INTO set_columns_part
    FROM attribute_directory.attribute
    WHERE attribute_store_id = $1.id;

    EXECUTE format(
        'UPDATE attribute_history.%I a '
        'SET modified = now(), %s '
        'FROM attribute_staging.%I m '
        'WHERE m.entity_id = a.entity_id AND m.timestamp = a.timestamp',
        table_name, set_columns_part, table_name || '_modified'
    );

    EXECUTE format('TRUNCATE attribute_staging.%I', table_name);

    PERFORM attribute_directory.mark_modified($1.id);

    RETURN row_count;
END;
$_$;


ALTER FUNCTION attribute_directory.transfer_staged(attribute_directory.attribute_store) OWNER TO postgres;

--
-- Name: transfer_staged(integer); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.transfer_staged(attribute_store_id integer) RETURNS integer
    LANGUAGE plpgsql
    AS $_$
DECLARE
  attribute attribute_directory.attribute_store;
BEGIN
  SELECT * FROM attribute_directory.attribute_store WHERE id = $1 INTO attribute;
  RETURN attribute_directory.transfer_staged(attribute);
END;
$_$;


ALTER FUNCTION attribute_directory.transfer_staged(attribute_store_id integer) OWNER TO postgres;

--
-- Name: update_curr_materialized(integer, timestamp with time zone); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.update_curr_materialized(attribute_store_id integer, materialized timestamp with time zone) RETURNS attribute_directory.attribute_store_curr_materialized
    LANGUAGE sql
    AS $_$
UPDATE attribute_directory.attribute_store_curr_materialized
SET materialized = greatest(materialized, $2)
WHERE attribute_store_id = $1
RETURNING attribute_store_curr_materialized;
$_$;


ALTER FUNCTION attribute_directory.update_curr_materialized(attribute_store_id integer, materialized timestamp with time zone) OWNER TO postgres;

--
-- Name: update_data_type(attribute_directory.attribute, text); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.update_data_type(attribute_directory.attribute, new_data_type text) RETURNS attribute_directory.attribute
    LANGUAGE sql
    AS $_$
UPDATE attribute_directory.attribute SET data_type = $2
  WHERE id = $1.id;
SELECT
    attribute_directory.create_dependees(
        attribute_directory.modify_column_type(
            attribute_directory.drop_dependees(attribute_store),
            $1.name,
            $2
        )
    )
FROM attribute_directory.attribute_store
WHERE id = $1.attribute_store_id;

SELECT $1;
$_$;


ALTER FUNCTION attribute_directory.update_data_type(attribute_directory.attribute, new_data_type text) OWNER TO postgres;

--
-- Name: update_data_type_on_change(); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.update_data_type_on_change() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    IF OLD.data_type <> NEW.data_type THEN
        PERFORM attribute_directory.modify_data_type(NEW);
    END IF;

    RETURN NEW;
END;
$$;


ALTER FUNCTION attribute_directory.update_data_type_on_change() OWNER TO postgres;

--
-- Name: upgrade_attribute_store(attribute_directory.attribute_store); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.upgrade_attribute_store(attribute_directory.attribute_store) RETURNS void
    LANGUAGE sql
    AS $_$
SELECT attribute_directory.drop_compacted_view($1);
SELECT attribute_directory.drop_run_length_view($1);
SELECT attribute_directory.drop_compacted_tmp_table($1);
$_$;


ALTER FUNCTION attribute_directory.upgrade_attribute_store(attribute_directory.attribute_store) OWNER TO postgres;

--
-- Name: view_to_attribute_staging_sql(text, attribute_directory.attribute_store); Type: FUNCTION; Schema: attribute_directory; Owner: postgres
--

CREATE FUNCTION attribute_directory.view_to_attribute_staging_sql(view text, attribute_directory.attribute_store) RETURNS text
    LANGUAGE sql
    AS $_$
SELECT format(
    'INSERT INTO attribute_staging.%1$I(%2$s) SELECT %2$s FROM %3$s',
    attribute_directory.to_table_name($2),
    array_to_string(ARRAY['entity_id', 'timestamp']::text[] || array_agg(quote_ident(attribute.name)), ', '),
    $1::regclass::text
)
FROM attribute_directory.attribute
WHERE attribute_store_id = $2.id;
$_$;


ALTER FUNCTION attribute_directory.view_to_attribute_staging_sql(view text, attribute_directory.attribute_store) OWNER TO postgres;

--
-- Name: hub_node; Type: TABLE; Schema: attribute_history; Owner: minerva_writer
--

CREATE TABLE attribute_history.hub_node (
    id integer NOT NULL,
    first_appearance timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    modified timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    hash character varying GENERATED ALWAYS AS (md5(((((
CASE
    WHEN (name IS NULL) THEN ''::text
    ELSE name
END ||
CASE
    WHEN (equipment_type IS NULL) THEN ''::text
    ELSE equipment_type
END) ||
CASE
    WHEN (equipment_serial IS NULL) THEN ''::text
    ELSE equipment_serial
END) ||
CASE
    WHEN (longitude IS NULL) THEN ''::text
    ELSE (longitude)::text
END) ||
CASE
    WHEN (latitude IS NULL) THEN ''::text
    ELSE (latitude)::text
END))) STORED,
    entity_id integer NOT NULL,
    "timestamp" timestamp with time zone NOT NULL,
    "end" timestamp with time zone,
    name text,
    equipment_type text,
    equipment_serial text,
    longitude real,
    latitude real
);


ALTER TABLE attribute_history.hub_node OWNER TO minerva_writer;

--
-- Name: hub_node_at(timestamp with time zone); Type: FUNCTION; Schema: attribute_history; Owner: minerva_writer
--

CREATE FUNCTION attribute_history.hub_node_at(timestamp with time zone) RETURNS SETOF attribute_history.hub_node
    LANGUAGE plpgsql STABLE
    AS $_$
          BEGIN
            RETURN QUERY SELECT a.*
            FROM
                attribute_history.hub_node a
            JOIN
                attribute_HISTORY.hub_node_at_ptr($1) at
            ON at.id = a.id;
          END;
        $_$;


ALTER FUNCTION attribute_history.hub_node_at(timestamp with time zone) OWNER TO minerva_writer;

--
-- Name: hub_node_at(integer, timestamp with time zone); Type: FUNCTION; Schema: attribute_history; Owner: minerva_writer
--

CREATE FUNCTION attribute_history.hub_node_at(entity_id integer, timestamp with time zone) RETURNS attribute_history.hub_node
    LANGUAGE plpgsql STABLE
    AS $_$
DECLARE
  result attribute_history.hub_node;
BEGIN
  SELECT *
    FROM attribute_history.hub_node
    WHERE id = attribute_history.hub_node_at_ptr($1, $2)
  INTO result;
  RETURN result;
END;
$_$;


ALTER FUNCTION attribute_history.hub_node_at(entity_id integer, timestamp with time zone) OWNER TO minerva_writer;

--
-- Name: hub_node_at_ptr(timestamp with time zone); Type: FUNCTION; Schema: attribute_history; Owner: minerva_writer
--

CREATE FUNCTION attribute_history.hub_node_at_ptr(timestamp with time zone) RETURNS TABLE(id integer)
    LANGUAGE plpgsql STABLE
    AS $_$
    BEGIN
        RETURN QUERY SELECT DISTINCT ON (entity_id) s.id
            FROM attribute_history.hub_node s
            WHERE timestamp <= $1
            ORDER BY entity_id, timestamp DESC;
    END;
$_$;


ALTER FUNCTION attribute_history.hub_node_at_ptr(timestamp with time zone) OWNER TO minerva_writer;

--
-- Name: hub_node_at_ptr(integer, timestamp with time zone); Type: FUNCTION; Schema: attribute_history; Owner: minerva_writer
--

CREATE FUNCTION attribute_history.hub_node_at_ptr(entity_id integer, timestamp with time zone) RETURNS integer
    LANGUAGE plpgsql STABLE
    AS $_$
  BEGIN
    RETURN a.id
    FROM
        attribute_history.hub_node a
    WHERE a.timestamp <= $2 AND a.entity_id = $1
    ORDER BY a.timestamp DESC LIMIT 1;
  END;
$_$;


ALTER FUNCTION attribute_history.hub_node_at_ptr(entity_id integer, timestamp with time zone) OWNER TO minerva_writer;

--
-- Name: minerva_entity_set; Type: TABLE; Schema: attribute_history; Owner: minerva_writer
--

CREATE TABLE attribute_history.minerva_entity_set (
    id integer NOT NULL,
    first_appearance timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    modified timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    hash character varying GENERATED ALWAYS AS (md5(((((((
CASE
    WHEN (name IS NULL) THEN ''::text
    ELSE name
END ||
CASE
    WHEN (fullname IS NULL) THEN ''::text
    ELSE fullname
END) ||
CASE
    WHEN ("group" IS NULL) THEN ''::text
    ELSE "group"
END) ||
CASE
    WHEN (source_entity_type IS NULL) THEN ''::text
    ELSE source_entity_type
END) ||
CASE
    WHEN (owner IS NULL) THEN ''::text
    ELSE owner
END) ||
CASE
    WHEN (description IS NULL) THEN ''::text
    ELSE description
END) ||
CASE
    WHEN (last_update IS NULL) THEN ''::text
    ELSE last_update
END))) STORED,
    entity_id integer NOT NULL,
    "timestamp" timestamp with time zone NOT NULL,
    "end" timestamp with time zone,
    name text,
    fullname text,
    "group" text,
    source_entity_type text,
    owner text,
    description text,
    last_update text
);


ALTER TABLE attribute_history.minerva_entity_set OWNER TO minerva_writer;

--
-- Name: minerva_entity_set_at(timestamp with time zone); Type: FUNCTION; Schema: attribute_history; Owner: minerva_writer
--

CREATE FUNCTION attribute_history.minerva_entity_set_at(timestamp with time zone) RETURNS SETOF attribute_history.minerva_entity_set
    LANGUAGE plpgsql STABLE
    AS $_$
          BEGIN
            RETURN QUERY SELECT a.*
            FROM
                attribute_history.minerva_entity_set a
            JOIN
                attribute_HISTORY.minerva_entity_set_at_ptr($1) at
            ON at.id = a.id;
          END;
        $_$;


ALTER FUNCTION attribute_history.minerva_entity_set_at(timestamp with time zone) OWNER TO minerva_writer;

--
-- Name: minerva_entity_set_at(integer, timestamp with time zone); Type: FUNCTION; Schema: attribute_history; Owner: minerva_writer
--

CREATE FUNCTION attribute_history.minerva_entity_set_at(entity_id integer, timestamp with time zone) RETURNS attribute_history.minerva_entity_set
    LANGUAGE plpgsql STABLE
    AS $_$
DECLARE
  result attribute_history.minerva_entity_set;
BEGIN
  SELECT *
    FROM attribute_history.minerva_entity_set
    WHERE id = attribute_history.minerva_entity_set_at_ptr($1, $2)
  INTO result;
  RETURN result;
END;
$_$;


ALTER FUNCTION attribute_history.minerva_entity_set_at(entity_id integer, timestamp with time zone) OWNER TO minerva_writer;

--
-- Name: minerva_entity_set_at_ptr(timestamp with time zone); Type: FUNCTION; Schema: attribute_history; Owner: minerva_writer
--

CREATE FUNCTION attribute_history.minerva_entity_set_at_ptr(timestamp with time zone) RETURNS TABLE(id integer)
    LANGUAGE plpgsql STABLE
    AS $_$
    BEGIN
        RETURN QUERY SELECT DISTINCT ON (entity_id) s.id
            FROM attribute_history.minerva_entity_set s
            WHERE timestamp <= $1
            ORDER BY entity_id, timestamp DESC;
    END;
$_$;


ALTER FUNCTION attribute_history.minerva_entity_set_at_ptr(timestamp with time zone) OWNER TO minerva_writer;

--
-- Name: minerva_entity_set_at_ptr(integer, timestamp with time zone); Type: FUNCTION; Schema: attribute_history; Owner: minerva_writer
--

CREATE FUNCTION attribute_history.minerva_entity_set_at_ptr(entity_id integer, timestamp with time zone) RETURNS integer
    LANGUAGE plpgsql STABLE
    AS $_$
  BEGIN
    RETURN a.id
    FROM
        attribute_history.minerva_entity_set a
    WHERE a.timestamp <= $2 AND a.entity_id = $1
    ORDER BY a.timestamp DESC LIMIT 1;
  END;
$_$;


ALTER FUNCTION attribute_history.minerva_entity_set_at_ptr(entity_id integer, timestamp with time zone) OWNER TO minerva_writer;

--
-- Name: cleanup_on_data_source_delete(integer); Type: FUNCTION; Schema: directory; Owner: postgres
--

CREATE FUNCTION directory.cleanup_on_data_source_delete(data_source_id integer) RETURNS void
    LANGUAGE sql
    AS $_$
SELECT attribute_directory.delete_attribute_store(s.id) FROM attribute_directory.attribute_store s WHERE s.data_source_id = $1;
DELETE FROM notification_directory.notification_store WHERE data_source_id = $1;
$_$;


ALTER FUNCTION directory.cleanup_on_data_source_delete(data_source_id integer) OWNER TO postgres;

--
-- Name: data_source; Type: TABLE; Schema: directory; Owner: postgres
--

CREATE TABLE directory.data_source (
    id integer NOT NULL,
    name character varying NOT NULL,
    description character varying NOT NULL
);


ALTER TABLE directory.data_source OWNER TO postgres;

--
-- Name: TABLE data_source; Type: COMMENT; Schema: directory; Owner: postgres
--

COMMENT ON TABLE directory.data_source IS 'Describes data_sources. A data_source is used to indicate where data came
from. Datasources are also used to prevent collisions between sets of
data from different sources, where names can be the same, but the meaning
of the data differs.';


--
-- Name: create_data_source(text); Type: FUNCTION; Schema: directory; Owner: postgres
--

CREATE FUNCTION directory.create_data_source(text) RETURNS directory.data_source
    LANGUAGE sql STRICT
    AS $_$
INSERT INTO directory.data_source (name, description)
VALUES ($1, 'default');
SELECT * FROM directory.data_source WHERE name = $1;
$_$;


ALTER FUNCTION directory.create_data_source(text) OWNER TO postgres;

--
-- Name: entity_type; Type: TABLE; Schema: directory; Owner: postgres
--

CREATE TABLE directory.entity_type (
    id integer NOT NULL,
    name character varying NOT NULL,
    description character varying NOT NULL
);


ALTER TABLE directory.entity_type OWNER TO postgres;

--
-- Name: TABLE entity_type; Type: COMMENT; Schema: directory; Owner: postgres
--

COMMENT ON TABLE directory.entity_type IS 'Stores the entity types that exist in the entity table. Entity types are
also used to give context to data that is stored for entities.';


--
-- Name: create_entity_type(text); Type: FUNCTION; Schema: directory; Owner: postgres
--

CREATE FUNCTION directory.create_entity_type(text) RETURNS directory.entity_type
    LANGUAGE sql STRICT
    AS $_$
SELECT directory.init_entity_type(directory.define_entity_type($1));
$_$;


ALTER FUNCTION directory.create_entity_type(text) OWNER TO postgres;

--
-- Name: define_entity_type(text); Type: FUNCTION; Schema: directory; Owner: postgres
--

CREATE FUNCTION directory.define_entity_type(text) RETURNS directory.entity_type
    LANGUAGE sql STRICT
    AS $_$
INSERT INTO directory.entity_type(name, description)
VALUES ($1, '')
ON CONFLICT DO NOTHING;
SELECT * FROM directory.entity_type WHERE name = $1;
$_$;


ALTER FUNCTION directory.define_entity_type(text) OWNER TO postgres;

--
-- Name: delete_data_source(text); Type: FUNCTION; Schema: directory; Owner: postgres
--

CREATE FUNCTION directory.delete_data_source(text) RETURNS directory.data_source
    LANGUAGE sql STRICT
    AS $_$
SELECT directory.cleanup_on_data_source_delete(s.id) FROM directory.data_source s WHERE s.name = $1;
DELETE FROM directory.data_source WHERE name = $1 RETURNING *;
$_$;


ALTER FUNCTION directory.delete_data_source(text) OWNER TO postgres;

--
-- Name: delete_entity_type(directory.entity_type); Type: FUNCTION; Schema: directory; Owner: postgres
--

CREATE FUNCTION directory.delete_entity_type(directory.entity_type) RETURNS void
    LANGUAGE sql
    AS $_$
SELECT attribute_directory.delete_attribute_store(s.id) FROM attribute_directory.attribute_store s WHERE s.entity_type_id = $1.id;
DELETE FROM directory.entity_type WHERE id = $1.id;
$_$;


ALTER FUNCTION directory.delete_entity_type(directory.entity_type) OWNER TO postgres;

--
-- Name: get_data_source(text); Type: FUNCTION; Schema: directory; Owner: postgres
--

CREATE FUNCTION directory.get_data_source(text) RETURNS directory.data_source
    LANGUAGE sql STABLE STRICT
    AS $_$
SELECT * FROM directory.data_source WHERE name = $1;
$_$;


ALTER FUNCTION directory.get_data_source(text) OWNER TO postgres;

--
-- Name: get_entity_type(text); Type: FUNCTION; Schema: directory; Owner: postgres
--

CREATE FUNCTION directory.get_entity_type(text) RETURNS directory.entity_type
    LANGUAGE sql STABLE STRICT
    AS $_$
SELECT entity_type FROM directory.entity_type WHERE lower(name) = lower($1);
$_$;


ALTER FUNCTION directory.get_entity_type(text) OWNER TO postgres;

--
-- Name: get_entity_type_name(integer); Type: FUNCTION; Schema: directory; Owner: postgres
--

CREATE FUNCTION directory.get_entity_type_name(integer) RETURNS text
    LANGUAGE sql STABLE STRICT
    AS $_$
SELECT name FROM directory.entity_type WHERE id = $1;
$_$;


ALTER FUNCTION directory.get_entity_type_name(integer) OWNER TO postgres;

--
-- Name: init_entity_type(directory.entity_type); Type: FUNCTION; Schema: directory; Owner: postgres
--

CREATE FUNCTION directory.init_entity_type(directory.entity_type) RETURNS directory.entity_type
    LANGUAGE sql STRICT
    AS $_$
SELECT entity.create_entity_table($1);
SELECT entity.create_get_entity_function($1);
SELECT entity.create_create_entity_function($1);
SELECT entity.create_to_entity_function($1);
$_$;


ALTER FUNCTION directory.init_entity_type(directory.entity_type) OWNER TO postgres;

--
-- Name: name_to_data_source(text); Type: FUNCTION; Schema: directory; Owner: postgres
--

CREATE FUNCTION directory.name_to_data_source(text) RETURNS directory.data_source
    LANGUAGE sql STRICT
    AS $_$
SELECT COALESCE(directory.get_data_source($1), directory.create_data_source($1));
$_$;


ALTER FUNCTION directory.name_to_data_source(text) OWNER TO postgres;

--
-- Name: name_to_entity_type(text); Type: FUNCTION; Schema: directory; Owner: postgres
--

CREATE FUNCTION directory.name_to_entity_type(text) RETURNS directory.entity_type
    LANGUAGE sql STRICT
    AS $_$
SELECT COALESCE(directory.get_entity_type($1), directory.create_entity_type($1));
$_$;


ALTER FUNCTION directory.name_to_entity_type(text) OWNER TO postgres;

--
-- Name: create_create_entity_function(directory.entity_type); Type: FUNCTION; Schema: entity; Owner: postgres
--

CREATE FUNCTION entity.create_create_entity_function(directory.entity_type) RETURNS directory.entity_type
    LANGUAGE sql
    AS $_$
SELECT public.action($1, entity.create_create_entity_function_sql($1));
$_$;


ALTER FUNCTION entity.create_create_entity_function(directory.entity_type) OWNER TO postgres;

--
-- Name: create_create_entity_function_sql(directory.entity_type); Type: FUNCTION; Schema: entity; Owner: postgres
--

CREATE FUNCTION entity.create_create_entity_function_sql(directory.entity_type) RETURNS text[]
    LANGUAGE sql
    AS $_$
SELECT ARRAY[
    format(
      'CREATE FUNCTION entity.%I(text) RETURNS entity.%I
      AS $$
        INSERT INTO entity.%I(name) VALUES ($1) ON CONFLICT DO NOTHING;
        SELECT e FROM entity.%I e WHERE name = $1;
      $$ LANGUAGE sql',
      entity.create_entity_function_name($1),
      $1.name,
      $1.name,
      $1.name,
      $1.name
    )
];
$_$;


ALTER FUNCTION entity.create_create_entity_function_sql(directory.entity_type) OWNER TO postgres;

--
-- Name: create_entity_function_name(directory.entity_type); Type: FUNCTION; Schema: entity; Owner: postgres
--

CREATE FUNCTION entity.create_entity_function_name(directory.entity_type) RETURNS name
    LANGUAGE sql STABLE
    AS $_$
SELECT format('create_%s', $1.name)::name;
$_$;


ALTER FUNCTION entity.create_entity_function_name(directory.entity_type) OWNER TO postgres;

--
-- Name: entity_set; Type: TABLE; Schema: entity; Owner: postgres
--

CREATE TABLE entity.entity_set (
    id integer NOT NULL,
    name text,
    created timestamp with time zone DEFAULT now()
);


ALTER TABLE entity.entity_set OWNER TO postgres;

--
-- Name: create_entity_set(text); Type: FUNCTION; Schema: entity; Owner: postgres
--

CREATE FUNCTION entity.create_entity_set(text) RETURNS entity.entity_set
    LANGUAGE sql
    AS $_$
        INSERT INTO entity.entity_set(name) VALUES ($1) ON CONFLICT DO NOTHING;
        SELECT e FROM entity.entity_set e WHERE name = $1;
      $_$;


ALTER FUNCTION entity.create_entity_set(text) OWNER TO postgres;

--
-- Name: create_entity_table(directory.entity_type); Type: FUNCTION; Schema: entity; Owner: postgres
--

CREATE FUNCTION entity.create_entity_table(directory.entity_type) RETURNS directory.entity_type
    LANGUAGE sql
    AS $_$
SELECT public.action($1, entity.create_entity_table_sql($1));
$_$;


ALTER FUNCTION entity.create_entity_table(directory.entity_type) OWNER TO postgres;

--
-- Name: create_entity_table_sql(directory.entity_type); Type: FUNCTION; Schema: entity; Owner: postgres
--

CREATE FUNCTION entity.create_entity_table_sql(directory.entity_type) RETURNS text[]
    LANGUAGE sql
    AS $_$
SELECT ARRAY[
    format(
      'CREATE TABLE IF NOT EXISTS entity.%I('
      'id serial,'
      'name text UNIQUE,'
      'created timestamp with time zone default now()'
      ');',
      $1.name
    ),
    format(
       'SELECT create_reference_table(''entity.%I'');',
       $1.name
    )
];
$_$;


ALTER FUNCTION entity.create_entity_table_sql(directory.entity_type) OWNER TO postgres;

--
-- Name: create_get_entity_function(directory.entity_type); Type: FUNCTION; Schema: entity; Owner: postgres
--

CREATE FUNCTION entity.create_get_entity_function(directory.entity_type) RETURNS directory.entity_type
    LANGUAGE sql
    AS $_$
SELECT public.action($1, entity.create_get_entity_function_sql($1));
$_$;


ALTER FUNCTION entity.create_get_entity_function(directory.entity_type) OWNER TO postgres;

--
-- Name: create_get_entity_function_sql(directory.entity_type); Type: FUNCTION; Schema: entity; Owner: postgres
--

CREATE FUNCTION entity.create_get_entity_function_sql(directory.entity_type) RETURNS text[]
    LANGUAGE sql
    AS $_$
SELECT ARRAY[
    format(
      'CREATE FUNCTION entity.%I(text) RETURNS entity.%I
      AS $$
        SELECT * FROM entity.%I WHERE name = $1;
      $$ LANGUAGE sql',
      entity.get_entity_function_name($1),
      $1.name,
      $1.name
    )
];
$_$;


ALTER FUNCTION entity.create_get_entity_function_sql(directory.entity_type) OWNER TO postgres;

--
-- Name: node; Type: TABLE; Schema: entity; Owner: postgres
--

CREATE TABLE entity.node (
    id integer NOT NULL,
    name text,
    created timestamp with time zone DEFAULT now()
);


ALTER TABLE entity.node OWNER TO postgres;

--
-- Name: create_node(text); Type: FUNCTION; Schema: entity; Owner: postgres
--

CREATE FUNCTION entity.create_node(text) RETURNS entity.node
    LANGUAGE sql
    AS $_$
        INSERT INTO entity.node(name) VALUES ($1) ON CONFLICT DO NOTHING;
        SELECT e FROM entity.node e WHERE name = $1;
      $_$;


ALTER FUNCTION entity.create_node(text) OWNER TO postgres;

--
-- Name: create_to_entity_function(directory.entity_type); Type: FUNCTION; Schema: entity; Owner: postgres
--

CREATE FUNCTION entity.create_to_entity_function(directory.entity_type) RETURNS directory.entity_type
    LANGUAGE sql
    AS $_$
SELECT public.action($1, entity.create_to_entity_function_sql($1));
$_$;


ALTER FUNCTION entity.create_to_entity_function(directory.entity_type) OWNER TO postgres;

--
-- Name: create_to_entity_function_sql(directory.entity_type); Type: FUNCTION; Schema: entity; Owner: postgres
--

CREATE FUNCTION entity.create_to_entity_function_sql(directory.entity_type) RETURNS text[]
    LANGUAGE sql
    AS $_$
SELECT ARRAY[
    format(
      'CREATE FUNCTION entity.%I(text) RETURNS entity.%I
      AS $$
        SELECT coalesce(entity.%I($1), entity.%I($1));
      $$ LANGUAGE sql',
      entity.to_entity_function_name($1),
      $1.name,
      entity.get_entity_function_name($1),
      entity.create_entity_function_name($1)
    )
];
$_$;


ALTER FUNCTION entity.create_to_entity_function_sql(directory.entity_type) OWNER TO postgres;

--
-- Name: v-network; Type: TABLE; Schema: entity; Owner: postgres
--

CREATE TABLE entity."v-network" (
    id integer NOT NULL,
    name text,
    created timestamp with time zone DEFAULT now()
);


ALTER TABLE entity."v-network" OWNER TO postgres;

--
-- Name: create_v-network(text); Type: FUNCTION; Schema: entity; Owner: postgres
--

CREATE FUNCTION entity."create_v-network"(text) RETURNS entity."v-network"
    LANGUAGE sql
    AS $_$
        INSERT INTO entity."v-network"(name) VALUES ($1) ON CONFLICT DO NOTHING;
        SELECT e FROM entity."v-network" e WHERE name = $1;
      $_$;


ALTER FUNCTION entity."create_v-network"(text) OWNER TO postgres;

--
-- Name: get_entity_function_name(directory.entity_type); Type: FUNCTION; Schema: entity; Owner: postgres
--

CREATE FUNCTION entity.get_entity_function_name(directory.entity_type) RETURNS name
    LANGUAGE sql STABLE
    AS $_$
SELECT format('get_%s', $1.name)::name;
$_$;


ALTER FUNCTION entity.get_entity_function_name(directory.entity_type) OWNER TO postgres;

--
-- Name: get_entity_set(text); Type: FUNCTION; Schema: entity; Owner: postgres
--

CREATE FUNCTION entity.get_entity_set(text) RETURNS entity.entity_set
    LANGUAGE sql
    AS $_$
        SELECT * FROM entity.entity_set WHERE name = $1;
      $_$;


ALTER FUNCTION entity.get_entity_set(text) OWNER TO postgres;

--
-- Name: get_node(text); Type: FUNCTION; Schema: entity; Owner: postgres
--

CREATE FUNCTION entity.get_node(text) RETURNS entity.node
    LANGUAGE sql
    AS $_$
        SELECT * FROM entity.node WHERE name = $1;
      $_$;


ALTER FUNCTION entity.get_node(text) OWNER TO postgres;

--
-- Name: get_v-network(text); Type: FUNCTION; Schema: entity; Owner: postgres
--

CREATE FUNCTION entity."get_v-network"(text) RETURNS entity."v-network"
    LANGUAGE sql
    AS $_$
        SELECT * FROM entity."v-network" WHERE name = $1;
      $_$;


ALTER FUNCTION entity."get_v-network"(text) OWNER TO postgres;

--
-- Name: to_entity_function_name(directory.entity_type); Type: FUNCTION; Schema: entity; Owner: postgres
--

CREATE FUNCTION entity.to_entity_function_name(directory.entity_type) RETURNS name
    LANGUAGE sql STABLE
    AS $_$
SELECT format('to_%s', $1.name)::name;
$_$;


ALTER FUNCTION entity.to_entity_function_name(directory.entity_type) OWNER TO postgres;

--
-- Name: to_entity_set(text); Type: FUNCTION; Schema: entity; Owner: postgres
--

CREATE FUNCTION entity.to_entity_set(text) RETURNS entity.entity_set
    LANGUAGE sql
    AS $_$
        SELECT coalesce(entity.get_entity_set($1), entity.create_entity_set($1));
      $_$;


ALTER FUNCTION entity.to_entity_set(text) OWNER TO postgres;

--
-- Name: to_node(text); Type: FUNCTION; Schema: entity; Owner: postgres
--

CREATE FUNCTION entity.to_node(text) RETURNS entity.node
    LANGUAGE sql
    AS $_$
        SELECT coalesce(entity.get_node($1), entity.create_node($1));
      $_$;


ALTER FUNCTION entity.to_node(text) OWNER TO postgres;

--
-- Name: to_v-network(text); Type: FUNCTION; Schema: entity; Owner: postgres
--

CREATE FUNCTION entity."to_v-network"(text) RETURNS entity."v-network"
    LANGUAGE sql
    AS $_$
        SELECT coalesce(entity."get_v-network"($1), entity."create_v-network"($1));
      $_$;


ALTER FUNCTION entity."to_v-network"(text) OWNER TO postgres;

--
-- Name: end_job(bigint); Type: FUNCTION; Schema: logging; Owner: postgres
--

CREATE FUNCTION logging.end_job(job_id bigint) RETURNS void
    LANGUAGE sql
    AS $_$
UPDATE logging.job SET finished=clock_timestamp() WHERE id=$1;
$_$;


ALTER FUNCTION logging.end_job(job_id bigint) OWNER TO postgres;

--
-- Name: start_job(jsonb); Type: FUNCTION; Schema: logging; Owner: postgres
--

CREATE FUNCTION logging.start_job(action jsonb) RETURNS bigint
    LANGUAGE sql
    AS $_$
INSERT INTO logging.job(action, started) VALUES ($1, clock_timestamp())
RETURNING job.id;
$_$;


ALTER FUNCTION logging.start_job(action jsonb) OWNER TO postgres;

--
-- Name: attribute; Type: TABLE; Schema: notification_directory; Owner: postgres
--

CREATE TABLE notification_directory.attribute (
    id integer NOT NULL,
    notification_store_id integer,
    name name NOT NULL,
    data_type name NOT NULL,
    description character varying NOT NULL
);


ALTER TABLE notification_directory.attribute OWNER TO postgres;

--
-- Name: TABLE attribute; Type: COMMENT; Schema: notification_directory; Owner: postgres
--

COMMENT ON TABLE notification_directory.attribute IS 'Describes attributes of notification stores. An attribute of a
notification store is an attribute that each notification stored in that
notification store has. An attribute corresponds directly to a column in
the main notification store table';


--
-- Name: add_attribute_column_sql(name, notification_directory.attribute); Type: FUNCTION; Schema: notification_directory; Owner: postgres
--

CREATE FUNCTION notification_directory.add_attribute_column_sql(name, notification_directory.attribute) RETURNS text
    LANGUAGE sql STABLE
    AS $_$
SELECT format(
    'ALTER TABLE %I.%I ADD COLUMN %I %s',
    notification_directory.notification_store_schema(),
    $1, $2.name, $2.data_type
);
$_$;


ALTER FUNCTION notification_directory.add_attribute_column_sql(name, notification_directory.attribute) OWNER TO postgres;

--
-- Name: add_staging_attribute_column_sql(notification_directory.attribute); Type: FUNCTION; Schema: notification_directory; Owner: postgres
--

CREATE FUNCTION notification_directory.add_staging_attribute_column_sql(notification_directory.attribute) RETURNS text
    LANGUAGE sql STABLE
    AS $_$
SELECT
    format(
        'ALTER TABLE %I.%I ADD COLUMN %I %s',
        notification_directory.notification_store_schema(),
        notification_directory.staging_table_name(notification_store), $1.name, $1.data_type
    )
FROM notification_directory.notification_store WHERE id = $1.notification_store_id;
$_$;


ALTER FUNCTION notification_directory.add_staging_attribute_column_sql(notification_directory.attribute) OWNER TO postgres;

--
-- Name: create_attribute_column(notification_directory.attribute); Type: FUNCTION; Schema: notification_directory; Owner: postgres
--

CREATE FUNCTION notification_directory.create_attribute_column(notification_directory.attribute) RETURNS notification_directory.attribute
    LANGUAGE sql
    AS $_$
SELECT public.action(
    $1,
    notification_directory.add_attribute_column_sql(
        notification_directory.table_name(notification_store),
        $1
    )
)
FROM notification_directory.notification_store WHERE id = $1.notification_store_id;

SELECT public.action(
    $1,
    notification_directory.add_attribute_column_sql(
        notification_directory.staging_table_name(notification_store),
        $1
    )
)
FROM notification_directory.notification_store WHERE id = $1.notification_store_id;
$_$;


ALTER FUNCTION notification_directory.create_attribute_column(notification_directory.attribute) OWNER TO postgres;

--
-- Name: notification_set_store; Type: TABLE; Schema: notification_directory; Owner: postgres
--

CREATE TABLE notification_directory.notification_set_store (
    id integer NOT NULL,
    name name NOT NULL,
    notification_store_id integer
);


ALTER TABLE notification_directory.notification_set_store OWNER TO postgres;

--
-- Name: TABLE notification_set_store; Type: COMMENT; Schema: notification_directory; Owner: postgres
--

COMMENT ON TABLE notification_directory.notification_set_store IS 'Describes notification_set_stores. A notification_set_store can hold information over sets of notifications that are related to each other.';


--
-- Name: create_notification_set_store(name, notification_directory.notification_store); Type: FUNCTION; Schema: notification_directory; Owner: postgres
--

CREATE FUNCTION notification_directory.create_notification_set_store(name name, notification_directory.notification_store) RETURNS notification_directory.notification_set_store
    LANGUAGE sql
    AS $_$
SELECT notification_directory.create_notification_set_store($1, $2.id);
$_$;


ALTER FUNCTION notification_directory.create_notification_set_store(name name, notification_directory.notification_store) OWNER TO postgres;

--
-- Name: create_notification_set_store(name, integer); Type: FUNCTION; Schema: notification_directory; Owner: postgres
--

CREATE FUNCTION notification_directory.create_notification_set_store(name name, notification_store_id integer) RETURNS notification_directory.notification_set_store
    LANGUAGE sql
    AS $_$
SELECT notification_directory.init_notification_set_store(
    notification_directory.define_notification_set_store($1, $2)
);
$_$;


ALTER FUNCTION notification_directory.create_notification_set_store(name name, notification_store_id integer) OWNER TO postgres;

--
-- Name: create_notification_store(integer); Type: FUNCTION; Schema: notification_directory; Owner: postgres
--

CREATE FUNCTION notification_directory.create_notification_store(data_source_id integer) RETURNS notification_directory.notification_store
    LANGUAGE sql
    AS $_$
SELECT notification_directory.create_notification_store(
    $1, ARRAY[]::notification_directory.attr_def[]
);
$_$;


ALTER FUNCTION notification_directory.create_notification_store(data_source_id integer) OWNER TO postgres;

--
-- Name: create_notification_store(text); Type: FUNCTION; Schema: notification_directory; Owner: postgres
--

CREATE FUNCTION notification_directory.create_notification_store(data_source_name text) RETURNS notification_directory.notification_store
    LANGUAGE sql
    AS $_$
SELECT notification_directory.create_notification_store(
    (directory.name_to_data_source($1)).id
);
$_$;


ALTER FUNCTION notification_directory.create_notification_store(data_source_name text) OWNER TO postgres;

--
-- Name: create_notification_store(integer, notification_directory.attr_def[]); Type: FUNCTION; Schema: notification_directory; Owner: postgres
--

CREATE FUNCTION notification_directory.create_notification_store(data_source_id integer, notification_directory.attr_def[]) RETURNS notification_directory.notification_store
    LANGUAGE sql
    AS $_$
SELECT notification_directory.initialize_notification_store(
    notification_directory.define_notification_store($1, $2)
);
$_$;


ALTER FUNCTION notification_directory.create_notification_store(data_source_id integer, notification_directory.attr_def[]) OWNER TO postgres;

--
-- Name: create_notification_store(text, notification_directory.attr_def[]); Type: FUNCTION; Schema: notification_directory; Owner: postgres
--

CREATE FUNCTION notification_directory.create_notification_store(data_source_name text, notification_directory.attr_def[]) RETURNS notification_directory.notification_store
    LANGUAGE sql
    AS $_$
SELECT notification_directory.create_notification_store(
    (directory.name_to_data_source($1)).id, $2
);
$_$;


ALTER FUNCTION notification_directory.create_notification_store(data_source_name text, notification_directory.attr_def[]) OWNER TO postgres;

--
-- Name: create_staging_table(notification_directory.notification_store); Type: FUNCTION; Schema: notification_directory; Owner: postgres
--

CREATE FUNCTION notification_directory.create_staging_table(notification_directory.notification_store) RETURNS notification_directory.notification_store
    LANGUAGE sql
    AS $_$
SELECT public.action($1, notification_directory.create_staging_table_sql($1));
$_$;


ALTER FUNCTION notification_directory.create_staging_table(notification_directory.notification_store) OWNER TO postgres;

--
-- Name: create_staging_table_sql(notification_directory.notification_store); Type: FUNCTION; Schema: notification_directory; Owner: postgres
--

CREATE FUNCTION notification_directory.create_staging_table_sql(notification_directory.notification_store) RETURNS text[]
    LANGUAGE sql STABLE
    AS $_$
SELECT ARRAY[
        format(
            'CREATE TABLE %I.%I ('
            '  entity_id integer NOT NULL,'
            '  "timestamp" timestamp with time zone NOT NULL'
            '  %s'
            ');',
            notification_directory.notification_store_schema(),
            notification_directory.staging_table_name($1),
            (SELECT array_to_string(array_agg(format(',%I %s', name, data_type)), ' ') FROM notification_directory.attribute WHERE notification_store_id = $1.id)
        ),
        format(
            'ALTER TABLE %I.%I OWNER TO minerva_writer;',
            notification_directory.notification_store_schema(),
            notification_directory.staging_table_name($1)
        )
    ];
$_$;


ALTER FUNCTION notification_directory.create_staging_table_sql(notification_directory.notification_store) OWNER TO postgres;

--
-- Name: create_table(notification_directory.notification_store); Type: FUNCTION; Schema: notification_directory; Owner: postgres
--

CREATE FUNCTION notification_directory.create_table(notification_directory.notification_store) RETURNS notification_directory.notification_store
    LANGUAGE sql
    AS $_$
SELECT public.action($1, notification_directory.create_table_sql($1));
$_$;


ALTER FUNCTION notification_directory.create_table(notification_directory.notification_store) OWNER TO postgres;

--
-- Name: create_table_sql(notification_directory.notification_store); Type: FUNCTION; Schema: notification_directory; Owner: postgres
--

CREATE FUNCTION notification_directory.create_table_sql(notification_directory.notification_store) RETURNS text[]
    LANGUAGE sql STABLE
    AS $_$
SELECT ARRAY[
        format(
            'CREATE TABLE %I.%I ('
            '  id serial PRIMARY KEY,'
            '  entity_id integer NOT NULL,'
            '  "timestamp" timestamp with time zone NOT NULL'
            '  %s'
            ');',
            notification_directory.notification_store_schema(),
            notification_directory.table_name($1),
            (SELECT array_to_string(array_agg(format(',%I %s', name, data_type)), ' ') FROM notification_directory.attribute WHERE notification_store_id = $1.id)
        ),
        format(
            'ALTER TABLE %I.%I OWNER TO minerva_writer;',
            notification_directory.notification_store_schema(),
            notification_directory.table_name($1)
        ),
        format(
            'CREATE INDEX %I ON %I.%I USING btree (timestamp);',
            'idx_notification_' || notification_directory.table_name($1) || '_timestamp',
            notification_directory.notification_store_schema(),
            notification_directory.table_name($1)
        )
    ];
$_$;


ALTER FUNCTION notification_directory.create_table_sql(notification_directory.notification_store) OWNER TO postgres;

--
-- Name: define_attribute(notification_directory.notification_store, name, name, text); Type: FUNCTION; Schema: notification_directory; Owner: postgres
--

CREATE FUNCTION notification_directory.define_attribute(notification_directory.notification_store, name, name, text) RETURNS SETOF notification_directory.attribute
    LANGUAGE sql
    AS $_$
INSERT INTO notification_directory.attribute(notification_store_id, name, data_type, description)
VALUES($1.id, $2, $3, $4) RETURNING *;
$_$;


ALTER FUNCTION notification_directory.define_attribute(notification_directory.notification_store, name, name, text) OWNER TO postgres;

--
-- Name: define_attributes(notification_directory.notification_store, notification_directory.attr_def[]); Type: FUNCTION; Schema: notification_directory; Owner: postgres
--

CREATE FUNCTION notification_directory.define_attributes(notification_directory.notification_store, notification_directory.attr_def[]) RETURNS notification_directory.notification_store
    LANGUAGE sql
    AS $_$
SELECT notification_directory.define_attribute($1, name, data_type, description)
FROM unnest($2);

SELECT $1;
$_$;


ALTER FUNCTION notification_directory.define_attributes(notification_directory.notification_store, notification_directory.attr_def[]) OWNER TO postgres;

--
-- Name: define_notification_set_store(name, integer); Type: FUNCTION; Schema: notification_directory; Owner: postgres
--

CREATE FUNCTION notification_directory.define_notification_set_store(name name, notification_store_id integer) RETURNS notification_directory.notification_set_store
    LANGUAGE sql
    AS $_$
INSERT INTO notification_directory.notification_set_store(name, notification_store_id)
VALUES ($1, $2)
RETURNING *;
$_$;


ALTER FUNCTION notification_directory.define_notification_set_store(name name, notification_store_id integer) OWNER TO postgres;

--
-- Name: define_notification_store(integer); Type: FUNCTION; Schema: notification_directory; Owner: postgres
--

CREATE FUNCTION notification_directory.define_notification_store(data_source_id integer) RETURNS notification_directory.notification_store
    LANGUAGE sql
    AS $_$
INSERT INTO notification_directory.notification_store(data_source_id)
VALUES ($1)
RETURNING *;
$_$;


ALTER FUNCTION notification_directory.define_notification_store(data_source_id integer) OWNER TO postgres;

--
-- Name: define_notification_store(integer, notification_directory.attr_def[]); Type: FUNCTION; Schema: notification_directory; Owner: postgres
--

CREATE FUNCTION notification_directory.define_notification_store(data_source_id integer, notification_directory.attr_def[]) RETURNS notification_directory.notification_store
    LANGUAGE sql
    AS $_$
SELECT notification_directory.define_attributes(
    notification_directory.define_notification_store($1),
    $2
);
$_$;


ALTER FUNCTION notification_directory.define_notification_store(data_source_id integer, notification_directory.attr_def[]) OWNER TO postgres;

--
-- Name: delete_notification_set_store(notification_directory.notification_set_store); Type: FUNCTION; Schema: notification_directory; Owner: postgres
--

CREATE FUNCTION notification_directory.delete_notification_set_store(notification_directory.notification_set_store) RETURNS void
    LANGUAGE plpgsql
    AS $_$
BEGIN
    EXECUTE format(
        'DROP TABLE IF EXISTS %I.%I',
        notification_directory.notification_store_schema(),
        $1.name || '_link'
    );

    EXECUTE format(
        'DROP TABLE IF EXISTS %I.%I',
        notification_directory.notification_store_schema(),
        $1.name
    );

    DELETE FROM notification_directory.notification_set_store WHERE id = $1.id;
END;
$_$;


ALTER FUNCTION notification_directory.delete_notification_set_store(notification_directory.notification_set_store) OWNER TO postgres;

--
-- Name: delete_notification_store(notification_directory.notification_store); Type: FUNCTION; Schema: notification_directory; Owner: postgres
--

CREATE FUNCTION notification_directory.delete_notification_store(notification_directory.notification_store) RETURNS void
    LANGUAGE plpgsql
    AS $_$
BEGIN
    EXECUTE format(
        'DROP TABLE IF EXISTS %I.%I CASCADE',
        notification_directory.notification_store_schema(),
        notification_directory.staging_table_name($1)
    );

    EXECUTE format(
        'DROP TABLE IF EXISTS %I.%I CASCADE',
        notification_directory.notification_store_schema(),
        notification_directory.table_name($1)
    );

    DELETE FROM notification_directory.notification_store WHERE id = $1.id;
END;
$_$;


ALTER FUNCTION notification_directory.delete_notification_store(notification_directory.notification_store) OWNER TO postgres;

--
-- Name: drop_staging_table(notification_directory.notification_store); Type: FUNCTION; Schema: notification_directory; Owner: postgres
--

CREATE FUNCTION notification_directory.drop_staging_table(notification_directory.notification_store) RETURNS notification_directory.notification_store
    LANGUAGE sql
    AS $_$
SELECT public.action($1, notification_directory.drop_staging_table_sql($1));
$_$;


ALTER FUNCTION notification_directory.drop_staging_table(notification_directory.notification_store) OWNER TO postgres;

--
-- Name: drop_staging_table_sql(notification_directory.notification_store); Type: FUNCTION; Schema: notification_directory; Owner: postgres
--

CREATE FUNCTION notification_directory.drop_staging_table_sql(notification_directory.notification_store) RETURNS text
    LANGUAGE sql STABLE
    AS $_$
SELECT format(
    'DROP TABLE %I.%I',
    notification_directory.notification_store_schema(),
    notification_directory.staging_table_name($1)
);
$_$;


ALTER FUNCTION notification_directory.drop_staging_table_sql(notification_directory.notification_store) OWNER TO postgres;

--
-- Name: get_attr_defs(notification_directory.notification_store); Type: FUNCTION; Schema: notification_directory; Owner: postgres
--

CREATE FUNCTION notification_directory.get_attr_defs(notification_directory.notification_store) RETURNS SETOF notification_directory.attr_def
    LANGUAGE sql STABLE
    AS $_$
SELECT (attname, typname, '')::notification_directory.attr_def
FROM pg_type
JOIN pg_attribute ON pg_attribute.atttypid = pg_type.oid
JOIN pg_class ON pg_class.oid = pg_attribute.attrelid
JOIN pg_namespace ON pg_namespace.oid = pg_class.relnamespace
WHERE
nspname = notification_directory.notification_store_schema() AND
relname = notification_directory.table_name($1) AND
attnum > 0 AND
NOT attname IN ('id', 'entity_id', 'timestamp') AND
NOT pg_attribute.attisdropped;
$_$;


ALTER FUNCTION notification_directory.get_attr_defs(notification_directory.notification_store) OWNER TO postgres;

--
-- Name: get_column_type_name(notification_directory.notification_store, name); Type: FUNCTION; Schema: notification_directory; Owner: postgres
--

CREATE FUNCTION notification_directory.get_column_type_name(notification_directory.notification_store, name) RETURNS name
    LANGUAGE sql STABLE
    AS $_$
SELECT notification_directory.get_column_type_name(
    notification_directory.notification_store_schema(),
    notification_directory.table_name($1),
    $2
);
$_$;


ALTER FUNCTION notification_directory.get_column_type_name(notification_directory.notification_store, name) OWNER TO postgres;

--
-- Name: get_column_type_name(name, name, name); Type: FUNCTION; Schema: notification_directory; Owner: postgres
--

CREATE FUNCTION notification_directory.get_column_type_name(namespace_name name, table_name name, column_name name) RETURNS name
    LANGUAGE sql STABLE
    AS $_$
SELECT typname
FROM pg_type
JOIN pg_attribute ON pg_attribute.atttypid = pg_type.oid
JOIN pg_class ON pg_class.oid = pg_attribute.attrelid
JOIN pg_namespace ON pg_namespace.oid = pg_class.relnamespace
WHERE nspname = $1 AND relname = $2 AND attname = $3 AND attnum > 0 AND not pg_attribute.attisdropped;
$_$;


ALTER FUNCTION notification_directory.get_column_type_name(namespace_name name, table_name name, column_name name) OWNER TO postgres;

--
-- Name: get_last_notification(text); Type: FUNCTION; Schema: notification_directory; Owner: postgres
--

CREATE FUNCTION notification_directory.get_last_notification(client text) RETURNS integer
    LANGUAGE sql STABLE
    AS $_$
SELECT notification_directory.get_last_notification($1, $1);
$_$;


ALTER FUNCTION notification_directory.get_last_notification(client text) OWNER TO postgres;

--
-- Name: get_last_notification(text, text); Type: FUNCTION; Schema: notification_directory; Owner: postgres
--

CREATE FUNCTION notification_directory.get_last_notification(client text, notification_store text) RETURNS integer
    LANGUAGE plpgsql STABLE
    AS $_$
DECLARE
  result integer;
BEGIN
  SELECT ln.last_notification FROM notification_directory.last_notification ln WHERE ln.name = $1 AND ln.notification_store = $2 INTO result;
  RETURN COALESCE(result, -1);
END;
$_$;


ALTER FUNCTION notification_directory.get_last_notification(client text, notification_store text) OWNER TO postgres;

--
-- Name: get_last_notifications(text, integer); Type: FUNCTION; Schema: notification_directory; Owner: postgres
--

CREATE FUNCTION notification_directory.get_last_notifications(notification_store text, max_notifications integer) RETURNS SETOF notification.generic_notification
    LANGUAGE plpgsql STABLE
    AS $_$
DECLARE
  entity_type text;
BEGIN
  SELECT et.name FROM notification_directory.notification_store ns
    JOIN directory.data_source ds ON ds.id = ns.data_source_id
    JOIN directory.entity_type et ON et.id = ns.entity_type_id
    WHERE ds.name = $1
    INTO entity_type;
  RETURN QUERY EXECUTE(FORMAT(
    'SELECT n.id as id, timestamp, r.name::text as rule, e.name::text as entity, weight, details, data '
    'FROM notification.%I n '
    'JOIN trigger.rule r ON n.rule_id = r.id '
    'JOIN entity.%I e on n.entity_id = e.id '
    'ORDER BY n.id DESC LIMIT %s',
    $1,
    entity_type,
    $2
  ));
END;
$_$;


ALTER FUNCTION notification_directory.get_last_notifications(notification_store text, max_notifications integer) OWNER TO postgres;

--
-- Name: get_next_notifications(text, integer, integer); Type: FUNCTION; Schema: notification_directory; Owner: postgres
--

CREATE FUNCTION notification_directory.get_next_notifications(notification_store text, last_notification_seen integer, max_notifications integer) RETURNS SETOF notification.generic_notification
    LANGUAGE plpgsql STABLE
    AS $_$
DECLARE
  entity_type text;
BEGIN
  SELECT et.name FROM notification_directory.notification_store ns
    JOIN directory.data_source ds ON ds.id = ns.data_source_id
    JOIN directory.entity_type et ON et.id = ns.entity_type_id
    WHERE ds.name = $1
    INTO entity_type;
  RETURN QUERY EXECUTE(FORMAT(
    'SELECT n.id as id, timestamp, r.name::text as rule, e.name::text as entity, weight, details, data '
    'FROM notification.%I n '
    'JOIN trigger.rule r ON n.rule_id = r.id '
    'JOIN entity.%I e on n.entity_id = e.id '
    'WHERE n.id > %s ORDER BY n.id  LIMIT %s',
    $1,
    entity_type,
    $2,
    $3
  ));
END;
$_$;


ALTER FUNCTION notification_directory.get_next_notifications(notification_store text, last_notification_seen integer, max_notifications integer) OWNER TO postgres;

--
-- Name: get_notification_store(name); Type: FUNCTION; Schema: notification_directory; Owner: postgres
--

CREATE FUNCTION notification_directory.get_notification_store(data_source_name name) RETURNS notification_directory.notification_store
    LANGUAGE sql STABLE
    AS $_$
SELECT ns
FROM notification_directory.notification_store ns
JOIN directory.data_source ds ON ds.id = ns.data_source_id
WHERE ds.name = $1;
$_$;


ALTER FUNCTION notification_directory.get_notification_store(data_source_name name) OWNER TO postgres;

--
-- Name: init_notification_set_store(notification_directory.notification_set_store); Type: FUNCTION; Schema: notification_directory; Owner: postgres
--

CREATE FUNCTION notification_directory.init_notification_set_store(notification_directory.notification_set_store) RETURNS notification_directory.notification_set_store
    LANGUAGE plpgsql
    AS $_$
BEGIN
    EXECUTE format(
        'CREATE TABLE %I.%I('
        '  id serial PRIMARY KEY'
        ')',
        notification_directory.notification_store_schema(),
        $1.name
    );

    EXECUTE format(
        'CREATE TABLE %I.%I('
        '  notification_id integer REFERENCES %I.%I ON DELETE CASCADE,'
        '  set_id integer REFERENCES %I.%I ON DELETE CASCADE'
        ')',
        notification_directory.notification_store_schema(),
        $1.name || '_link',
        notification_directory.notification_store_schema(),
        notification_directory.table_name(notification_directory.notification_store($1)),
        notification_directory.notification_store_schema(),
        $1.name
    );

    RETURN $1;
END;
$_$;


ALTER FUNCTION notification_directory.init_notification_set_store(notification_directory.notification_set_store) OWNER TO postgres;

--
-- Name: initialize_notification_store(notification_directory.notification_store); Type: FUNCTION; Schema: notification_directory; Owner: postgres
--

CREATE FUNCTION notification_directory.initialize_notification_store(notification_directory.notification_store) RETURNS notification_directory.notification_store
    LANGUAGE sql
    AS $_$
SELECT notification_directory.create_table($1);
SELECT notification_directory.create_staging_table($1);
$_$;


ALTER FUNCTION notification_directory.initialize_notification_store(notification_directory.notification_store) OWNER TO postgres;

--
-- Name: notification_store(notification_directory.notification_set_store); Type: FUNCTION; Schema: notification_directory; Owner: postgres
--

CREATE FUNCTION notification_directory.notification_store(notification_directory.notification_set_store) RETURNS notification_directory.notification_store
    LANGUAGE sql STABLE
    AS $_$
SELECT notification_store FROM notification_directory.notification_store WHERE id = $1.notification_store_id;
$_$;


ALTER FUNCTION notification_directory.notification_store(notification_directory.notification_set_store) OWNER TO postgres;

--
-- Name: notification_store_schema(); Type: FUNCTION; Schema: notification_directory; Owner: postgres
--

CREATE FUNCTION notification_directory.notification_store_schema() RETURNS name
    LANGUAGE sql IMMUTABLE
    AS $$
SELECT 'notification'::name;
$$;


ALTER FUNCTION notification_directory.notification_store_schema() OWNER TO postgres;

--
-- Name: notification_store_to_char(integer); Type: FUNCTION; Schema: notification_directory; Owner: postgres
--

CREATE FUNCTION notification_directory.notification_store_to_char(notification_store_id integer) RETURNS text
    LANGUAGE sql STABLE STRICT
    AS $_$
SELECT data_source.name
  FROM notification_directory.notification_store
    JOIN directory.data_source ON data_source.id = notification_store.data_source_id
  WHERE notification_store.id = $1;
$_$;


ALTER FUNCTION notification_directory.notification_store_to_char(notification_store_id integer) OWNER TO postgres;

--
-- Name: set_last_notification(text, integer); Type: FUNCTION; Schema: notification_directory; Owner: postgres
--

CREATE FUNCTION notification_directory.set_last_notification(client text, value integer) RETURNS void
    LANGUAGE sql
    AS $_$
SELECT notification_directory.set_last_notification($1, $1, $2);
$_$;


ALTER FUNCTION notification_directory.set_last_notification(client text, value integer) OWNER TO postgres;

--
-- Name: set_last_notification(text, text, integer); Type: FUNCTION; Schema: notification_directory; Owner: postgres
--

CREATE FUNCTION notification_directory.set_last_notification(client text, notification_store text, value integer) RETURNS void
    LANGUAGE sql
    AS $_$
INSERT INTO notification_directory.last_notification (name, notification_store, last_notification)
  VALUES ($1, $2, $3) ON CONFLICT (name, notification_store) DO UPDATE SET last_notification = $3;
$_$;


ALTER FUNCTION notification_directory.set_last_notification(client text, notification_store text, value integer) OWNER TO postgres;

--
-- Name: staging_table_name(notification_directory.notification_store); Type: FUNCTION; Schema: notification_directory; Owner: postgres
--

CREATE FUNCTION notification_directory.staging_table_name(notification_directory.notification_store) RETURNS name
    LANGUAGE sql STABLE
    AS $_$
SELECT (notification_directory.table_name($1) || '_staging')::name;
$_$;


ALTER FUNCTION notification_directory.staging_table_name(notification_directory.notification_store) OWNER TO postgres;

--
-- Name: table_name(notification_directory.notification_store); Type: FUNCTION; Schema: notification_directory; Owner: postgres
--

CREATE FUNCTION notification_directory.table_name(notification_directory.notification_store) RETURNS name
    LANGUAGE sql STABLE
    AS $_$
SELECT name::name
FROM directory.data_source
WHERE id = $1.data_source_id;
$_$;


ALTER FUNCTION notification_directory.table_name(notification_directory.notification_store) OWNER TO postgres;

--
-- Name: action(text); Type: FUNCTION; Schema: public; Owner: postgres
--

CREATE FUNCTION public.action(sql text) RETURNS void
    LANGUAGE plpgsql
    AS $$
BEGIN
    EXECUTE sql;
END;
$$;


ALTER FUNCTION public.action(sql text) OWNER TO postgres;

--
-- Name: action(anyelement, text[]); Type: FUNCTION; Schema: public; Owner: postgres
--

CREATE FUNCTION public.action(anyelement, sql text[]) RETURNS anyelement
    LANGUAGE plpgsql
    AS $_$
DECLARE
    statement text;
BEGIN
    FOREACH statement IN ARRAY sql LOOP
        EXECUTE statement;
    END LOOP;

    RETURN $1;
END;
$_$;


ALTER FUNCTION public.action(anyelement, sql text[]) OWNER TO postgres;

--
-- Name: action(anyelement, text); Type: FUNCTION; Schema: public; Owner: postgres
--

CREATE FUNCTION public.action(anyelement, sql text) RETURNS anyelement
    LANGUAGE plpgsql
    AS $_$
BEGIN
    EXECUTE sql;

    RETURN $1;
END;
$_$;


ALTER FUNCTION public.action(anyelement, sql text) OWNER TO postgres;

--
-- Name: action_count(text); Type: FUNCTION; Schema: public; Owner: postgres
--

CREATE FUNCTION public.action_count(sql text) RETURNS integer
    LANGUAGE plpgsql
    AS $$
DECLARE
    row_count integer;
BEGIN
    EXECUTE sql;

    GET DIAGNOSTICS row_count = ROW_COUNT;

    RETURN row_count;
END;
$$;


ALTER FUNCTION public.action_count(sql text) OWNER TO postgres;

--
-- Name: add_array(anyarray, anyarray); Type: FUNCTION; Schema: public; Owner: postgres
--

CREATE FUNCTION public.add_array(anyarray, anyarray) RETURNS anyarray
    LANGUAGE sql STABLE STRICT
    AS $_$
SELECT array_agg((arr1 + arr2)) FROM
(
    SELECT
        unnest($1[1:least(array_length($1,1), array_length($2,1))]) AS arr1,
        unnest($2[1:least(array_length($1,1), array_length($2,1))]) AS arr2
) AS foo;
$_$;


ALTER FUNCTION public.add_array(anyarray, anyarray) OWNER TO postgres;

--
-- Name: array_sum(anyarray); Type: FUNCTION; Schema: public; Owner: postgres
--

CREATE FUNCTION public.array_sum(anyarray) RETURNS anyelement
    LANGUAGE sql IMMUTABLE STRICT
    AS $_$
SELECT sum(t) FROM unnest($1) t;
$_$;


ALTER FUNCTION public.array_sum(anyarray) OWNER TO postgres;

--
-- Name: array_to_text(anyarray); Type: FUNCTION; Schema: public; Owner: postgres
--

CREATE FUNCTION public.array_to_text(anyarray) RETURNS text
    LANGUAGE sql IMMUTABLE STRICT
    AS $_$
SELECT array_to_string($1, ',')
$_$;


ALTER FUNCTION public.array_to_text(anyarray) OWNER TO postgres;

--
-- Name: column_names(name, name); Type: FUNCTION; Schema: public; Owner: postgres
--

CREATE FUNCTION public.column_names(namespace name, "table" name) RETURNS SETOF name
    LANGUAGE sql STABLE
    AS $_$
SELECT a.attname
FROM pg_catalog.pg_class c
JOIN pg_catalog.pg_namespace n ON c.relnamespace = n.oid
JOIN pg_catalog.pg_attribute a ON a.attrelid = c.oid
WHERE
    n.nspname = $1 AND
    c.relname = $2 AND
    a.attisdropped = false AND
    a.attnum > 0;
$_$;


ALTER FUNCTION public.column_names(namespace name, "table" name) OWNER TO postgres;

--
-- Name: divide_array(anyarray, anyarray); Type: FUNCTION; Schema: public; Owner: postgres
--

CREATE FUNCTION public.divide_array(anyarray, anyarray) RETURNS anyarray
    LANGUAGE sql STABLE STRICT
    AS $_$
SELECT array_agg(public.safe_division(arr1, arr2)) FROM
(
    SELECT
    unnest($1[1:least(array_length($1,1), array_length($2,1))]) AS arr1,
    unnest($2[1:least(array_length($1,1), array_length($2,1))]) AS arr2
) AS foo;
$_$;


ALTER FUNCTION public.divide_array(anyarray, anyarray) OWNER TO postgres;

--
-- Name: divide_array(anyarray, anyelement); Type: FUNCTION; Schema: public; Owner: postgres
--

CREATE FUNCTION public.divide_array(anyarray, anyelement) RETURNS anyarray
    LANGUAGE sql STABLE STRICT
    AS $_$
SELECT array_agg(arr / $2) FROM
(
    SELECT unnest($1) AS arr
) AS foo;
$_$;


ALTER FUNCTION public.divide_array(anyarray, anyelement) OWNER TO postgres;

--
-- Name: fst(anyelement, anyelement); Type: FUNCTION; Schema: public; Owner: postgres
--

CREATE FUNCTION public.fst(anyelement, anyelement) RETURNS anyelement
    LANGUAGE sql IMMUTABLE STRICT
    AS $_$
SELECT $1;
$_$;


ALTER FUNCTION public.fst(anyelement, anyelement) OWNER TO postgres;

--
-- Name: integer_to_array(integer); Type: FUNCTION; Schema: public; Owner: postgres
--

CREATE FUNCTION public.integer_to_array(value integer) RETURNS integer[]
    LANGUAGE plpgsql STABLE STRICT
    AS $$
BEGIN
    RETURN ARRAY[value];
END;
$$;


ALTER FUNCTION public.integer_to_array(value integer) OWNER TO postgres;

--
-- Name: raise_exception(anyelement); Type: FUNCTION; Schema: public; Owner: postgres
--

CREATE FUNCTION public.raise_exception(message anyelement) RETURNS void
    LANGUAGE plpgsql
    AS $$
BEGIN
    RAISE EXCEPTION '%', message;
END;
$$;


ALTER FUNCTION public.raise_exception(message anyelement) OWNER TO postgres;

--
-- Name: raise_info(anyelement); Type: FUNCTION; Schema: public; Owner: postgres
--

CREATE FUNCTION public.raise_info(message anyelement) RETURNS void
    LANGUAGE plpgsql
    AS $$
BEGIN
    RAISE INFO '%', message;
END;
$$;


ALTER FUNCTION public.raise_info(message anyelement) OWNER TO postgres;

--
-- Name: safe_division(anyelement, anyelement); Type: FUNCTION; Schema: public; Owner: postgres
--

CREATE FUNCTION public.safe_division(numerator anyelement, denominator anyelement) RETURNS anyelement
    LANGUAGE sql IMMUTABLE
    AS $_$
SELECT CASE
    WHEN $2 = 0 THEN
        NULL
    ELSE
        $1 / $2
    END;
$_$;


ALTER FUNCTION public.safe_division(numerator anyelement, denominator anyelement) OWNER TO postgres;

--
-- Name: safe_division(anyelement, anyelement, anyelement); Type: FUNCTION; Schema: public; Owner: postgres
--

CREATE FUNCTION public.safe_division(numerator anyelement, denominator anyelement, division_by_zero_indicator anyelement) RETURNS anyelement
    LANGUAGE sql IMMUTABLE
    AS $_$
SELECT CASE
    WHEN $2 = 0 THEN
        $3
    ELSE
        $1 / $2
    END;
$_$;


ALTER FUNCTION public.safe_division(numerator anyelement, denominator anyelement, division_by_zero_indicator anyelement) OWNER TO postgres;

--
-- Name: smallint_to_array(smallint); Type: FUNCTION; Schema: public; Owner: postgres
--

CREATE FUNCTION public.smallint_to_array(value smallint) RETURNS smallint[]
    LANGUAGE plpgsql STABLE STRICT
    AS $$
BEGIN
    RETURN ARRAY[value];
END;
$$;


ALTER FUNCTION public.smallint_to_array(value smallint) OWNER TO postgres;

--
-- Name: smallint_to_timestamp_with_time_zone(smallint); Type: FUNCTION; Schema: public; Owner: postgres
--

CREATE FUNCTION public.smallint_to_timestamp_with_time_zone(smallint) RETURNS timestamp with time zone
    LANGUAGE plpgsql STABLE STRICT
    AS $$
BEGIN
    RETURN NULL;
END;
$$;


ALTER FUNCTION public.smallint_to_timestamp_with_time_zone(smallint) OWNER TO postgres;

--
-- Name: smallint_to_timestamp_without_time_zone(smallint); Type: FUNCTION; Schema: public; Owner: postgres
--

CREATE FUNCTION public.smallint_to_timestamp_without_time_zone(smallint) RETURNS timestamp without time zone
    LANGUAGE plpgsql STABLE STRICT
    AS $$
BEGIN
    RETURN NULL;
END;
$$;


ALTER FUNCTION public.smallint_to_timestamp_without_time_zone(smallint) OWNER TO postgres;

--
-- Name: snd(anyelement, anyelement); Type: FUNCTION; Schema: public; Owner: postgres
--

CREATE FUNCTION public.snd(anyelement, anyelement) RETURNS anyelement
    LANGUAGE sql IMMUTABLE STRICT
    AS $_$
SELECT $2;
$_$;


ALTER FUNCTION public.snd(anyelement, anyelement) OWNER TO postgres;

--
-- Name: switch_off_citus(); Type: FUNCTION; Schema: public; Owner: postgres
--

CREATE FUNCTION public.switch_off_citus() RETURNS void
    LANGUAGE sql
    AS $_$
CREATE OR REPLACE FUNCTION create_distributed_table(text, text) RETURNS VOID AS $$ SELECT 42; $$ LANGUAGE sql STABLE;
CREATE OR REPLACE FUNCTION create_reference_table(text) RETURNS VOID AS $$ SELECT 42; $$ LANGUAGE sql STABLE;
CREATE OR REPLACE FUNCTION create_distributed_function(text) RETURNS VOID AS $$ SELECT 42; $$ LANGUAGE sql STABLE;
CREATE OR REPLACE FUNCTION run_command_on_workers(text) RETURNS VOID AS $$ SELECT 42; $$ LANGUAGE sql STABLE;
$_$;


ALTER FUNCTION public.switch_off_citus() OWNER TO postgres;

--
-- Name: table_exists(name, name); Type: FUNCTION; Schema: public; Owner: postgres
--

CREATE FUNCTION public.table_exists(schema_name name, table_name name) RETURNS boolean
    LANGUAGE sql STABLE
    AS $_$
SELECT exists(
    SELECT 1
    FROM pg_class
    JOIN pg_namespace ON pg_class.relnamespace = pg_namespace.oid
    WHERE relname = $2 AND relkind = 'r' AND pg_namespace.nspname = $1
);
$_$;


ALTER FUNCTION public.table_exists(schema_name name, table_name name) OWNER TO postgres;

--
-- Name: to_pdf(text); Type: FUNCTION; Schema: public; Owner: postgres
--

CREATE FUNCTION public.to_pdf(text) RETURNS integer[]
    LANGUAGE sql STABLE STRICT
    AS $_$
SELECT array_agg(nullif(x, '')::int)
FROM unnest(string_to_array($1, ',')) AS x;
$_$;


ALTER FUNCTION public.to_pdf(text) OWNER TO postgres;

--
-- Name: add_entities_to_set(integer, text[]); Type: FUNCTION; Schema: relation_directory; Owner: postgres
--

CREATE FUNCTION relation_directory.add_entities_to_set(minerva_entity_set_id integer, entities text[]) RETURNS void
    LANGUAGE sql
    AS $_$
SELECT relation_directory.add_entity_to_set($1, e) FROM unnest($2) e;
$_$;


ALTER FUNCTION relation_directory.add_entities_to_set(minerva_entity_set_id integer, entities text[]) OWNER TO postgres;

--
-- Name: minerva_entity_set_curr_ptr; Type: TABLE; Schema: attribute_history; Owner: minerva_writer
--

CREATE TABLE attribute_history.minerva_entity_set_curr_ptr (
    id integer NOT NULL
);


ALTER TABLE attribute_history.minerva_entity_set_curr_ptr OWNER TO minerva_writer;

--
-- Name: minerva_entity_set; Type: VIEW; Schema: attribute; Owner: minerva_writer
--

CREATE VIEW attribute.minerva_entity_set AS
 SELECT h.id,
    h.first_appearance,
    h.modified,
    h.hash,
    h.entity_id,
    h."timestamp",
    h."end",
    h.name,
    h.fullname,
    h."group",
    h.source_entity_type,
    h.owner,
    h.description,
    h.last_update
   FROM (attribute_history.minerva_entity_set h
     JOIN attribute_history.minerva_entity_set_curr_ptr c ON ((h.id = c.id)));


ALTER VIEW attribute.minerva_entity_set OWNER TO minerva_writer;

--
-- Name: add_entity_to_set(integer, text); Type: FUNCTION; Schema: relation_directory; Owner: postgres
--

CREATE FUNCTION relation_directory.add_entity_to_set(minerva_entity_set_id integer, entity text) RETURNS attribute.minerva_entity_set
    LANGUAGE plpgsql
    AS $_$
DECLARE
  set attribute.minerva_entity_set;
BEGIN
  SELECT * FROM attribute.minerva_entity_set WHERE entity_id = $1 INTO set;
  PERFORM relation_directory.update_entity_set_attributes($1);
  EXECUTE FORMAT(
    'INSERT INTO relation.%I (source_id, target_id) '
    'SELECT source.id AS source_id, $1 AS target '
    'FROM entity.%I source '
    'WHERE source.name = $2 '
    'ON CONFLICT DO NOTHING;',
    set.source_entity_type || '->entity_set',
    set.source_entity_type
  ) USING set.entity_id, $2;
  RETURN set;
END;
$_$;


ALTER FUNCTION relation_directory.add_entity_to_set(minerva_entity_set_id integer, entity text) OWNER TO postgres;

--
-- Name: change_set_entities(integer, text[]); Type: FUNCTION; Schema: relation_directory; Owner: postgres
--

CREATE FUNCTION relation_directory.change_set_entities(minerva_entity_set_id integer, entities text[]) RETURNS void
    LANGUAGE plpgsql
    AS $_$
DECLARE
  set attribute.minerva_entity_set;
BEGIN
  SELECT * FROM attribute.minerva_entity_set WHERE entity_id = $1 INTO set;
  PERFORM action(FORMAT(
    'DELETE FROM relation."%s->entity_set" '
    'WHERE target_id = %s;',
    set.source_entity_type,
    set.entity_id
  ));
  PERFORM relation_directory.add_entities_to_set($1, $2);
END;
$_$;


ALTER FUNCTION relation_directory.change_set_entities(minerva_entity_set_id integer, entities text[]) OWNER TO postgres;

--
-- Name: FUNCTION change_set_entities(minerva_entity_set_id integer, entities text[]); Type: COMMENT; Schema: relation_directory; Owner: postgres
--

COMMENT ON FUNCTION relation_directory.change_set_entities(minerva_entity_set_id integer, entities text[]) IS 'Set the entities in the set to exactly the specified entities';


--
-- Name: change_set_entities_guarded(integer, text[]); Type: FUNCTION; Schema: relation_directory; Owner: postgres
--

CREATE FUNCTION relation_directory.change_set_entities_guarded(minerva_entity_set_id integer, entities text[]) RETURNS text[]
    LANGUAGE plpgsql
    AS $_$
DECLARE
  set attribute.minerva_entity_set;
  entity text;
  real_entity text;
  result text[];
  newresult text[];
BEGIN
  SELECT * FROM attribute.minerva_entity_set WHERE entity_id = $1 INTO set;
  SELECT $2 INTO result;
  FOREACH entity IN ARRAY $2 LOOP
    EXECUTE FORMAT(
      'SELECT name FROM entity.%I WHERE name = $1;',
      set.source_entity_type
    ) INTO real_entity USING entity;
    SELECT array_remove(result, real_entity) INTO result;
  END LOOP;
  IF ARRAY_LENGTH(result, 1) IS NULL THEN
    PERFORM relation_directory.change_set_entities($1, $2);
  END IF;
  RETURN result;
END;
$_$;


ALTER FUNCTION relation_directory.change_set_entities_guarded(minerva_entity_set_id integer, entities text[]) OWNER TO postgres;

--
-- Name: FUNCTION change_set_entities_guarded(minerva_entity_set_id integer, entities text[]); Type: COMMENT; Schema: relation_directory; Owner: postgres
--

COMMENT ON FUNCTION relation_directory.change_set_entities_guarded(minerva_entity_set_id integer, entities text[]) IS 'Only sets the entities if all specified entities are actually valid.
Returns those entities that were invalid.';


--
-- Name: create_entity_set(text, text, text, text, text); Type: FUNCTION; Schema: relation_directory; Owner: postgres
--

CREATE FUNCTION relation_directory.create_entity_set(name text, "group" text, entity_type_name text, owner text, description text) RETURNS attribute.minerva_entity_set
    LANGUAGE plpgsql
    AS $_$
DECLARE
  entity_id integer;
BEGIN
  EXECUTE FORMAT(
    'CREATE TABLE IF NOT EXISTS relation."%s->entity_set"('
    'source_id integer, '
    'target_id integer, '
    'PRIMARY KEY (source_id, target_id));',
    entity_type_name
  );
  PERFORM relation_directory.name_to_type(entity_type_name || '->entity_set');
  SELECT id FROM entity.to_entity_set(name || '_' || "group" || '_' || owner) INTO entity_id;
  INSERT INTO attribute_staging.minerva_entity_set(
      entity_id, timestamp, name, fullname, "group", source_entity_type, owner, description, last_update
    ) VALUES (
      entity_id,
      now(),
      name,
      name || '_' || "group" || '_' || owner,
      "group",
      entity_type_name,
      owner,
      description,
      CURRENT_DATE::text
    );
  PERFORM attribute_directory.transfer_staged(attribute_directory.get_attribute_store('minerva', 'entity_set'));
  PERFORM attribute_directory.materialize_curr_ptr(attribute_directory.get_attribute_store('minerva', 'entity_set'));
  RETURN es FROM attribute.minerva_entity_set es WHERE es.name = $1 AND es.owner = $4;
END;
$_$;


ALTER FUNCTION relation_directory.create_entity_set(name text, "group" text, entity_type_name text, owner text, description text) OWNER TO postgres;

--
-- Name: create_entity_set_guarded(text, text, text, text, text, text[]); Type: FUNCTION; Schema: relation_directory; Owner: postgres
--

CREATE FUNCTION relation_directory.create_entity_set_guarded(name text, "group" text, entity_type_name text, owner text, description text, entities text[]) RETURNS text[]
    LANGUAGE plpgsql
    AS $_$
DECLARE
  entity text;
  real_entity text;
  result text[];
  newresult text[];
  entityset integer;
BEGIN
  SELECT $6 INTO result;
  FOREACH entity IN ARRAY $6 LOOP
    EXECUTE FORMAT(
      'SELECT name FROM entity.%I WHERE name = $1;',
      entity_type_name
    ) INTO real_entity USING entity;
    SELECT array_remove(result, real_entity) INTO result;
  END LOOP;
  IF ARRAY_LENGTH(result, 1) IS NULL THEN
    SELECT entity_id FROM relation_directory.create_entity_set($1, $2, $3, $4, $5) INTO entityset;
    PERFORM relation_directory.change_set_entities(entityset, $6);
  END IF;
  RETURN result;
END;
$_$;


ALTER FUNCTION relation_directory.create_entity_set_guarded(name text, "group" text, entity_type_name text, owner text, description text, entities text[]) OWNER TO postgres;

--
-- Name: entity_set_exists(text, text); Type: FUNCTION; Schema: relation_directory; Owner: postgres
--

CREATE FUNCTION relation_directory.entity_set_exists(owner text, name text) RETURNS boolean
    LANGUAGE plpgsql STABLE
    AS $_$
DECLARE
  row_count integer;
BEGIN
  SELECT action_count(format(
    'SELECT * FROM attribute.minerva_entity_set '
    'WHERE owner = %L AND name = %L;',
    $1,
    $2
  )) INTO row_count;
  RETURN CASE row_count
    WHEN 0 THEN false
    ELSE true
  END;
END;
$_$;


ALTER FUNCTION relation_directory.entity_set_exists(owner text, name text) OWNER TO postgres;

--
-- Name: get_entity_set_data(text, text); Type: FUNCTION; Schema: relation_directory; Owner: postgres
--

CREATE FUNCTION relation_directory.get_entity_set_data(name text, owner text) RETURNS attribute.minerva_entity_set
    LANGUAGE sql STABLE
    AS $_$
SELECT * FROM attribute.minerva_entity_set WHERE name = $1 AND owner = $2;
$_$;


ALTER FUNCTION relation_directory.get_entity_set_data(name text, owner text) OWNER TO postgres;

--
-- Name: get_entity_set_members(integer); Type: FUNCTION; Schema: relation_directory; Owner: postgres
--

CREATE FUNCTION relation_directory.get_entity_set_members(minerva_entity_set_id integer) RETURNS text[]
    LANGUAGE plpgsql STABLE
    AS $_$
DECLARE
  set attribute.minerva_entity_set;
  result text[];
BEGIN
  SELECT * FROM attribute.minerva_entity_set WHERE entity_id = $1 INTO set;
  EXECUTE FORMAT(
    'SELECT array_agg(e.name) '
    'FROM relation."%s->entity_set" es JOIN entity.%I e ON es.source_id = e.id '
    'WHERE es.target_id = %s',
    set.source_entity_type,
    set.source_entity_type,
    set.entity_id
  ) INTO result;
  RETURN result;
END;
$_$;


ALTER FUNCTION relation_directory.get_entity_set_members(minerva_entity_set_id integer) OWNER TO postgres;

--
-- Name: get_entity_set_members(text, text); Type: FUNCTION; Schema: relation_directory; Owner: postgres
--

CREATE FUNCTION relation_directory.get_entity_set_members(name text, owner text) RETURNS text[]
    LANGUAGE sql STABLE
    AS $_$
SELECT relation_directory.get_entity_set_members(es.entity_id)
  FROM attribute.minerva_entity_set es
  WHERE owner = $2 AND name = $1;
$_$;


ALTER FUNCTION relation_directory.get_entity_set_members(name text, owner text) OWNER TO postgres;

--
-- Name: type; Type: TABLE; Schema: relation_directory; Owner: postgres
--

CREATE TABLE relation_directory.type (
    id integer NOT NULL,
    name name NOT NULL,
    cardinality relation_directory.type_cardinality_enum
);


ALTER TABLE relation_directory.type OWNER TO postgres;

--
-- Name: get_type(name); Type: FUNCTION; Schema: relation_directory; Owner: postgres
--

CREATE FUNCTION relation_directory.get_type(name) RETURNS relation_directory.type
    LANGUAGE sql STABLE STRICT
    AS $_$
SELECT type FROM relation_directory.type WHERE name = $1;
$_$;


ALTER FUNCTION relation_directory.get_type(name) OWNER TO postgres;

--
-- Name: name_to_type(name); Type: FUNCTION; Schema: relation_directory; Owner: postgres
--

CREATE FUNCTION relation_directory.name_to_type(name) RETURNS relation_directory.type
    LANGUAGE sql STRICT
    AS $_$
SELECT COALESCE(
  relation_directory.get_type($1),
  relation_directory.register_type($1)
);
$_$;


ALTER FUNCTION relation_directory.name_to_type(name) OWNER TO postgres;

--
-- Name: register_type(name); Type: FUNCTION; Schema: relation_directory; Owner: postgres
--

CREATE FUNCTION relation_directory.register_type(name) RETURNS relation_directory.type
    LANGUAGE sql STRICT
    AS $_$
INSERT INTO relation_directory.type (name) VALUES ($1) RETURNING type;
$_$;


ALTER FUNCTION relation_directory.register_type(name) OWNER TO postgres;

--
-- Name: remove(name); Type: FUNCTION; Schema: relation_directory; Owner: postgres
--

CREATE FUNCTION relation_directory.remove(name) RETURNS text
    LANGUAGE plpgsql
    AS $_$
DECLARE
  result text;
BEGIN
  SELECT name FROM relation_directory.type WHERE name = $1 INTO result;
  PERFORM public.action(format('DROP TABLE IF EXISTS relation.%I', $1));
  PERFORM public.action(format('DROP FUNCTION IF EXISTS relation_directory.%I', format('materialize_%s', $1)));
  DELETE FROM relation_directory.type WHERE name = $1;
  RETURN result;
END;
$_$;


ALTER FUNCTION relation_directory.remove(name) OWNER TO postgres;

--
-- Name: remove_entity_from_set(integer, text); Type: FUNCTION; Schema: relation_directory; Owner: postgres
--

CREATE FUNCTION relation_directory.remove_entity_from_set(minerva_entity_set_id integer, entity text) RETURNS void
    LANGUAGE plpgsql
    AS $_$
DECLARE
  set attribute.minerva_entity_set;
BEGIN
  SELECT * FROM attribute.minerva_entity_set WHERE entity_id = $1 INTO set;
  PERFORM relation_directory.update_entity_set_attributes($1);
  EXECUTE FORMAT(
    'DELETE es FROM relation.%I es '
    'JOIN entity.%I source ON es.source_id = source.id '
    'WHERE source.name = $1 AND target_id = $2',
    set.source_entity_type || '->entity_set',
    set.source_entity_type
  ) USING $2, set.entity_id;
END;
$_$;


ALTER FUNCTION relation_directory.remove_entity_from_set(minerva_entity_set_id integer, entity text) OWNER TO postgres;

--
-- Name: table_schema(); Type: FUNCTION; Schema: relation_directory; Owner: postgres
--

CREATE FUNCTION relation_directory.table_schema() RETURNS name
    LANGUAGE sql IMMUTABLE
    AS $$
SELECT 'relation'::name;
$$;


ALTER FUNCTION relation_directory.table_schema() OWNER TO postgres;

--
-- Name: update_entity_set_attributes(integer); Type: FUNCTION; Schema: relation_directory; Owner: postgres
--

CREATE FUNCTION relation_directory.update_entity_set_attributes(minerva_entity_set_id integer) RETURNS void
    LANGUAGE plpgsql
    AS $_$
BEGIN
  INSERT INTO attribute_staging.minerva_entity_set(
    entity_id, timestamp, name, fullname, "group", source_entity_type, owner, description, last_update
  ) SELECT
    t.entity_id,
    now(),
    t.name,
    t.fullname,
    t."group",
    t.source_entity_type,
    t.owner,
    t.description,
    CURRENT_DATE::text
  FROM attribute.minerva_entity_set t
  WHERE t.id = $1;
END;
$_$;


ALTER FUNCTION relation_directory.update_entity_set_attributes(minerva_entity_set_id integer) OWNER TO postgres;

--
-- Name: setting; Type: TABLE; Schema: system; Owner: postgres
--

CREATE TABLE system.setting (
    id integer NOT NULL,
    name text NOT NULL,
    value text
);


ALTER TABLE system.setting OWNER TO postgres;

--
-- Name: add_setting(text, text); Type: FUNCTION; Schema: system; Owner: postgres
--

CREATE FUNCTION system.add_setting(name text, value text) RETURNS system.setting
    LANGUAGE sql STRICT
    AS $_$
INSERT INTO system.setting (name, value) VALUES ($1, $2) RETURNING setting;
$_$;


ALTER FUNCTION system.add_setting(name text, value text) OWNER TO postgres;

--
-- Name: get_setting(text); Type: FUNCTION; Schema: system; Owner: postgres
--

CREATE FUNCTION system.get_setting(name text) RETURNS system.setting
    LANGUAGE sql STABLE STRICT
    AS $_$
SELECT setting FROM system.setting WHERE name = $1;
$_$;


ALTER FUNCTION system.get_setting(name text) OWNER TO postgres;

--
-- Name: get_setting_value(text); Type: FUNCTION; Schema: system; Owner: postgres
--

CREATE FUNCTION system.get_setting_value(name text) RETURNS text
    LANGUAGE sql STABLE STRICT
    AS $_$
SELECT value FROM system.setting WHERE name = $1;
$_$;


ALTER FUNCTION system.get_setting_value(name text) OWNER TO postgres;

--
-- Name: get_setting_value(text, text); Type: FUNCTION; Schema: system; Owner: postgres
--

CREATE FUNCTION system.get_setting_value(name text, "default" text) RETURNS text
    LANGUAGE sql STABLE STRICT
    AS $_$
SELECT COALESCE(system.get_setting_value($1), $2);
$_$;


ALTER FUNCTION system.get_setting_value(name text, "default" text) OWNER TO postgres;

--
-- Name: set_setting(text, text); Type: FUNCTION; Schema: system; Owner: postgres
--

CREATE FUNCTION system.set_setting(name text, value text) RETURNS system.setting
    LANGUAGE sql STRICT
    AS $_$
SELECT COALESCE(system.update_setting($1, $2), system.add_setting($1, $2));
$_$;


ALTER FUNCTION system.set_setting(name text, value text) OWNER TO postgres;

--
-- Name: update_setting(text, text); Type: FUNCTION; Schema: system; Owner: postgres
--

CREATE FUNCTION system.update_setting(name text, value text) RETURNS system.setting
    LANGUAGE sql STRICT
    AS $_$
UPDATE system.setting SET value = $2 WHERE name = $1 RETURNING setting;
$_$;


ALTER FUNCTION system.update_setting(name text, value text) OWNER TO postgres;

--
-- Name: version(); Type: FUNCTION; Schema: system; Owner: postgres
--

CREATE FUNCTION system.version() RETURNS system.version_tuple
    LANGUAGE sql IMMUTABLE
    AS $$
SELECT (6,0,0)::system.version_tuple;
$$;


ALTER FUNCTION system.version() OWNER TO postgres;

--
-- Name: create_dynamic_source_description(text, integer, text, interval); Type: FUNCTION; Schema: trend; Owner: postgres
--

CREATE FUNCTION trend.create_dynamic_source_description(trend text, counter integer, entity text, granularity interval) RETURNS text
    LANGUAGE sql STABLE
    AS $_$
SELECT FORMAT( 'trend.%I t%s %s ', trend_directory.get_table_name_for_trend($1, $3, $4), $2, CASE $2 WHEN 1 THEN '' ELSE FORMAT('ON t%s.entity_id = t1.entity_id AND t%s.timestamp = t1.timestamp', $2, $2) END );
$_$;


ALTER FUNCTION trend.create_dynamic_source_description(trend text, counter integer, entity text, granularity interval) OWNER TO postgres;

--
-- Name: get_dynamic_trend_data(timestamp with time zone, text, interval, text[]); Type: FUNCTION; Schema: trend; Owner: postgres
--

CREATE FUNCTION trend.get_dynamic_trend_data("timestamp" timestamp with time zone, entity_type_name text, granularity interval, counter_names text[]) RETURNS SETOF trend.trend_data
    LANGUAGE plpgsql STABLE
    AS $_$
DECLARE r trend.trend_data%rowtype; BEGIN IF $4 = ARRAY[]::text[] THEN FOR r IN EXECUTE FORMAT('SELECT ''%s''::timestamptz, e.id, ARRAY[]::numeric[] from entity.%I e', $1, $2) LOOP RETURN NEXT r; END LOOP; ELSE FOR r IN EXECUTE trend.get_dynamic_trend_data_sql($1, $2, $3, $4) LOOP RETURN NEXT r; END LOOP; END IF; RETURN; END;
$_$;


ALTER FUNCTION trend.get_dynamic_trend_data("timestamp" timestamp with time zone, entity_type_name text, granularity interval, counter_names text[]) OWNER TO postgres;

--
-- Name: get_dynamic_trend_data_sql(timestamp with time zone, text, interval, text[]); Type: FUNCTION; Schema: trend; Owner: postgres
--

CREATE FUNCTION trend.get_dynamic_trend_data_sql("timestamp" timestamp with time zone, entity_type_name text, granularity interval, counter_names text[]) RETURNS text
    LANGUAGE sql STABLE
    AS $_$
WITH ref as (
  SELECT
    FORMAT('t%s.%I::numeric', i, c) as column_description,
    trend.create_dynamic_source_description(c, i::integer, $2, $3) as join_data             
  FROM unnest($4) WITH ORDINALITY as counters(c,i)
)
SELECT FORMAT(
    'SELECT ''%s''::timestamp, t1.entity_id, ARRAY[%s] '
    'FROM %s'
    'JOIN entity.%I ent ON ent.id = t1.entity_id '
    'WHERE t1.timestamp = ''%s'';',
  $1,
  string_agg(column_description, ', '),
  string_agg(join_data, ' JOIN '),
  $2,
  $1
  ) FROM ref;
$_$;


ALTER FUNCTION trend.get_dynamic_trend_data_sql("timestamp" timestamp with time zone, entity_type_name text, granularity interval, counter_names text[]) OWNER TO postgres;

--
-- Name: hub-kpi_node_main_15m_fingerprint(timestamp with time zone); Type: FUNCTION; Schema: trend; Owner: postgres
--

CREATE FUNCTION trend."hub-kpi_node_main_15m_fingerprint"(timestamp with time zone) RETURNS trend_directory.fingerprint
    LANGUAGE sql STABLE
    AS $_$
SELECT modified.last, format('{"hub_node_main_15m": "%s"}', modified.last)::jsonb
FROM trend_directory.modified
JOIN trend_directory.trend_store_part ttsp ON ttsp.id = modified.trend_store_part_id
WHERE ttsp::name = 'hub_node_main_15m' AND modified.timestamp = $1;

$_$;


ALTER FUNCTION trend."hub-kpi_node_main_15m_fingerprint"(timestamp with time zone) OWNER TO postgres;

--
-- Name: hub_node_main_1d(timestamp with time zone); Type: FUNCTION; Schema: trend; Owner: postgres
--

CREATE FUNCTION trend.hub_node_main_1d(timestamp with time zone) RETURNS TABLE(entity_id integer, "timestamp" timestamp with time zone, samples smallint, outside_temp numeric, inside_temp numeric, power_kwh numeric, freq_power numeric)
    LANGUAGE plpgsql
    AS $_$
BEGIN
RETURN QUERY EXECUTE $query$
    SELECT
      entity_id,
      $2 AS timestamp,
      (count(*))::smallint AS samples,
      SUM(t."outside_temp")::numeric AS "outside_temp",
      SUM(t."inside_temp")::numeric AS "inside_temp",
      SUM(t."power_kwh")::numeric AS "power_kwh",
      SUM(t."freq_power")::numeric AS "freq_power"
    FROM trend."hub_node_main_15m" AS t
    WHERE $1 < timestamp AND timestamp <= $2
    GROUP BY entity_id
$query$ USING $1 - interval '1d', $1;
END;

$_$;


ALTER FUNCTION trend.hub_node_main_1d(timestamp with time zone) OWNER TO postgres;

--
-- Name: hub_node_main_1d_fingerprint(timestamp with time zone); Type: FUNCTION; Schema: trend; Owner: postgres
--

CREATE FUNCTION trend.hub_node_main_1d_fingerprint(timestamp with time zone) RETURNS trend_directory.fingerprint
    LANGUAGE sql STABLE
    AS $_$
SELECT max(modified.last), format('{%s}', string_agg(format('"%s":"%s"', t, modified.last), ','))::jsonb
FROM generate_series($1 - interval '1d' + interval '15m', $1, interval '15m') t
LEFT JOIN (
  SELECT timestamp, last
  FROM trend_directory.trend_store_part part
  JOIN trend_directory.modified ON modified.trend_store_part_id = part.id
  WHERE part.name = 'hub_node_main_15m'
) modified ON modified.timestamp = t;

$_$;


ALTER FUNCTION trend.hub_node_main_1d_fingerprint(timestamp with time zone) OWNER TO postgres;

--
-- Name: hub_node_main_1h(timestamp with time zone); Type: FUNCTION; Schema: trend; Owner: postgres
--

CREATE FUNCTION trend.hub_node_main_1h(timestamp with time zone) RETURNS TABLE(entity_id integer, "timestamp" timestamp with time zone, samples smallint, outside_temp numeric, inside_temp numeric, power_kwh numeric, freq_power numeric)
    LANGUAGE plpgsql
    AS $_$
BEGIN
RETURN QUERY EXECUTE $query$
    SELECT
      entity_id,
      $2 AS timestamp,
      (count(*))::smallint AS samples,
      SUM(t."outside_temp")::numeric AS "outside_temp",
      SUM(t."inside_temp")::numeric AS "inside_temp",
      SUM(t."power_kwh")::numeric AS "power_kwh",
      SUM(t."freq_power")::numeric AS "freq_power"
    FROM trend."hub_node_main_15m" AS t
    WHERE $1 < timestamp AND timestamp <= $2
    GROUP BY entity_id
$query$ USING $1 - interval '1h', $1;
END;

$_$;


ALTER FUNCTION trend.hub_node_main_1h(timestamp with time zone) OWNER TO postgres;

--
-- Name: hub_node_main_1h_fingerprint(timestamp with time zone); Type: FUNCTION; Schema: trend; Owner: postgres
--

CREATE FUNCTION trend.hub_node_main_1h_fingerprint(timestamp with time zone) RETURNS trend_directory.fingerprint
    LANGUAGE sql STABLE
    AS $_$
SELECT max(modified.last), format('{%s}', string_agg(format('"%s":"%s"', t, modified.last), ','))::jsonb
FROM generate_series($1 - interval '1h' + interval '15m', $1, interval '15m') t
LEFT JOIN (
  SELECT timestamp, last
  FROM trend_directory.trend_store_part part
  JOIN trend_directory.modified ON modified.trend_store_part_id = part.id
  WHERE part.name = 'hub_node_main_15m'
) modified ON modified.timestamp = t;

$_$;


ALTER FUNCTION trend.hub_node_main_1h_fingerprint(timestamp with time zone) OWNER TO postgres;

--
-- Name: hub_node_main_1month(timestamp with time zone); Type: FUNCTION; Schema: trend; Owner: postgres
--

CREATE FUNCTION trend.hub_node_main_1month(timestamp with time zone) RETURNS TABLE(entity_id integer, "timestamp" timestamp with time zone, freq_power numeric, inside_temp numeric, outside_temp numeric, power_kwh numeric, samples bigint)
    LANGUAGE plpgsql
    AS $_$
BEGIN
RETURN QUERY EXECUTE $query$
    SELECT
      entity_id,
      $2 AS timestamp,
      SUM(t."freq_power")::numeric AS "freq_power",
      SUM(t."inside_temp")::numeric AS "inside_temp",
      SUM(t."outside_temp")::numeric AS "outside_temp",
      SUM(t."power_kwh")::numeric AS "power_kwh",
      sum(t."samples")::bigint AS "samples"
    FROM trend."hub_node_main_1d" AS t
    WHERE $1 < timestamp AND timestamp <= $2
    GROUP BY entity_id
$query$ USING $1 - interval '1month', $1;
END;

$_$;


ALTER FUNCTION trend.hub_node_main_1month(timestamp with time zone) OWNER TO postgres;

--
-- Name: hub_node_main_1month_fingerprint(timestamp with time zone); Type: FUNCTION; Schema: trend; Owner: postgres
--

CREATE FUNCTION trend.hub_node_main_1month_fingerprint(timestamp with time zone) RETURNS trend_directory.fingerprint
    LANGUAGE sql STABLE
    AS $_$
SELECT max(modified.last), format('{%s}', string_agg(format('"%s":"%s"', t, modified.last), ','))::jsonb
FROM generate_series($1 - interval '1month' + interval '1d', $1, interval '1d') t
LEFT JOIN (
  SELECT timestamp, last
  FROM trend_directory.trend_store_part part
  JOIN trend_directory.modified ON modified.trend_store_part_id = part.id
  WHERE part.name = 'hub_node_main_1d'
) modified ON modified.timestamp = t;

$_$;


ALTER FUNCTION trend.hub_node_main_1month_fingerprint(timestamp with time zone) OWNER TO postgres;

--
-- Name: hub_node_main_1w(timestamp with time zone); Type: FUNCTION; Schema: trend; Owner: postgres
--

CREATE FUNCTION trend.hub_node_main_1w(timestamp with time zone) RETURNS TABLE(entity_id integer, "timestamp" timestamp with time zone, freq_power numeric, inside_temp numeric, outside_temp numeric, power_kwh numeric, samples bigint)
    LANGUAGE plpgsql
    AS $_$
BEGIN
RETURN QUERY EXECUTE $query$
    SELECT
      entity_id,
      $2 AS timestamp,
      SUM(t."freq_power")::numeric AS "freq_power",
      SUM(t."inside_temp")::numeric AS "inside_temp",
      SUM(t."outside_temp")::numeric AS "outside_temp",
      SUM(t."power_kwh")::numeric AS "power_kwh",
      sum(t."samples")::bigint AS "samples"
    FROM trend."hub_node_main_1d" AS t
    WHERE $1 < timestamp AND timestamp <= $2
    GROUP BY entity_id
$query$ USING $1 - interval '1w', $1;
END;

$_$;


ALTER FUNCTION trend.hub_node_main_1w(timestamp with time zone) OWNER TO postgres;

--
-- Name: hub_node_main_1w_fingerprint(timestamp with time zone); Type: FUNCTION; Schema: trend; Owner: postgres
--

CREATE FUNCTION trend.hub_node_main_1w_fingerprint(timestamp with time zone) RETURNS trend_directory.fingerprint
    LANGUAGE sql STABLE
    AS $_$
SELECT max(modified.last), format('{%s}', string_agg(format('"%s":"%s"', t, modified.last), ','))::jsonb
FROM generate_series($1 - interval '1w' + interval '1d', $1, interval '1d') t
LEFT JOIN (
  SELECT timestamp, last
  FROM trend_directory.trend_store_part part
  JOIN trend_directory.modified ON modified.trend_store_part_id = part.id
  WHERE part.name = 'hub_node_main_1d'
) modified ON modified.timestamp = t;

$_$;


ALTER FUNCTION trend.hub_node_main_1w_fingerprint(timestamp with time zone) OWNER TO postgres;

--
-- Name: mapping_15m->1d(timestamp with time zone); Type: FUNCTION; Schema: trend; Owner: postgres
--

CREATE FUNCTION trend."mapping_15m->1d"(timestamp with time zone) RETURNS timestamp with time zone
    LANGUAGE sql IMMUTABLE
    AS $_$
SELECT date_trunc('day', $1 - interval '15m') + interval '1d';
$_$;


ALTER FUNCTION trend."mapping_15m->1d"(timestamp with time zone) OWNER TO postgres;

--
-- Name: mapping_15m->1h(timestamp with time zone); Type: FUNCTION; Schema: trend; Owner: postgres
--

CREATE FUNCTION trend."mapping_15m->1h"(timestamp with time zone) RETURNS timestamp with time zone
    LANGUAGE sql IMMUTABLE
    AS $_$
SELECT date_trunc('hour', $1 - interval '15m') + interval '1h';
$_$;


ALTER FUNCTION trend."mapping_15m->1h"(timestamp with time zone) OWNER TO postgres;

--
-- Name: mapping_1d->1month(timestamp with time zone); Type: FUNCTION; Schema: trend; Owner: postgres
--

CREATE FUNCTION trend."mapping_1d->1month"(timestamp with time zone) RETURNS timestamp with time zone
    LANGUAGE sql IMMUTABLE
    AS $_$
SELECT date_trunc('month', $1 - interval '1d') + interval '1month';
$_$;


ALTER FUNCTION trend."mapping_1d->1month"(timestamp with time zone) OWNER TO postgres;

--
-- Name: mapping_1d->1w(timestamp with time zone); Type: FUNCTION; Schema: trend; Owner: postgres
--

CREATE FUNCTION trend."mapping_1d->1w"(timestamp with time zone) RETURNS timestamp with time zone
    LANGUAGE sql IMMUTABLE
    AS $_$
SELECT date_trunc('week', $1 - interval '1d') + interval '1w';
$_$;


ALTER FUNCTION trend."mapping_1d->1w"(timestamp with time zone) OWNER TO postgres;

--
-- Name: mapping_30m->1d(timestamp with time zone); Type: FUNCTION; Schema: trend; Owner: postgres
--

CREATE FUNCTION trend."mapping_30m->1d"(timestamp with time zone) RETURNS timestamp with time zone
    LANGUAGE sql IMMUTABLE
    AS $_$
SELECT date_trunc('day', $1 - interval '30m') + interval '1d';
$_$;


ALTER FUNCTION trend."mapping_30m->1d"(timestamp with time zone) OWNER TO postgres;

--
-- Name: mapping_30m->1h(timestamp with time zone); Type: FUNCTION; Schema: trend; Owner: postgres
--

CREATE FUNCTION trend."mapping_30m->1h"(timestamp with time zone) RETURNS timestamp with time zone
    LANGUAGE sql IMMUTABLE
    AS $_$
SELECT date_trunc('hour', $1 - interval '30m') + interval '1h';
$_$;


ALTER FUNCTION trend."mapping_30m->1h"(timestamp with time zone) OWNER TO postgres;

--
-- Name: mapping_id(timestamp with time zone); Type: FUNCTION; Schema: trend; Owner: postgres
--

CREATE FUNCTION trend.mapping_id(timestamp with time zone) RETURNS timestamp with time zone
    LANGUAGE sql IMMUTABLE
    AS $_$
SELECT $1;
$_$;


ALTER FUNCTION trend.mapping_id(timestamp with time zone) OWNER TO postgres;

--
-- Name: table_trend; Type: TABLE; Schema: trend_directory; Owner: postgres
--

CREATE TABLE trend_directory.table_trend (
    id integer NOT NULL,
    trend_store_part_id integer NOT NULL,
    name name NOT NULL,
    data_type text NOT NULL,
    extra_data jsonb DEFAULT '{}'::jsonb NOT NULL,
    description text NOT NULL,
    time_aggregation text NOT NULL,
    entity_aggregation text NOT NULL
);


ALTER TABLE trend_directory.table_trend OWNER TO postgres;

--
-- Name: add_column_sql_part(trend_directory.table_trend); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.add_column_sql_part(trend_directory.table_trend) RETURNS text
    LANGUAGE sql STABLE
    AS $_$
SELECT format('ADD COLUMN %I %s', $1.name, $1.data_type);
$_$;


ALTER FUNCTION trend_directory.add_column_sql_part(trend_directory.table_trend) OWNER TO postgres;

--
-- Name: generated_table_trend; Type: TABLE; Schema: trend_directory; Owner: postgres
--

CREATE TABLE trend_directory.generated_table_trend (
    id integer NOT NULL,
    trend_store_part_id integer NOT NULL,
    name name NOT NULL,
    data_type text NOT NULL,
    expression text NOT NULL,
    extra_data jsonb DEFAULT '{}'::jsonb NOT NULL,
    description text NOT NULL
);


ALTER TABLE trend_directory.generated_table_trend OWNER TO postgres;

--
-- Name: add_generated_column_sql_part(trend_directory.generated_table_trend); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.add_generated_column_sql_part(trend_directory.generated_table_trend) RETURNS text
    LANGUAGE sql STABLE
    AS $_$
SELECT format(
  'ADD COLUMN %I %s GENERATED ALWAYS AS (%s) STORED',
  $1.name, $1.data_type, $1.expression
);
$_$;


ALTER FUNCTION trend_directory.add_generated_column_sql_part(trend_directory.generated_table_trend) OWNER TO postgres;

--
-- Name: add_generated_trends_to_trend_store_part(trend_directory.trend_store_part, trend_directory.generated_table_trend[]); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.add_generated_trends_to_trend_store_part(trend_directory.trend_store_part, trend_directory.generated_table_trend[]) RETURNS trend_directory.trend_store_part
    LANGUAGE sql
    AS $_$
SELECT public.action(
  $1,
  ARRAY[
    format(
      'ALTER TABLE %I.%I %s;',
      trend_directory.base_table_schema(),
      trend_directory.base_table_name($1),
      (SELECT string_agg(trend_directory.add_generated_column_sql_part(t), ',') FROM unnest($2) AS t)
    )
  ]
);
$_$;


ALTER FUNCTION trend_directory.add_generated_trends_to_trend_store_part(trend_directory.trend_store_part, trend_directory.generated_table_trend[]) OWNER TO postgres;

--
-- Name: trend_store; Type: TABLE; Schema: trend_directory; Owner: postgres
--

CREATE TABLE trend_directory.trend_store (
    id integer NOT NULL,
    entity_type_id integer,
    data_source_id integer,
    granularity interval NOT NULL,
    partition_size interval NOT NULL,
    retention_period interval DEFAULT '1 mon'::interval NOT NULL
);


ALTER TABLE trend_directory.trend_store OWNER TO postgres;

--
-- Name: TABLE trend_store; Type: COMMENT; Schema: trend_directory; Owner: postgres
--

COMMENT ON TABLE trend_directory.trend_store IS 'Table based trend stores describing the common properties of all its
partitions like entity type, data granularity, etc.';


--
-- Name: add_missing_trend_store_parts(trend_directory.trend_store, trend_directory.trend_store_part_descr[]); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.add_missing_trend_store_parts(trend_directory.trend_store, parts trend_directory.trend_store_part_descr[]) RETURNS trend_directory.trend_store
    LANGUAGE sql
    AS $_$
SELECT trend_directory.get_or_create_trend_store_part($1.id, name)
  FROM unnest($2);
SELECT $1;
$_$;


ALTER FUNCTION trend_directory.add_missing_trend_store_parts(trend_directory.trend_store, parts trend_directory.trend_store_part_descr[]) OWNER TO postgres;

--
-- Name: add_new_state(); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.add_new_state() RETURNS integer
    LANGUAGE plpgsql
    AS $$
DECLARE
    count integer;
BEGIN
    INSERT INTO trend_directory.state(materialization_id, timestamp, max_modified, source_states)
    SELECT materialization_id, timestamp, max_modified, source_states
    FROM trend_directory.new_materializables;

    GET DIAGNOSTICS count = ROW_COUNT;

    RETURN count;
END;
$$;


ALTER FUNCTION trend_directory.add_new_state() OWNER TO postgres;

--
-- Name: add_trends(trend_directory.trend_store_part_descr); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.add_trends(part trend_directory.trend_store_part_descr) RETURNS text[]
    LANGUAGE sql
    AS $_$
SELECT trend_directory.assure_table_trends_exist(
  trend_store_part.trend_store_id,
  $1.name,
  $1.trends,
  $1.generated_trends
)
FROM trend_directory.trend_store_part
WHERE name = $1.name;
$_$;


ALTER FUNCTION trend_directory.add_trends(part trend_directory.trend_store_part_descr) OWNER TO postgres;

--
-- Name: add_trends(trend_directory.trend_store, trend_directory.trend_store_part_descr[]); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.add_trends(trend_directory.trend_store, parts trend_directory.trend_store_part_descr[]) RETURNS text[]
    LANGUAGE plpgsql
    AS $_$
DECLARE
  result text[];
  partresult text[];
BEGIN
  FOR partresult IN
    SELECT trend_directory.assure_table_trends_exist(
      $1.id,
      name,
      trends,
      generated_trends
    )
    FROM unnest($2)
  LOOP
    SELECT result || partresult INTO result;
  END LOOP;
  RETURN result;
END;
$_$;


ALTER FUNCTION trend_directory.add_trends(trend_directory.trend_store, parts trend_directory.trend_store_part_descr[]) OWNER TO postgres;

--
-- Name: add_trends_to_trend_store_part(trend_directory.trend_store_part, trend_directory.table_trend[]); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.add_trends_to_trend_store_part(trend_directory.trend_store_part, trend_directory.table_trend[]) RETURNS trend_directory.trend_store_part
    LANGUAGE sql
    AS $_$
SELECT public.action(
  $1,
  ARRAY[
    format(
      'ALTER TABLE %I.%I %s;',
      trend_directory.base_table_schema(),
      trend_directory.base_table_name($1),
      (SELECT string_agg(trend_directory.add_column_sql_part(t), ',') FROM unnest($2) AS t)
    ),
    format(
      'ALTER TABLE %I.%I %s;',
      trend_directory.staging_table_schema(),
      trend_directory.staging_table_name($1),
      (SELECT string_agg(trend_directory.add_column_sql_part(t), ',') FROM unnest($2) AS t)
    )
  ]
);
$_$;


ALTER FUNCTION trend_directory.add_trends_to_trend_store_part(trend_directory.trend_store_part, trend_directory.table_trend[]) OWNER TO postgres;

--
-- Name: alter_trend_name(trend_directory.trend_store_part, name, name); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.alter_trend_name(trend_directory.trend_store_part, trend_name name, new_name name) RETURNS trend_directory.trend_store_part
    LANGUAGE plpgsql
    AS $_$
DECLARE
  table_trend_id integer;
BEGIN
  FOR table_trend_id IN
    SELECT id FROM trend_directory.table_trend WHERE trend_store_part_id = $1.id AND name = $2
  LOOP
    UPDATE trend_directory.table_trend SET name = $3 WHERE id = table_trend_id;
    PERFORM trend_directory.changes_on_trend_update(table_trend_id, $2, $3);
  END LOOP;
  RETURN $1;
END;
$_$;


ALTER FUNCTION trend_directory.alter_trend_name(trend_directory.trend_store_part, trend_name name, new_name name) OWNER TO postgres;

--
-- Name: assure_table_trends_exist(integer, text, trend_directory.trend_descr[], trend_directory.generated_trend_descr[]); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.assure_table_trends_exist(trend_store_id integer, trend_store_part_name text, trend_directory.trend_descr[], trend_directory.generated_trend_descr[]) RETURNS text[]
    LANGUAGE plpgsql
    AS $_$
DECLARE
  tsp trend_directory.trend_store_part;
  result text[];
BEGIN
  SELECT * FROM trend_directory.get_or_create_trend_store_part($1, $2) INTO tsp;

  CREATE TEMP TABLE missing_trends(trend trend_directory.trend_descr);
  CREATE TEMP TABLE missing_generated_trends(trend trend_directory.generated_trend_descr);

  -- Normal trends
  INSERT INTO missing_trends SELECT trend_directory.missing_table_trends(tsp, $3);

  IF EXISTS (SELECT * FROM missing_trends LIMIT 1) THEN
    PERFORM trend_directory.create_table_trends(tsp, ARRAY(SELECT trend FROM missing_trends));
  END IF;

  -- Generated trends
  INSERT INTO missing_generated_trends SELECT trend_directory.missing_generated_table_trends(tsp, $4);

  IF EXISTS (SELECT * FROM missing_generated_trends LIMIT 1) THEN
    PERFORM trend_directory.create_generated_table_trends(tsp, missing_generated_trends);
  END IF;

  SELECT ARRAY(SELECT (mt).trend.name FROM missing_trends mt UNION SELECT (mt).trend.name FROM missing_generated_trends mt) INTO result;
  DROP TABLE missing_trends;
  DROP TABLE missing_generated_trends;

  RETURN result;
END;
$_$;


ALTER FUNCTION trend_directory.assure_table_trends_exist(trend_store_id integer, trend_store_part_name text, trend_directory.trend_descr[], trend_directory.generated_trend_descr[]) OWNER TO postgres;

--
-- Name: base_table_name_by_trend_store_part_id(integer); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.base_table_name_by_trend_store_part_id(trend_store_part_id integer) RETURNS name
    LANGUAGE sql IMMUTABLE STRICT
    AS $_$
SELECT name FROM trend_directory.trend_store_part
  WHERE id = $1;
$_$;


ALTER FUNCTION trend_directory.base_table_name_by_trend_store_part_id(trend_store_part_id integer) OWNER TO postgres;

--
-- Name: base_table_schema(); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.base_table_schema() RETURNS name
    LANGUAGE sql IMMUTABLE
    AS $$
SELECT 'trend'::name;
$$;


ALTER FUNCTION trend_directory.base_table_schema() OWNER TO postgres;

--
-- Name: change_all_trend_data(trend_directory.trend_store, trend_directory.trend_store_part_descr[]); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.change_all_trend_data(trend_directory.trend_store, parts trend_directory.trend_store_part_descr[]) RETURNS text[]
    LANGUAGE plpgsql
    AS $_$
DECLARE
  result text[];
  partresult text;
BEGIN
  FOR partresult IN
    SELECT trend_directory.change_trend_data_unsafe(
      trend_directory.get_trends_for_trend_store($1), trends, name)
    FROM unnest($2)
  LOOP
    IF partresult IS NOT null THEN
      SELECT result || partresult INTO result;
    END IF;
  END LOOP;
  RETURN result;
END;
$_$;


ALTER FUNCTION trend_directory.change_all_trend_data(trend_directory.trend_store, parts trend_directory.trend_store_part_descr[]) OWNER TO postgres;

--
-- Name: change_table_trend_data_safe(integer, text, text, text); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.change_table_trend_data_safe(trend_id integer, data_type text, entity_aggregation text, time_aggregation text) RETURNS text
    LANGUAGE plpgsql
    AS $_$
DECLARE
  trend trend_directory.table_trend;
BEGIN
  SELECT * FROM trend_directory.table_trend WHERE id = $1 INTO trend;
  RETURN trend_directory.change_table_trend_data_unsafe(
    $1,
    trend_directory.greatest_data_type($2, trend.data_type),
    $3,
    $4);
END;
$_$;


ALTER FUNCTION trend_directory.change_table_trend_data_safe(trend_id integer, data_type text, entity_aggregation text, time_aggregation text) OWNER TO postgres;

--
-- Name: change_table_trend_data_unsafe(integer, text, text, text); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.change_table_trend_data_unsafe(trend_id integer, data_type text, entity_aggregation text, time_aggregation text) RETURNS text
    LANGUAGE plpgsql
    AS $_$
DECLARE
  result text;
  trend trend_directory.table_trend;
BEGIN
  SELECT * FROM trend_directory.table_trend WHERE id = $1 INTO trend;
  IF trend.data_type <> $2 OR trend.entity_aggregation <> $3 OR trend.time_aggregation <> $4
  THEN
    UPDATE trend_directory.table_trend SET
      data_type = $2,
      entity_aggregation = $3,
      time_aggregation = $4
    WHERE id = $1;
    SELECT trend.name INTO result;
  END IF;

  IF trend.data_type <> $2
  THEN
    EXECUTE format('ALTER TABLE trend.%I ALTER %I TYPE %s USING CAST(%I AS %s)',
      trend_directory.trend_store_part_name_for_trend($1),
      trend.name,
      $2,
      trend.name,
      $2);
  END IF;

  RETURN result;
END;
$_$;


ALTER FUNCTION trend_directory.change_table_trend_data_unsafe(trend_id integer, data_type text, entity_aggregation text, time_aggregation text) OWNER TO postgres;

--
-- Name: change_trend_data(trend_directory.trend_store_part_descr); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.change_trend_data(part trend_directory.trend_store_part_descr) RETURNS text[]
    LANGUAGE sql
    AS $_$
SELECT array_agg(trend_directory.change_table_trend_data_unsafe(
  table_trend.id,
  t.data_type,
  t.entity_aggregation,
  t.time_aggregation
))
FROM trend_directory.trend_store_part
  JOIN trend_directory.table_trend ON table_trend.trend_store_part_id = trend_store_part.id
  JOIN UNNEST($1.trends) AS t ON t.name = table_trend.name
  WHERE trend_store_part.name = $1.name AND trend_directory.trend_has_update(table_trend.id, t);
$_$;


ALTER FUNCTION trend_directory.change_trend_data(part trend_directory.trend_store_part_descr) OWNER TO postgres;

--
-- Name: change_trend_data(trend_directory.trend_store, trend_directory.trend_store_part_descr[]); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.change_trend_data(trend_directory.trend_store, parts trend_directory.trend_store_part_descr[]) RETURNS text[]
    LANGUAGE plpgsql
    AS $_$
DECLARE
  result text[];
  partresult text;
BEGIN
  FOR partresult IN
    SELECT trend_directory.change_trend_data_unsafe(
      trend_directory.get_trends_for_trend_store($1), trends, name)
    FROM unnest($2)
  LOOP
    IF partresult IS NOT null THEN
      SELECT result || partresult INTO result;
    END IF;
  END LOOP;
  RETURN result;
END;
$_$;


ALTER FUNCTION trend_directory.change_trend_data(trend_directory.trend_store, parts trend_directory.trend_store_part_descr[]) OWNER TO postgres;

--
-- Name: change_trend_data_safe(trend_directory.table_trend, trend_directory.trend_descr[], text); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.change_trend_data_safe(trend trend_directory.table_trend, trends trend_directory.trend_descr[], partname text) RETURNS text
    LANGUAGE sql
    AS $_$
SELECT trend_directory.change_table_trend_data_safe($1.id, t.data_type, t.entity_aggregation, t.time_aggregation)
  FROM unnest($2) t
  WHERE t.name = $1.name AND trend_directory.trend_store_part_name_for_trend($1) = $3;
$_$;


ALTER FUNCTION trend_directory.change_trend_data_safe(trend trend_directory.table_trend, trends trend_directory.trend_descr[], partname text) OWNER TO postgres;

--
-- Name: change_trend_data_unsafe(trend_directory.table_trend, trend_directory.trend_descr[], text); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.change_trend_data_unsafe(trend trend_directory.table_trend, trends trend_directory.trend_descr[], partname text) RETURNS text
    LANGUAGE sql
    AS $_$
SELECT trend_directory.change_table_trend_data_unsafe($1.id, t.data_type, t.entity_aggregation, t.time_aggregation)
  FROM unnest($2) t
  WHERE t.name = $1.name AND trend_directory.trend_store_part_name_for_trend($1) = $3;
$_$;


ALTER FUNCTION trend_directory.change_trend_data_unsafe(trend trend_directory.table_trend, trends trend_directory.trend_descr[], partname text) OWNER TO postgres;

--
-- Name: change_trend_data_upward(trend_directory.trend_store_part_descr); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.change_trend_data_upward(part trend_directory.trend_store_part_descr) RETURNS text[]
    LANGUAGE sql
    AS $_$
SELECT array_agg(trend_directory.change_table_trend_data_safe(
  table_trend.id,
  t.data_type,
  t.entity_aggregation,
  t.time_aggregation
))
FROM trend_directory.trend_store_part
  JOIN trend_directory.table_trend ON table_trend.trend_store_part_id = trend_store_part.id
  JOIN UNNEST($1.trends) AS t ON t.name = table_trend.name
  WHERE trend_store_part.name = $1.name AND trend_directory.trend_has_update(table_trend.id, t);
$_$;


ALTER FUNCTION trend_directory.change_trend_data_upward(part trend_directory.trend_store_part_descr) OWNER TO postgres;

--
-- Name: change_trend_data_upward(trend_directory.trend_store, trend_directory.trend_store_part_descr[]); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.change_trend_data_upward(trend_directory.trend_store, parts trend_directory.trend_store_part_descr[]) RETURNS text[]
    LANGUAGE plpgsql
    AS $_$
DECLARE
  result text[];
  partresult text;
BEGIN
  FOR partresult IN
    SELECT trend_directory.change_trend_data_safe(
      trend_directory.get_trends_for_trend_store($1), trends, name)
    FROM unnest($2)
  LOOP
    IF partresult IS NOT null THEN
      SELECT result || partresult INTO result;
    END IF;
  END LOOP;
  RETURN result;
END;
$_$;


ALTER FUNCTION trend_directory.change_trend_data_upward(trend_directory.trend_store, parts trend_directory.trend_store_part_descr[]) OWNER TO postgres;

--
-- Name: change_trend_store_part_strong(trend_directory.trend_store_part_descr); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.change_trend_store_part_strong(part trend_directory.trend_store_part_descr) RETURNS trend_directory.change_trend_store_part_result
    LANGUAGE plpgsql
    AS $_$
DECLARE
  result trend_directory.change_trend_store_part_result;
BEGIN
  SELECT trend_directory.add_trends($1) INTO result.added_trends;

  SELECT trend_directory.remove_extra_trends($1) INTO result.removed_trends;

  SELECT trend_directory.change_trend_data($1) INTO result.changed_trends;

  RETURN result;
END;
$_$;


ALTER FUNCTION trend_directory.change_trend_store_part_strong(part trend_directory.trend_store_part_descr) OWNER TO postgres;

--
-- Name: change_trend_store_part_weak(trend_directory.trend_store_part_descr); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.change_trend_store_part_weak(part trend_directory.trend_store_part_descr) RETURNS trend_directory.change_trend_store_part_result
    LANGUAGE plpgsql
    AS $_$
DECLARE
  result trend_directory.change_trend_store_part_result;
BEGIN
  SELECT trend_directory.add_trends($1) INTO result.added_trends;

  SELECT trend_directory.remove_extra_trends($1) INTO result.removed_trends;

  SELECT trend_directory.change_trend_data_upward($1) INTO result.changed_trends;

  RETURN result;
END;
$_$;


ALTER FUNCTION trend_directory.change_trend_store_part_weak(part trend_directory.trend_store_part_descr) OWNER TO postgres;

--
-- Name: change_trendstore_strong(trend_directory.trend_store, trend_directory.trend_store_part_descr[]); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.change_trendstore_strong(trend_directory.trend_store, parts trend_directory.trend_store_part_descr[]) RETURNS text[]
    LANGUAGE plpgsql
    AS $_$
DECLARE
  result text[];
  partresult text[];
BEGIN
  SELECT trend_directory.add_trends($1, $2) INTO partresult;
  IF array_ndims(partresult) > 0
  THEN
    SELECT result || ARRAY['added trends:'] || partresult INTO result;
  ELSE
    SELECT result || ARRAY['no trends added'] INTO result;
  END IF;
  
  SELECT trend_directory.remove_extra_trends($1, $2) INTO partresult;
  IF array_ndims(partresult) > 0
  THEN
    SELECT result || ARRAY['removed trends:'] || partresult INTO result;
  ELSE
    SELECT result || ARRAY['no trends removed'] INTO result;
  END IF;

  SELECT trend_directory.change_all_trend_data($1, $2) INTO partresult;
  IF array_ndims(partresult) > 0
  THEN
    SELECT result || ARRAY['changed trends:'] || partresult INTO result;
  ELSE
    SELECT result || ARRAY['no trends changed'] INTO result;
  END IF;
  RETURN result;
END;
$_$;


ALTER FUNCTION trend_directory.change_trendstore_strong(trend_directory.trend_store, parts trend_directory.trend_store_part_descr[]) OWNER TO postgres;

--
-- Name: change_trendstore_weak(trend_directory.trend_store, trend_directory.trend_store_part_descr[]); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.change_trendstore_weak(trend_directory.trend_store, parts trend_directory.trend_store_part_descr[]) RETURNS text[]
    LANGUAGE plpgsql
    AS $_$
DECLARE
  result text[];
  partresult text[];
BEGIN
  SELECT trend_directory.add_trends($1, $2) INTO partresult;
  IF array_ndims(partresult) > 0
  THEN
    SELECT result || ARRAY['added trends:'] || partresult INTO result;
  ELSE
    SELECT result || ARRAY['no trends added'] INTO result;
  END IF;
  
  SELECT trend_directory.remove_extra_trends($1, $2) INTO partresult;
  IF array_ndims(partresult) > 0
  THEN
    SELECT result || ARRAY['removed trends:'] || partresult INTO result;
  ELSE
    SELECT result || ARRAY['no trends removed'] INTO result;
  END IF;

  SELECT trend_directory.change_trend_data_upward($1, $2) INTO partresult;
  IF array_ndims(partresult) > 0
  THEN
    SELECT result || ARRAY['changed trends:'] || partresult INTO result;
  ELSE
    SELECT result || ARRAY['no trends changed'] INTO result;
  END IF;
  RETURN result;
END;
$_$;


ALTER FUNCTION trend_directory.change_trendstore_weak(trend_directory.trend_store, parts trend_directory.trend_store_part_descr[]) OWNER TO postgres;

--
-- Name: changes_on_trend_update(integer, text, text); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.changes_on_trend_update("trend_directory.table_trend_id" integer, oldname text, newname text) RETURNS void
    LANGUAGE plpgsql
    AS $_$
DECLARE
    base_table_name text;
BEGIN
    IF $3 <> $2 THEN
        FOR base_table_name IN
            SELECT trend_directory.base_table_name_by_trend_store_part_id(trend_store_part.id)
            FROM trend_directory.table_trend
            JOIN trend_directory.trend_store_part ON table_trend.trend_store_part_id = trend_store_part.id
            WHERE table_trend.id = $1
        LOOP
            EXECUTE format('ALTER TABLE trend.%I RENAME COLUMN %I TO %I', base_table_name, $2, $3);
        END LOOP;
    END IF;
END;
$_$;


ALTER FUNCTION trend_directory.changes_on_trend_update("trend_directory.table_trend_id" integer, oldname text, newname text) OWNER TO postgres;

--
-- Name: cleanup_for_function_materialization(); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.cleanup_for_function_materialization() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    EXECUTE format('DROP FUNCTION %s', OLD.src_function);

    RETURN OLD;
END;
$$;


ALTER FUNCTION trend_directory.cleanup_for_function_materialization() OWNER TO postgres;

--
-- Name: cleanup_for_materialization(text); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.cleanup_for_materialization(materialization_name text) RETURNS void
    LANGUAGE sql
    AS $_$
SELECT trend_directory.cleanup_for_materialization(m)
  FROM trend_directory.materialization m
  WHERE m::text = $1;
$_$;


ALTER FUNCTION trend_directory.cleanup_for_materialization(materialization_name text) OWNER TO postgres;

--
-- Name: cleanup_for_materialization(trend_directory.materialization); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.cleanup_for_materialization(trend_directory.materialization) RETURNS void
    LANGUAGE plpgsql
    AS $_$
BEGIN
  EXECUTE format(
    'DROP FUNCTION trend.%I(timestamp with time zone)',
    trend_directory.fingerprint_function_name($1)
  );
END;
$_$;


ALTER FUNCTION trend_directory.cleanup_for_materialization(trend_directory.materialization) OWNER TO postgres;

--
-- Name: cleanup_for_view_materialization(); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.cleanup_for_view_materialization() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    EXECUTE format('DROP VIEW %s', OLD.src_view);

    RETURN OLD;
END;
$$;


ALTER FUNCTION trend_directory.cleanup_for_view_materialization() OWNER TO postgres;

--
-- Name: clear(trend_directory.trend_store_part, timestamp with time zone); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.clear(trend_directory.trend_store_part, timestamp with time zone) RETURNS integer
    LANGUAGE plpgsql
    AS $_$
DECLARE
    row_count integer;
BEGIN
    EXECUTE format(
        'DELETE FROM %I.%I WHERE timestamp = $1',
        trend_directory.base_table_schema(),
        trend_directory.base_table_name($1)
    ) USING $2;

    GET DIAGNOSTICS row_count = ROW_COUNT;

    RETURN row_count;
END;
$_$;


ALTER FUNCTION trend_directory.clear(trend_directory.trend_store_part, timestamp with time zone) OWNER TO postgres;

--
-- Name: clear_trend_store_part(integer, timestamp with time zone); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.clear_trend_store_part(trend_store_part_id integer, "timestamp" timestamp with time zone) RETURNS integer
    LANGUAGE plpgsql
    AS $_$
DECLARE
    row_count integer;
BEGIN
    EXECUTE trend_directory.clear_trend_store_part_sql($1) USING $2;

    GET DIAGNOSTICS row_count = ROW_COUNT;

    RETURN row_count;
END;
$_$;


ALTER FUNCTION trend_directory.clear_trend_store_part(trend_store_part_id integer, "timestamp" timestamp with time zone) OWNER TO postgres;

--
-- Name: FUNCTION clear_trend_store_part(trend_store_part_id integer, "timestamp" timestamp with time zone); Type: COMMENT; Schema: trend_directory; Owner: postgres
--

COMMENT ON FUNCTION trend_directory.clear_trend_store_part(trend_store_part_id integer, "timestamp" timestamp with time zone) IS 'Removes all records of the specified timestamp from the trend_store_part and returns the removed record count.
';


--
-- Name: clear_trend_store_part_sql(integer); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.clear_trend_store_part_sql(trend_store_part_id integer) RETURNS text
    LANGUAGE sql
    AS $_$
SELECT format('DELETE FROM trend.%I WHERE timestamp = $1', ttsp.name)
FROM trend_directory.trend_store_part ttsp WHERE id = $1;
$_$;


ALTER FUNCTION trend_directory.clear_trend_store_part_sql(trend_store_part_id integer) OWNER TO postgres;

--
-- Name: FUNCTION clear_trend_store_part_sql(trend_store_part_id integer); Type: COMMENT; Schema: trend_directory; Owner: postgres
--

COMMENT ON FUNCTION trend_directory.clear_trend_store_part_sql(trend_store_part_id integer) IS 'Return the query to remove all records of the specified timestamp from the trend_store_part.
';


--
-- Name: column_exists(name, name); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.column_exists(table_name name, column_name name) RETURNS boolean
    LANGUAGE sql
    AS $$
SELECT EXISTS(
    SELECT 1
    FROM pg_attribute a
    JOIN pg_class c ON c.oid = a.attrelid
    JOIN pg_namespace n ON c.relnamespace = n.oid
    WHERE c.relname = table_name AND a.attname = column_name AND n.nspname = 'trend'
);
$$;


ALTER FUNCTION trend_directory.column_exists(table_name name, column_name name) OWNER TO postgres;

--
-- Name: column_spec(trend_directory.generated_table_trend); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.column_spec(trend_directory.generated_table_trend) RETURNS text
    LANGUAGE sql IMMUTABLE
    AS $_$
SELECT format('%I %s GENERATED ALWAYS AS (%s) STORED', $1.name, $1.data_type, $1.expression);
$_$;


ALTER FUNCTION trend_directory.column_spec(trend_directory.generated_table_trend) OWNER TO postgres;

--
-- Name: column_spec(trend_directory.table_trend); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.column_spec(trend_directory.table_trend) RETURNS text
    LANGUAGE sql IMMUTABLE
    AS $_$
SELECT format('%I %s', $1.name, $1.data_type);
$_$;


ALTER FUNCTION trend_directory.column_spec(trend_directory.table_trend) OWNER TO postgres;

--
-- Name: column_specs(trend_directory.trend_store_part); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.column_specs(trend_directory.trend_store_part) RETURNS text[]
    LANGUAGE sql STABLE
    AS $_$
SELECT array_agg(c) FROM (
  SELECT trend_directory.trend_column_spec(t.id) AS c
  FROM trend_directory.table_trend t
  WHERE t.trend_store_part_id = $1.id
  UNION ALL
  SELECT trend_directory.generated_trend_column_spec(t.id) AS c
  FROM trend_directory.generated_table_trend t
  WHERE t.trend_store_part_id = $1.id
) combined_columns;
$_$;


ALTER FUNCTION trend_directory.column_specs(trend_directory.trend_store_part) OWNER TO postgres;

--
-- Name: columns_part(integer); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.columns_part(trend_store_part_id integer) RETURNS text
    LANGUAGE sql STABLE
    AS $_$
WITH columns AS (
  SELECT t.name
  FROM trend_directory.trend_store_part ttsp
  JOIN trend_directory.table_trend t ON t.trend_store_part_id = ttsp.id
  WHERE ttsp.id = $1
)
SELECT
  array_to_string(array_agg(quote_ident(name)), ', ')
FROM columns;
$_$;


ALTER FUNCTION trend_directory.columns_part(trend_store_part_id integer) OWNER TO postgres;

--
-- Name: FUNCTION columns_part(trend_store_part_id integer); Type: COMMENT; Schema: trend_directory; Owner: postgres
--

COMMENT ON FUNCTION trend_directory.columns_part(trend_store_part_id integer) IS 'Return the comma separated, quoted list of column names to be used in queries';


--
-- Name: completeness(name, timestamp with time zone, timestamp with time zone); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.completeness(name, start timestamp with time zone, "end" timestamp with time zone) RETURNS TABLE("timestamp" timestamp with time zone, count bigint)
    LANGUAGE plpgsql
    AS $_$
DECLARE
    gran interval;
    truncated_start timestamptz;
    truncated_end timestamptz;
BEGIN
    SELECT granularity INTO gran
    FROM trend_directory.trend_store_part tsp
    JOIN trend_directory.trend_store ts ON ts.id = tsp.trend_store_id
    WHERE tsp.name = $1;

    CASE gran
    WHEN '1month' THEN
        SELECT date_trunc('month', $2) INTO truncated_start;
        SELECT date_trunc('month', $3) INTO truncated_end;
    WHEN '1w' THEN
        SELECT date_trunc('week', $2) INTO truncated_start;
        SELECT date_trunc('week', $3) INTO truncated_end;
    WHEN '1d' THEN
        SELECT date_trunc('day', $2) INTO truncated_start;
        SELECT date_trunc('day', $3) INTO truncated_end;
    WHEN '1h' THEN
        SELECT date_trunc('hour', $2) INTO truncated_start;
        SELECT date_trunc('hour', $3) INTO truncated_end;
    ELSE
        SELECT trend_directory.index_to_timestamp(gran, trend_directory.timestamp_to_index(gran, $2)) INTO truncated_start;
        SELECT trend_directory.index_to_timestamp(gran, trend_directory.timestamp_to_index(gran, $3)) INTO truncated_end;
    END CASE;

    RETURN QUERY
    WITH trend_data AS (
        SELECT s.timestamp, s.count from trend_directory.trend_store_part_stats s
        JOIN trend_directory.trend_store_part p ON s.trend_store_part_id = p.id
        WHERE s.timestamp >= truncated_start and s.timestamp <= truncated_end and p.name = $1
    )
    SELECT t, coalesce(d.count, 0)::bigint
        FROM generate_series(truncated_start, truncated_end, gran) t
        LEFT JOIN trend_data d on d.timestamp = t ORDER BY t asc;
END;
$_$;


ALTER FUNCTION trend_directory.completeness(name, start timestamp with time zone, "end" timestamp with time zone) OWNER TO postgres;

--
-- Name: FUNCTION completeness(name, start timestamp with time zone, "end" timestamp with time zone); Type: COMMENT; Schema: trend_directory; Owner: postgres
--

COMMENT ON FUNCTION trend_directory.completeness(name, start timestamp with time zone, "end" timestamp with time zone) IS 'Return table with record counts grouped by timestamp';


--
-- Name: partition; Type: TABLE; Schema: trend_directory; Owner: postgres
--

CREATE TABLE trend_directory.partition (
    id integer NOT NULL,
    trend_store_part_id integer NOT NULL,
    name name NOT NULL,
    index integer NOT NULL,
    "from" timestamp with time zone NOT NULL,
    "to" timestamp with time zone NOT NULL,
    is_columnar boolean DEFAULT false NOT NULL
);


ALTER TABLE trend_directory.partition OWNER TO postgres;

--
-- Name: TABLE partition; Type: COMMENT; Schema: trend_directory; Owner: postgres
--

COMMENT ON TABLE trend_directory.partition IS 'The parts of a vertically partitioned trend store part.';


--
-- Name: convert_to_columnar(trend_directory.partition); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.convert_to_columnar(trend_directory.partition) RETURNS void
    LANGUAGE sql
    AS $_$
SELECT alter_table_set_access_method(format('%I.%I', trend_directory.partition_schema(), $1.name)::regclass, 'columnar');
UPDATE trend_directory.partition SET is_columnar = 'true' WHERE id = $1.id;
$_$;


ALTER FUNCTION trend_directory.convert_to_columnar(trend_directory.partition) OWNER TO postgres;

--
-- Name: create_base_table(trend_directory.trend_store_part); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.create_base_table(trend_directory.trend_store_part) RETURNS trend_directory.trend_store_part
    LANGUAGE sql STRICT SECURITY DEFINER
    AS $_$
SELECT public.action($1, trend_directory.create_base_table_sql($1))
$_$;


ALTER FUNCTION trend_directory.create_base_table(trend_directory.trend_store_part) OWNER TO postgres;

--
-- Name: create_base_table_sql(trend_directory.trend_store_part); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.create_base_table_sql(trend_directory.trend_store_part) RETURNS text[]
    LANGUAGE sql STABLE STRICT
    AS $_$
SELECT ARRAY[
    format(
        'CREATE TABLE %I.%I ('
        'entity_id integer NOT NULL, '
        '"timestamp" timestamp with time zone NOT NULL, '
        'created timestamp with time zone NOT NULL, '
        '%s'
        ') PARTITION BY RANGE ("timestamp");',
        trend_directory.base_table_schema(),
        trend_directory.base_table_name($1),
        array_to_string(ARRAY['job_id bigint NOT NULL'] || trend_directory.column_specs($1), ',')
    ),
    format(
        'ALTER TABLE %I.%I ADD PRIMARY KEY (entity_id, "timestamp");',
        trend_directory.base_table_schema(),
        trend_directory.base_table_name($1)
    ),
    format(
        'CREATE INDEX ON %I.%I USING btree (job_id)',
        trend_directory.base_table_schema(),
        trend_directory.base_table_name($1)
    ),
    format(
        'CREATE INDEX ON %I.%I USING btree (timestamp);',
        trend_directory.base_table_schema(),
        trend_directory.base_table_name($1)
    ),
    format(
        'SELECT create_distributed_table(''%I.%I'', ''entity_id'')',
        trend_directory.base_table_schema(),
        trend_directory.base_table_name($1)
    )
];
$_$;


ALTER FUNCTION trend_directory.create_base_table_sql(trend_directory.trend_store_part) OWNER TO postgres;

--
-- Name: create_generated_table_trends(trend_directory.trend_store_part, trend_directory.generated_trend_descr[]); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.create_generated_table_trends(trend_directory.trend_store_part, trend_directory.generated_trend_descr[]) RETURNS trend_directory.trend_store_part
    LANGUAGE sql
    AS $_$
SELECT trend_directory.add_generated_trends_to_trend_store_part(
  $1,
  array_agg(trend_directory.define_generated_table_trend($1.id, t))
) FROM unnest($2) AS t
$_$;


ALTER FUNCTION trend_directory.create_generated_table_trends(trend_directory.trend_store_part, trend_directory.generated_trend_descr[]) OWNER TO postgres;

--
-- Name: create_metrics_for_materialization(); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.create_metrics_for_materialization() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    INSERT INTO trend_directory.materialization_metrics(materialization_id)
    VALUES (NEW.id);

    RETURN NEW;
END;
$$;


ALTER FUNCTION trend_directory.create_metrics_for_materialization() OWNER TO postgres;

--
-- Name: create_missing_trend_store_part_stats(); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.create_missing_trend_store_part_stats() RETURNS void
    LANGUAGE sql
    AS $$
INSERT INTO trend_directory.trend_store_part_stats (trend_store_part_id, timestamp, modified, count)
  SELECT m.trend_store_part_id, m.timestamp, '2000-01-01 00:00:00+02', 0
    FROM trend_directory.modified m
      LEFT JOIN trend_directory.trend_store_part_stats s
      ON s.trend_store_part_id = m.trend_store_part_id AND s.timestamp = m.timestamp
      WHERE s IS NULL;
$$;


ALTER FUNCTION trend_directory.create_missing_trend_store_part_stats() OWNER TO postgres;

--
-- Name: FUNCTION create_missing_trend_store_part_stats(); Type: COMMENT; Schema: trend_directory; Owner: postgres
--

COMMENT ON FUNCTION trend_directory.create_missing_trend_store_part_stats() IS 'Create trend_store_part_stat where it does not exist yet.';


--
-- Name: create_partition(integer, integer); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.create_partition(trend_store_part_id integer, integer) RETURNS trend_directory.partition
    LANGUAGE plpgsql
    AS $_$
DECLARE
  tsp trend_directory.trend_store_part;
BEGIN
  SELECT * FROM trend_directory.trend_store_part WHERE id = $1 INTO tsp;
  RETURN trend_directory.create_partition(tsp, $2);
END;
$_$;


ALTER FUNCTION trend_directory.create_partition(trend_store_part_id integer, integer) OWNER TO postgres;

--
-- Name: create_partition(trend_directory.trend_store_part, integer); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.create_partition(trend_directory.trend_store_part, integer) RETURNS trend_directory.partition
    LANGUAGE plpgsql
    AS $_$
DECLARE
  result trend_directory.partition;
BEGIN
  PERFORM public.action($1, trend_directory.create_partition_sql($1, $2));

  INSERT INTO trend_directory.partition(trend_store_part_id, index, name, "from", "to")
    SELECT $1.id, $2, trend_directory.partition_name($1, $2), trend_directory.index_to_timestamp(trend_store.partition_size, $2), trend_directory.index_to_timestamp(trend_store.partition_size, $2 + 1)
    FROM trend_directory.trend_store WHERE id = $1.trend_store_id
    RETURNING * INTO result;
  RETURN result;
END;
$_$;


ALTER FUNCTION trend_directory.create_partition(trend_directory.trend_store_part, integer) OWNER TO postgres;

--
-- Name: create_partition(trend_directory.trend_store_part, timestamp with time zone); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.create_partition(trend_directory.trend_store_part, timestamp with time zone) RETURNS trend_directory.partition
    LANGUAGE sql
    AS $_$
SELECT trend_directory.create_partition($1, trend_directory.timestamp_to_index(trend_directory.get_partition_size($1), $2));
$_$;


ALTER FUNCTION trend_directory.create_partition(trend_directory.trend_store_part, timestamp with time zone) OWNER TO postgres;

--
-- Name: create_partition_sql(trend_directory.trend_store_part, integer); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.create_partition_sql(trend_directory.trend_store_part, integer) RETURNS text[]
    LANGUAGE sql STABLE
    AS $_$
SELECT ARRAY[
    format(
        'CREATE TABLE %I.%I '
        'PARTITION OF %I.%I '
        'FOR VALUES FROM (''%s'') TO (''%s'')',
        trend_directory.partition_schema(),
        trend_directory.partition_name($1, $2),
        trend_directory.base_table_schema(),
        trend_directory.base_table_name($1),
        trend_directory.index_to_timestamp(trend_store.partition_size, $2),
        trend_directory.index_to_timestamp(trend_store.partition_size, $2 + 1)
    )
]
FROM trend_directory.trend_store WHERE id = $1.trend_store_id;
$_$;


ALTER FUNCTION trend_directory.create_partition_sql(trend_directory.trend_store_part, integer) OWNER TO postgres;

--
-- Name: create_staging_table(trend_directory.trend_store_part); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.create_staging_table(trend_directory.trend_store_part) RETURNS trend_directory.trend_store_part
    LANGUAGE sql STRICT SECURITY DEFINER
    AS $_$
SELECT public.action($1, trend_directory.create_staging_table_sql($1));
$_$;


ALTER FUNCTION trend_directory.create_staging_table(trend_directory.trend_store_part) OWNER TO postgres;

--
-- Name: create_staging_table_sql(trend_directory.trend_store_part); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.create_staging_table_sql(trend_directory.trend_store_part) RETURNS text[]
    LANGUAGE sql STABLE
    AS $_$
SELECT ARRAY[
    format(
        'CREATE UNLOGGED TABLE %I.%I (entity_id integer, "timestamp" timestamp with time zone, created timestamp with time zone, job_id bigint%s);',
        trend_directory.staging_table_schema(),
        trend_directory.staging_table_name($1),
        (
            SELECT string_agg(format(', %I %s', t.name, t.data_type), ' ')
            FROM trend_directory.table_trend t
            WHERE t.trend_store_part_id = $1.id
        )
    ),
    format(
        'ALTER TABLE ONLY %I.%I ADD PRIMARY KEY (entity_id, "timestamp");',
        trend_directory.staging_table_schema(),
        trend_directory.staging_table_name($1)
    )
];
$_$;


ALTER FUNCTION trend_directory.create_staging_table_sql(trend_directory.trend_store_part) OWNER TO postgres;

--
-- Name: create_stats_on_creation(); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.create_stats_on_creation() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
  INSERT INTO trend_directory.trend_store_part_stats (trend_store_part_id, timestamp, modified, count)
    VALUES (NEW.trend_store_part_id, NEW.timestamp, '2000-01-01 00:00:00+02', 1);
  RETURN NEW;
END;
$$;


ALTER FUNCTION trend_directory.create_stats_on_creation() OWNER TO postgres;

--
-- Name: create_table_trends(trend_directory.trend_store_part, trend_directory.trend_descr[]); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.create_table_trends(trend_directory.trend_store_part, trend_directory.trend_descr[]) RETURNS trend_directory.trend_store_part
    LANGUAGE sql
    AS $_$
SELECT trend_directory.add_trends_to_trend_store_part(
  $1,
  array_agg(trend_directory.define_table_trend($1.id, t))
) FROM unnest($2) AS t
$_$;


ALTER FUNCTION trend_directory.create_table_trends(trend_directory.trend_store_part, trend_directory.trend_descr[]) OWNER TO postgres;

--
-- Name: create_trend_store(text, text, interval, interval, trend_directory.trend_store_part_descr[]); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.create_trend_store(data_source_name text, entity_type_name text, granularity interval, partition_size interval, parts trend_directory.trend_store_part_descr[]) RETURNS trend_directory.trend_store
    LANGUAGE plpgsql
    AS $_$
DECLARE
  dsource directory.data_source;
  etype directory.entity_type;
BEGIN
  SELECT * FROM directory.name_to_data_source($1) into dsource;
  SELECT * FROM directory.name_to_entity_type($2) into etype;
  RETURN trend_directory.initialize_trend_store(
    trend_directory.define_trend_store(dsource, etype, $3, $4, $5)
  );
END;
$_$;


ALTER FUNCTION trend_directory.create_trend_store(data_source_name text, entity_type_name text, granularity interval, partition_size interval, parts trend_directory.trend_store_part_descr[]) OWNER TO postgres;

--
-- Name: create_trend_store_part(integer, name); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.create_trend_store_part(trend_store_id integer, name name) RETURNS trend_directory.trend_store_part
    LANGUAGE sql
    AS $_$
SELECT trend_directory.initialize_trend_store_part(
    trend_directory.define_trend_store_part($1, $2)
  );
$_$;


ALTER FUNCTION trend_directory.create_trend_store_part(trend_store_id integer, name name) OWNER TO postgres;

--
-- Name: trend_view; Type: TABLE; Schema: trend_directory; Owner: postgres
--

CREATE TABLE trend_directory.trend_view (
    id integer NOT NULL,
    entity_type_id integer,
    data_source_id integer,
    granularity interval NOT NULL
);


ALTER TABLE trend_directory.trend_view OWNER TO postgres;

--
-- Name: TABLE trend_view; Type: COMMENT; Schema: trend_directory; Owner: postgres
--

COMMENT ON TABLE trend_directory.trend_view IS 'View based trend stores describing the properties like entity type, data granularity, etc.';


--
-- Name: create_trend_view(text, text, interval, trend_directory.trend_view_part_descr[]); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.create_trend_view(data_source_name text, entity_type_name text, granularity interval, trend_directory.trend_view_part_descr[]) RETURNS trend_directory.trend_view
    LANGUAGE plpgsql
    AS $_$
DECLARE
  dsource directory.data_source;
  etype directory.entity_type;
BEGIN
  SELECT directory.name_to_data_source($1) into dsource;
  SELECT directory.name_to_entity_type($2) into etype;
  RETURN trend_directory.initialize_trend_view(
    trend_directory.define_trend_view(dsource, etype, $3), $4
  );
END;
$_$;


ALTER FUNCTION trend_directory.create_trend_view(data_source_name text, entity_type_name text, granularity interval, trend_directory.trend_view_part_descr[]) OWNER TO postgres;

--
-- Name: create_view_sql(trend_directory.trend_view_part, text); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.create_view_sql(trend_directory.trend_view_part, sql text) RETURNS text[]
    LANGUAGE sql STABLE
    AS $_$
SELECT ARRAY[
    format('CREATE VIEW %I.%I AS %s;', trend_directory.view_schema(), trend_directory.view_name($1), $2)
];
$_$;


ALTER FUNCTION trend_directory.create_view_sql(trend_directory.trend_view_part, sql text) OWNER TO postgres;

--
-- Name: data_type_order(text); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.data_type_order(data_type text) RETURNS integer
    LANGUAGE plpgsql IMMUTABLE STRICT
    AS $$
BEGIN
    CASE data_type
        WHEN 'smallint' THEN
            RETURN 1;
        WHEN 'integer' THEN
            RETURN 2;
        WHEN 'bigint' THEN
            RETURN 3;
        WHEN 'real' THEN
            RETURN 4;
        WHEN 'double precision' THEN
            RETURN 5;
        WHEN 'numeric' THEN
            RETURN 6;
        WHEN 'timestamp without time zone' THEN
            RETURN 7;
        WHEN 'smallint[]' THEN
            RETURN 8;
        WHEN 'integer[]' THEN
            RETURN 9;
        WHEN 'numeric[]' THEN
            RETURN 10;
        WHEN 'text[]' THEN
            RETURN 11;
        WHEN 'text' THEN
            RETURN 12;
        WHEN NULL THEN
            RETURN NULL;
        ELSE
            RAISE EXCEPTION 'Unsupported data type: %', data_type;
    END CASE;
END;
$$;


ALTER FUNCTION trend_directory.data_type_order(data_type text) OWNER TO postgres;

--
-- Name: default_columnar_period(); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.default_columnar_period() RETURNS interval
    LANGUAGE sql IMMUTABLE
    AS $$
SELECT '2w'::interval;
$$;


ALTER FUNCTION trend_directory.default_columnar_period() OWNER TO postgres;

--
-- Name: function_materialization; Type: TABLE; Schema: trend_directory; Owner: postgres
--

CREATE TABLE trend_directory.function_materialization (
    id integer NOT NULL,
    materialization_id integer NOT NULL,
    src_function text NOT NULL
);


ALTER TABLE trend_directory.function_materialization OWNER TO postgres;

--
-- Name: TABLE function_materialization; Type: COMMENT; Schema: trend_directory; Owner: postgres
--

COMMENT ON TABLE trend_directory.function_materialization IS 'A ``function_materialization`` is a materialization that uses the data
from the function registered in the ``src_function`` column to populate
the target ``trend_store_part``.

The function must have the form of::
  
  (timestamp with time zone) -> TABLE(
    entity_id integer,
    timestamp timestamp with time zone,
    ...
  )';


--
-- Name: COLUMN function_materialization.id; Type: COMMENT; Schema: trend_directory; Owner: postgres
--

COMMENT ON COLUMN trend_directory.function_materialization.id IS 'The unique identifier of this function materialization';


--
-- Name: define_function_materialization(integer, interval, interval, interval, regproc); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.define_function_materialization(dst_trend_store_part_id integer, processing_delay interval, stability_delay interval, reprocessing_period interval, src_function regproc) RETURNS trend_directory.function_materialization
    LANGUAGE sql
    AS $_$
SELECT trend_directory.define_function_materialization($1, $2, $3, $4, $5, '{}'::jsonb)
$_$;


ALTER FUNCTION trend_directory.define_function_materialization(dst_trend_store_part_id integer, processing_delay interval, stability_delay interval, reprocessing_period interval, src_function regproc) OWNER TO postgres;

--
-- Name: FUNCTION define_function_materialization(dst_trend_store_part_id integer, processing_delay interval, stability_delay interval, reprocessing_period interval, src_function regproc); Type: COMMENT; Schema: trend_directory; Owner: postgres
--

COMMENT ON FUNCTION trend_directory.define_function_materialization(dst_trend_store_part_id integer, processing_delay interval, stability_delay interval, reprocessing_period interval, src_function regproc) IS 'Define a materialization that uses a function as source';


--
-- Name: define_function_materialization(integer, interval, interval, interval, regproc, jsonb); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.define_function_materialization(dst_trend_store_part_id integer, processing_delay interval, stability_delay interval, reprocessing_period interval, src_function regproc, description jsonb) RETURNS trend_directory.function_materialization
    LANGUAGE sql
    AS $_$
INSERT INTO trend_directory.function_materialization(materialization_id, src_function)
VALUES((trend_directory.define_materialization($1, $2, $3, $4, $6)).id, $5::text)
ON CONFLICT DO NOTHING
RETURNING *;
$_$;


ALTER FUNCTION trend_directory.define_function_materialization(dst_trend_store_part_id integer, processing_delay interval, stability_delay interval, reprocessing_period interval, src_function regproc, description jsonb) OWNER TO postgres;

--
-- Name: FUNCTION define_function_materialization(dst_trend_store_part_id integer, processing_delay interval, stability_delay interval, reprocessing_period interval, src_function regproc, description jsonb); Type: COMMENT; Schema: trend_directory; Owner: postgres
--

COMMENT ON FUNCTION trend_directory.define_function_materialization(dst_trend_store_part_id integer, processing_delay interval, stability_delay interval, reprocessing_period interval, src_function regproc, description jsonb) IS 'Define a materialization that uses a function as source';


--
-- Name: define_generated_table_trend(integer, trend_directory.generated_trend_descr); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.define_generated_table_trend(trend_store_part_id integer, trend_directory.generated_trend_descr) RETURNS trend_directory.generated_table_trend
    LANGUAGE sql
    AS $_$
INSERT INTO trend_directory.generated_table_trend (trend_store_part_id, name, data_type, expression, extra_data, description)
VALUES ($1, $2.name, $2.data_type, $2.expression, $2.extra_data, $2.description);
SELECT * FROM trend_directory.generated_table_trend WHERE trend_store_part_id = $1 AND name = $2.name;
$_$;


ALTER FUNCTION trend_directory.define_generated_table_trend(trend_store_part_id integer, trend_directory.generated_trend_descr) OWNER TO postgres;

--
-- Name: define_generated_table_trends(trend_directory.trend_store_part, trend_directory.generated_trend_descr[]); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.define_generated_table_trends(trend_directory.trend_store_part, trend_directory.generated_trend_descr[]) RETURNS trend_directory.trend_store_part
    LANGUAGE plpgsql
    AS $_$
BEGIN
  INSERT INTO trend_directory.generated_table_trend(trend_store_part_id, name, data_type, expression, extra_data, description) (
    SELECT $1.id, name, data_type, expression, extra_data, description
    FROM unnest($2)
  );

  RETURN $1;
END;
$_$;


ALTER FUNCTION trend_directory.define_generated_table_trends(trend_directory.trend_store_part, trend_directory.generated_trend_descr[]) OWNER TO postgres;

--
-- Name: define_materialization(integer, interval, interval, interval, jsonb); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.define_materialization(dst_trend_store_part_id integer, processing_delay interval, stability_delay interval, reprocessing_period interval, description jsonb) RETURNS trend_directory.materialization
    LANGUAGE sql
    AS $_$
INSERT INTO trend_directory.materialization(dst_trend_store_part_id, processing_delay, stability_delay, reprocessing_period, description)
VALUES ($1, $2, $3, $4, $5)
ON CONFLICT DO NOTHING
RETURNING *;
$_$;


ALTER FUNCTION trend_directory.define_materialization(dst_trend_store_part_id integer, processing_delay interval, stability_delay interval, reprocessing_period interval, description jsonb) OWNER TO postgres;

--
-- Name: FUNCTION define_materialization(dst_trend_store_part_id integer, processing_delay interval, stability_delay interval, reprocessing_period interval, description jsonb); Type: COMMENT; Schema: trend_directory; Owner: postgres
--

COMMENT ON FUNCTION trend_directory.define_materialization(dst_trend_store_part_id integer, processing_delay interval, stability_delay interval, reprocessing_period interval, description jsonb) IS 'Define a materialization';


--
-- Name: define_table_trend(integer, trend_directory.trend_descr); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.define_table_trend(trend_store_part_id integer, trend_directory.trend_descr) RETURNS trend_directory.table_trend
    LANGUAGE sql
    AS $_$
INSERT INTO trend_directory.table_trend (trend_store_part_id, name, data_type, description, time_aggregation, entity_aggregation)
VALUES ($1, $2.name, $2.data_type, $2.description, $2.time_aggregation, $2.entity_aggregation);
SELECT * FROM trend_directory.table_trend WHERE trend_store_part_id = $1 AND name = $2.name;
$_$;


ALTER FUNCTION trend_directory.define_table_trend(trend_store_part_id integer, trend_directory.trend_descr) OWNER TO postgres;

--
-- Name: define_table_trends(trend_directory.trend_store_part, trend_directory.trend_descr[]); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.define_table_trends(trend_directory.trend_store_part, trend_directory.trend_descr[]) RETURNS trend_directory.trend_store_part
    LANGUAGE plpgsql
    AS $_$
BEGIN
  INSERT INTO trend_directory.table_trend(name, data_type, trend_store_part_id, description, time_aggregation, entity_aggregation, extra_data) (
    SELECT name, data_type, $1.id, description, time_aggregation, entity_aggregation, extra_data
    FROM unnest($2)
  );
  RETURN $1;
END;
$_$;


ALTER FUNCTION trend_directory.define_table_trends(trend_directory.trend_store_part, trend_directory.trend_descr[]) OWNER TO postgres;

--
-- Name: define_trend_store(trend_directory.trend_store, trend_directory.trend_store_part_descr[]); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.define_trend_store(trend_directory.trend_store, trend_directory.trend_store_part_descr[]) RETURNS trend_directory.trend_store
    LANGUAGE sql
    AS $_$
SELECT trend_directory.define_trend_store_part($1.id, name, trends, generated_trends)
FROM unnest($2);

SELECT $1;
$_$;


ALTER FUNCTION trend_directory.define_trend_store(trend_directory.trend_store, trend_directory.trend_store_part_descr[]) OWNER TO postgres;

--
-- Name: define_trend_store(directory.data_source, directory.entity_type, interval, interval); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.define_trend_store(directory.data_source, directory.entity_type, granularity interval, partition_size interval) RETURNS trend_directory.trend_store
    LANGUAGE sql
    AS $_$
INSERT INTO trend_directory.trend_store (
    data_source_id,
    entity_type_id,
    granularity,
    partition_size
)
VALUES (
    $1.id,
    $2.id,
    $3,
    $4
) RETURNING *;
$_$;


ALTER FUNCTION trend_directory.define_trend_store(directory.data_source, directory.entity_type, granularity interval, partition_size interval) OWNER TO postgres;

--
-- Name: define_trend_store(directory.data_source, directory.entity_type, interval, interval, trend_directory.trend_store_part_descr[]); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.define_trend_store(directory.data_source, directory.entity_type, granularity interval, partition_size interval, trends trend_directory.trend_store_part_descr[]) RETURNS trend_directory.trend_store
    LANGUAGE sql
    AS $_$
SELECT trend_directory.define_trend_store(
    trend_directory.define_trend_store($1, $2, $3, $4),
    $5
);
$_$;


ALTER FUNCTION trend_directory.define_trend_store(directory.data_source, directory.entity_type, granularity interval, partition_size interval, trends trend_directory.trend_store_part_descr[]) OWNER TO postgres;

--
-- Name: define_trend_store_part(integer, name); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.define_trend_store_part(trend_store_id integer, name name) RETURNS trend_directory.trend_store_part
    LANGUAGE sql
    AS $_$
INSERT INTO trend_directory.trend_store_part (trend_store_id, name)
VALUES ($1, $2)
RETURNING *;
$_$;


ALTER FUNCTION trend_directory.define_trend_store_part(trend_store_id integer, name name) OWNER TO postgres;

--
-- Name: define_trend_store_part(integer, name, trend_directory.trend_descr[], trend_directory.generated_trend_descr[]); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.define_trend_store_part(trend_store_id integer, name name, trends trend_directory.trend_descr[], generated_trends trend_directory.generated_trend_descr[]) RETURNS trend_directory.trend_store_part
    LANGUAGE sql
    AS $_$
SELECT 
trend_directory.define_generated_table_trends(
  trend_directory.define_table_trends(
      trend_directory.define_trend_store_part($1, $2),
      $3
  ),
  $4
);
$_$;


ALTER FUNCTION trend_directory.define_trend_store_part(trend_store_id integer, name name, trends trend_directory.trend_descr[], generated_trends trend_directory.generated_trend_descr[]) OWNER TO postgres;

--
-- Name: define_trend_view(directory.data_source, directory.entity_type, interval); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.define_trend_view(directory.data_source, directory.entity_type, granularity interval) RETURNS trend_directory.trend_view
    LANGUAGE sql
    AS $_$
INSERT INTO trend_directory.trend_view (
    data_source_id,
    entity_type_id,
    granularity
)
VALUES (
    $1.id,
    $2.id,
    $3
) RETURNING *;
$_$;


ALTER FUNCTION trend_directory.define_trend_view(directory.data_source, directory.entity_type, granularity interval) OWNER TO postgres;

--
-- Name: view_materialization; Type: TABLE; Schema: trend_directory; Owner: postgres
--

CREATE TABLE trend_directory.view_materialization (
    id integer NOT NULL,
    materialization_id integer NOT NULL,
    src_view text NOT NULL
);


ALTER TABLE trend_directory.view_materialization OWNER TO postgres;

--
-- Name: TABLE view_materialization; Type: COMMENT; Schema: trend_directory; Owner: postgres
--

COMMENT ON TABLE trend_directory.view_materialization IS 'A ``view_materialization`` is a materialization that uses the data
from the view registered in the ``src_view`` column to populate
the target ``trend_store_part``.';


--
-- Name: COLUMN view_materialization.id; Type: COMMENT; Schema: trend_directory; Owner: postgres
--

COMMENT ON COLUMN trend_directory.view_materialization.id IS 'The unique identifier of this view materialization';


--
-- Name: define_view_materialization(integer, interval, interval, interval, regclass); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.define_view_materialization(dst_trend_store_part_id integer, processing_delay interval, stability_delay interval, reprocessing_period interval, src_view regclass) RETURNS trend_directory.view_materialization
    LANGUAGE sql
    AS $_$
SELECT trend_directory.define_view_materialization($1, $2, $3, $4, $5, '{}'::jsonb);
$_$;


ALTER FUNCTION trend_directory.define_view_materialization(dst_trend_store_part_id integer, processing_delay interval, stability_delay interval, reprocessing_period interval, src_view regclass) OWNER TO postgres;

--
-- Name: FUNCTION define_view_materialization(dst_trend_store_part_id integer, processing_delay interval, stability_delay interval, reprocessing_period interval, src_view regclass); Type: COMMENT; Schema: trend_directory; Owner: postgres
--

COMMENT ON FUNCTION trend_directory.define_view_materialization(dst_trend_store_part_id integer, processing_delay interval, stability_delay interval, reprocessing_period interval, src_view regclass) IS 'Define a materialization that uses a view as source';


--
-- Name: define_view_materialization(integer, interval, interval, interval, regclass, jsonb); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.define_view_materialization(dst_trend_store_part_id integer, processing_delay interval, stability_delay interval, reprocessing_period interval, src_view regclass, description jsonb) RETURNS trend_directory.view_materialization
    LANGUAGE sql
    AS $_$
INSERT INTO trend_directory.view_materialization(materialization_id, src_view)
VALUES((trend_directory.define_materialization($1, $2, $3, $4, $6)).id, $5) RETURNING *;
$_$;


ALTER FUNCTION trend_directory.define_view_materialization(dst_trend_store_part_id integer, processing_delay interval, stability_delay interval, reprocessing_period interval, src_view regclass, description jsonb) OWNER TO postgres;

--
-- Name: FUNCTION define_view_materialization(dst_trend_store_part_id integer, processing_delay interval, stability_delay interval, reprocessing_period interval, src_view regclass, description jsonb); Type: COMMENT; Schema: trend_directory; Owner: postgres
--

COMMENT ON FUNCTION trend_directory.define_view_materialization(dst_trend_store_part_id integer, processing_delay interval, stability_delay interval, reprocessing_period interval, src_view regclass, description jsonb) IS 'Define a materialization that uses a view as source';


--
-- Name: deinitialize_trend_store_part(integer); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.deinitialize_trend_store_part(trend_store_part_id integer) RETURNS void
    LANGUAGE plpgsql
    AS $_$
DECLARE
  tsp trend_directory.trend_store_part;
BEGIN
  SELECT * FROM trend_directory.trend_store_part
    WHERE trend_store_part.id = $1 INTO tsp;
  EXECUTE trend_directory.drop_base_table_sql(tsp);
  EXECUTE trend_directory.drop_staging_table_sql(tsp);
END;
$_$;


ALTER FUNCTION trend_directory.deinitialize_trend_store_part(trend_store_part_id integer) OWNER TO postgres;

--
-- Name: deinitialize_trend_store_part(trend_directory.trend_store_part); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.deinitialize_trend_store_part(trend_directory.trend_store_part) RETURNS trend_directory.trend_store_part
    LANGUAGE sql
    AS $_$
SELECT trend_directory.drop_base_table($1);
SELECT trend_directory.drop_staging_table($1);
$_$;


ALTER FUNCTION trend_directory.deinitialize_trend_store_part(trend_directory.trend_store_part) OWNER TO postgres;

--
-- Name: FUNCTION deinitialize_trend_store_part(trend_directory.trend_store_part); Type: COMMENT; Schema: trend_directory; Owner: postgres
--

COMMENT ON FUNCTION trend_directory.deinitialize_trend_store_part(trend_directory.trend_store_part) IS 'Remove all database objects related to the table trend store part';


--
-- Name: delete_obsolete_state(); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.delete_obsolete_state() RETURNS integer
    LANGUAGE plpgsql
    AS $$
DECLARE
    count integer;
BEGIN
    DELETE FROM trend_directory.state
    USING trend_directory.obsolete_state
    WHERE
        state.materialization_id = obsolete_state.materialization_id AND
        state.timestamp = obsolete_state.timestamp;

    GET DIAGNOSTICS count = ROW_COUNT;

    RETURN count;
END;
$$;


ALTER FUNCTION trend_directory.delete_obsolete_state() OWNER TO postgres;

--
-- Name: delete_trend_store(integer); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.delete_trend_store(integer) RETURNS void
    LANGUAGE sql
    AS $_$
SELECT trend_directory.deinitialize_trend_store_part(part.id)
FROM trend_directory.trend_store_part AS part
WHERE trend_store_id = $1;

DELETE FROM trend_directory.trend_store WHERE id = $1;
$_$;


ALTER FUNCTION trend_directory.delete_trend_store(integer) OWNER TO postgres;

--
-- Name: delete_trend_store(text, text, interval); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.delete_trend_store(data_source_name text, entity_type_name text, granularity interval) RETURNS void
    LANGUAGE sql
    AS $_$
SELECT trend_directory.delete_trend_store((trend_directory.get_trend_store($1, $2, $3)).id);
$_$;


ALTER FUNCTION trend_directory.delete_trend_store(data_source_name text, entity_type_name text, granularity interval) OWNER TO postgres;

--
-- Name: delete_trend_store_part(trend_directory.trend_store_part); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.delete_trend_store_part(trend_directory.trend_store_part) RETURNS void
    LANGUAGE plpgsql
    AS $_$
DECLARE
    table_name text;
BEGIN

    EXECUTE format(
        'DROP TABLE IF EXISTS trend.%I CASCADE',
        trend_directory.base_table_name($1)
    );

    DELETE FROM trend_directory.trend_store_part tp WHERE tp.id = $1.id;
END;
$_$;


ALTER FUNCTION trend_directory.delete_trend_store_part(trend_directory.trend_store_part) OWNER TO postgres;

--
-- Name: delete_trend_view(trend_directory.trend_view); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.delete_trend_view(trend_directory.trend_view) RETURNS void
    LANGUAGE sql
    AS $_$
SELECT trend_directory.drop_view(tvp)
  FROM trend_directory.trend_view_part tvp
  WHERE tvp.trend_view_id = $1.id;
DELETE FROM trend_directory.trend_view WHERE id = $1.id;
$_$;


ALTER FUNCTION trend_directory.delete_trend_view(trend_directory.trend_view) OWNER TO postgres;

--
-- Name: drop_base_table(trend_directory.trend_store_part); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.drop_base_table(trend_directory.trend_store_part) RETURNS trend_directory.trend_store_part
    LANGUAGE sql
    AS $_$
SELECT public.action($1, trend_directory.drop_base_table_sql($1))
$_$;


ALTER FUNCTION trend_directory.drop_base_table(trend_directory.trend_store_part) OWNER TO postgres;

--
-- Name: drop_base_table_sql(trend_directory.trend_store_part); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.drop_base_table_sql(trend_directory.trend_store_part) RETURNS text
    LANGUAGE sql STABLE STRICT
    AS $_$
SELECT format(
    'DROP TABLE %I.%I',
    trend_directory.base_table_schema(),
    trend_directory.base_table_name($1)
);
$_$;


ALTER FUNCTION trend_directory.drop_base_table_sql(trend_directory.trend_store_part) OWNER TO postgres;

--
-- Name: drop_staging_table(trend_directory.trend_store_part); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.drop_staging_table(trend_directory.trend_store_part) RETURNS trend_directory.trend_store_part
    LANGUAGE sql STRICT SECURITY DEFINER
    AS $_$
SELECT public.action($1, trend_directory.drop_staging_table_sql($1));
$_$;


ALTER FUNCTION trend_directory.drop_staging_table(trend_directory.trend_store_part) OWNER TO postgres;

--
-- Name: drop_staging_table_sql(trend_directory.trend_store_part); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.drop_staging_table_sql(trend_directory.trend_store_part) RETURNS text
    LANGUAGE sql STABLE
    AS $_$
SELECT format(
    'DROP TABLE %I.%I',
    trend_directory.staging_table_schema(),
    trend_directory.staging_table_name($1)
);
$_$;


ALTER FUNCTION trend_directory.drop_staging_table_sql(trend_directory.trend_store_part) OWNER TO postgres;

--
-- Name: drop_view(trend_directory.trend_view_part); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.drop_view(trend_directory.trend_view_part) RETURNS trend_directory.trend_view_part
    LANGUAGE sql
    AS $_$
SELECT public.action($1, trend_directory.drop_view_sql($1));

SELECT $1;
$_$;


ALTER FUNCTION trend_directory.drop_view(trend_directory.trend_view_part) OWNER TO postgres;

--
-- Name: drop_view_sql(trend_directory.trend_view_part); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.drop_view_sql(trend_directory.trend_view_part) RETURNS text
    LANGUAGE sql STABLE
    AS $_$
SELECT format(
    'DROP VIEW IF EXISTS %I.%I',
    trend_directory.view_schema(),
    trend_directory.view_name($1)
);
$_$;


ALTER FUNCTION trend_directory.drop_view_sql(trend_directory.trend_view_part) OWNER TO postgres;

--
-- Name: dst_trend_store_part(integer); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.dst_trend_store_part(materialization_id integer) RETURNS trend_directory.trend_store_part
    LANGUAGE sql STABLE
    AS $_$
SELECT tsp.* FROM trend_directory.trend_store_part tsp
  JOIN trend_directory.materialization m ON tsp.id = m.dst_trend_store_part_id
  WHERE m.id = $1;
$_$;


ALTER FUNCTION trend_directory.dst_trend_store_part(materialization_id integer) OWNER TO postgres;

--
-- Name: dst_trend_store_part(trend_directory.function_materialization); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.dst_trend_store_part(trend_directory.function_materialization) RETURNS trend_directory.trend_store_part
    LANGUAGE sql STABLE
    AS $_$
SELECT p.*
FROM trend_directory.trend_store_part p
JOIN trend_directory.materialization m ON m.dst_trend_store_part_id = p.id
WHERE m.id = $1.materialization_id;
$_$;


ALTER FUNCTION trend_directory.dst_trend_store_part(trend_directory.function_materialization) OWNER TO postgres;

--
-- Name: dst_trend_store_part(trend_directory.materialization); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.dst_trend_store_part(trend_directory.materialization) RETURNS trend_directory.trend_store_part
    LANGUAGE sql STABLE
    AS $_$
SELECT * FROM trend_directory.trend_store_part WHERE id = $1.dst_trend_store_part_id;
$_$;


ALTER FUNCTION trend_directory.dst_trend_store_part(trend_directory.materialization) OWNER TO postgres;

--
-- Name: dst_trend_store_part(trend_directory.view_materialization); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.dst_trend_store_part(trend_directory.view_materialization) RETURNS trend_directory.trend_store_part
    LANGUAGE sql STABLE
    AS $_$
SELECT p.*
FROM trend_directory.trend_store_part p
JOIN trend_directory.materialization m ON m.dst_trend_store_part_id = p.id
WHERE m.id = $1.materialization_id;
$_$;


ALTER FUNCTION trend_directory.dst_trend_store_part(trend_directory.view_materialization) OWNER TO postgres;

--
-- Name: fingerprint_function_name(trend_directory.materialization); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.fingerprint_function_name(trend_directory.materialization) RETURNS name
    LANGUAGE sql
    AS $_$
SELECT format('%s_fingerprint', trend_directory.materialization_to_char($1.id))::name;
$_$;


ALTER FUNCTION trend_directory.fingerprint_function_name(trend_directory.materialization) OWNER TO postgres;

--
-- Name: function_materialization_columns_part(integer); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.function_materialization_columns_part(materialization_id integer) RETURNS text
    LANGUAGE sql STABLE
    AS $_$
SELECT trend_directory.columns_part(m.dst_trend_store_part_id)
FROM trend_directory.materialization m
WHERE id = $1;
$_$;


ALTER FUNCTION trend_directory.function_materialization_columns_part(materialization_id integer) OWNER TO postgres;

--
-- Name: FUNCTION function_materialization_columns_part(materialization_id integer); Type: COMMENT; Schema: trend_directory; Owner: postgres
--

COMMENT ON FUNCTION trend_directory.function_materialization_columns_part(materialization_id integer) IS 'Return the comma separated, quoted list of column names to be used in queries';


--
-- Name: function_materialization_transfer(integer, timestamp with time zone); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.function_materialization_transfer(materialization_id integer, "timestamp" timestamp with time zone) RETURNS integer
    LANGUAGE plpgsql
    AS $_$
DECLARE
    mat trend_directory.function_materialization;
    columns_part text;
    row_count integer;
    job_id bigint;
BEGIN
    SELECT * FROM trend_directory.function_materialization fm WHERE fm.materialization_id = $1 into mat;
    IF mat IS NULL
      THEN RETURN NULL;
    END IF;
    SELECT logging.start_job(format('{"function_materialization": "%s", "timestamp": "%s"}', m::text, $2::text)::jsonb) INTO job_id
    FROM trend_directory.materialization m WHERE id = $1;

    SELECT trend_directory.function_materialization_columns_part($1) INTO columns_part;

    EXECUTE format(
        'INSERT INTO trend.%I (entity_id, timestamp, created, job_id, %s) SELECT entity_id, timestamp, now(), %s, %s FROM %s($1)',
        (trend_directory.dst_trend_store_part($1)).name,
        columns_part,
        job_id,
        columns_part,
        mat.src_function::regproc
    ) USING timestamp;

    GET DIAGNOSTICS row_count = ROW_COUNT;

    PERFORM logging.end_job(job_id);

    RETURN row_count;
END;
$_$;


ALTER FUNCTION trend_directory.function_materialization_transfer(materialization_id integer, "timestamp" timestamp with time zone) OWNER TO postgres;

--
-- Name: generated_trend_column_spec(integer); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.generated_trend_column_spec(generated_table_trend_id integer) RETURNS text
    LANGUAGE plpgsql IMMUTABLE
    AS $_$
DECLARE
  tt trend_directory.generated_table_trend;
BEGIN
  SELECT * FROM trend_directory.generated_table_trend WHERE id = $1 INTO tt;
  RETURN trend_directory.column_spec(tt);
END;
$_$;


ALTER FUNCTION trend_directory.generated_trend_column_spec(generated_table_trend_id integer) OWNER TO postgres;

--
-- Name: get_count(integer, timestamp with time zone); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.get_count(trend_store_part_id integer, "timestamp" timestamp with time zone) RETURNS integer
    LANGUAGE plpgsql STABLE
    AS $_$
DECLARE
  result integer;
BEGIN
  EXECUTE FORMAT('SELECT COUNT(*)::integer FROM trend.%I WHERE timestamp = ''%s''',
    trend_directory.base_table_name_by_trend_store_part_id($1),
    $2) INTO result;
  RETURN result;
END;
$_$;


ALTER FUNCTION trend_directory.get_count(trend_store_part_id integer, "timestamp" timestamp with time zone) OWNER TO postgres;

--
-- Name: get_default_partition_size(interval); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.get_default_partition_size(granularity interval) RETURNS interval
    LANGUAGE sql IMMUTABLE STRICT
    AS $_$
SELECT CASE $1
    WHEN '300'::interval THEN
        '3 hours'::interval
    WHEN '900'::interval THEN
        '6 hours'::interval
    WHEN '1800'::interval THEN
        '6 hours'::interval
    WHEN '1 hour'::interval THEN
        '1 day'::interval
    WHEN '12 hours'::interval THEN
        '7 days'::interval
    WHEN '1 day'::interval THEN
        '7 days'::interval
    WHEN '1 week'::interval THEN
        '1 month'::interval
    WHEN '1 month'::interval THEN
        '1 year'::interval
    END;
$_$;


ALTER FUNCTION trend_directory.get_default_partition_size(granularity interval) OWNER TO postgres;

--
-- Name: FUNCTION get_default_partition_size(granularity interval); Type: COMMENT; Schema: trend_directory; Owner: postgres
--

COMMENT ON FUNCTION trend_directory.get_default_partition_size(granularity interval) IS 'Return the default partition size in seconds for a particular granularity';


--
-- Name: get_index_on(name, name); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.get_index_on(name, name) RETURNS name
    LANGUAGE sql STABLE
    AS $_$
SELECT
        i.relname
FROM
        pg_class t,
        pg_class i,
        pg_index ix,
        pg_attribute a
WHERE
        t.oid = ix.indrelid
        and i.oid = ix.indexrelid
        and a.attrelid = t.oid
        and a.attnum = ANY(ix.indkey)
        and t.relkind = 'r'
        and t.relname = $1
        and a.attname = $2;
$_$;


ALTER FUNCTION trend_directory.get_index_on(name, name) OWNER TO postgres;

--
-- Name: get_max_modified(trend_directory.trend_store, timestamp with time zone); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.get_max_modified(trend_directory.trend_store, timestamp with time zone) RETURNS timestamp with time zone
    LANGUAGE plpgsql STABLE
    AS $_$
DECLARE
    max_modified timestamp with time zone;
BEGIN
    EXECUTE format(
        'SELECT max(modified) FROM trend_directory.%I WHERE timestamp = $1',
        trend_directory.base_table_name($1)
    ) INTO max_modified USING $2;

    RETURN max_modified;
END;
$_$;


ALTER FUNCTION trend_directory.get_max_modified(trend_directory.trend_store, timestamp with time zone) OWNER TO postgres;

--
-- Name: get_most_recent_timestamp(interval, timestamp with time zone); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.get_most_recent_timestamp(dest_granularity interval, ts timestamp with time zone) RETURNS timestamp with time zone
    LANGUAGE plpgsql IMMUTABLE
    AS $$
DECLARE
    minute integer;
    rounded_minutes integer;
BEGIN
    IF dest_granularity < '1 hour'::interval THEN
        minute := extract(minute FROM ts);
        rounded_minutes := minute - (minute % (dest_granularity / 60));

        return date_trunc('hour', ts) + (rounded_minutes || 'minutes')::INTERVAL;
    ELSIF dest_granularity = '1 hour'::interval THEN
        return date_trunc('hour', ts);
    ELSIF dest_granularity = '1 day'::interval THEN
        return date_trunc('day', ts);
    ELSIF dest_granularity = '1 week'::interval THEN
        return date_trunc('week', ts);
    ELSE
        RAISE EXCEPTION 'Invalid granularity: %', dest_granularity;
    END IF;
END;
$$;


ALTER FUNCTION trend_directory.get_most_recent_timestamp(dest_granularity interval, ts timestamp with time zone) OWNER TO postgres;

--
-- Name: get_most_recent_timestamp(character varying, timestamp with time zone); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.get_most_recent_timestamp(dest_granularity character varying, ts timestamp with time zone) RETURNS timestamp with time zone
    LANGUAGE plpgsql IMMUTABLE
    AS $$
DECLARE
    minute integer;
    rounded_minutes integer;
    seconds integer;
BEGIN
    IF trend_directory.is_integer(dest_granularity) THEN
        seconds = cast(dest_granularity as integer);

        return trend_directory.get_most_recent_timestamp(seconds, ts);
    ELSIF dest_granularity = 'month' THEN
        return date_trunc('month', ts);
    ELSE
        RAISE EXCEPTION 'Invalid granularity: %', dest_granularity;
    END IF;

    return seconds;
END;
$$;


ALTER FUNCTION trend_directory.get_most_recent_timestamp(dest_granularity character varying, ts timestamp with time zone) OWNER TO postgres;

--
-- Name: get_or_create_trend_store_part(integer, name); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.get_or_create_trend_store_part(trend_store_id integer, name name) RETURNS trend_directory.trend_store_part
    LANGUAGE sql
    AS $_$
SELECT COALESCE(
  trend_directory.get_trend_store_part($1, $2),
  trend_directory.create_trend_store_part($1, $2)
);
$_$;


ALTER FUNCTION trend_directory.get_or_create_trend_store_part(trend_store_id integer, name name) OWNER TO postgres;

--
-- Name: get_partition_size(trend_directory.trend_store_part); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.get_partition_size(trend_directory.trend_store_part) RETURNS interval
    LANGUAGE sql STABLE
    AS $_$
SELECT partition_size FROM trend_directory.trend_store WHERE trend_store.id = $1.trend_store_id;
$_$;


ALTER FUNCTION trend_directory.get_partition_size(trend_directory.trend_store_part) OWNER TO postgres;

--
-- Name: get_table_name_for_trend(text, text, interval); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.get_table_name_for_trend(trend text, entity text, granularity interval) RETURNS name
    LANGUAGE sql STABLE
    AS $_$
SELECT tsp.name FROM trend_directory.table_trend t
  JOIN trend_directory.trend_store_part tsp ON tsp.id = t.trend_store_part_id
  JOIN trend_directory.trend_store ts ON ts.id = tsp.trend_store_id
  JOIN directory.entity_type et ON et.id = ts.entity_type_id
  WHERE t.name = $1
    AND ts.granularity = $3
    AND et.name = $2;
$_$;


ALTER FUNCTION trend_directory.get_table_name_for_trend(trend text, entity text, granularity interval) OWNER TO postgres;

--
-- Name: get_table_trend(trend_directory.trend_store_part, name); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.get_table_trend(trend_directory.trend_store_part, name) RETURNS trend_directory.table_trend
    LANGUAGE sql STABLE
    AS $_$
SELECT table_trend
FROM trend_directory.table_trend
WHERE trend_store_part_id = $1.id AND name = $2;
$_$;


ALTER FUNCTION trend_directory.get_table_trend(trend_directory.trend_store_part, name) OWNER TO postgres;

--
-- Name: get_timestamp_for(interval, timestamp with time zone); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.get_timestamp_for(granularity interval, ts timestamp with time zone) RETURNS timestamp with time zone
    LANGUAGE plpgsql IMMUTABLE
    AS $_$
DECLARE
    most_recent_timestamp timestamp with time zone;
BEGIN
    most_recent_timestamp = trend_directory.get_most_recent_timestamp($1, $2);

    IF most_recent_timestamp != ts THEN
        IF granularity = 86400 THEN
            return most_recent_timestamp + ('1 day')::INTERVAL;
        ELSE
            return most_recent_timestamp + ($1 || ' seconds')::INTERVAL;
        END IF;
    ELSE
        return most_recent_timestamp;
    END IF;
END;
$_$;


ALTER FUNCTION trend_directory.get_timestamp_for(granularity interval, ts timestamp with time zone) OWNER TO postgres;

--
-- Name: get_timestamp_for(character varying, timestamp with time zone); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.get_timestamp_for(granularity character varying, ts timestamp with time zone) RETURNS timestamp with time zone
    LANGUAGE plpgsql IMMUTABLE
    AS $_$
DECLARE
    most_recent_timestamp timestamp with time zone;
BEGIN
    most_recent_timestamp = trend_directory.get_most_recent_timestamp($1, $2);

    IF most_recent_timestamp != ts THEN
        IF trend_directory.is_integer(granularity) THEN
            IF granularity = '86400' THEN
                return most_recent_timestamp + ('1 day')::INTERVAL;
            ELSE
                return most_recent_timestamp + ($1 || ' seconds')::INTERVAL;
            END IF;
        ELSIF granularity = 'month' THEN
            return most_recent_timestamp + '1 month'::INTERVAL;
        END IF;
    ELSE
        return most_recent_timestamp;
    END IF;
END;
$_$;


ALTER FUNCTION trend_directory.get_timestamp_for(granularity character varying, ts timestamp with time zone) OWNER TO postgres;

--
-- Name: get_trend_if_defined(trend_directory.table_trend, trend_directory.trend_descr[]); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.get_trend_if_defined(trend trend_directory.table_trend, trends trend_directory.trend_descr[]) RETURNS name
    LANGUAGE sql
    AS $_$
SELECT t.name FROM trend_directory.table_trend t JOIN unnest($2) t2
  ON t.name = t2.name WHERE t.id = $1.id
$_$;


ALTER FUNCTION trend_directory.get_trend_if_defined(trend trend_directory.table_trend, trends trend_directory.trend_descr[]) OWNER TO postgres;

--
-- Name: FUNCTION get_trend_if_defined(trend trend_directory.table_trend, trends trend_directory.trend_descr[]); Type: COMMENT; Schema: trend_directory; Owner: postgres
--

COMMENT ON FUNCTION trend_directory.get_trend_if_defined(trend trend_directory.table_trend, trends trend_directory.trend_descr[]) IS 'Return the trend, but only if it is a trend defined by trends';


--
-- Name: get_trend_store(text, text, interval); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.get_trend_store(data_source_name text, entity_type_name text, granularity interval) RETURNS trend_directory.trend_store
    LANGUAGE sql STABLE
    AS $_$
SELECT ts
FROM trend_directory.trend_store ts
JOIN directory.data_source ds ON ds.id = ts.data_source_id
JOIN directory.entity_type et ON et.id = ts.entity_type_id
WHERE
    lower(ds.name) = lower($1) AND
    lower(et.name) = lower($2) AND
    ts.granularity = $3;
$_$;


ALTER FUNCTION trend_directory.get_trend_store(data_source_name text, entity_type_name text, granularity interval) OWNER TO postgres;

--
-- Name: get_trend_store_id(trend_directory.trend_store); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.get_trend_store_id(trend_directory.trend_store) RETURNS integer
    LANGUAGE sql
    AS $_$
SELECT $1.id;
$_$;


ALTER FUNCTION trend_directory.get_trend_store_id(trend_directory.trend_store) OWNER TO postgres;

--
-- Name: get_trend_store_part(integer, name); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.get_trend_store_part(trend_store_id integer, name name) RETURNS trend_directory.trend_store_part
    LANGUAGE sql
    AS $_$
SELECT * FROM trend_directory.trend_store_part WHERE trend_store_id = $1 AND name = $2;
$_$;


ALTER FUNCTION trend_directory.get_trend_store_part(trend_store_id integer, name name) OWNER TO postgres;

--
-- Name: get_trend_store_part_id(trend_directory.trend_store_part); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.get_trend_store_part_id(trend_directory.trend_store_part) RETURNS integer
    LANGUAGE sql
    AS $_$
SELECT $1.id;
$_$;


ALTER FUNCTION trend_directory.get_trend_store_part_id(trend_directory.trend_store_part) OWNER TO postgres;

--
-- Name: get_trend_store_parts(integer); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.get_trend_store_parts(trend_store_id integer) RETURNS trend_directory.trend_store_part
    LANGUAGE sql
    AS $_$
SELECT trend_store_part FROM trend_directory.trend_store_part WHERE trend_store_id = $1;
$_$;


ALTER FUNCTION trend_directory.get_trend_store_parts(trend_store_id integer) OWNER TO postgres;

--
-- Name: get_trend_view(text, text, interval); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.get_trend_view(data_source_name text, entity_type_name text, granularity interval) RETURNS trend_directory.trend_view
    LANGUAGE sql STABLE
    AS $_$
SELECT ts
    FROM trend_directory.trend_view ts
    JOIN directory.data_source ds ON ds.id = ts.data_source_id
    JOIN directory.entity_type et ON et.id = ts.entity_type_id
    WHERE lower(ds.name) = lower($1) AND lower(et.name) = lower($2) AND ts.granularity = $3;
$_$;


ALTER FUNCTION trend_directory.get_trend_view(data_source_name text, entity_type_name text, granularity interval) OWNER TO postgres;

--
-- Name: get_trends_for_trend_store(integer); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.get_trends_for_trend_store(trend_store_id integer) RETURNS SETOF trend_directory.table_trend
    LANGUAGE sql STABLE
    AS $_$
SELECT table_trend
  FROM trend_directory.table_trend
  LEFT JOIN trend_directory.trend_store_part
  ON table_trend.trend_store_part_id = trend_store_part.id
  WHERE trend_store_part.trend_store_id = $1;
$_$;


ALTER FUNCTION trend_directory.get_trends_for_trend_store(trend_store_id integer) OWNER TO postgres;

--
-- Name: get_trends_for_trend_store(trend_directory.trend_store); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.get_trends_for_trend_store(trend_directory.trend_store) RETURNS SETOF trend_directory.table_trend
    LANGUAGE sql STABLE
    AS $_$
SELECT trend_directory.get_trends_for_trend_store($1.id);
$_$;


ALTER FUNCTION trend_directory.get_trends_for_trend_store(trend_directory.trend_store) OWNER TO postgres;

--
-- Name: get_trends_for_trend_store_part(integer); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.get_trends_for_trend_store_part(trend_store_part_id integer) RETURNS SETOF trend_directory.table_trend
    LANGUAGE plpgsql STABLE
    AS $_$
BEGIN
  RETURN QUERY EXECUTE FORMAT('SELECT * FROM trend_directory.table_trend WHERE table_trend.trend_store_part_id = %s;', $1);
END;
$_$;


ALTER FUNCTION trend_directory.get_trends_for_trend_store_part(trend_store_part_id integer) OWNER TO postgres;

--
-- Name: get_trends_for_trend_store_part(trend_directory.trend_store_part); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.get_trends_for_trend_store_part(trend_directory.trend_store_part) RETURNS SETOF trend_directory.table_trend
    LANGUAGE plpgsql STABLE
    AS $_$
BEGIN
  RETURN QUERY EXECUTE FORMAT('SELECT trend_directory.get_trends_for_trend_store_part(%s);', $1.id);
END;
$_$;


ALTER FUNCTION trend_directory.get_trends_for_trend_store_part(trend_directory.trend_store_part) OWNER TO postgres;

--
-- Name: get_view_trends(name); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.get_view_trends(view_name name) RETURNS SETOF trend_directory.trend_descr
    LANGUAGE sql STABLE
    AS $_$
SELECT (a.attname, format_type(a.atttypid, a.atttypmod), 'deduced from view', 'sum', 'sum', '{}')::trend_directory.trend_descr
FROM pg_class c
JOIN pg_attribute a ON a.attrelid = c.oid
WHERE c.relname = $1 AND a.attnum >= 0 AND NOT a.attisdropped;
$_$;


ALTER FUNCTION trend_directory.get_view_trends(view_name name) OWNER TO postgres;

--
-- Name: granularity_to_text(interval); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.granularity_to_text(interval) RETURNS text
    LANGUAGE sql IMMUTABLE STRICT
    AS $_$
SELECT CASE $1
    WHEN '300'::interval THEN
        '5m'
    WHEN '900'::interval THEN
        'qtr'
    WHEN '1 hour'::interval THEN
        'hr'
    WHEN '12 hours'::interval THEN
        '12hr'
    WHEN '1 day'::interval THEN
        'day'
    WHEN '1 week'::interval THEN
        'wk'
    WHEN '1 month'::interval THEN
        'month'
    ELSE
        $1::text
    END;
$_$;


ALTER FUNCTION trend_directory.granularity_to_text(interval) OWNER TO postgres;

--
-- Name: greatest_data_type(text, text); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.greatest_data_type(data_type_a text, data_type_b text) RETURNS text
    LANGUAGE sql IMMUTABLE
    AS $_$
SELECT
    CASE WHEN trend_directory.data_type_order($2) > trend_directory.data_type_order($1) THEN
        $2
    ELSE
        $1
    END;
$_$;


ALTER FUNCTION trend_directory.greatest_data_type(data_type_a text, data_type_b text) OWNER TO postgres;

--
-- Name: index_to_timestamp(interval, integer); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.index_to_timestamp(partition_size interval, index integer) RETURNS timestamp with time zone
    LANGUAGE sql IMMUTABLE STRICT
    AS $_$
SELECT to_timestamp(extract(epoch from $1) * $2);
$_$;


ALTER FUNCTION trend_directory.index_to_timestamp(partition_size interval, index integer) OWNER TO postgres;

--
-- Name: initialize_trend_store(trend_directory.trend_store); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.initialize_trend_store(trend_directory.trend_store) RETURNS trend_directory.trend_store
    LANGUAGE sql
    AS $_$
SELECT trend_directory.initialize_trend_store_part(trend_store_part.id)
FROM trend_directory.trend_store_part WHERE trend_store_id = $1.id;

SELECT $1;
$_$;


ALTER FUNCTION trend_directory.initialize_trend_store(trend_directory.trend_store) OWNER TO postgres;

--
-- Name: initialize_trend_store_part(integer); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.initialize_trend_store_part(integer) RETURNS trend_directory.trend_store_part
    LANGUAGE plpgsql
    AS $_$
DECLARE
  tsp trend_directory.trend_store_part;
BEGIN
  SELECT * from trend_directory.trend_store_part WHERE id = $1 into tsp;
  PERFORM trend_directory.initialize_trend_store_part(tsp);
  RETURN tsp;
END;
$_$;


ALTER FUNCTION trend_directory.initialize_trend_store_part(integer) OWNER TO postgres;

--
-- Name: initialize_trend_store_part(trend_directory.trend_store_part); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.initialize_trend_store_part(trend_directory.trend_store_part) RETURNS trend_directory.trend_store_part
    LANGUAGE sql
    AS $_$
SELECT trend_directory.create_base_table($1);
SELECT trend_directory.create_staging_table($1);

SELECT $1;
$_$;


ALTER FUNCTION trend_directory.initialize_trend_store_part(trend_directory.trend_store_part) OWNER TO postgres;

--
-- Name: FUNCTION initialize_trend_store_part(trend_directory.trend_store_part); Type: COMMENT; Schema: trend_directory; Owner: postgres
--

COMMENT ON FUNCTION trend_directory.initialize_trend_store_part(trend_directory.trend_store_part) IS 'Create all database objects required for the trend store part to be fully functional and capable of storing data.';


--
-- Name: initialize_trend_view(trend_directory.trend_view, trend_directory.trend_view_part_descr[]); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.initialize_trend_view(trend_directory.trend_view, trend_directory.trend_view_part_descr[]) RETURNS trend_directory.trend_view
    LANGUAGE sql SECURITY DEFINER
    AS $_$
SELECT $1;
$_$;


ALTER FUNCTION trend_directory.initialize_trend_view(trend_directory.trend_view, trend_directory.trend_view_part_descr[]) OWNER TO postgres;

--
-- Name: initialize_trend_view_part(trend_directory.trend_view_part, text); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.initialize_trend_view_part(trend_directory.trend_view_part, query text) RETURNS trend_directory.trend_view_part
    LANGUAGE sql SECURITY DEFINER
    AS $_$
SELECT public.action($1, trend_directory.create_view_sql($1, $2));

SELECT $1;
$_$;


ALTER FUNCTION trend_directory.initialize_trend_view_part(trend_directory.trend_view_part, query text) OWNER TO postgres;

--
-- Name: is_integer(character varying); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.is_integer(character varying) RETURNS boolean
    LANGUAGE sql IMMUTABLE
    AS $_$
SELECT $1 ~ '^[1-9][0-9]*$'
$_$;


ALTER FUNCTION trend_directory.is_integer(character varying) OWNER TO postgres;

--
-- Name: map_timestamp(integer, integer, timestamp with time zone); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.map_timestamp(materialization_id integer, trend_store_part_id integer, timestamp with time zone) RETURNS timestamp with time zone
    LANGUAGE plpgsql
    AS $_$
DECLARE
  mts_link trend_directory.materialization_trend_store_link;
  result timestamp with time zone;
BEGIN
  SELECT * FROM trend_directory.materialization_trend_store_link tsl WHERE tsl.materialization_id = $1 AND tsl.trend_store_part_id = $2 INTO mts_link;
  EXECUTE format('SELECT %s($1)', mts_link.timestamp_mapping_func::regproc::name) INTO result USING $3;
  RETURN result;
END;
$_$;


ALTER FUNCTION trend_directory.map_timestamp(materialization_id integer, trend_store_part_id integer, timestamp with time zone) OWNER TO postgres;

--
-- Name: FUNCTION map_timestamp(materialization_id integer, trend_store_part_id integer, timestamp with time zone); Type: COMMENT; Schema: trend_directory; Owner: postgres
--

COMMENT ON FUNCTION trend_directory.map_timestamp(materialization_id integer, trend_store_part_id integer, timestamp with time zone) IS 'Map timestamp using the mapping function defined in the link';


--
-- Name: mark_modified(integer, timestamp with time zone); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.mark_modified(trend_store_id integer, "timestamp" timestamp with time zone) RETURNS void
    LANGUAGE sql
    AS $_$
INSERT INTO trend_directory.modified_log(trend_store_part_id, timestamp, modified)
VALUES ($1, $2, now());
$_$;


ALTER FUNCTION trend_directory.mark_modified(trend_store_id integer, "timestamp" timestamp with time zone) OWNER TO postgres;

--
-- Name: FUNCTION mark_modified(trend_store_id integer, "timestamp" timestamp with time zone); Type: COMMENT; Schema: trend_directory; Owner: postgres
--

COMMENT ON FUNCTION trend_directory.mark_modified(trend_store_id integer, "timestamp" timestamp with time zone) IS 'Stores a record in the trend_directory.modified_log table.
';


--
-- Name: mark_modified(integer, timestamp with time zone, timestamp with time zone); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.mark_modified(trend_store_part_id integer, "timestamp" timestamp with time zone, modified timestamp with time zone) RETURNS void
    LANGUAGE sql
    AS $_$
INSERT INTO trend_directory.modified_log(trend_store_part_id, timestamp, modified)
VALUES ($1, $2, $3);
$_$;


ALTER FUNCTION trend_directory.mark_modified(trend_store_part_id integer, "timestamp" timestamp with time zone, modified timestamp with time zone) OWNER TO postgres;

--
-- Name: FUNCTION mark_modified(trend_store_part_id integer, "timestamp" timestamp with time zone, modified timestamp with time zone); Type: COMMENT; Schema: trend_directory; Owner: postgres
--

COMMENT ON FUNCTION trend_directory.mark_modified(trend_store_part_id integer, "timestamp" timestamp with time zone, modified timestamp with time zone) IS 'Stores a record in the trend_directory.modified_log table.
';


--
-- Name: materialization_to_char(integer); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.materialization_to_char(materialization_id integer) RETURNS text
    LANGUAGE sql STABLE STRICT
    AS $_$
SELECT trend_store_part.name::text
  FROM trend_directory.trend_store_part
  JOIN trend_directory.materialization
    ON trend_store_part.id = materialization.dst_trend_store_part_id
  WHERE materialization.id = $1;
$_$;


ALTER FUNCTION trend_directory.materialization_to_char(materialization_id integer) OWNER TO postgres;

--
-- Name: materialize(integer, timestamp with time zone); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.materialize(materialization_id integer, "timestamp" timestamp with time zone) RETURNS trend_directory.transfer_result
    LANGUAGE plpgsql
    AS $_$
DECLARE
    mat trend_directory.materialization;
    start timestamp with time zone;
    duration interval;
    columns_part text;
    result trend_directory.transfer_result;
BEGIN
    SELECT * FROM trend_directory.materialization WHERE id = $1 INTO mat;

    start = clock_timestamp();

    -- Remove all records in the target table for the timestamp to materialize
    PERFORM trend_directory.clear_trend_store_part(
        mat.dst_trend_store_part_id, $2
    );

    result.row_count = trend_directory.transfer($1, $2);

    -- Update the state of this materialization
    UPDATE trend_directory.materialization_state vms
    SET processed_fingerprint = vms.source_fingerprint
    WHERE vms.materialization_id = $1 AND vms.timestamp = $2;

    -- Log the change in the target trend store part
    PERFORM trend_directory.mark_modified(mat.dst_trend_store_part_id, $2, now());

    duration = clock_timestamp() - start;

    UPDATE trend_directory.materialization_metrics
    SET execution_count = execution_count + 1, total_duration = total_duration + duration
    WHERE materialization_metrics.materialization_id = $1;

    RETURN result;
END;
$_$;


ALTER FUNCTION trend_directory.materialize(materialization_id integer, "timestamp" timestamp with time zone) OWNER TO postgres;

--
-- Name: FUNCTION materialize(materialization_id integer, "timestamp" timestamp with time zone); Type: COMMENT; Schema: trend_directory; Owner: postgres
--

COMMENT ON FUNCTION trend_directory.materialize(materialization_id integer, "timestamp" timestamp with time zone) IS 'Materialize the data produced by the referenced view of the materialization by clearing the timestamp in the target trend_store_part and inserting the data resulting from the view into it.
';


--
-- Name: materialize(trend_directory.materialization, timestamp with time zone); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.materialize(trend_directory.materialization, "timestamp" timestamp with time zone) RETURNS trend_directory.transfer_result
    LANGUAGE sql
    AS $_$
SELECT trend_directory.materialize($1.id, $2);
$_$;


ALTER FUNCTION trend_directory.materialize(trend_directory.materialization, "timestamp" timestamp with time zone) OWNER TO postgres;

--
-- Name: max_modified(integer, timestamp with time zone); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.max_modified(dst_trend_store_part_id integer, timestamp with time zone) RETURNS timestamp with time zone
    LANGUAGE sql STABLE
    AS $_$
SELECT max(last) FROM trend_directory.modified
  WHERE trend_store_part_id = $1
  AND timestamp < $2;
$_$;


ALTER FUNCTION trend_directory.max_modified(dst_trend_store_part_id integer, timestamp with time zone) OWNER TO postgres;

--
-- Name: missing_generated_table_trends(name, trend_directory.generated_trend_descr[]); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.missing_generated_table_trends(trend_store_part name, required trend_directory.generated_trend_descr[]) RETURNS SETOF trend_directory.generated_trend_descr
    LANGUAGE sql STABLE
    AS $_$
SELECT trend_directory.missing_generated_table_trends(trend_store_part, $2)
FROM trend_directory.trend_store_part WHERE name = $1
$_$;


ALTER FUNCTION trend_directory.missing_generated_table_trends(trend_store_part name, required trend_directory.generated_trend_descr[]) OWNER TO postgres;

--
-- Name: missing_generated_table_trends(trend_directory.trend_store_part, trend_directory.generated_trend_descr[]); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.missing_generated_table_trends(trend_directory.trend_store_part, required trend_directory.generated_trend_descr[]) RETURNS SETOF trend_directory.generated_trend_descr
    LANGUAGE sql STABLE
    AS $_$
SELECT required
FROM unnest($2) required
LEFT JOIN trend_directory.generated_table_trend
ON generated_table_trend.name = required.name AND generated_table_trend.trend_store_part_id = $1.id
WHERE generated_table_trend.id IS NULL;
$_$;


ALTER FUNCTION trend_directory.missing_generated_table_trends(trend_directory.trend_store_part, required trend_directory.generated_trend_descr[]) OWNER TO postgres;

--
-- Name: missing_table_trends(trend_directory.trend_store_part, trend_directory.trend_descr[]); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.missing_table_trends(trend_directory.trend_store_part, required trend_directory.trend_descr[]) RETURNS SETOF trend_directory.trend_descr
    LANGUAGE sql STABLE
    AS $_$
SELECT required
FROM unnest($2) required
LEFT JOIN trend_directory.table_trend ON table_trend.name = required.name AND table_trend.trend_store_part_id = $1.id
WHERE table_trend.id IS NULL;
$_$;


ALTER FUNCTION trend_directory.missing_table_trends(trend_directory.trend_store_part, required trend_directory.trend_descr[]) OWNER TO postgres;

--
-- Name: needs_columnar_store(trend_directory.partition); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.needs_columnar_store(trend_directory.partition) RETURNS boolean
    LANGUAGE sql STABLE
    AS $_$
SELECT not p.is_columnar and p.to + COALESCE(m.reprocessing_period, trend_directory.default_columnar_period()) < now()
FROM trend_directory.partition p
  JOIN trend_directory.trend_store_part tsp ON p.trend_store_part_id = tsp.id
  LEFT JOIN trend_directory.materialization m ON m.dst_trend_store_part_id = p.id
WHERE p.id = $1.id;
$_$;


ALTER FUNCTION trend_directory.needs_columnar_store(trend_directory.partition) OWNER TO postgres;

--
-- Name: new_modified(); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.new_modified() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    PERFORM "trend_directory"."update_materialization_state"(NEW.trend_store_part_id, NEW.timestamp);

    RETURN NEW;
END;
$$;


ALTER FUNCTION trend_directory.new_modified() OWNER TO postgres;

--
-- Name: partition_name(trend_directory.trend_store_part, integer); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.partition_name(trend_directory.trend_store_part, integer) RETURNS name
    LANGUAGE sql STABLE
    AS $_$
SELECT format('%s_%s', trend_directory.base_table_name($1), $2)::name;
$_$;


ALTER FUNCTION trend_directory.partition_name(trend_directory.trend_store_part, integer) OWNER TO postgres;

--
-- Name: partition_schema(); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.partition_schema() RETURNS name
    LANGUAGE sql IMMUTABLE
    AS $$
SELECT 'trend_partition'::name;
$$;


ALTER FUNCTION trend_directory.partition_schema() OWNER TO postgres;

--
-- Name: process_modified_log(); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.process_modified_log() RETURNS bigint
    LANGUAGE sql
    AS $$
WITH processed AS (
  SELECT trend_directory.process_modified_log(coalesce(max(last_processed_id), 0)) AS last_processed_id
  FROM trend_directory.modified_log_processing_state WHERE name = 'current'
)
INSERT INTO trend_directory.modified_log_processing_state(name, last_processed_id)
SELECT 'current', processed.last_processed_id
FROM processed
ON CONFLICT (name) DO UPDATE SET last_processed_id = EXCLUDED.last_processed_id
RETURNING last_processed_id;
$$;


ALTER FUNCTION trend_directory.process_modified_log() OWNER TO postgres;

--
-- Name: process_modified_log(bigint); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.process_modified_log(start_id bigint) RETURNS bigint
    LANGUAGE sql
    AS $_$
WITH process_log AS (
  SELECT
    max(id) AS max_id,
    trend_directory.update_modified(trend_store_part_id, timestamp, max(modified)) AS update
  FROM trend_directory.modified_log
  WHERE id > $1
  GROUP BY trend_store_part_id, timestamp
) SELECT coalesce(max(max_id), $1) FROM process_log;
$_$;


ALTER FUNCTION trend_directory.process_modified_log(start_id bigint) OWNER TO postgres;

--
-- Name: recalculate_trend_store_part_stats(integer, timestamp with time zone); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.recalculate_trend_store_part_stats(trend_store_part_id integer, "timestamp" timestamp with time zone) RETURNS void
    LANGUAGE sql
    AS $_$
UPDATE trend_directory.trend_store_part_stats
  SET modified = now(), count = trend_directory.get_count($1, $2)
  WHERE trend_store_part_id = $1
    AND timestamp = $2;
$_$;


ALTER FUNCTION trend_directory.recalculate_trend_store_part_stats(trend_store_part_id integer, "timestamp" timestamp with time zone) OWNER TO postgres;

--
-- Name: remove_extra_trends(trend_directory.trend_store_part_descr); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.remove_extra_trends(part trend_directory.trend_store_part_descr) RETURNS text[]
    LANGUAGE sql
    AS $_$
SELECT trend_directory.remove_extra_trends(
  id,
  $1.trends
)
FROM trend_directory.trend_store_part
WHERE name = $1.name;
$_$;


ALTER FUNCTION trend_directory.remove_extra_trends(part trend_directory.trend_store_part_descr) OWNER TO postgres;

--
-- Name: remove_extra_trends(integer, trend_directory.trend_descr[]); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.remove_extra_trends(trend_store_part_id integer, trend_directory.trend_descr[]) RETURNS text[]
    LANGUAGE plpgsql
    AS $_$
DECLARE
  trend trend_directory.table_trend;
  removal_result text;
  result text[];
BEGIN
  FOR trend IN SELECT * FROM trend_directory.get_trends_for_trend_store_part($1)
  LOOP
    SELECT trend_directory.remove_trend_if_extraneous(trend, $2) INTO removal_result;
    IF removal_result IS NOT NULL THEN
      SELECT result || removal_result INTO result;
    END IF;
  END LOOP;
  RETURN result;
END;
$_$;


ALTER FUNCTION trend_directory.remove_extra_trends(trend_store_part_id integer, trend_directory.trend_descr[]) OWNER TO postgres;

--
-- Name: remove_extra_trends(trend_directory.trend_store, trend_directory.trend_store_part_descr[]); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.remove_extra_trends(trend_directory.trend_store, parts trend_directory.trend_store_part_descr[]) RETURNS text[]
    LANGUAGE plpgsql
    AS $_$
DECLARE
  result text[];
  partresult text[];
BEGIN
  FOR partresult IN
    SELECT trend_directory.remove_extra_trends(
      trend_directory.get_trend_store_part($1.id, name), trends)
    FROM unnest($2)
  LOOP
    SELECT result || partresult INTO result;
  END LOOP;
  RETURN result;
END;
$_$;


ALTER FUNCTION trend_directory.remove_extra_trends(trend_directory.trend_store, parts trend_directory.trend_store_part_descr[]) OWNER TO postgres;

--
-- Name: remove_table_trend(trend_directory.table_trend); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.remove_table_trend(trend trend_directory.table_trend) RETURNS trend_directory.table_trend
    LANGUAGE plpgsql
    AS $$
BEGIN
  EXECUTE FORMAT('ALTER TABLE trend.%I DROP COLUMN %I',
    trend_directory.trend_store_part_name_for_trend(trend), trend.name);
  EXECUTE FORMAT('ALTER TABLE trend.%I DROP COLUMN %I',
    trend_directory.trend_store_part_name_for_trend(trend)::text || '_staging', trend.name);
  DELETE FROM trend_directory.table_trend WHERE id = trend.id;
  RETURN t FROM trend_directory.table_trend t WHERE 0=1;
END;
$$;


ALTER FUNCTION trend_directory.remove_table_trend(trend trend_directory.table_trend) OWNER TO postgres;

--
-- Name: remove_trend_if_extraneous(trend_directory.table_trend, trend_directory.trend_descr[]); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.remove_trend_if_extraneous(trend trend_directory.table_trend, trends trend_directory.trend_descr[]) RETURNS text
    LANGUAGE plpgsql
    AS $_$
DECLARE
  result text;
  defined_trend name;
BEGIN
  SELECT trend_directory.get_trend_if_defined($1, $2) INTO defined_trend;
  IF defined_trend IS NULL THEN
    SELECT $1.name INTO result;
    PERFORM trend_directory.remove_table_trend($1);
  END IF;
  RETURN result;
END;
$_$;


ALTER FUNCTION trend_directory.remove_trend_if_extraneous(trend trend_directory.table_trend, trends trend_directory.trend_descr[]) OWNER TO postgres;

--
-- Name: FUNCTION remove_trend_if_extraneous(trend trend_directory.table_trend, trends trend_directory.trend_descr[]); Type: COMMENT; Schema: trend_directory; Owner: postgres
--

COMMENT ON FUNCTION trend_directory.remove_trend_if_extraneous(trend trend_directory.table_trend, trends trend_directory.trend_descr[]) IS 'Remove the trend if it is not one that is described by trends';


--
-- Name: rename_partitions(trend_directory.trend_store_part, name); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.rename_partitions(trend_directory.trend_store_part, new_name name) RETURNS trend_directory.trend_store_part
    LANGUAGE plpgsql
    AS $_$
DECLARE
  partition trend_directory.partition;
BEGIN
  FOR partition in SELECT * FROM trend_directory.partition WHERE trend_store_part_id = $1.id
  LOOP
    EXECUTE format(
        'ALTER TABLE trend_partition.%I RENAME TO %I',
        partition.name,
        $2 || '_' || partition.index
    );
    EXECUTE 'UPDATE trend_directory.partition SET name = $1 WHERE id = $2'
        USING $2 || '_' || partition.index, partition.id;
  END LOOP;
  RETURN $1;
END;
$_$;


ALTER FUNCTION trend_directory.rename_partitions(trend_directory.trend_store_part, new_name name) OWNER TO postgres;

--
-- Name: rename_trend_store_part(trend_directory.trend_store_part, name); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.rename_trend_store_part(trend_directory.trend_store_part, name) RETURNS trend_directory.trend_store_part
    LANGUAGE plpgsql
    AS $_$
BEGIN
  EXECUTE format(
    'ALTER TABLE %I.%I RENAME TO %I',
    trend_directory.base_table_schema(),
    $1.name,
    $2
  );
  RETURN $1;
END; 
$_$;


ALTER FUNCTION trend_directory.rename_trend_store_part(trend_directory.trend_store_part, name) OWNER TO postgres;

--
-- Name: rename_trend_store_part_full(trend_directory.trend_store_part, name); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.rename_trend_store_part_full(trend_directory.trend_store_part, name) RETURNS trend_directory.trend_store_part
    LANGUAGE plpgsql
    AS $_$
DECLARE
  old_name text;
  new_name text;
BEGIN
  SET LOCAL citus.multi_shard_modify_mode TO 'sequential';
  SELECT trend_directory.to_char($1) INTO old_name;
  SELECT $2::text INTO new_name;
  PERFORM trend_directory.rename_trend_store_part($1, $2);
  EXECUTE format(
      'ALTER TABLE %I.%I RENAME TO %I',
      trend_directory.staging_table_schema(),
      old_name || '_staging',
      new_name || '_staging'
  );
  PERFORM trend_directory.rename_partitions($1, $2);
  EXECUTE
      'UPDATE trend_directory.view_materialization '
      'SET src_view = $1 '
      'WHERE src_view = $2'
      USING 'trend."_' || new_name || '"', 'trend."_' || old_name || '"';
  EXECUTE
      'UPDATE trend_directory.function_materialization '
      'SET src_function = $1 '
      'WHERE src_function = $2'
      USING 'trend."' || new_name || '"', 'trend."' || old_name || '"';
  RETURN $1;
END
$_$;


ALTER FUNCTION trend_directory.rename_trend_store_part_full(trend_directory.trend_store_part, name) OWNER TO postgres;

--
-- Name: show_trends(integer); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.show_trends(trend_store_part_id integer) RETURNS SETOF trend_directory.trend_descr
    LANGUAGE sql STABLE
    AS $_$
SELECT trend_directory.show_trends(trend_store_part)
FROM trend_directory.trend_store_part
WHERE id = $1;
$_$;


ALTER FUNCTION trend_directory.show_trends(trend_store_part_id integer) OWNER TO postgres;

--
-- Name: show_trends(trend_directory.trend_store_part); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.show_trends(trend_directory.trend_store_part) RETURNS SETOF trend_directory.trend_descr
    LANGUAGE sql STABLE
    AS $_$
SELECT
    table_trend.name::name,
    format_type(a.atttypid, a.atttypmod)::text,
    table_trend.description,
    table_trend.time_aggregation,
    table_trend.entity_aggregation,
    table_trend.extra_data
FROM trend_directory.table_trend
JOIN pg_catalog.pg_class c ON c.relname = $1::text
JOIN pg_catalog.pg_namespace n ON c.relnamespace = n.oid
JOIN pg_catalog.pg_attribute a ON a.attrelid = c.oid AND a.attname = table_trend.name
WHERE
    n.nspname = 'trend' AND
    a.attisdropped = false AND
    a.attnum > 0 AND table_trend.trend_store_part_id = $1.id;
$_$;


ALTER FUNCTION trend_directory.show_trends(trend_directory.trend_store_part) OWNER TO postgres;

--
-- Name: source_fingerprint(integer, timestamp with time zone); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.source_fingerprint(materialization_id integer, timestamp with time zone) RETURNS trend_directory.fingerprint
    LANGUAGE plpgsql STABLE
    AS $_$
DECLARE
    result trend_directory.fingerprint;
BEGIN
    EXECUTE trend_directory.source_fingerprint_sql($1) INTO result USING $2;

    RETURN result;
END;
$_$;


ALTER FUNCTION trend_directory.source_fingerprint(materialization_id integer, timestamp with time zone) OWNER TO postgres;

--
-- Name: FUNCTION source_fingerprint(materialization_id integer, timestamp with time zone); Type: COMMENT; Schema: trend_directory; Owner: postgres
--

COMMENT ON FUNCTION trend_directory.source_fingerprint(materialization_id integer, timestamp with time zone) IS 'Returns the fingerprint of the combined states of all sources required to calculate the data for the target timestamp.
';


--
-- Name: source_fingerprint_sql(integer); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.source_fingerprint_sql(materialization_id integer) RETURNS text
    LANGUAGE sql STABLE
    AS $_$
SELECT format('SELECT * FROM trend.%I($1)', trend_directory.materialization_to_char($1) || '_fingerprint');
$_$;


ALTER FUNCTION trend_directory.source_fingerprint_sql(materialization_id integer) OWNER TO postgres;

--
-- Name: FUNCTION source_fingerprint_sql(materialization_id integer); Type: COMMENT; Schema: trend_directory; Owner: postgres
--

COMMENT ON FUNCTION trend_directory.source_fingerprint_sql(materialization_id integer) IS 'Returns the query to generate fingerprints for the specified view materialization.';


--
-- Name: staged_timestamps(trend_directory.trend_store_part); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.staged_timestamps(part trend_directory.trend_store_part) RETURNS SETOF timestamp with time zone
    LANGUAGE plpgsql STABLE
    AS $$
BEGIN
    RETURN QUERY EXECUTE format(
        'SELECT timestamp FROM %I.%I GROUP BY timestamp',
        trend_directory.staging_table_schema(),
        trend_directory.staging_table_name(part)
    );
END;
$$;


ALTER FUNCTION trend_directory.staged_timestamps(part trend_directory.trend_store_part) OWNER TO postgres;

--
-- Name: staging_table_name(trend_directory.trend_store_part); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.staging_table_name(trend_directory.trend_store_part) RETURNS name
    LANGUAGE sql STABLE
    AS $_$
SELECT (trend_directory.base_table_name($1) || '_staging')::name;
$_$;


ALTER FUNCTION trend_directory.staging_table_name(trend_directory.trend_store_part) OWNER TO postgres;

--
-- Name: staging_table_schema(); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.staging_table_schema() RETURNS name
    LANGUAGE sql IMMUTABLE
    AS $$
SELECT 'trend'::name;
$$;


ALTER FUNCTION trend_directory.staging_table_schema() OWNER TO postgres;

--
-- Name: table_columns(oid); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.table_columns(oid) RETURNS SETOF trend_directory.column_info
    LANGUAGE sql STABLE
    AS $_$
SELECT
    a.attname,
    format_type(a.atttypid, a.atttypmod)
FROM
    pg_catalog.pg_class c
JOIN
    pg_catalog.pg_attribute a ON a.attrelid = c.oid
WHERE
    c.oid = $1 AND
    a.attisdropped = false AND
    a.attnum > 0;
$_$;


ALTER FUNCTION trend_directory.table_columns(oid) OWNER TO postgres;

--
-- Name: table_columns(name, name); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.table_columns(namespace name, "table" name) RETURNS SETOF trend_directory.column_info
    LANGUAGE sql STABLE
    AS $_$
SELECT
    a.attname,
    format_type(a.atttypid, a.atttypmod)
FROM
    pg_catalog.pg_class c
JOIN
    pg_catalog.pg_namespace n ON c.relnamespace = n.oid
JOIN
    pg_catalog.pg_attribute a ON a.attrelid = c.oid
WHERE
    n.nspname = $1 AND
    c.relname = $2 AND
    a.attisdropped = false AND
    a.attnum > 0;
$_$;


ALTER FUNCTION trend_directory.table_columns(namespace name, "table" name) OWNER TO postgres;

--
-- Name: timestamp_to_index(interval, timestamp with time zone); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.timestamp_to_index(interval, timestamp with time zone) RETURNS integer
    LANGUAGE sql STABLE
    AS $_$
SELECT extract(epoch from $2)::integer / extract(epoch from $1)::integer;
$_$;


ALTER FUNCTION trend_directory.timestamp_to_index(interval, timestamp with time zone) OWNER TO postgres;

--
-- Name: timestamp_to_index(trend_directory.trend_store, timestamp with time zone); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.timestamp_to_index(trend_directory.trend_store, timestamp with time zone) RETURNS integer
    LANGUAGE sql STABLE
    AS $_$
SELECT trend_directory.timestamp_to_index($1.partition_size, $2);
$_$;


ALTER FUNCTION trend_directory.timestamp_to_index(trend_directory.trend_store, timestamp with time zone) OWNER TO postgres;

--
-- Name: transfer(integer, timestamp with time zone); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.transfer(materialization_id integer, "timestamp" timestamp with time zone) RETURNS integer
    LANGUAGE plpgsql
    AS $_$
DECLARE
  row_count integer;
BEGIN
  SELECT trend_directory.function_materialization_transfer($1, $2) INTO row_count;

  IF row_count IS NULL THEN
    SELECT trend_directory.view_materialization_transfer($1, $2) INTO row_count;
  END IF;

  RETURN row_count;
END;
$_$;


ALTER FUNCTION trend_directory.transfer(materialization_id integer, "timestamp" timestamp with time zone) OWNER TO postgres;

--
-- Name: transfer_staged(trend_directory.trend_store_part); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.transfer_staged(trend_store_part trend_directory.trend_store_part) RETURNS trend_directory.trend_store_part
    LANGUAGE sql
    AS $_$
SELECT
    trend_directory.transfer_staged(trend_store_part, timestamp)
FROM trend_directory.staged_timestamps(trend_store_part) timestamp;

SELECT public.action(
    $1,
    format(
        'TRUNCATE %I.%I',
        trend_directory.staging_table_schema(),
        trend_directory.staging_table_name(trend_store_part)
    )
);
$_$;


ALTER FUNCTION trend_directory.transfer_staged(trend_store_part trend_directory.trend_store_part) OWNER TO postgres;

--
-- Name: transfer_staged(trend_directory.trend_store_part, timestamp with time zone); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.transfer_staged(trend_store_part trend_directory.trend_store_part, "timestamp" timestamp with time zone) RETURNS integer
    LANGUAGE plpgsql
    AS $_$
DECLARE
    row_count integer;
BEGIN
    EXECUTE format(
        'INSERT INTO %I.%I SELECT * FROM %I.%I WHERE timestamp = $1',
        trend_directory.base_table_schema(),
        trend_directory.base_table_name(trend_store_part),
        trend_directory.staging_table_schema(),
        trend_directory.staging_table_name(trend_store_part)
    ) USING timestamp;

    GET DIAGNOSTICS row_count = ROW_COUNT;

    RETURN row_count;
END;
$_$;


ALTER FUNCTION trend_directory.transfer_staged(trend_store_part trend_directory.trend_store_part, "timestamp" timestamp with time zone) OWNER TO postgres;

--
-- Name: trend_column_spec(integer); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.trend_column_spec(table_trend_id integer) RETURNS text
    LANGUAGE plpgsql IMMUTABLE
    AS $_$
DECLARE
  tt trend_directory.table_trend;
BEGIN
  SELECT * FROM trend_directory.table_trend WHERE id = $1 INTO tt;
  RETURN trend_directory.column_spec(tt);
END;
$_$;


ALTER FUNCTION trend_directory.trend_column_spec(table_trend_id integer) OWNER TO postgres;

--
-- Name: trend_has_update(integer, trend_directory.trend_descr); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.trend_has_update(trend_id integer, trend_update trend_directory.trend_descr) RETURNS boolean
    LANGUAGE plpgsql
    AS $_$
DECLARE
  trend trend_directory.table_trend;
BEGIN
  SELECT * FROM trend_directory.table_trend WHERE id = $1 INTO trend;
  RETURN
    trend.data_type != $2.data_type
      OR
    trend.time_aggregation != $2.time_aggregation
      OR
    trend.entity_aggregation != $2.entity_aggregation;
END;
$_$;


ALTER FUNCTION trend_directory.trend_has_update(trend_id integer, trend_update trend_directory.trend_descr) OWNER TO postgres;

--
-- Name: trend_store(trend_directory.trend_store_part); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.trend_store(trend_directory.trend_store_part) RETURNS trend_directory.trend_store
    LANGUAGE sql STABLE
    AS $_$
SELECT *
FROM trend_directory.trend_store
WHERE id = $1.trend_store_id;
$_$;


ALTER FUNCTION trend_directory.trend_store(trend_directory.trend_store_part) OWNER TO postgres;

--
-- Name: trend_store_part_has_trend_with_name(trend_directory.trend_store_part, name); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.trend_store_part_has_trend_with_name(part trend_directory.trend_store_part, trend_name name) RETURNS boolean
    LANGUAGE sql STABLE
    AS $_$
SELECT exists(
    SELECT 1
    FROM trend_directory.table_trend
    WHERE trend_store_part_id = $1.id AND name = $2
);
$_$;


ALTER FUNCTION trend_directory.trend_store_part_has_trend_with_name(part trend_directory.trend_store_part, trend_name name) OWNER TO postgres;

--
-- Name: trend_store_part_name_for_trend(integer); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.trend_store_part_name_for_trend(trend_id integer) RETURNS name
    LANGUAGE sql STABLE
    AS $_$
SELECT trend_store_part.name FROM trend_directory.table_trend LEFT JOIN trend_directory.trend_store_part
  ON table_trend.trend_store_part_id = trend_store_part.id
  WHERE table_trend.id = $1;
$_$;


ALTER FUNCTION trend_directory.trend_store_part_name_for_trend(trend_id integer) OWNER TO postgres;

--
-- Name: trend_store_part_name_for_trend(trend_directory.table_trend); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.trend_store_part_name_for_trend(trend trend_directory.table_trend) RETURNS name
    LANGUAGE sql STABLE
    AS $$
SELECT trend_store_part.name FROM trend_directory.table_trend LEFT JOIN trend_directory.trend_store_part
  ON table_trend.trend_store_part_id = trend_store_part.id
  WHERE table_trend.id = trend.id;
$$;


ALTER FUNCTION trend_directory.trend_store_part_name_for_trend(trend trend_directory.table_trend) OWNER TO postgres;

--
-- Name: undefine_materialization(name); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.undefine_materialization(name name) RETURNS void
    LANGUAGE sql
    AS $_$
SELECT trend_directory."cleanup_for_materialization"($1);
DELETE FROM trend_directory.materialization
  WHERE materialization::text = $1;
$_$;


ALTER FUNCTION trend_directory.undefine_materialization(name name) OWNER TO postgres;

--
-- Name: FUNCTION undefine_materialization(name name); Type: COMMENT; Schema: trend_directory; Owner: postgres
--

COMMENT ON FUNCTION trend_directory.undefine_materialization(name name) IS 'Undefine and remove a materialization';


--
-- Name: update_materialization_state(integer, timestamp with time zone); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.update_materialization_state(integer, timestamp with time zone) RETURNS bigint
    LANGUAGE sql
    AS $_$
SELECT count(*) FROM (
    WITH mapped_modified AS (
      SELECT
        materialization_id,
        trend_directory.map_timestamp(materialization_trend_store_link.materialization_id, materialization_trend_store_link.trend_store_part_id, $2) AS dst_timestamp
      FROM trend_directory.materialization_trend_store_link
      WHERE trend_store_part_id = $1
    )
    SELECT trend_directory.update_source_fingerprint(m.id, dst_timestamp)
    FROM mapped_modified
    JOIN trend_directory.materialization m ON m.id = materialization_id
    GROUP BY m.id, dst_timestamp
) update_result;
$_$;


ALTER FUNCTION trend_directory.update_materialization_state(integer, timestamp with time zone) OWNER TO postgres;

--
-- Name: update_modified(integer, timestamp with time zone, timestamp with time zone); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.update_modified(trend_store_part_id integer, "timestamp" timestamp with time zone, modified timestamp with time zone) RETURNS void
    LANGUAGE sql
    AS $_$
INSERT INTO trend_directory.modified AS m (trend_store_part_id, timestamp, first, last)
VALUES ($1, $2, $3, $3)
ON CONFLICT ON CONSTRAINT modified_pkey DO UPDATE SET last = EXCLUDED.last;
$_$;


ALTER FUNCTION trend_directory.update_modified(trend_store_part_id integer, "timestamp" timestamp with time zone, modified timestamp with time zone) OWNER TO postgres;

--
-- Name: update_modified_column(); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.update_modified_column() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    NEW.modified = NOW();

    RETURN NEW;
END;
$$;


ALTER FUNCTION trend_directory.update_modified_column() OWNER TO postgres;

--
-- Name: update_modified_state(); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.update_modified_state() RETURNS integer
    LANGUAGE plpgsql
    AS $$
DECLARE
    count integer;
BEGIN
    UPDATE trend_directory.state
    SET
        max_modified = mzb.max_modified,
        source_states = mzb.source_states
    FROM trend_directory.modified_materializables mzb
    WHERE
        state.materialization_id = mzb.materialization_id AND
        state.timestamp = mzb.timestamp;

    GET DIAGNOSTICS count = ROW_COUNT;

    RETURN count;
END;
$$;


ALTER FUNCTION trend_directory.update_modified_state() OWNER TO postgres;

--
-- Name: update_source_fingerprint(integer, timestamp with time zone); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.update_source_fingerprint(materialization_id integer, timestamp with time zone) RETURNS void
    LANGUAGE plpgsql
    AS $_$
DECLARE
  materialization trend_directory.materialization;
  fingerprint trend_directory.fingerprint := trend_directory.source_fingerprint($1, $2);
BEGIN
  SELECT * FROM trend_directory.materialization WHERE id = $1 INTO materialization;
  IF action_count(format('SELECT * FROM trend_directory.materialization_state WHERE materialization_id = %s AND timestamp = ''%s''', $1, $2)) = 0
    THEN INSERT INTO trend_directory.materialization_state(materialization_id, timestamp, source_fingerprint, max_modified, processed_fingerprint, job_id)
      VALUES ($1, $2, fingerprint.body, fingerprint.modified, null, null);
    ELSE UPDATE trend_directory.materialization_state ms SET source_fingerprint = fingerprint.body, max_modified = fingerprint.modified WHERE ms.materialization_id = $1 AND ms.timestamp = $2;
  END IF;
END;
$_$;


ALTER FUNCTION trend_directory.update_source_fingerprint(materialization_id integer, timestamp with time zone) OWNER TO postgres;

--
-- Name: FUNCTION update_source_fingerprint(materialization_id integer, timestamp with time zone); Type: COMMENT; Schema: trend_directory; Owner: postgres
--

COMMENT ON FUNCTION trend_directory.update_source_fingerprint(materialization_id integer, timestamp with time zone) IS 'Update the fingerprint of the sources in the materialization_state table.';


--
-- Name: update_state(); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.update_state() RETURNS text
    LANGUAGE sql
    AS $$
SELECT 'added: ' || trend_directory.add_new_state() || ', updated: ' || trend_directory.update_modified_state() || ', deleted: ' || trend_directory.delete_obsolete_state();
$$;


ALTER FUNCTION trend_directory.update_state() OWNER TO postgres;

--
-- Name: update_trend_store_part_stats(); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.update_trend_store_part_stats() RETURNS void
    LANGUAGE sql
    AS $$
SELECT trend_directory.recalculate_trend_store_part_stats(trend_store_part_id, timestamp)
  FROM trend_directory.trend_store_part_stats_to_update;
$$;


ALTER FUNCTION trend_directory.update_trend_store_part_stats() OWNER TO postgres;

--
-- Name: view_materialization_columns_part(integer); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.view_materialization_columns_part(materialization_id integer) RETURNS text
    LANGUAGE sql STABLE
    AS $_$
SELECT trend_directory.columns_part(m.dst_trend_store_part_id)
  FROM trend_directory.materialization m
  WHERE id = $1;
$_$;


ALTER FUNCTION trend_directory.view_materialization_columns_part(materialization_id integer) OWNER TO postgres;

--
-- Name: FUNCTION view_materialization_columns_part(materialization_id integer); Type: COMMENT; Schema: trend_directory; Owner: postgres
--

COMMENT ON FUNCTION trend_directory.view_materialization_columns_part(materialization_id integer) IS 'Return the comma separated, quoted list of column names to be used in queries';


--
-- Name: view_materialization_transfer(integer, timestamp with time zone); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.view_materialization_transfer(materialization_id integer, "timestamp" timestamp with time zone) RETURNS integer
    LANGUAGE plpgsql
    AS $_$
DECLARE
    mat trend_directory.view_materialization;
    columns_part text;
    row_count integer;
    job_id bigint;
BEGIN
    SELECT * FROM trend_directory.view_materialization WHERE view_materialization.materialization_id = $1 INTO mat;
    SELECT logging.start_job(format('{"view_materialization": "%s", "timestamp": "%s"}', m::text, $2::text)::jsonb) INTO job_id
    FROM trend_directory.materialization m WHERE id = $1;

    SELECT trend_directory.view_materialization_columns_part($1) INTO columns_part;

    EXECUTE format(
        'INSERT INTO trend.%I (entity_id, timestamp, created, job_id, %s) SELECT entity_id, timestamp, now(), %s, %s FROM %s WHERE timestamp = $1',
        (trend_directory.dst_trend_store_part($1)).name,
        columns_part,
        job_id,
        columns_part,
        mat.src_view
    ) USING timestamp;

    GET DIAGNOSTICS row_count = ROW_COUNT;

    PERFORM logging.end_job(job_id);

    RETURN row_count;
END;
$_$;


ALTER FUNCTION trend_directory.view_materialization_transfer(materialization_id integer, "timestamp" timestamp with time zone) OWNER TO postgres;

--
-- Name: FUNCTION view_materialization_transfer(materialization_id integer, "timestamp" timestamp with time zone); Type: COMMENT; Schema: trend_directory; Owner: postgres
--

COMMENT ON FUNCTION trend_directory.view_materialization_transfer(materialization_id integer, "timestamp" timestamp with time zone) IS 'Transfer all records of the specified timestamp from the view to the target trend store of the materialization.';


--
-- Name: view_schema(); Type: FUNCTION; Schema: trend_directory; Owner: postgres
--

CREATE FUNCTION trend_directory.view_schema() RETURNS name
    LANGUAGE sql IMMUTABLE
    AS $$
SELECT 'trend'::name;
$$;


ALTER FUNCTION trend_directory.view_schema() OWNER TO postgres;

--
-- Name: action(anyelement, text[]); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.action(anyelement, sql text[]) RETURNS anyelement
    LANGUAGE plpgsql
    AS $_$
DECLARE
    statement text;
BEGIN
    FOREACH statement IN ARRAY sql LOOP
        EXECUTE statement;
    END LOOP;

    RETURN $1;
END;
$_$;


ALTER FUNCTION trigger.action(anyelement, sql text[]) OWNER TO postgres;

--
-- Name: action(anyelement, text); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.action(anyelement, sql text) RETURNS anyelement
    LANGUAGE plpgsql
    AS $_$
BEGIN
    EXECUTE sql;

    RETURN $1;
END;
$_$;


ALTER FUNCTION trigger.action(anyelement, sql text) OWNER TO postgres;

--
-- Name: add_insert_trigger(notification_directory.notification_store); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.add_insert_trigger(notification_directory.notification_store) RETURNS notification_directory.notification_store
    LANGUAGE plpgsql
    AS $_$
BEGIN
    EXECUTE format(
        $query$
        CREATE OR REPLACE FUNCTION notification.%I()
            RETURNS trigger AS
        $fnbody$
        BEGIN
            IF new.weight IS NULL THEN
                RAISE WARNING 'notification of rule %% entity %% timestamp %% has weight NULL', new.rule_id, new.entity_id, new.timestamp;
                RETURN NULL;
            ELSE
                RETURN new;
            END IF;
        END;
        $fnbody$ LANGUAGE plpgsql IMMUTABLE;
        $query$,
        notification_directory.staging_table_name($1) || '_insert_checks'
    );

    EXECUTE format(
        $query$
        CREATE TRIGGER check_notifications_trigger
            BEFORE INSERT
            ON notification.%I
            FOR EACH ROW
            EXECUTE PROCEDURE notification.%I();
        $query$,
        notification_directory.staging_table_name($1),
        notification_directory.staging_table_name($1) || '_insert_checks'
    );

    RETURN $1;
END;
$_$;


ALTER FUNCTION trigger.add_insert_trigger(notification_directory.notification_store) OWNER TO postgres;

--
-- Name: rule; Type: TABLE; Schema: trigger; Owner: postgres
--

CREATE TABLE trigger.rule (
    id integer NOT NULL,
    name name,
    notification_store_id integer,
    granularity interval,
    default_interval interval,
    enabled boolean DEFAULT false NOT NULL,
    description text
);


ALTER TABLE trigger.rule OWNER TO postgres;

--
-- Name: add_rule(name); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.add_rule(name) RETURNS trigger.rule
    LANGUAGE sql
    AS $_$
INSERT INTO "trigger".rule (name)
VALUES ($1) RETURNING rule;
$_$;


ALTER FUNCTION trigger.add_rule(name) OWNER TO postgres;

--
-- Name: change_exception_threshold_fn_name(trigger.rule); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.change_exception_threshold_fn_name(trigger.rule) RETURNS name
    LANGUAGE sql IMMUTABLE
    AS $_$
SELECT ($1.name || '_add_or_change_threshold_exception')::name;
$_$;


ALTER FUNCTION trigger.change_exception_threshold_fn_name(trigger.rule) OWNER TO postgres;

--
-- Name: cleanup_rule(trigger.rule); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.cleanup_rule(trigger.rule) RETURNS trigger.rule
    LANGUAGE plpgsql
    AS $_$
BEGIN
    EXECUTE trigger.drop_set_thresholds_fn_sql($1);
    EXECUTE trigger.drop_rule_fn_sql($1);
    EXECUTE trigger.drop_kpi_function_sql($1);
    EXECUTE trigger.drop_notification_fn_sql($1);
    EXECUTE trigger.drop_runnable_fn_sql($1);
    EXECUTE trigger.drop_fingerprint_fn_sql($1);
    EXECUTE trigger.drop_with_threshold_fn_sql($1);
    EXECUTE trigger.drop_weight_fn_sql($1);
    EXECUTE trigger.drop_notification_message_fn_sql($1);
    EXECUTE trigger.drop_exception_weight_table_sql($1);
    EXECUTE trigger.drop_thresholds_view_sql($1);
    EXECUTE trigger.drop_exception_threshold_table_sql($1);
    EXECUTE trigger.drop_notification_type_sql($1);
    EXECUTE trigger.drop_details_type_sql($1);
    EXECUTE trigger.drop_kpi_type_sql($1);

    RETURN $1;
END;
$_$;


ALTER FUNCTION trigger.cleanup_rule(trigger.rule) OWNER TO postgres;

--
-- Name: create_change_exception_threshold_fn_sql(trigger.rule, trigger.threshold_def[]); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.create_change_exception_threshold_fn_sql(trigger.rule, trigger.threshold_def[]) RETURNS text
    LANGUAGE sql
    AS $_$
SELECT format(
  'CREATE OR REPLACE FUNCTION trigger_rule.%I(entity integer, new_start timestamp with time zone, new_expires timestamp with time zone, %s) RETURNS VOID AS $fn$%s$fn$ LANGUAGE sql VOLATILE',
  trigger.change_exception_threshold_fn_name($1),
  string_agg(threshold.name || '_new ' || threshold.data_type, ', '),
  format(
    'SELECT trigger_rule.%I(entity); '
    'UPDATE trigger_rule.%I SET (start, expires, %s) = (new_start, new_expires, %s) WHERE entity_id = entity;',
    trigger.get_or_create_exception_threshold_fn_name($1),
    trigger.exception_threshold_table_name($1),
    string_agg(threshold.name, ', '),
    string_agg(threshold.name || '_new', ', ')
  )
) FROM unnest($2) threshold;
$_$;


ALTER FUNCTION trigger.create_change_exception_threshold_fn_sql(trigger.rule, trigger.threshold_def[]) OWNER TO postgres;

--
-- Name: create_details_type(trigger.rule, trigger.threshold_def[]); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.create_details_type(trigger.rule, trigger.threshold_def[]) RETURNS trigger.rule
    LANGUAGE sql
    AS $_$
SELECT public.action($1, trigger.create_details_type_sql($1, $2));
$_$;


ALTER FUNCTION trigger.create_details_type(trigger.rule, trigger.threshold_def[]) OWNER TO postgres;

--
-- Name: create_details_type_sql(trigger.rule, trigger.threshold_def[]); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.create_details_type_sql(trigger.rule, trigger.threshold_def[]) RETURNS text
    LANGUAGE sql STABLE
    AS $_$
SELECT format(
    'CREATE TYPE trigger_rule.%I AS ('
    '%s'
    ');',
    trigger.details_type_name($1),
    array_to_string(
        array_agg(format('%I %s', (c.col).name, (c.col).data_type)),
        ','
    )
) FROM (
    SELECT unnest(
        ARRAY[
            ('entity_id', 'integer'),
            ('timestamp', 'timestamp with time zone')
        ]::trigger.threshold_def[]
    ) AS col
    UNION ALL
    SELECT (kpi.name, kpi.data_type)::trigger.threshold_def AS col
    FROM trigger.get_kpi_defs($1) kpi
    UNION ALL
    SELECT unnest($2) AS col
) c;
$_$;


ALTER FUNCTION trigger.create_details_type_sql(trigger.rule, trigger.threshold_def[]) OWNER TO postgres;

--
-- Name: create_dummy_default_weight(trigger.rule); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.create_dummy_default_weight(trigger.rule) RETURNS trigger.rule
    LANGUAGE sql
    AS $_$
SELECT trigger.set_weight($1, 'SELECT 1');
$_$;


ALTER FUNCTION trigger.create_dummy_default_weight(trigger.rule) OWNER TO postgres;

--
-- Name: create_dummy_notification_data_fn(trigger.rule); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.create_dummy_notification_data_fn(trigger.rule) RETURNS trigger.rule
    LANGUAGE sql
    AS $_$
SELECT trigger.create_notification_data_fn($1, format('''"{}"''::json', $1.name));
$_$;


ALTER FUNCTION trigger.create_dummy_notification_data_fn(trigger.rule) OWNER TO postgres;

--
-- Name: create_dummy_notification_message_fn(trigger.rule); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.create_dummy_notification_message_fn(trigger.rule) RETURNS trigger.rule
    LANGUAGE sql
    AS $_$
SELECT trigger.create_notification_message_fn($1, quote_literal($1.name));
$_$;


ALTER FUNCTION trigger.create_dummy_notification_message_fn(trigger.rule) OWNER TO postgres;

--
-- Name: create_dummy_thresholds(trigger.rule, name[]); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.create_dummy_thresholds(trigger.rule, name[]) RETURNS trigger.rule
    LANGUAGE sql
    AS $_$
SELECT trigger.set_thresholds(
    $1,
    array_to_string(array_agg(format('NULL::%I %I', kpi.data_type, kpi.name)), ', ')
) FROM (
    SELECT (trigger.get_kpi_def($1, kpi_name)).* FROM unnest($2) kpi_name
) kpi;
$_$;


ALTER FUNCTION trigger.create_dummy_thresholds(trigger.rule, name[]) OWNER TO postgres;

--
-- Name: create_dummy_thresholds(trigger.rule, trigger.threshold_def[]); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.create_dummy_thresholds(trigger.rule, trigger.threshold_def[]) RETURNS trigger.rule
    LANGUAGE sql
    AS $_$
SELECT trigger.set_thresholds(
    $1,
    array_to_string(array_agg(format('NULL::%s %I', threshold.data_type, threshold.name)), ', ')
) FROM unnest($2) threshold;
$_$;


ALTER FUNCTION trigger.create_dummy_thresholds(trigger.rule, trigger.threshold_def[]) OWNER TO postgres;

--
-- Name: create_exception_threshold_fn_name(trigger.rule); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.create_exception_threshold_fn_name(trigger.rule) RETURNS name
    LANGUAGE sql IMMUTABLE
    AS $_$
SELECT ($1.name || '_create_exception_threshold')::name;
$_$;


ALTER FUNCTION trigger.create_exception_threshold_fn_name(trigger.rule) OWNER TO postgres;

--
-- Name: create_exception_threshold_fn_sql(trigger.rule); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.create_exception_threshold_fn_sql(trigger.rule) RETURNS text
    LANGUAGE sql
    AS $_$
SELECT format(
  'CREATE OR REPLACE FUNCTION trigger_rule.%I(entity integer) RETURNS trigger_rule.%I AS $fn$%s$fn$ LANGUAGE sql VOLATILE',
  trigger.create_exception_threshold_fn_name($1),
  trigger.exception_threshold_table_name($1),
  format(
    'INSERT INTO trigger_rule.%I(entity_id) VALUES ($1) RETURNING *;',
    trigger.exception_threshold_table_name($1)
  )
);
$_$;


ALTER FUNCTION trigger.create_exception_threshold_fn_sql(trigger.rule) OWNER TO postgres;

--
-- Name: create_exception_threshold_table(trigger.rule, trigger.threshold_def[]); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.create_exception_threshold_table(trigger.rule, trigger.threshold_def[]) RETURNS trigger.rule
    LANGUAGE sql
    AS $_$
SELECT public.action($1, trigger.create_exception_threshold_table_sql($1, $2));
SELECT public.action($1, trigger.get_exception_threshold_fn_sql($1));
SELECT public.action($1, trigger.create_exception_threshold_fn_sql($1));
SELECT public.action($1, trigger.get_or_create_exception_threshold_fn_sql($1));
SELECT public.action($1, trigger.create_change_exception_threshold_fn_sql($1, $2));
SELECT public.action($1, format('ALTER TABLE trigger_rule.%I OWNER TO minerva_admin', trigger.exception_threshold_table_name($1)));
$_$;


ALTER FUNCTION trigger.create_exception_threshold_table(trigger.rule, trigger.threshold_def[]) OWNER TO postgres;

--
-- Name: create_exception_threshold_table_sql(trigger.rule, trigger.threshold_def[]); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.create_exception_threshold_table_sql(trigger.rule, trigger.threshold_def[]) RETURNS text
    LANGUAGE sql STABLE
    AS $_$
SELECT format(
    'CREATE TABLE trigger_rule.%I(%s);',
    trigger.exception_threshold_table_name($1),
    array_to_string(col_def, ',')
)
FROM (
    SELECT
        ARRAY[
            'id serial',
            'entity_id integer',
            'created timestamp with time zone default now()',
            'start timestamp with time zone',
            'expires timestamp with time zone',
            'remark text'
        ]::text[] ||
        array_agg(quote_ident(threshold.name) || ' ' || threshold.data_type) AS col_def
    FROM unnest($2) threshold
) c;
$_$;


ALTER FUNCTION trigger.create_exception_threshold_table_sql(trigger.rule, trigger.threshold_def[]) OWNER TO postgres;

--
-- Name: create_exception_weight_table(trigger.rule); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.create_exception_weight_table(trigger.rule) RETURNS trigger.rule
    LANGUAGE sql
    AS $_$
SELECT trigger.action($1, trigger.exception_weight_table_sql($1));
SELECT trigger.action($1, format('ALTER TABLE trigger_rule.%I OWNER TO minerva_admin', trigger.exception_weight_table_name($1)));
$_$;


ALTER FUNCTION trigger.create_exception_weight_table(trigger.rule) OWNER TO postgres;

--
-- Name: create_fingerprint_fn(trigger.rule); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.create_fingerprint_fn(trigger.rule) RETURNS trigger.rule
    LANGUAGE sql
    AS $_$
SELECT public.action($1, trigger.create_fingerprint_fn_sql($1));
$_$;


ALTER FUNCTION trigger.create_fingerprint_fn(trigger.rule) OWNER TO postgres;

--
-- Name: create_fingerprint_fn_sql(trigger.rule); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.create_fingerprint_fn_sql(trigger.rule) RETURNS text
    LANGUAGE sql STABLE
    AS $_$
SELECT trigger.create_fingerprint_fn_sql(
    $1,
    $fn_body$SELECT now()::text;$fn_body$
);
$_$;


ALTER FUNCTION trigger.create_fingerprint_fn_sql(trigger.rule) OWNER TO postgres;

--
-- Name: create_fingerprint_fn_sql(trigger.rule, text); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.create_fingerprint_fn_sql(trigger.rule, fn_sql text) RETURNS text
    LANGUAGE sql STABLE
    AS $_$
SELECT format(
    $fn$CREATE OR REPLACE FUNCTION trigger_rule.%I(timestamp with time zone)
      RETURNS text
    AS $function$
      %s
    $function$ LANGUAGE sql STABLE$fn$,
    trigger.fingerprint_fn_name($1),
    $2
);
$_$;


ALTER FUNCTION trigger.create_fingerprint_fn_sql(trigger.rule, fn_sql text) OWNER TO postgres;

--
-- Name: create_kpi_view(trigger.rule, text); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.create_kpi_view(trigger.rule, sql text) RETURNS trigger.rule
    LANGUAGE sql
    AS $_$
SELECT trigger.action($1, trigger.create_kpi_view_sql($1, $2));
$_$;


ALTER FUNCTION trigger.create_kpi_view(trigger.rule, sql text) OWNER TO postgres;

--
-- Name: create_kpi_view_sql(trigger.rule, text); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.create_kpi_view_sql(trigger.rule, sql text) RETURNS text[]
    LANGUAGE sql STABLE
    AS $_$
SELECT ARRAY[
    trigger.kpi_view_sql($1, $2),
    format('ALTER VIEW trigger_rule.%I OWNER TO minerva_admin', trigger.kpi_view_name($1))
];
$_$;


ALTER FUNCTION trigger.create_kpi_view_sql(trigger.rule, sql text) OWNER TO postgres;

--
-- Name: create_notification_data_fn(trigger.rule, text); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.create_notification_data_fn(trigger.rule, expression text) RETURNS trigger.rule
    LANGUAGE sql
    AS $_$
SELECT public.action(
    $1,
    ARRAY[
        trigger.create_notification_data_fn_sql($1, $2),
        format(
            'ALTER FUNCTION trigger_rule.%I(trigger_rule.%I) OWNER TO minerva_admin',
            trigger.notification_data_fn_name($1),
            trigger.details_type_name($1)
        )
    ]
);
$_$;


ALTER FUNCTION trigger.create_notification_data_fn(trigger.rule, expression text) OWNER TO postgres;

--
-- Name: create_notification_data_fn_sql(trigger.rule, text); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.create_notification_data_fn_sql(trigger.rule, expression text) RETURNS text
    LANGUAGE sql
    AS $_$
SELECT format(
'CREATE OR REPLACE FUNCTION trigger_rule.%I(trigger_rule.%I)
    RETURNS json
AS $function$
DECLARE
  data json;
BEGIN
SELECT (%s) INTO data;
RETURN data;
END;
$function$ LANGUAGE PLPGSQL STABLE',
    trigger.notification_data_fn_name($1),
    trigger.details_type_name($1),
    $2
);
$_$;


ALTER FUNCTION trigger.create_notification_data_fn_sql(trigger.rule, expression text) OWNER TO postgres;

--
-- Name: create_notification_fn(trigger.rule); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.create_notification_fn(trigger.rule) RETURNS trigger.rule
    LANGUAGE sql
    AS $_$
SELECT public.action($1, trigger.create_notification_fn_sql($1));
$_$;


ALTER FUNCTION trigger.create_notification_fn(trigger.rule) OWNER TO postgres;

--
-- Name: create_notification_fn_sql(trigger.rule); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.create_notification_fn_sql(trigger.rule) RETURNS text[]
    LANGUAGE sql STABLE
    AS $_$
SELECT ARRAY[
    trigger.notification_fn_sql($1),
    format('ALTER FUNCTION trigger_rule.%I(timestamp with time zone) OWNER TO minerva_admin', trigger.notification_fn_name($1))
];
$_$;


ALTER FUNCTION trigger.create_notification_fn_sql(trigger.rule) OWNER TO postgres;

--
-- Name: create_notification_message_fn(trigger.rule, text); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.create_notification_message_fn(trigger.rule, expression text) RETURNS trigger.rule
    LANGUAGE sql
    AS $_$
SELECT public.action(
    $1,
    ARRAY[
        trigger.notification_message_fn_sql($1, $2),
        format(
            'ALTER FUNCTION trigger_rule.%I(trigger_rule.%I) OWNER TO minerva_admin',
            trigger.notification_message_fn_name($1),
            trigger.details_type_name($1)
        )
    ]
);
$_$;


ALTER FUNCTION trigger.create_notification_message_fn(trigger.rule, expression text) OWNER TO postgres;

--
-- Name: create_notification_type(trigger.rule); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.create_notification_type(trigger.rule) RETURNS trigger.rule
    LANGUAGE sql
    AS $_$
SELECT trigger.action(
    $1,
    trigger.create_notification_type_sql($1)
);
$_$;


ALTER FUNCTION trigger.create_notification_type(trigger.rule) OWNER TO postgres;

--
-- Name: create_notification_type_sql(trigger.rule); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.create_notification_type_sql(trigger.rule) RETURNS text
    LANGUAGE sql IMMUTABLE
    AS $_$
SELECT format(
    $type$
    CREATE TYPE trigger_rule.%I AS (
        entity_id integer,
        timestamp timestamp with time zone,
        details text
    )
    $type$,
    trigger.notification_type_name($1)
);
$_$;


ALTER FUNCTION trigger.create_notification_type_sql(trigger.rule) OWNER TO postgres;

--
-- Name: create_notifications(name); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.create_notifications(rule_name name) RETURNS integer
    LANGUAGE sql
    AS $_$
SELECT trigger.create_notifications(trigger.get_rule($1));
$_$;


ALTER FUNCTION trigger.create_notifications(rule_name name) OWNER TO postgres;

--
-- Name: create_notifications(trigger.rule); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.create_notifications(trigger.rule) RETURNS integer
    LANGUAGE sql
    AS $_$
SELECT trigger.create_notifications($1, notification_store, $1.default_interval)
FROM notification_directory.notification_store
WHERE id = $1.notification_store_id;
$_$;


ALTER FUNCTION trigger.create_notifications(trigger.rule) OWNER TO postgres;

--
-- Name: create_notifications(name, interval); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.create_notifications(rule_name name, interval) RETURNS integer
    LANGUAGE sql
    AS $_$
SELECT trigger.create_notifications(trigger.get_rule($1), $2);
$_$;


ALTER FUNCTION trigger.create_notifications(rule_name name, interval) OWNER TO postgres;

--
-- Name: create_notifications(name, timestamp with time zone); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.create_notifications(rule_name name, timestamp with time zone) RETURNS integer
    LANGUAGE sql
    AS $_$
SELECT trigger.create_notifications(rule, notification_store, $2)
FROM trigger.rule
JOIN notification_directory.notification_store ON notification_store.id = rule.notification_store_id
WHERE rule.name = $1;
$_$;


ALTER FUNCTION trigger.create_notifications(rule_name name, timestamp with time zone) OWNER TO postgres;

--
-- Name: create_notifications(trigger.rule, interval); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.create_notifications(trigger.rule, interval) RETURNS integer
    LANGUAGE sql
    AS $_$
SELECT trigger.create_notifications($1, notification_store, $2)
FROM notification_directory.notification_store
WHERE id = $1.notification_store_id;
$_$;


ALTER FUNCTION trigger.create_notifications(trigger.rule, interval) OWNER TO postgres;

--
-- Name: create_notifications(trigger.rule, timestamp with time zone); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.create_notifications(trigger.rule, timestamp with time zone) RETURNS integer
    LANGUAGE sql
    AS $_$
SELECT
    trigger.create_notifications($1, notification_store, $2)
FROM notification_directory.notification_store
WHERE id = $1.notification_store_id;
$_$;


ALTER FUNCTION trigger.create_notifications(trigger.rule, timestamp with time zone) OWNER TO postgres;

--
-- Name: create_notifications(name, name, interval); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.create_notifications(rule_name name, notification_store_name name, interval) RETURNS integer
    LANGUAGE sql
    AS $_$
SELECT trigger.create_notifications(
    trigger.get_rule($1),
    notification_directory.get_notification_store($2),
    $3
);
$_$;


ALTER FUNCTION trigger.create_notifications(rule_name name, notification_store_name name, interval) OWNER TO postgres;

--
-- Name: create_notifications(name, name, timestamp with time zone); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.create_notifications(rule_name name, notification_store_name name, timestamp with time zone) RETURNS integer
    LANGUAGE sql
    AS $_$
SELECT trigger.create_notifications(
    trigger.get_rule($1),
    notification_directory.get_notification_store($2),
    $3
);
$_$;


ALTER FUNCTION trigger.create_notifications(rule_name name, notification_store_name name, timestamp with time zone) OWNER TO postgres;

--
-- Name: create_notifications(trigger.rule, notification_directory.notification_store, interval); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.create_notifications(trigger.rule, notification_directory.notification_store, interval) RETURNS integer
    LANGUAGE plpgsql
    AS $_$
DECLARE
    num_rows integer;
BEGIN
    EXECUTE format(
$query$
INSERT INTO notification.%I(entity_id, timestamp, created, rule_id, weight, details)
(SELECT entity_id, timestamp, now(), $1, weight, details FROM trigger_rule.%I WHERE timestamp > now() - $2)
$query$,
        notification_directory.staging_table_name($2), trigger.notification_view_name($1)
    )
    USING $1.id, $3;

    SELECT trigger.transfer_notifications_from_staging($2) INTO num_rows;

    RETURN num_rows;
END;
$_$;


ALTER FUNCTION trigger.create_notifications(trigger.rule, notification_directory.notification_store, interval) OWNER TO postgres;

--
-- Name: create_notifications(trigger.rule, notification_directory.notification_store, timestamp with time zone); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.create_notifications(trigger.rule, notification_directory.notification_store, timestamp with time zone) RETURNS integer
    LANGUAGE plpgsql
    AS $_$
DECLARE
    num_rows integer;
BEGIN
    EXECUTE format(
$query$
INSERT INTO notification.%I(entity_id, timestamp, created, rule_id, weight, details, data)
(SELECT entity_id, timestamp, now(), $1, weight, details, data FROM trigger_rule.%I($2) WHERE data IS NOT NULL)
$query$,
        notification_directory.staging_table_name($2), trigger.notification_fn_name($1)
    )
    USING $1.id, $3;

    SELECT trigger.transfer_notifications_from_staging($2) INTO num_rows;

    RETURN num_rows;
END;
$_$;


ALTER FUNCTION trigger.create_notifications(trigger.rule, notification_directory.notification_store, timestamp with time zone) OWNER TO postgres;

--
-- Name: create_rule(text, trigger.threshold_def[]); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.create_rule(text, trigger.threshold_def[]) RETURNS trigger.rule
    LANGUAGE sql
    AS $_$
SELECT trigger.setup_rule(trigger.define($1::name), $2);
$_$;


ALTER FUNCTION trigger.create_rule(text, trigger.threshold_def[]) OWNER TO postgres;

--
-- Name: create_rule_fn(trigger.rule, text); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.create_rule_fn(trigger.rule, rule_view_sql text) RETURNS trigger.rule
    LANGUAGE sql
    AS $_$
SELECT public.action($1, trigger.create_rule_fn_sql($1, $2));
$_$;


ALTER FUNCTION trigger.create_rule_fn(trigger.rule, rule_view_sql text) OWNER TO postgres;

--
-- Name: create_rule_fn_sql(trigger.rule, text); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.create_rule_fn_sql(trigger.rule, rule_view_sql text) RETURNS text[]
    LANGUAGE sql IMMUTABLE
    AS $_$
SELECT ARRAY[
    format(
        'CREATE OR REPLACE FUNCTION trigger_rule.%I(timestamp with time zone) RETURNS SETOF trigger_rule.%I AS $fn$ %s $fn$ LANGUAGE sql STABLE',
        trigger.rule_fn_name($1),
        trigger.details_type_name($1),
        $2
    ),
    format(
        'ALTER FUNCTION trigger_rule.%I(timestamp with time zone) OWNER TO minerva_admin',
        trigger.rule_fn_name($1)
    )
];
$_$;


ALTER FUNCTION trigger.create_rule_fn_sql(trigger.rule, rule_view_sql text) OWNER TO postgres;

--
-- Name: create_rule_view(trigger.rule, text); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.create_rule_view(trigger.rule, rule_view_sql text) RETURNS trigger.rule
    LANGUAGE sql SECURITY DEFINER
    AS $_$
SELECT trigger.action($1, trigger.create_rule_view_sql($1, $2));
$_$;


ALTER FUNCTION trigger.create_rule_view(trigger.rule, rule_view_sql text) OWNER TO postgres;

--
-- Name: create_rule_view_sql(trigger.rule, text); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.create_rule_view_sql(trigger.rule, rule_view_sql text) RETURNS text[]
    LANGUAGE sql STABLE
    AS $_$
SELECT ARRAY[
    format('CREATE OR REPLACE VIEW trigger_rule.%I AS %s', trigger.rule_view_name($1), $2),
    format('ALTER VIEW trigger_rule.%I OWNER TO minerva_admin', trigger.rule_view_name($1))
];
$_$;


ALTER FUNCTION trigger.create_rule_view_sql(trigger.rule, rule_view_sql text) OWNER TO postgres;

--
-- Name: create_runnable_fn(trigger.rule); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.create_runnable_fn(trigger.rule) RETURNS trigger.rule
    LANGUAGE sql
    AS $_$
SELECT public.action($1, trigger.create_runnable_fn_sql($1));
$_$;


ALTER FUNCTION trigger.create_runnable_fn(trigger.rule) OWNER TO postgres;

--
-- Name: create_runnable_fn_sql(trigger.rule); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.create_runnable_fn_sql(trigger.rule) RETURNS text
    LANGUAGE sql STABLE
    AS $_$
SELECT trigger.create_runnable_fn_sql($1, 'SELECT TRUE;');
$_$;


ALTER FUNCTION trigger.create_runnable_fn_sql(trigger.rule) OWNER TO postgres;

--
-- Name: create_runnable_fn_sql(trigger.rule, text); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.create_runnable_fn_sql(trigger.rule, fn_body text) RETURNS text
    LANGUAGE sql STABLE
    AS $_$
SELECT format(
    $fn$CREATE OR REPLACE FUNCTION trigger_rule.%I(timestamp with time zone)
      RETURNS boolean
    AS $function$
    %s
    $function$ LANGUAGE sql STABLE$fn$,
    trigger.runnable_fn_name($1),
    $2
);
$_$;


ALTER FUNCTION trigger.create_runnable_fn_sql(trigger.rule, fn_body text) OWNER TO postgres;

--
-- Name: create_set_thresholds_fn(trigger.rule); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.create_set_thresholds_fn(trigger.rule) RETURNS trigger.rule
    LANGUAGE sql
    AS $_$
SELECT trigger.action($1, trigger.create_set_thresholds_fn_sql($1));
$_$;


ALTER FUNCTION trigger.create_set_thresholds_fn(trigger.rule) OWNER TO postgres;

--
-- Name: create_set_thresholds_fn_sql(trigger.rule); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.create_set_thresholds_fn_sql(trigger.rule) RETURNS text
    LANGUAGE sql IMMUTABLE
    AS $_$
SELECT format(
    $def$CREATE OR REPLACE FUNCTION trigger_rule.%I(%s)
    RETURNS integer SECURITY DEFINER AS
    $function$
    BEGIN
        EXECUTE format('CREATE OR REPLACE VIEW trigger_rule.%I AS SELECT %s', %s);
        RETURN 42;
    END;
    $function$ LANGUAGE plpgsql VOLATILE$def$,
    trigger.set_thresholds_fn_name($1),
    array_to_string(array_agg(format('%I %s', t.name, t.data_type)), ', '),
    trigger.threshold_view_name($1),
    array_to_string(array_agg(format('%%L::%s AS %I', t.data_type, t.name)), ', '),
    array_to_string(array_agg(format('$%s', t.row_num)), ', ')
) FROM (SELECT d.*, row_number() over() AS row_num FROM trigger.get_threshold_defs($1) d) t;
$_$;


ALTER FUNCTION trigger.create_set_thresholds_fn_sql(trigger.rule) OWNER TO postgres;

--
-- Name: create_trigger_notification_store(name); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.create_trigger_notification_store(name) RETURNS notification_directory.notification_store
    LANGUAGE sql
    AS $_$
SELECT trigger.add_insert_trigger(
        notification_directory.create_notification_store($1, ARRAY[
            ('created', 'timestamp with time zone', 'time of notification creation'),
            ('rule_id', 'integer', 'source rule for this notification'),
            ('weight', 'integer', 'weight/importance of the notification'),
            ('details', 'text', 'extra information')
        ]::notification_directory.attr_def[])
);
$_$;


ALTER FUNCTION trigger.create_trigger_notification_store(name) OWNER TO postgres;

--
-- Name: create_with_threshold_fn(trigger.rule); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.create_with_threshold_fn(trigger.rule) RETURNS trigger.rule
    LANGUAGE sql
    AS $_$
SELECT public.action(
    $1,
    ARRAY[
        format(
            'CREATE OR REPLACE FUNCTION trigger_rule.%I(timestamp with time zone) RETURNS SETOF trigger_rule.%I AS $fn$%s$fn$ LANGUAGE sql STABLE',
            trigger.with_threshold_fn_name($1),
            trigger.details_type_name($1),
            trigger.with_threshold_fn_sql($1)
        ),
        format(
            'ALTER FUNCTION trigger_rule.%I(timestamp with time zone) OWNER TO minerva_admin',
            trigger.with_threshold_fn_name($1)
        )
    ]
);
$_$;


ALTER FUNCTION trigger.create_with_threshold_fn(trigger.rule) OWNER TO postgres;

--
-- Name: create_with_threshold_view(trigger.rule); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.create_with_threshold_view(trigger.rule) RETURNS trigger.rule
    LANGUAGE sql
    AS $_$
SELECT trigger.action($1, format('CREATE OR REPLACE VIEW trigger_rule.%I AS %s', trigger.with_threshold_view_name($1), trigger.with_threshold_view_sql($1)));
SELECT trigger.action($1, format('ALTER VIEW trigger_rule.%I OWNER TO minerva_admin', trigger.with_threshold_view_name($1)));
$_$;


ALTER FUNCTION trigger.create_with_threshold_view(trigger.rule) OWNER TO postgres;

--
-- Name: define(name); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.define(name) RETURNS trigger.rule
    LANGUAGE sql
    AS $_$
SELECT COALESCE(trigger.get_rule($1), trigger.add_rule($1));
$_$;


ALTER FUNCTION trigger.define(name) OWNER TO postgres;

--
-- Name: define_notification_data(name, text); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.define_notification_data(name, expression text) RETURNS trigger.rule
    LANGUAGE sql
    AS $_$
SELECT trigger.create_notification_data_fn(trigger.get_rule($1), $2);
$_$;


ALTER FUNCTION trigger.define_notification_data(name, expression text) OWNER TO postgres;

--
-- Name: define_notification_message(name, text); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.define_notification_message(name, expression text) RETURNS trigger.rule
    LANGUAGE sql
    AS $_$
SELECT trigger.create_notification_message_fn(trigger.get_rule($1), $2);
$_$;


ALTER FUNCTION trigger.define_notification_message(name, expression text) OWNER TO postgres;

--
-- Name: define_thresholds(trigger.rule, trigger.threshold_def[]); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.define_thresholds(trigger.rule, trigger.threshold_def[]) RETURNS trigger.rule
    LANGUAGE sql
    AS $_$
SELECT trigger.create_details_type($1, $2);
SELECT CASE WHEN array_length($2, 1) > 0 THEN
    trigger.create_dummy_thresholds($1, $2)
END;
SELECT trigger.create_exception_threshold_table($1, $2);
SELECT CASE WHEN array_length($2, 1) > 0 THEN
    trigger.create_set_thresholds_fn($1)
END;
SELECT trigger.create_with_threshold_fn($1);
$_$;


ALTER FUNCTION trigger.define_thresholds(trigger.rule, trigger.threshold_def[]) OWNER TO postgres;

--
-- Name: delete_rule(name); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.delete_rule(name) RETURNS bigint
    LANGUAGE sql
    AS $_$
SELECT trigger.cleanup_rule(rule) FROM trigger.rule WHERE name = $1;
WITH deleted AS ( DELETE FROM trigger.rule WHERE name = $1 RETURNING * ) SELECT count(*) FROM deleted;
$_$;


ALTER FUNCTION trigger.delete_rule(name) OWNER TO postgres;

--
-- Name: details_type_name(trigger.rule); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.details_type_name(trigger.rule) RETURNS name
    LANGUAGE sql IMMUTABLE
    AS $_$
SELECT ($1.name || '_details')::name;
$_$;


ALTER FUNCTION trigger.details_type_name(trigger.rule) OWNER TO postgres;

--
-- Name: drop_details_type(trigger.rule); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.drop_details_type(trigger.rule) RETURNS trigger.rule
    LANGUAGE sql
    AS $_$
SELECT public.action($1, trigger.drop_details_type_sql($1));
$_$;


ALTER FUNCTION trigger.drop_details_type(trigger.rule) OWNER TO postgres;

--
-- Name: drop_details_type_sql(trigger.rule); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.drop_details_type_sql(trigger.rule) RETURNS text
    LANGUAGE sql IMMUTABLE
    AS $_$
SELECT format(
    'DROP TYPE IF EXISTS trigger_rule.%I CASCADE;',
    trigger.details_type_name($1)
);
$_$;


ALTER FUNCTION trigger.drop_details_type_sql(trigger.rule) OWNER TO postgres;

--
-- Name: drop_exception_threshold_table_sql(trigger.rule); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.drop_exception_threshold_table_sql(trigger.rule) RETURNS text
    LANGUAGE sql IMMUTABLE
    AS $_$
SELECT format('DROP TABLE IF EXISTS trigger_rule.%I', trigger.exception_threshold_table_name($1))
$_$;


ALTER FUNCTION trigger.drop_exception_threshold_table_sql(trigger.rule) OWNER TO postgres;

--
-- Name: drop_exception_weight_table_sql(trigger.rule); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.drop_exception_weight_table_sql(trigger.rule) RETURNS text
    LANGUAGE sql IMMUTABLE
    AS $_$
SELECT format('DROP TABLE IF EXISTS trigger_rule.%I', trigger.exception_weight_table_name($1));
$_$;


ALTER FUNCTION trigger.drop_exception_weight_table_sql(trigger.rule) OWNER TO postgres;

--
-- Name: drop_fingerprint_fn(trigger.rule); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.drop_fingerprint_fn(trigger.rule) RETURNS trigger.rule
    LANGUAGE sql
    AS $_$
SELECT public.action($1, trigger.drop_fingerprint_fn_sql($1));
$_$;


ALTER FUNCTION trigger.drop_fingerprint_fn(trigger.rule) OWNER TO postgres;

--
-- Name: drop_fingerprint_fn_sql(trigger.rule); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.drop_fingerprint_fn_sql(trigger.rule) RETURNS text
    LANGUAGE sql IMMUTABLE
    AS $_$
SELECT format('DROP FUNCTION IF EXISTS trigger_rule.%I(timestamp with time zone)', trigger.fingerprint_fn_name($1));
$_$;


ALTER FUNCTION trigger.drop_fingerprint_fn_sql(trigger.rule) OWNER TO postgres;

--
-- Name: drop_kpi_function_sql(trigger.rule); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.drop_kpi_function_sql(trigger.rule) RETURNS text
    LANGUAGE sql IMMUTABLE
    AS $_$
SELECT format('DROP FUNCTION trigger_rule.%I(timestamp with time zone) CASCADE', "trigger".kpi_function_name($1));
$_$;


ALTER FUNCTION trigger.drop_kpi_function_sql(trigger.rule) OWNER TO postgres;

--
-- Name: drop_kpi_type_sql(trigger.rule); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.drop_kpi_type_sql(trigger.rule) RETURNS text
    LANGUAGE sql IMMUTABLE
    AS $_$
SELECT format(
    'DROP TYPE IF EXISTS trigger_rule.%I CASCADE;',
    trigger.kpi_type_name($1)
);
$_$;


ALTER FUNCTION trigger.drop_kpi_type_sql(trigger.rule) OWNER TO postgres;

--
-- Name: drop_kpi_view_sql(trigger.rule); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.drop_kpi_view_sql(trigger.rule) RETURNS text
    LANGUAGE sql IMMUTABLE
    AS $_$
SELECT format('DROP VIEW trigger_rule.%I CASCADE', "trigger".kpi_view_name($1));
$_$;


ALTER FUNCTION trigger.drop_kpi_view_sql(trigger.rule) OWNER TO postgres;

--
-- Name: drop_notification_data_fn_sql(trigger.rule); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.drop_notification_data_fn_sql(trigger.rule) RETURNS text
    LANGUAGE sql STABLE
    AS $_$
SELECT format(
    'DROP FUNCTION trigger_rule.%I',
    trigger.notification_data_fn_name($1),
    trigger.details_type_name($1)
);
$_$;


ALTER FUNCTION trigger.drop_notification_data_fn_sql(trigger.rule) OWNER TO postgres;

--
-- Name: drop_notification_fn_sql(trigger.rule); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.drop_notification_fn_sql(trigger.rule) RETURNS text
    LANGUAGE sql IMMUTABLE
    AS $_$
SELECT format('DROP FUNCTION trigger_rule.%I(timestamp with time zone)', trigger.notification_fn_name($1));
$_$;


ALTER FUNCTION trigger.drop_notification_fn_sql(trigger.rule) OWNER TO postgres;

--
-- Name: drop_notification_message_fn_sql(trigger.rule); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.drop_notification_message_fn_sql(trigger.rule) RETURNS text
    LANGUAGE sql STABLE
    AS $_$
SELECT format(
    'DROP FUNCTION trigger_rule.%I',
    trigger.notification_message_fn_name($1),
    trigger.details_type_name($1)
);
$_$;


ALTER FUNCTION trigger.drop_notification_message_fn_sql(trigger.rule) OWNER TO postgres;

--
-- Name: drop_notification_type_sql(trigger.rule); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.drop_notification_type_sql(trigger.rule) RETURNS text
    LANGUAGE sql IMMUTABLE
    AS $_$
SELECT format(
    'DROP TYPE IF EXISTS trigger_rule.%I',
    trigger.notification_type_name($1)
);
$_$;


ALTER FUNCTION trigger.drop_notification_type_sql(trigger.rule) OWNER TO postgres;

--
-- Name: drop_rule_fn_sql(trigger.rule); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.drop_rule_fn_sql(trigger.rule) RETURNS text
    LANGUAGE sql IMMUTABLE
    AS $_$
SELECT format(
    'DROP FUNCTION trigger_rule.%I(timestamp with time zone)',
    trigger.rule_fn_name($1)
);
$_$;


ALTER FUNCTION trigger.drop_rule_fn_sql(trigger.rule) OWNER TO postgres;

--
-- Name: drop_rule_view_sql(trigger.rule); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.drop_rule_view_sql(trigger.rule) RETURNS text
    LANGUAGE sql IMMUTABLE
    AS $_$
SELECT format('DROP VIEW trigger_rule.%I CASCADE', $1.name);
$_$;


ALTER FUNCTION trigger.drop_rule_view_sql(trigger.rule) OWNER TO postgres;

--
-- Name: drop_runnable_fn_sql(trigger.rule); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.drop_runnable_fn_sql(trigger.rule) RETURNS text
    LANGUAGE sql STABLE
    AS $_$
SELECT format('DROP FUNCTION trigger_rule.%I(timestamp with time zone)', trigger.runnable_fn_name($1));
$_$;


ALTER FUNCTION trigger.drop_runnable_fn_sql(trigger.rule) OWNER TO postgres;

--
-- Name: drop_set_thresholds_fn_sql(trigger.rule); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.drop_set_thresholds_fn_sql(trigger.rule) RETURNS text
    LANGUAGE sql IMMUTABLE
    AS $_$
SELECT format(
    'DROP FUNCTION trigger_rule.%I(%s)',
    trigger.set_thresholds_fn_name($1),
    array_to_string(array_agg(format('%s', t.data_type)), ', ')
)
FROM trigger.get_threshold_defs($1) t;
$_$;


ALTER FUNCTION trigger.drop_set_thresholds_fn_sql(trigger.rule) OWNER TO postgres;

--
-- Name: drop_thresholds_view_sql(trigger.rule); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.drop_thresholds_view_sql(trigger.rule) RETURNS text
    LANGUAGE sql IMMUTABLE
    AS $_$
SELECT format('DROP VIEW trigger_rule.%I', trigger.threshold_view_name($1))
$_$;


ALTER FUNCTION trigger.drop_thresholds_view_sql(trigger.rule) OWNER TO postgres;

--
-- Name: drop_weight_fn_sql(trigger.rule); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.drop_weight_fn_sql(trigger.rule) RETURNS text
    LANGUAGE sql IMMUTABLE
    AS $_$
SELECT format(
    'DROP FUNCTION trigger_rule.%I(trigger_rule.%I)',
    trigger.weight_fn_name($1),
    trigger.details_type_name($1)
);
$_$;


ALTER FUNCTION trigger.drop_weight_fn_sql(trigger.rule) OWNER TO postgres;

--
-- Name: drop_with_threshold_fn_sql(trigger.rule); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.drop_with_threshold_fn_sql(trigger.rule) RETURNS text
    LANGUAGE sql STABLE
    AS $_$
SELECT format(
    'DROP FUNCTION trigger_rule.%I(timestamp with time zone)',
    trigger.with_threshold_fn_name($1)
);
$_$;


ALTER FUNCTION trigger.drop_with_threshold_fn_sql(trigger.rule) OWNER TO postgres;

--
-- Name: drop_with_threshold_view_sql(trigger.rule); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.drop_with_threshold_view_sql(trigger.rule) RETURNS text
    LANGUAGE sql IMMUTABLE
    AS $_$
SELECT format('DROP VIEW trigger_rule.%I', trigger.with_threshold_view_name($1));
$_$;


ALTER FUNCTION trigger.drop_with_threshold_view_sql(trigger.rule) OWNER TO postgres;

--
-- Name: exception_threshold_table_name(trigger.rule); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.exception_threshold_table_name(trigger.rule) RETURNS name
    LANGUAGE sql IMMUTABLE
    AS $_$
SELECT ($1.name || '_exception_threshold')::name;
$_$;


ALTER FUNCTION trigger.exception_threshold_table_name(trigger.rule) OWNER TO postgres;

--
-- Name: exception_weight_table_name(trigger.rule); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.exception_weight_table_name(trigger.rule) RETURNS name
    LANGUAGE sql IMMUTABLE
    AS $_$
SELECT ($1.name || '_exception_weight')::name;
$_$;


ALTER FUNCTION trigger.exception_weight_table_name(trigger.rule) OWNER TO postgres;

--
-- Name: exception_weight_table_sql(trigger.rule); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.exception_weight_table_sql(trigger.rule) RETURNS text
    LANGUAGE sql IMMUTABLE
    AS $_$
SELECT format(
    $$CREATE TABLE trigger_rule.%I
    (
        id serial,
        entity_id integer,
        created timestamp with time zone not null default now(),
        start timestamp with time zone not null default now(),
        expires timestamp with time zone not null default now() + interval '3 months',
        weight integer not null
    );$$,
    trigger.exception_weight_table_name($1)
);
$_$;


ALTER FUNCTION trigger.exception_weight_table_sql(trigger.rule) OWNER TO postgres;

--
-- Name: fingerprint_fn_name(trigger.rule); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.fingerprint_fn_name(trigger.rule) RETURNS name
    LANGUAGE sql IMMUTABLE
    AS $_$
SELECT ($1.name || '_fingerprint')::name;
$_$;


ALTER FUNCTION trigger.fingerprint_fn_name(trigger.rule) OWNER TO postgres;

--
-- Name: get_exception_threshold_fn_name(trigger.rule); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.get_exception_threshold_fn_name(trigger.rule) RETURNS name
    LANGUAGE sql IMMUTABLE
    AS $_$
SELECT ($1.name || '_get_exception_threshold')::name;
$_$;


ALTER FUNCTION trigger.get_exception_threshold_fn_name(trigger.rule) OWNER TO postgres;

--
-- Name: get_exception_threshold_fn_sql(trigger.rule); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.get_exception_threshold_fn_sql(trigger.rule) RETURNS text
    LANGUAGE sql STABLE
    AS $_$
SELECT format(
  'CREATE OR REPLACE FUNCTION trigger_rule.%I(entity integer) RETURNS trigger_rule.%I AS $fn$%s$fn$ LANGUAGE sql VOLATILE',
  trigger.get_exception_threshold_fn_name($1),
  trigger.exception_threshold_table_name($1),
  format(
    'SELECT * FROM trigger_rule.%I WHERE entity_id = entity;',
    trigger.exception_threshold_table_name($1)
  )
);
$_$;


ALTER FUNCTION trigger.get_exception_threshold_fn_sql(trigger.rule) OWNER TO postgres;

--
-- Name: get_kpi_def(trigger.rule, name); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.get_kpi_def(trigger.rule, name) RETURNS trigger.kpi_def
    LANGUAGE plpgsql STABLE
    AS $_$
DECLARE
    result trigger.kpi_def;
BEGIN
    SELECT INTO result * FROM trigger.get_kpi_defs($1) WHERE name = $2;

    IF NOT FOUND THEN
        RAISE EXCEPTION 'no such KPI: ''%''', $2;
    END IF;

    RETURN result;
END;
$_$;


ALTER FUNCTION trigger.get_kpi_def(trigger.rule, name) OWNER TO postgres;

--
-- Name: get_kpi_defs(trigger.rule); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.get_kpi_defs(trigger.rule) RETURNS SETOF trigger.kpi_def
    LANGUAGE sql STABLE
    AS $_$
SELECT (attname, typname)::trigger.kpi_def
FROM pg_type
JOIN pg_attribute ON pg_attribute.atttypid = pg_type.oid
JOIN pg_class ON pg_class.oid = pg_attribute.attrelid
JOIN pg_namespace ON pg_namespace.oid = pg_class.relnamespace
WHERE
nspname = 'trigger_rule' AND
relname = "trigger".kpi_view_name($1) AND
attnum > 0 AND
NOT attname IN ('entity_id', 'timestamp') AND
NOT pg_attribute.attisdropped;
$_$;


ALTER FUNCTION trigger.get_kpi_defs(trigger.rule) OWNER TO postgres;

--
-- Name: get_kpi_view_sql(trigger.rule); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.get_kpi_view_sql(trigger.rule) RETURNS text
    LANGUAGE sql STABLE
    AS $_$
SELECT pg_get_viewdef(oid, true)
FROM pg_class
WHERE relname = trigger.kpi_view_name($1);
$_$;


ALTER FUNCTION trigger.get_kpi_view_sql(trigger.rule) OWNER TO postgres;

--
-- Name: get_notification_fn_sql(trigger.rule); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.get_notification_fn_sql(trigger.rule) RETURNS text
    LANGUAGE sql STABLE
    AS $_$
SELECT pg_get_functiondef(oid)
FROM pg_proc
WHERE proname = trigger.notification_fn_name($1);
$_$;


ALTER FUNCTION trigger.get_notification_fn_sql(trigger.rule) OWNER TO postgres;

--
-- Name: get_or_create_exception_threshold_fn_name(trigger.rule); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.get_or_create_exception_threshold_fn_name(trigger.rule) RETURNS name
    LANGUAGE sql IMMUTABLE
    AS $_$
SELECT ($1.name || '_get_or_create_exception_threshold')::name;
$_$;


ALTER FUNCTION trigger.get_or_create_exception_threshold_fn_name(trigger.rule) OWNER TO postgres;

--
-- Name: get_or_create_exception_threshold_fn_sql(trigger.rule); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.get_or_create_exception_threshold_fn_sql(trigger.rule) RETURNS text
    LANGUAGE sql
    AS $_$
SELECT format(
  'CREATE OR REPLACE FUNCTION trigger_rule.%I(entity integer) RETURNS trigger_rule.%I AS $fn$%s$fn$ LANGUAGE sql VOLATILE',
  trigger.get_or_create_exception_threshold_fn_name($1),
  trigger.exception_threshold_table_name($1),
  format('SELECT COALESCE(trigger_rule.%I($1), trigger_rule.%I($1));',
     trigger.get_exception_threshold_fn_name($1),
     trigger.create_exception_threshold_fn_name($1)
     )
);
$_$;


ALTER FUNCTION trigger.get_or_create_exception_threshold_fn_sql(trigger.rule) OWNER TO postgres;

--
-- Name: get_rule(name); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.get_rule(name) RETURNS trigger.rule
    LANGUAGE sql STABLE
    AS $_$
SELECT rule FROM "trigger".rule WHERE name = $1;
$_$;


ALTER FUNCTION trigger.get_rule(name) OWNER TO postgres;

--
-- Name: get_rule_view_sql(trigger.rule); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.get_rule_view_sql(trigger.rule) RETURNS text
    LANGUAGE sql STABLE
    AS $_$
SELECT
	pg_get_viewdef(oid, true)
FROM pg_class
WHERE relname = trigger.rule_view_name($1);
$_$;


ALTER FUNCTION trigger.get_rule_view_sql(trigger.rule) OWNER TO postgres;

--
-- Name: get_threshold_defs(trigger.rule); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.get_threshold_defs(trigger.rule) RETURNS SETOF trigger.kpi_def
    LANGUAGE sql STABLE
    AS $_$
SELECT (attname, typname)::trigger.kpi_def
FROM pg_type
JOIN pg_attribute ON pg_attribute.atttypid = pg_type.oid
JOIN pg_class ON pg_class.oid = pg_attribute.attrelid
JOIN pg_namespace ON pg_namespace.oid = pg_class.relnamespace
WHERE
nspname = 'trigger_rule' AND
relname = "trigger".threshold_view_name($1) AND
attnum > 0 AND
NOT pg_attribute.attisdropped;
$_$;


ALTER FUNCTION trigger.get_threshold_defs(trigger.rule) OWNER TO postgres;

--
-- Name: get_weight_fn_sql(trigger.rule); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.get_weight_fn_sql(trigger.rule) RETURNS text
    LANGUAGE sql STABLE
    AS $_$
SELECT pg_get_functiondef(oid) FROM pg_proc WHERE proname = trigger.weight_fn_name($1);
$_$;


ALTER FUNCTION trigger.get_weight_fn_sql(trigger.rule) OWNER TO postgres;

--
-- Name: get_with_threshold_view_sql(trigger.rule); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.get_with_threshold_view_sql(trigger.rule) RETURNS text
    LANGUAGE sql STABLE
    AS $_$
SELECT pg_get_viewdef(oid, true) FROM pg_class WHERE relname = trigger.with_threshold_view_name($1);
$_$;


ALTER FUNCTION trigger.get_with_threshold_view_sql(trigger.rule) OWNER TO postgres;

--
-- Name: has_thresholds(trigger.rule); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.has_thresholds(trigger.rule) RETURNS boolean
    LANGUAGE sql STABLE
    AS $_$
SELECT EXISTS(
    SELECT 1
    FROM pg_class
    WHERE relname = trigger.threshold_view_name($1) AND relkind = 'v'
);
$_$;


ALTER FUNCTION trigger.has_thresholds(trigger.rule) OWNER TO postgres;

--
-- Name: FUNCTION has_thresholds(trigger.rule); Type: COMMENT; Schema: trigger; Owner: postgres
--

COMMENT ON FUNCTION trigger.has_thresholds(trigger.rule) IS 'Return true if there is a view with thresholds for the specified rule';


--
-- Name: kpi_fn_name(trigger.rule); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.kpi_fn_name(trigger.rule) RETURNS name
    LANGUAGE sql IMMUTABLE
    AS $_$
SELECT ($1.name || '_kpi')::name;
$_$;


ALTER FUNCTION trigger.kpi_fn_name(trigger.rule) OWNER TO postgres;

--
-- Name: kpi_function_name(trigger.rule); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.kpi_function_name(trigger.rule) RETURNS name
    LANGUAGE sql IMMUTABLE
    AS $_$
SELECT ($1.name || '_kpi')::name;
$_$;


ALTER FUNCTION trigger.kpi_function_name(trigger.rule) OWNER TO postgres;

--
-- Name: kpi_type_name(trigger.rule); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.kpi_type_name(trigger.rule) RETURNS name
    LANGUAGE sql IMMUTABLE
    AS $_$
SELECT ($1.name || '_kpi')::name;
$_$;


ALTER FUNCTION trigger.kpi_type_name(trigger.rule) OWNER TO postgres;

--
-- Name: kpi_view_name(trigger.rule); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.kpi_view_name(trigger.rule) RETURNS name
    LANGUAGE sql IMMUTABLE
    AS $_$
SELECT ($1.name || '_kpi')::name;
$_$;


ALTER FUNCTION trigger.kpi_view_name(trigger.rule) OWNER TO postgres;

--
-- Name: kpi_view_sql(trigger.rule, text); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.kpi_view_sql(trigger.rule, sql text) RETURNS text
    LANGUAGE sql IMMUTABLE
    AS $_$
SELECT format(
    'DROP VIEW IF EXISTS trigger_rule.%I',
    "trigger".kpi_view_name($1)
);
SELECT format(
    'CREATE VIEW trigger_rule.%I AS %s',
    "trigger".kpi_view_name($1), $2
);
$_$;


ALTER FUNCTION trigger.kpi_view_sql(trigger.rule, sql text) OWNER TO postgres;

--
-- Name: notification_data_fn_name(trigger.rule); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.notification_data_fn_name(trigger.rule) RETURNS name
    LANGUAGE sql IMMUTABLE
    AS $_$
SELECT ($1.name || '_notification_data')::name;
$_$;


ALTER FUNCTION trigger.notification_data_fn_name(trigger.rule) OWNER TO postgres;

--
-- Name: notification_fn_name(trigger.rule); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.notification_fn_name(trigger.rule) RETURNS name
    LANGUAGE sql IMMUTABLE
    AS $_$
SELECT ($1.name || '_create_notification')::name;
$_$;


ALTER FUNCTION trigger.notification_fn_name(trigger.rule) OWNER TO postgres;

--
-- Name: notification_fn_sql(trigger.rule); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.notification_fn_sql(trigger.rule) RETURNS text
    LANGUAGE sql STABLE
    AS $_$
SELECT format(
    'CREATE OR REPLACE FUNCTION trigger_rule.%I(timestamp with time zone)
    RETURNS SETOF trigger.notification
AS $fn$
SELECT
    n.entity_id,
    n.timestamp,
    COALESCE(exc.weight, trigger_rule.%I(n)) AS weight,
    trigger_rule.%I(n) AS details,
    trigger_rule.%I(n) AS data
FROM trigger_rule.%I($1) AS n
LEFT JOIN trigger_rule.%I AS exc ON
    exc.entity_id = n.entity_id AND
    exc.start <= n.timestamp AND
    exc.expires > n.timestamp $fn$ LANGUAGE sql STABLE',
    trigger.notification_fn_name($1),
    trigger.weight_fn_name($1),
    trigger.notification_message_fn_name($1),
    trigger.notification_data_fn_name($1),
    $1.name,
    trigger.exception_weight_table_name($1)
);
$_$;


ALTER FUNCTION trigger.notification_fn_sql(trigger.rule) OWNER TO postgres;

--
-- Name: notification_message_fn_name(trigger.rule); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.notification_message_fn_name(trigger.rule) RETURNS name
    LANGUAGE sql IMMUTABLE
    AS $_$
SELECT ($1.name || '_notification_message')::name;
$_$;


ALTER FUNCTION trigger.notification_message_fn_name(trigger.rule) OWNER TO postgres;

--
-- Name: notification_message_fn_sql(trigger.rule, text); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.notification_message_fn_sql(trigger.rule, expression text) RETURNS text
    LANGUAGE sql
    AS $_$
SELECT format(
'CREATE OR REPLACE FUNCTION trigger_rule.%I(trigger_rule.%I)
    RETURNS text
AS $function$
SELECT (%s)::text
$function$ LANGUAGE SQL IMMUTABLE',
    trigger.notification_message_fn_name($1),
    trigger.details_type_name($1),
    $2
);
$_$;


ALTER FUNCTION trigger.notification_message_fn_sql(trigger.rule, expression text) OWNER TO postgres;

--
-- Name: notification_test_threshold_fn_sql(trigger.rule); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.notification_test_threshold_fn_sql(trigger.rule) RETURNS text
    LANGUAGE sql IMMUTABLE
    AS $_$
SELECT format(
    'CREATE OR REPLACE FUNCTION trigger_rule.%I AS
SELECT
    n.entity_id,
    n.timestamp,
    trigger_rule.%I(n) AS weight,
    trigger_rule.%I(n) AS details
FROM trigger_rule.%I AS n',
    trigger.notification_threshold_test_fn_name($1),
    trigger.weight_fn_name($1),
    trigger.notification_fn_name($1),
    $1.name
);
$_$;


ALTER FUNCTION trigger.notification_test_threshold_fn_sql(trigger.rule) OWNER TO postgres;

--
-- Name: notification_threshold_test_fn_name(trigger.rule); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.notification_threshold_test_fn_name(trigger.rule) RETURNS name
    LANGUAGE sql IMMUTABLE
    AS $_$
SELECT ($1.name || '_notification_test_threshold')::name;
$_$;


ALTER FUNCTION trigger.notification_threshold_test_fn_name(trigger.rule) OWNER TO postgres;

--
-- Name: notification_type_name(trigger.rule); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.notification_type_name(trigger.rule) RETURNS name
    LANGUAGE sql IMMUTABLE
    AS $_$
SELECT ($1.name || '_notification_details')::name;
$_$;


ALTER FUNCTION trigger.notification_type_name(trigger.rule) OWNER TO postgres;

--
-- Name: notification_view_name(trigger.rule); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.notification_view_name(trigger.rule) RETURNS name
    LANGUAGE sql IMMUTABLE
    AS $_$
SELECT ($1.name || '_notification')::name;
$_$;


ALTER FUNCTION trigger.notification_view_name(trigger.rule) OWNER TO postgres;

--
-- Name: rule_fn_name(trigger.rule); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.rule_fn_name(trigger.rule) RETURNS name
    LANGUAGE sql IMMUTABLE
    AS $_$
SELECT $1.name;
$_$;


ALTER FUNCTION trigger.rule_fn_name(trigger.rule) OWNER TO postgres;

--
-- Name: rule_fn_sql(trigger.rule, text); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.rule_fn_sql(trigger.rule, where_clause text) RETURNS text
    LANGUAGE sql IMMUTABLE
    AS $_$
SELECT format(
    'SELECT * FROM trigger_rule.%I($1) WHERE %s;',
    trigger.with_threshold_fn_name($1), $2
);
$_$;


ALTER FUNCTION trigger.rule_fn_sql(trigger.rule, where_clause text) OWNER TO postgres;

--
-- Name: rule_view_name(trigger.rule); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.rule_view_name(trigger.rule) RETURNS name
    LANGUAGE sql IMMUTABLE
    AS $_$
SELECT $1.name;
$_$;


ALTER FUNCTION trigger.rule_view_name(trigger.rule) OWNER TO postgres;

--
-- Name: rule_view_sql(trigger.rule, text); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.rule_view_sql(trigger.rule, where_clause text) RETURNS text
    LANGUAGE sql IMMUTABLE
    AS $_$
SELECT format(
    'SELECT * FROM trigger_rule.%I WHERE %s;',
    trigger.with_threshold_view_name($1), $2
);
$_$;


ALTER FUNCTION trigger.rule_view_sql(trigger.rule, where_clause text) OWNER TO postgres;

--
-- Name: runnable_fn_name(trigger.rule); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.runnable_fn_name(trigger.rule) RETURNS name
    LANGUAGE sql IMMUTABLE
    AS $_$
SELECT ($1.name || '_runnable')::name;
$_$;


ALTER FUNCTION trigger.runnable_fn_name(trigger.rule) OWNER TO postgres;

--
-- Name: set_condition(trigger.rule, text); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.set_condition(trigger.rule, sql text) RETURNS trigger.rule
    LANGUAGE sql
    AS $_$
SELECT trigger.create_rule_fn($1, trigger.rule_fn_sql($1, $2));
$_$;


ALTER FUNCTION trigger.set_condition(trigger.rule, sql text) OWNER TO postgres;

--
-- Name: set_runnable(trigger.rule, text); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.set_runnable(trigger.rule, fn_sql text) RETURNS trigger.rule
    LANGUAGE sql
    AS $_$
SELECT public.action($1, trigger.create_runnable_fn_sql($1, $2));
$_$;


ALTER FUNCTION trigger.set_runnable(trigger.rule, fn_sql text) OWNER TO postgres;

--
-- Name: set_thresholds(name, text); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.set_thresholds(name, exprs text) RETURNS trigger.rule
    LANGUAGE sql
    AS $_$
SELECT trigger.set_thresholds(trigger.get_rule($1), $2);
$_$;


ALTER FUNCTION trigger.set_thresholds(name, exprs text) OWNER TO postgres;

--
-- Name: set_thresholds(trigger.rule, text); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.set_thresholds(trigger.rule, exprs text) RETURNS trigger.rule
    LANGUAGE sql SECURITY DEFINER
    AS $_$
SELECT trigger.action($1, format(
    'DROP VIEW IF EXISTS  trigger_rule.%I',
    trigger.threshold_view_name($1)
));
SELECT trigger.action($1, format(
    'CREATE VIEW trigger_rule.%I AS '
    'SELECT %s',
    trigger.threshold_view_name($1),
    $2
));
SELECT trigger.action($1, format(
    'ALTER VIEW trigger_rule.%I OWNER TO minerva_admin',
    trigger.threshold_view_name($1)
));

SELECT $1;
$_$;


ALTER FUNCTION trigger.set_thresholds(trigger.rule, exprs text) OWNER TO postgres;

--
-- Name: set_thresholds_fn_name(trigger.rule); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.set_thresholds_fn_name(trigger.rule) RETURNS name
    LANGUAGE sql IMMUTABLE
    AS $_$
SELECT ($1.name || '_set_thresholds')::name;
$_$;


ALTER FUNCTION trigger.set_thresholds_fn_name(trigger.rule) OWNER TO postgres;

--
-- Name: set_weight(name, text); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.set_weight(name, expression text) RETURNS trigger.rule
    LANGUAGE sql
    AS $_$
SELECT trigger.set_weight(trigger.get_rule($1), $2);
$_$;


ALTER FUNCTION trigger.set_weight(name, expression text) OWNER TO postgres;

--
-- Name: set_weight(trigger.rule, text); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.set_weight(trigger.rule, expression text) RETURNS trigger.rule
    LANGUAGE sql
    AS $_$
SELECT public.action(
    $1,
    ARRAY[
        trigger.weight_fn_sql($1, $2),
        format(
            'ALTER FUNCTION trigger_rule.%I(trigger_rule.%I) OWNER TO minerva_admin',
            trigger.weight_fn_name($1), trigger.details_type_name($1)
        )
    ]
);
$_$;


ALTER FUNCTION trigger.set_weight(trigger.rule, expression text) OWNER TO postgres;

--
-- Name: setup_rule(trigger.rule, trigger.threshold_def[]); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.setup_rule(trigger.rule, trigger.threshold_def[]) RETURNS trigger.rule
    LANGUAGE sql
    AS $_$
SELECT trigger.define_thresholds($1, $2);
SELECT trigger.create_exception_weight_table($1);
SELECT trigger.create_dummy_default_weight($1);
SELECT trigger.create_dummy_notification_message_fn($1);
SELECT trigger.create_dummy_notification_data_fn($1);
SELECT trigger.set_condition($1, 'true');
SELECT trigger.create_notification_fn($1);
SELECT trigger.create_fingerprint_fn($1);
SELECT trigger.create_runnable_fn($1);
$_$;


ALTER FUNCTION trigger.setup_rule(trigger.rule, trigger.threshold_def[]) OWNER TO postgres;

--
-- Name: table_exists(name, name); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.table_exists(schema_name name, table_name name) RETURNS boolean
    LANGUAGE sql STABLE
    AS $_$
SELECT exists(
    SELECT 1
    FROM pg_class
    JOIN pg_namespace ON pg_class.relnamespace = pg_namespace.oid
    WHERE relname = $2 AND relkind = 'r' AND pg_namespace.nspname = $1
);
$_$;


ALTER FUNCTION trigger.table_exists(schema_name name, table_name name) OWNER TO postgres;

--
-- Name: rule_tag_link; Type: TABLE; Schema: trigger; Owner: postgres
--

CREATE TABLE trigger.rule_tag_link (
    rule_id integer NOT NULL,
    tag_id integer NOT NULL
);


ALTER TABLE trigger.rule_tag_link OWNER TO postgres;

--
-- Name: tag(character varying, integer); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.tag(tag_name character varying, rule_id integer) RETURNS trigger.rule_tag_link
    LANGUAGE sql
    AS $_$
INSERT INTO trigger.rule_tag_link (rule_id, tag_id)
SELECT $2, tag.id FROM directory.tag WHERE name = $1
RETURNING *;
$_$;


ALTER FUNCTION trigger.tag(tag_name character varying, rule_id integer) OWNER TO postgres;

--
-- Name: FUNCTION tag(tag_name character varying, rule_id integer); Type: COMMENT; Schema: trigger; Owner: postgres
--

COMMENT ON FUNCTION trigger.tag(tag_name character varying, rule_id integer) IS 'Add tag with name tag_name to rule with id rule_id.

The tag must already exist.';


--
-- Name: threshold_view_name(trigger.rule); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.threshold_view_name(trigger.rule) RETURNS name
    LANGUAGE sql IMMUTABLE
    AS $_$
SELECT ($1.name || '_threshold')::name;
$_$;


ALTER FUNCTION trigger.threshold_view_name(trigger.rule) OWNER TO postgres;

--
-- Name: timestamps(trigger.rule); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.timestamps(trigger.rule) RETURNS SETOF timestamp with time zone
    LANGUAGE sql STABLE
    AS $_$
SELECT generate_series(
    trigger.truncate(now(), $1.granularity),
    trigger.truncate(now(), $1.granularity) - $1.default_interval,
    - $1.granularity
);
$_$;


ALTER FUNCTION trigger.timestamps(trigger.rule) OWNER TO postgres;

--
-- Name: transfer_notifications_from_staging(notification_directory.notification_store); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.transfer_notifications_from_staging(notification_directory.notification_store) RETURNS integer
    LANGUAGE plpgsql
    AS $_$
DECLARE
    num_rows integer;
BEGIN
    EXECUTE format(
$query$
INSERT INTO notification.%I(entity_id, timestamp, created, rule_id, weight, details, data)
SELECT staging.entity_id, staging.timestamp, staging.created, staging.rule_id, staging.weight, staging.details, staging.data
FROM notification.%I staging
LEFT JOIN notification.%I target ON target.entity_id = staging.entity_id AND target.timestamp = staging.timestamp AND target.rule_id = staging.rule_id
WHERE target.entity_id IS NULL;
$query$,
        notification_directory.table_name($1), notification_directory.staging_table_name($1), notification_directory.table_name($1));

    GET DIAGNOSTICS num_rows = ROW_COUNT;

    EXECUTE format('DELETE FROM notification.%I', notification_directory.staging_table_name($1));

    RETURN num_rows;
END;
$_$;


ALTER FUNCTION trigger.transfer_notifications_from_staging(notification_directory.notification_store) OWNER TO postgres;

--
-- Name: truncate(timestamp with time zone, interval); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.truncate(timestamp with time zone, interval) RETURNS timestamp with time zone
    LANGUAGE sql STABLE
    AS $_$
SELECT CASE
    WHEN $2 = '1 day' THEN
        date_trunc('day', $1)
    WHEN $2 = '1 week' THEN
        date_trunc('week', $1)
    ELSE
        to_timestamp((
            extract(epoch FROM $1)::integer / extract(epoch FROM $2)::integer
        )::integer * extract(epoch FROM $2))
    END;
$_$;


ALTER FUNCTION trigger.truncate(timestamp with time zone, interval) OWNER TO postgres;

--
-- Name: view_exists(name, name); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.view_exists(schema_name name, table_name name) RETURNS boolean
    LANGUAGE sql STABLE
    AS $_$
SELECT exists(
    SELECT 1
    FROM pg_class
    JOIN pg_namespace ON pg_class.relnamespace = pg_namespace.oid
    WHERE relname = $2 AND relkind = 'v' AND pg_namespace.nspname = $1
);
$_$;


ALTER FUNCTION trigger.view_exists(schema_name name, table_name name) OWNER TO postgres;

--
-- Name: weight_fn_name(trigger.rule); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.weight_fn_name(trigger.rule) RETURNS name
    LANGUAGE sql IMMUTABLE
    AS $_$
SELECT ($1.name || '_weight')::name;
$_$;


ALTER FUNCTION trigger.weight_fn_name(trigger.rule) OWNER TO postgres;

--
-- Name: weight_fn_sql(trigger.rule, text); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.weight_fn_sql(trigger.rule, expression text) RETURNS text
    LANGUAGE sql IMMUTABLE
    AS $_$
SELECT format(
  $function$
  CREATE OR REPLACE FUNCTION trigger_rule.%I(trigger_rule.%I)
      RETURNS integer AS
  $weight_fn$SELECT (%s)$weight_fn$ LANGUAGE SQL IMMUTABLE;
  $function$,
  trigger.weight_fn_name($1),
  trigger.details_type_name($1),
  $2
);
$_$;


ALTER FUNCTION trigger.weight_fn_sql(trigger.rule, expression text) OWNER TO postgres;

--
-- Name: with_threshold_fn_name(trigger.rule); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.with_threshold_fn_name(trigger.rule) RETURNS name
    LANGUAGE sql IMMUTABLE
    AS $_$
SELECT ($1.name || '_with_threshold')::name;
$_$;


ALTER FUNCTION trigger.with_threshold_fn_name(trigger.rule) OWNER TO postgres;

--
-- Name: with_threshold_fn_sql(trigger.rule); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.with_threshold_fn_sql(trigger.rule) RETURNS text
    LANGUAGE sql STABLE
    AS $_$
--SELECT CASE WHEN trigger.has_thresholds($1) THEN
SELECT CASE WHEN true THEN
    trigger.with_threshold_fn_sql_normal($1)
ELSE
    trigger.with_threshold_fn_sql_no_thresholds($1)
END;
$_$;


ALTER FUNCTION trigger.with_threshold_fn_sql(trigger.rule) OWNER TO postgres;

--
-- Name: with_threshold_fn_sql_no_thresholds(trigger.rule); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.with_threshold_fn_sql_no_thresholds(trigger.rule) RETURNS text
    LANGUAGE sql STABLE
    AS $_$
SELECT format(
    'SELECT * FROM trigger_rule.%I($1)',
    trigger.kpi_fn_name($1)
);
$_$;


ALTER FUNCTION trigger.with_threshold_fn_sql_no_thresholds(trigger.rule) OWNER TO postgres;

--
-- Name: with_threshold_fn_sql_normal(trigger.rule); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.with_threshold_fn_sql_normal(trigger.rule) RETURNS text
    LANGUAGE sql STABLE
    AS $_$
SELECT format(
$view$
SELECT %s
FROM trigger_rule.%I AS threshold, trigger_rule.%I($1) AS kpi
LEFT JOIN trigger_rule.%I exc ON
    exc.entity_id = kpi.entity_id AND
    exc.start <= timestamp AND
    exc.expires > timestamp
$view$,
    array_to_string(col_def, ','),
    trigger.threshold_view_name($1),
    trigger.kpi_fn_name($1),
    trigger.exception_threshold_table_name($1)
)
FROM (
    SELECT
        ARRAY['kpi.*']::text[] || array_agg(format('COALESCE(exc.%I, threshold.%I) AS %I', threshold.name, threshold.name, threshold.name)) AS col_def
    FROM trigger.get_threshold_defs($1) threshold
) c;
$_$;


ALTER FUNCTION trigger.with_threshold_fn_sql_normal(trigger.rule) OWNER TO postgres;

--
-- Name: with_threshold_view_name(trigger.rule); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.with_threshold_view_name(trigger.rule) RETURNS name
    LANGUAGE sql IMMUTABLE
    AS $_$
SELECT ($1.name || '_with_threshold')::name;
$_$;


ALTER FUNCTION trigger.with_threshold_view_name(trigger.rule) OWNER TO postgres;

--
-- Name: with_threshold_view_sql(trigger.rule); Type: FUNCTION; Schema: trigger; Owner: postgres
--

CREATE FUNCTION trigger.with_threshold_view_sql(trigger.rule) RETURNS text
    LANGUAGE sql STABLE
    AS $_$
SELECT format(
$view$
SELECT
    kpi.*,
    %s
FROM trigger_rule.%I AS threshold, trigger_rule.%I AS kpi
LEFT JOIN trigger_rule.%I exc ON
    exc.entity_id = kpi.entity_id AND
    exc.start <= timestamp AND
    exc.expires > timestamp
$view$,
    array_to_string(array_agg(format('COALESCE(exc.%I, threshold.%I) AS %I', kpi.name, kpi.name, 'threshold_' || kpi.name)), ', '),
    trigger.threshold_view_name($1),
    trigger.kpi_view_name($1),
    trigger.exception_threshold_table_name($1)
)
FROM trigger.get_threshold_defs($1) kpi;
$_$;


ALTER FUNCTION trigger.with_threshold_view_sql(trigger.rule) OWNER TO postgres;

--
-- Name: node/15m/highpowerusage(timestamp with time zone); Type: FUNCTION; Schema: trigger_rule; Owner: minerva_admin
--

CREATE FUNCTION trigger_rule."node/15m/highpowerusage"(timestamp with time zone) RETURNS SETOF trigger_rule."node/15m/highpowerusage_details"
    LANGUAGE sql STABLE
    AS $_$ SELECT * FROM trigger_rule."node/15m/highpowerusage_with_threshold"($1) WHERE power_kwh > max_power; $_$;


ALTER FUNCTION trigger_rule."node/15m/highpowerusage"(timestamp with time zone) OWNER TO minerva_admin;

--
-- Name: node/15m/highpowerusage_add_or_change_threshold_exception(integer, timestamp with time zone, timestamp with time zone, numeric); Type: FUNCTION; Schema: trigger_rule; Owner: postgres
--

CREATE FUNCTION trigger_rule."node/15m/highpowerusage_add_or_change_threshold_exception"(entity integer, new_start timestamp with time zone, new_expires timestamp with time zone, max_power_new numeric) RETURNS void
    LANGUAGE sql
    AS $$SELECT trigger_rule."node/15m/highpowerusage_get_or_create_exception_threshold"(entity); UPDATE trigger_rule."node/15m/highpowerusage_exception_threshold" SET (start, expires, max_power) = (new_start, new_expires, max_power_new) WHERE entity_id = entity;$$;


ALTER FUNCTION trigger_rule."node/15m/highpowerusage_add_or_change_threshold_exception"(entity integer, new_start timestamp with time zone, new_expires timestamp with time zone, max_power_new numeric) OWNER TO postgres;

--
-- Name: node/15m/highpowerusage_exception_threshold; Type: TABLE; Schema: trigger_rule; Owner: minerva_admin
--

CREATE TABLE trigger_rule."node/15m/highpowerusage_exception_threshold" (
    id integer NOT NULL,
    entity_id integer,
    created timestamp with time zone DEFAULT now(),
    start timestamp with time zone,
    expires timestamp with time zone,
    remark text,
    max_power numeric
);


ALTER TABLE trigger_rule."node/15m/highpowerusage_exception_threshold" OWNER TO minerva_admin;

--
-- Name: node/15m/highpowerusage_create_exception_threshold(integer); Type: FUNCTION; Schema: trigger_rule; Owner: postgres
--

CREATE FUNCTION trigger_rule."node/15m/highpowerusage_create_exception_threshold"(entity integer) RETURNS trigger_rule."node/15m/highpowerusage_exception_threshold"
    LANGUAGE sql
    AS $_$INSERT INTO trigger_rule."node/15m/highpowerusage_exception_threshold"(entity_id) VALUES ($1) RETURNING *;$_$;


ALTER FUNCTION trigger_rule."node/15m/highpowerusage_create_exception_threshold"(entity integer) OWNER TO postgres;

--
-- Name: node/15m/highpowerusage_create_notification(timestamp with time zone); Type: FUNCTION; Schema: trigger_rule; Owner: minerva_admin
--

CREATE FUNCTION trigger_rule."node/15m/highpowerusage_create_notification"(timestamp with time zone) RETURNS SETOF trigger.notification
    LANGUAGE sql STABLE
    AS $_$
SELECT
    n.entity_id,
    n.timestamp,
    COALESCE(exc.weight, trigger_rule."node/15m/highpowerusage_weight"(n)) AS weight,
    trigger_rule."node/15m/highpowerusage_notification_message"(n) AS details,
    trigger_rule."node/15m/highpowerusage_notification_data"(n) AS data
FROM trigger_rule."node/15m/highpowerusage"($1) AS n
LEFT JOIN trigger_rule."node/15m/highpowerusage_exception_weight" AS exc ON
    exc.entity_id = n.entity_id AND
    exc.start <= n.timestamp AND
    exc.expires > n.timestamp $_$;


ALTER FUNCTION trigger_rule."node/15m/highpowerusage_create_notification"(timestamp with time zone) OWNER TO minerva_admin;

--
-- Name: node/15m/highpowerusage_fingerprint(timestamp with time zone); Type: FUNCTION; Schema: trigger_rule; Owner: postgres
--

CREATE FUNCTION trigger_rule."node/15m/highpowerusage_fingerprint"(timestamp with time zone) RETURNS text
    LANGUAGE sql STABLE
    AS $$
      SELECT now()::text;
    $$;


ALTER FUNCTION trigger_rule."node/15m/highpowerusage_fingerprint"(timestamp with time zone) OWNER TO postgres;

--
-- Name: node/15m/highpowerusage_get_exception_threshold(integer); Type: FUNCTION; Schema: trigger_rule; Owner: postgres
--

CREATE FUNCTION trigger_rule."node/15m/highpowerusage_get_exception_threshold"(entity integer) RETURNS trigger_rule."node/15m/highpowerusage_exception_threshold"
    LANGUAGE sql
    AS $$SELECT * FROM trigger_rule."node/15m/highpowerusage_exception_threshold" WHERE entity_id = entity;$$;


ALTER FUNCTION trigger_rule."node/15m/highpowerusage_get_exception_threshold"(entity integer) OWNER TO postgres;

--
-- Name: node/15m/highpowerusage_get_or_create_exception_threshold(integer); Type: FUNCTION; Schema: trigger_rule; Owner: postgres
--

CREATE FUNCTION trigger_rule."node/15m/highpowerusage_get_or_create_exception_threshold"(entity integer) RETURNS trigger_rule."node/15m/highpowerusage_exception_threshold"
    LANGUAGE sql
    AS $_$SELECT COALESCE(trigger_rule."node/15m/highpowerusage_get_exception_threshold"($1), trigger_rule."node/15m/highpowerusage_create_exception_threshold"($1));$_$;


ALTER FUNCTION trigger_rule."node/15m/highpowerusage_get_or_create_exception_threshold"(entity integer) OWNER TO postgres;

--
-- Name: node/15m/highpowerusage_kpi(timestamp with time zone); Type: FUNCTION; Schema: trigger_rule; Owner: postgres
--

CREATE FUNCTION trigger_rule."node/15m/highpowerusage_kpi"(timestamp with time zone) RETURNS SETOF trigger_rule."node/15m/highpowerusage_kpi"
    LANGUAGE plpgsql STABLE
    AS $_$BEGIN
    RETURN QUERY EXECUTE $query$
    SELECT
        t.entity_id,
        t.timestamp,
        t.power_kwh
    FROM trend."hub_node_main_15m" AS t
    WHERE
        t.timestamp = $1
    $query$ USING $1;
END; $_$;


ALTER FUNCTION trigger_rule."node/15m/highpowerusage_kpi"(timestamp with time zone) OWNER TO postgres;

--
-- Name: node/15m/highpowerusage_notification_data(trigger_rule."node/15m/highpowerusage_details"); Type: FUNCTION; Schema: trigger_rule; Owner: minerva_admin
--

CREATE FUNCTION trigger_rule."node/15m/highpowerusage_notification_data"(trigger_rule."node/15m/highpowerusage_details") RETURNS json
    LANGUAGE plpgsql STABLE
    AS $_$
DECLARE
  data json;
BEGIN
SELECT (SELECT json_build_object(
  'power_kwh', $1.power_kwh
)) INTO data;
RETURN data;
END;
$_$;


ALTER FUNCTION trigger_rule."node/15m/highpowerusage_notification_data"(trigger_rule."node/15m/highpowerusage_details") OWNER TO minerva_admin;

--
-- Name: node/15m/highpowerusage_notification_message(trigger_rule."node/15m/highpowerusage_details"); Type: FUNCTION; Schema: trigger_rule; Owner: minerva_admin
--

CREATE FUNCTION trigger_rule."node/15m/highpowerusage_notification_message"(trigger_rule."node/15m/highpowerusage_details") RETURNS text
    LANGUAGE sql IMMUTABLE
    AS $_$
SELECT (SELECT array_to_string(
    ARRAY[
        'HighPowerUsage',
        format('%s > %s', $1.power_kwh, $1.max_power)
    ],
    E'\n'
))::text
$_$;


ALTER FUNCTION trigger_rule."node/15m/highpowerusage_notification_message"(trigger_rule."node/15m/highpowerusage_details") OWNER TO minerva_admin;

--
-- Name: node/15m/highpowerusage_runnable(timestamp with time zone); Type: FUNCTION; Schema: trigger_rule; Owner: postgres
--

CREATE FUNCTION trigger_rule."node/15m/highpowerusage_runnable"(timestamp with time zone) RETURNS boolean
    LANGUAGE sql STABLE
    AS $$
    SELECT TRUE;
    $$;


ALTER FUNCTION trigger_rule."node/15m/highpowerusage_runnable"(timestamp with time zone) OWNER TO postgres;

--
-- Name: node/15m/highpowerusage_set_thresholds(numeric); Type: FUNCTION; Schema: trigger_rule; Owner: postgres
--

CREATE FUNCTION trigger_rule."node/15m/highpowerusage_set_thresholds"(max_power numeric) RETURNS integer
    LANGUAGE plpgsql SECURITY DEFINER
    AS $_$
    BEGIN
        EXECUTE format('CREATE OR REPLACE VIEW trigger_rule."node/15m/highpowerusage_threshold" AS SELECT %L::numeric AS max_power', $1);
        RETURN 42;
    END;
    $_$;


ALTER FUNCTION trigger_rule."node/15m/highpowerusage_set_thresholds"(max_power numeric) OWNER TO postgres;

--
-- Name: node/15m/highpowerusage_weight(trigger_rule."node/15m/highpowerusage_details"); Type: FUNCTION; Schema: trigger_rule; Owner: minerva_admin
--

CREATE FUNCTION trigger_rule."node/15m/highpowerusage_weight"(trigger_rule."node/15m/highpowerusage_details") RETURNS integer
    LANGUAGE sql IMMUTABLE
    AS $_$SELECT (SELECT
    CASE
        WHEN $1.power_kwh > 1 THEN 500
        WHEN $1.power_kwh > 2 THEN 800
        ELSE 300
    END)$_$;


ALTER FUNCTION trigger_rule."node/15m/highpowerusage_weight"(trigger_rule."node/15m/highpowerusage_details") OWNER TO minerva_admin;

--
-- Name: node/15m/highpowerusage_with_threshold(timestamp with time zone); Type: FUNCTION; Schema: trigger_rule; Owner: minerva_admin
--

CREATE FUNCTION trigger_rule."node/15m/highpowerusage_with_threshold"(timestamp with time zone) RETURNS SETOF trigger_rule."node/15m/highpowerusage_details"
    LANGUAGE sql STABLE
    AS $_$
SELECT kpi.*,COALESCE(exc.max_power, threshold.max_power) AS max_power
FROM trigger_rule."node/15m/highpowerusage_threshold" AS threshold, trigger_rule."node/15m/highpowerusage_kpi"($1) AS kpi
LEFT JOIN trigger_rule."node/15m/highpowerusage_exception_threshold" exc ON
    exc.entity_id = kpi.entity_id AND
    exc.start <= timestamp AND
    exc.expires > timestamp
$_$;


ALTER FUNCTION trigger_rule."node/15m/highpowerusage_with_threshold"(timestamp with time zone) OWNER TO minerva_admin;

--
-- Name: node/1d/highpowerusage(timestamp with time zone); Type: FUNCTION; Schema: trigger_rule; Owner: minerva_admin
--

CREATE FUNCTION trigger_rule."node/1d/highpowerusage"(timestamp with time zone) RETURNS SETOF trigger_rule."node/1d/highpowerusage_details"
    LANGUAGE sql STABLE
    AS $_$ SELECT * FROM trigger_rule."node/1d/highpowerusage_with_threshold"($1) WHERE power_kwh > max_power; $_$;


ALTER FUNCTION trigger_rule."node/1d/highpowerusage"(timestamp with time zone) OWNER TO minerva_admin;

--
-- Name: node/1d/highpowerusage_add_or_change_threshold_exception(integer, timestamp with time zone, timestamp with time zone, numeric); Type: FUNCTION; Schema: trigger_rule; Owner: postgres
--

CREATE FUNCTION trigger_rule."node/1d/highpowerusage_add_or_change_threshold_exception"(entity integer, new_start timestamp with time zone, new_expires timestamp with time zone, max_power_new numeric) RETURNS void
    LANGUAGE sql
    AS $$SELECT trigger_rule."node/1d/highpowerusage_get_or_create_exception_threshold"(entity); UPDATE trigger_rule."node/1d/highpowerusage_exception_threshold" SET (start, expires, max_power) = (new_start, new_expires, max_power_new) WHERE entity_id = entity;$$;


ALTER FUNCTION trigger_rule."node/1d/highpowerusage_add_or_change_threshold_exception"(entity integer, new_start timestamp with time zone, new_expires timestamp with time zone, max_power_new numeric) OWNER TO postgres;

--
-- Name: node/1d/highpowerusage_exception_threshold; Type: TABLE; Schema: trigger_rule; Owner: minerva_admin
--

CREATE TABLE trigger_rule."node/1d/highpowerusage_exception_threshold" (
    id integer NOT NULL,
    entity_id integer,
    created timestamp with time zone DEFAULT now(),
    start timestamp with time zone,
    expires timestamp with time zone,
    remark text,
    max_power numeric
);


ALTER TABLE trigger_rule."node/1d/highpowerusage_exception_threshold" OWNER TO minerva_admin;

--
-- Name: node/1d/highpowerusage_create_exception_threshold(integer); Type: FUNCTION; Schema: trigger_rule; Owner: postgres
--

CREATE FUNCTION trigger_rule."node/1d/highpowerusage_create_exception_threshold"(entity integer) RETURNS trigger_rule."node/1d/highpowerusage_exception_threshold"
    LANGUAGE sql
    AS $_$INSERT INTO trigger_rule."node/1d/highpowerusage_exception_threshold"(entity_id) VALUES ($1) RETURNING *;$_$;


ALTER FUNCTION trigger_rule."node/1d/highpowerusage_create_exception_threshold"(entity integer) OWNER TO postgres;

--
-- Name: node/1d/highpowerusage_create_notification(timestamp with time zone); Type: FUNCTION; Schema: trigger_rule; Owner: minerva_admin
--

CREATE FUNCTION trigger_rule."node/1d/highpowerusage_create_notification"(timestamp with time zone) RETURNS SETOF trigger.notification
    LANGUAGE sql STABLE
    AS $_$
SELECT
    n.entity_id,
    n.timestamp,
    COALESCE(exc.weight, trigger_rule."node/1d/highpowerusage_weight"(n)) AS weight,
    trigger_rule."node/1d/highpowerusage_notification_message"(n) AS details,
    trigger_rule."node/1d/highpowerusage_notification_data"(n) AS data
FROM trigger_rule."node/1d/highpowerusage"($1) AS n
LEFT JOIN trigger_rule."node/1d/highpowerusage_exception_weight" AS exc ON
    exc.entity_id = n.entity_id AND
    exc.start <= n.timestamp AND
    exc.expires > n.timestamp $_$;


ALTER FUNCTION trigger_rule."node/1d/highpowerusage_create_notification"(timestamp with time zone) OWNER TO minerva_admin;

--
-- Name: node/1d/highpowerusage_fingerprint(timestamp with time zone); Type: FUNCTION; Schema: trigger_rule; Owner: postgres
--

CREATE FUNCTION trigger_rule."node/1d/highpowerusage_fingerprint"(timestamp with time zone) RETURNS text
    LANGUAGE sql STABLE
    AS $$
      SELECT now()::text;
    $$;


ALTER FUNCTION trigger_rule."node/1d/highpowerusage_fingerprint"(timestamp with time zone) OWNER TO postgres;

--
-- Name: node/1d/highpowerusage_get_exception_threshold(integer); Type: FUNCTION; Schema: trigger_rule; Owner: postgres
--

CREATE FUNCTION trigger_rule."node/1d/highpowerusage_get_exception_threshold"(entity integer) RETURNS trigger_rule."node/1d/highpowerusage_exception_threshold"
    LANGUAGE sql
    AS $$SELECT * FROM trigger_rule."node/1d/highpowerusage_exception_threshold" WHERE entity_id = entity;$$;


ALTER FUNCTION trigger_rule."node/1d/highpowerusage_get_exception_threshold"(entity integer) OWNER TO postgres;

--
-- Name: node/1d/highpowerusage_get_or_create_exception_threshold(integer); Type: FUNCTION; Schema: trigger_rule; Owner: postgres
--

CREATE FUNCTION trigger_rule."node/1d/highpowerusage_get_or_create_exception_threshold"(entity integer) RETURNS trigger_rule."node/1d/highpowerusage_exception_threshold"
    LANGUAGE sql
    AS $_$SELECT COALESCE(trigger_rule."node/1d/highpowerusage_get_exception_threshold"($1), trigger_rule."node/1d/highpowerusage_create_exception_threshold"($1));$_$;


ALTER FUNCTION trigger_rule."node/1d/highpowerusage_get_or_create_exception_threshold"(entity integer) OWNER TO postgres;

--
-- Name: node/1d/highpowerusage_kpi(timestamp with time zone); Type: FUNCTION; Schema: trigger_rule; Owner: postgres
--

CREATE FUNCTION trigger_rule."node/1d/highpowerusage_kpi"(timestamp with time zone) RETURNS SETOF trigger_rule."node/1d/highpowerusage_kpi"
    LANGUAGE plpgsql STABLE
    AS $_$BEGIN
    RETURN QUERY EXECUTE $query$
    SELECT
        t.entity_id,
        t.timestamp,
        t.power_kwh
    FROM trend."hub_node_main_15m" AS t
    WHERE
        t.timestamp = $1
    $query$ USING $1;
END;$_$;


ALTER FUNCTION trigger_rule."node/1d/highpowerusage_kpi"(timestamp with time zone) OWNER TO postgres;

--
-- Name: node/1d/highpowerusage_notification_data(trigger_rule."node/1d/highpowerusage_details"); Type: FUNCTION; Schema: trigger_rule; Owner: minerva_admin
--

CREATE FUNCTION trigger_rule."node/1d/highpowerusage_notification_data"(trigger_rule."node/1d/highpowerusage_details") RETURNS json
    LANGUAGE plpgsql STABLE
    AS $_$
DECLARE
  data json;
BEGIN
SELECT (SELECT json_build_object(
  'power_kwh', $1.power_kwh
)) INTO data;
RETURN data;
END;
$_$;


ALTER FUNCTION trigger_rule."node/1d/highpowerusage_notification_data"(trigger_rule."node/1d/highpowerusage_details") OWNER TO minerva_admin;

--
-- Name: node/1d/highpowerusage_notification_message(trigger_rule."node/1d/highpowerusage_details"); Type: FUNCTION; Schema: trigger_rule; Owner: minerva_admin
--

CREATE FUNCTION trigger_rule."node/1d/highpowerusage_notification_message"(trigger_rule."node/1d/highpowerusage_details") RETURNS text
    LANGUAGE sql IMMUTABLE
    AS $_$
SELECT (SELECT array_to_string(
    ARRAY[
        'HighPowerUsage',
        format('%s > %s', $1.power_kwh, $1.max_power)
    ],
    E'\n'
))::text
$_$;


ALTER FUNCTION trigger_rule."node/1d/highpowerusage_notification_message"(trigger_rule."node/1d/highpowerusage_details") OWNER TO minerva_admin;

--
-- Name: node/1d/highpowerusage_runnable(timestamp with time zone); Type: FUNCTION; Schema: trigger_rule; Owner: postgres
--

CREATE FUNCTION trigger_rule."node/1d/highpowerusage_runnable"(timestamp with time zone) RETURNS boolean
    LANGUAGE sql STABLE
    AS $$
    SELECT TRUE;
    $$;


ALTER FUNCTION trigger_rule."node/1d/highpowerusage_runnable"(timestamp with time zone) OWNER TO postgres;

--
-- Name: node/1d/highpowerusage_set_thresholds(numeric); Type: FUNCTION; Schema: trigger_rule; Owner: postgres
--

CREATE FUNCTION trigger_rule."node/1d/highpowerusage_set_thresholds"(max_power numeric) RETURNS integer
    LANGUAGE plpgsql SECURITY DEFINER
    AS $_$
    BEGIN
        EXECUTE format('CREATE OR REPLACE VIEW trigger_rule."node/1d/highpowerusage_threshold" AS SELECT %L::numeric AS max_power', $1);
        RETURN 42;
    END;
    $_$;


ALTER FUNCTION trigger_rule."node/1d/highpowerusage_set_thresholds"(max_power numeric) OWNER TO postgres;

--
-- Name: node/1d/highpowerusage_weight(trigger_rule."node/1d/highpowerusage_details"); Type: FUNCTION; Schema: trigger_rule; Owner: minerva_admin
--

CREATE FUNCTION trigger_rule."node/1d/highpowerusage_weight"(trigger_rule."node/1d/highpowerusage_details") RETURNS integer
    LANGUAGE sql IMMUTABLE
    AS $_$SELECT (SELECT
    CASE
        WHEN $1.power_kwh > 1 THEN 500
        WHEN $1.power_kwh > 2 THEN 800
        ELSE 300
    END)$_$;


ALTER FUNCTION trigger_rule."node/1d/highpowerusage_weight"(trigger_rule."node/1d/highpowerusage_details") OWNER TO minerva_admin;

--
-- Name: node/1d/highpowerusage_with_threshold(timestamp with time zone); Type: FUNCTION; Schema: trigger_rule; Owner: minerva_admin
--

CREATE FUNCTION trigger_rule."node/1d/highpowerusage_with_threshold"(timestamp with time zone) RETURNS SETOF trigger_rule."node/1d/highpowerusage_details"
    LANGUAGE sql STABLE
    AS $_$
SELECT kpi.*,COALESCE(exc.max_power, threshold.max_power) AS max_power
FROM trigger_rule."node/1d/highpowerusage_threshold" AS threshold, trigger_rule."node/1d/highpowerusage_kpi"($1) AS kpi
LEFT JOIN trigger_rule."node/1d/highpowerusage_exception_threshold" exc ON
    exc.entity_id = kpi.entity_id AND
    exc.start <= timestamp AND
    exc.expires > timestamp
$_$;


ALTER FUNCTION trigger_rule."node/1d/highpowerusage_with_threshold"(timestamp with time zone) OWNER TO minerva_admin;

--
-- Name: node/1h/highpowerusage(timestamp with time zone); Type: FUNCTION; Schema: trigger_rule; Owner: minerva_admin
--

CREATE FUNCTION trigger_rule."node/1h/highpowerusage"(timestamp with time zone) RETURNS SETOF trigger_rule."node/1h/highpowerusage_details"
    LANGUAGE sql STABLE
    AS $_$ SELECT * FROM trigger_rule."node/1h/highpowerusage_with_threshold"($1) WHERE power_kwh > max_power; $_$;


ALTER FUNCTION trigger_rule."node/1h/highpowerusage"(timestamp with time zone) OWNER TO minerva_admin;

--
-- Name: node/1h/highpowerusage_add_or_change_threshold_exception(integer, timestamp with time zone, timestamp with time zone, numeric); Type: FUNCTION; Schema: trigger_rule; Owner: postgres
--

CREATE FUNCTION trigger_rule."node/1h/highpowerusage_add_or_change_threshold_exception"(entity integer, new_start timestamp with time zone, new_expires timestamp with time zone, max_power_new numeric) RETURNS void
    LANGUAGE sql
    AS $$SELECT trigger_rule."node/1h/highpowerusage_get_or_create_exception_threshold"(entity); UPDATE trigger_rule."node/1h/highpowerusage_exception_threshold" SET (start, expires, max_power) = (new_start, new_expires, max_power_new) WHERE entity_id = entity;$$;


ALTER FUNCTION trigger_rule."node/1h/highpowerusage_add_or_change_threshold_exception"(entity integer, new_start timestamp with time zone, new_expires timestamp with time zone, max_power_new numeric) OWNER TO postgres;

--
-- Name: node/1h/highpowerusage_exception_threshold; Type: TABLE; Schema: trigger_rule; Owner: minerva_admin
--

CREATE TABLE trigger_rule."node/1h/highpowerusage_exception_threshold" (
    id integer NOT NULL,
    entity_id integer,
    created timestamp with time zone DEFAULT now(),
    start timestamp with time zone,
    expires timestamp with time zone,
    remark text,
    max_power numeric
);


ALTER TABLE trigger_rule."node/1h/highpowerusage_exception_threshold" OWNER TO minerva_admin;

--
-- Name: node/1h/highpowerusage_create_exception_threshold(integer); Type: FUNCTION; Schema: trigger_rule; Owner: postgres
--

CREATE FUNCTION trigger_rule."node/1h/highpowerusage_create_exception_threshold"(entity integer) RETURNS trigger_rule."node/1h/highpowerusage_exception_threshold"
    LANGUAGE sql
    AS $_$INSERT INTO trigger_rule."node/1h/highpowerusage_exception_threshold"(entity_id) VALUES ($1) RETURNING *;$_$;


ALTER FUNCTION trigger_rule."node/1h/highpowerusage_create_exception_threshold"(entity integer) OWNER TO postgres;

--
-- Name: node/1h/highpowerusage_create_notification(timestamp with time zone); Type: FUNCTION; Schema: trigger_rule; Owner: minerva_admin
--

CREATE FUNCTION trigger_rule."node/1h/highpowerusage_create_notification"(timestamp with time zone) RETURNS SETOF trigger.notification
    LANGUAGE sql STABLE
    AS $_$
SELECT
    n.entity_id,
    n.timestamp,
    COALESCE(exc.weight, trigger_rule."node/1h/highpowerusage_weight"(n)) AS weight,
    trigger_rule."node/1h/highpowerusage_notification_message"(n) AS details,
    trigger_rule."node/1h/highpowerusage_notification_data"(n) AS data
FROM trigger_rule."node/1h/highpowerusage"($1) AS n
LEFT JOIN trigger_rule."node/1h/highpowerusage_exception_weight" AS exc ON
    exc.entity_id = n.entity_id AND
    exc.start <= n.timestamp AND
    exc.expires > n.timestamp $_$;


ALTER FUNCTION trigger_rule."node/1h/highpowerusage_create_notification"(timestamp with time zone) OWNER TO minerva_admin;

--
-- Name: node/1h/highpowerusage_fingerprint(timestamp with time zone); Type: FUNCTION; Schema: trigger_rule; Owner: postgres
--

CREATE FUNCTION trigger_rule."node/1h/highpowerusage_fingerprint"(timestamp with time zone) RETURNS text
    LANGUAGE sql STABLE
    AS $$
      SELECT now()::text;
    $$;


ALTER FUNCTION trigger_rule."node/1h/highpowerusage_fingerprint"(timestamp with time zone) OWNER TO postgres;

--
-- Name: node/1h/highpowerusage_get_exception_threshold(integer); Type: FUNCTION; Schema: trigger_rule; Owner: postgres
--

CREATE FUNCTION trigger_rule."node/1h/highpowerusage_get_exception_threshold"(entity integer) RETURNS trigger_rule."node/1h/highpowerusage_exception_threshold"
    LANGUAGE sql
    AS $$SELECT * FROM trigger_rule."node/1h/highpowerusage_exception_threshold" WHERE entity_id = entity;$$;


ALTER FUNCTION trigger_rule."node/1h/highpowerusage_get_exception_threshold"(entity integer) OWNER TO postgres;

--
-- Name: node/1h/highpowerusage_get_or_create_exception_threshold(integer); Type: FUNCTION; Schema: trigger_rule; Owner: postgres
--

CREATE FUNCTION trigger_rule."node/1h/highpowerusage_get_or_create_exception_threshold"(entity integer) RETURNS trigger_rule."node/1h/highpowerusage_exception_threshold"
    LANGUAGE sql
    AS $_$SELECT COALESCE(trigger_rule."node/1h/highpowerusage_get_exception_threshold"($1), trigger_rule."node/1h/highpowerusage_create_exception_threshold"($1));$_$;


ALTER FUNCTION trigger_rule."node/1h/highpowerusage_get_or_create_exception_threshold"(entity integer) OWNER TO postgres;

--
-- Name: node/1h/highpowerusage_kpi(timestamp with time zone); Type: FUNCTION; Schema: trigger_rule; Owner: postgres
--

CREATE FUNCTION trigger_rule."node/1h/highpowerusage_kpi"(timestamp with time zone) RETURNS SETOF trigger_rule."node/1h/highpowerusage_kpi"
    LANGUAGE plpgsql STABLE
    AS $_$BEGIN
    RETURN QUERY EXECUTE $query$
    SELECT
        t.entity_id,
        t.timestamp,
        t.power_kwh
    FROM trend."hub_node_main_15m" AS t
    WHERE
        t.timestamp = $1
    $query$ USING $1;
END;$_$;


ALTER FUNCTION trigger_rule."node/1h/highpowerusage_kpi"(timestamp with time zone) OWNER TO postgres;

--
-- Name: node/1h/highpowerusage_notification_data(trigger_rule."node/1h/highpowerusage_details"); Type: FUNCTION; Schema: trigger_rule; Owner: minerva_admin
--

CREATE FUNCTION trigger_rule."node/1h/highpowerusage_notification_data"(trigger_rule."node/1h/highpowerusage_details") RETURNS json
    LANGUAGE plpgsql STABLE
    AS $_$
DECLARE
  data json;
BEGIN
SELECT (SELECT json_build_object(
  'power_kwh', $1.power_kwh
)) INTO data;
RETURN data;
END;
$_$;


ALTER FUNCTION trigger_rule."node/1h/highpowerusage_notification_data"(trigger_rule."node/1h/highpowerusage_details") OWNER TO minerva_admin;

--
-- Name: node/1h/highpowerusage_notification_message(trigger_rule."node/1h/highpowerusage_details"); Type: FUNCTION; Schema: trigger_rule; Owner: minerva_admin
--

CREATE FUNCTION trigger_rule."node/1h/highpowerusage_notification_message"(trigger_rule."node/1h/highpowerusage_details") RETURNS text
    LANGUAGE sql IMMUTABLE
    AS $_$
SELECT (SELECT array_to_string(
    ARRAY[
        'HighPowerUsage',
        format('%s > %s', $1.power_kwh, $1.max_power)
    ],
    E'\n'
))::text
$_$;


ALTER FUNCTION trigger_rule."node/1h/highpowerusage_notification_message"(trigger_rule."node/1h/highpowerusage_details") OWNER TO minerva_admin;

--
-- Name: node/1h/highpowerusage_runnable(timestamp with time zone); Type: FUNCTION; Schema: trigger_rule; Owner: postgres
--

CREATE FUNCTION trigger_rule."node/1h/highpowerusage_runnable"(timestamp with time zone) RETURNS boolean
    LANGUAGE sql STABLE
    AS $$
    SELECT TRUE;
    $$;


ALTER FUNCTION trigger_rule."node/1h/highpowerusage_runnable"(timestamp with time zone) OWNER TO postgres;

--
-- Name: node/1h/highpowerusage_set_thresholds(numeric); Type: FUNCTION; Schema: trigger_rule; Owner: postgres
--

CREATE FUNCTION trigger_rule."node/1h/highpowerusage_set_thresholds"(max_power numeric) RETURNS integer
    LANGUAGE plpgsql SECURITY DEFINER
    AS $_$
    BEGIN
        EXECUTE format('CREATE OR REPLACE VIEW trigger_rule."node/1h/highpowerusage_threshold" AS SELECT %L::numeric AS max_power', $1);
        RETURN 42;
    END;
    $_$;


ALTER FUNCTION trigger_rule."node/1h/highpowerusage_set_thresholds"(max_power numeric) OWNER TO postgres;

--
-- Name: node/1h/highpowerusage_weight(trigger_rule."node/1h/highpowerusage_details"); Type: FUNCTION; Schema: trigger_rule; Owner: minerva_admin
--

CREATE FUNCTION trigger_rule."node/1h/highpowerusage_weight"(trigger_rule."node/1h/highpowerusage_details") RETURNS integer
    LANGUAGE sql IMMUTABLE
    AS $_$SELECT (SELECT
    CASE
        WHEN $1.power_kwh > 1 THEN 500
        WHEN $1.power_kwh > 2 THEN 800
        ELSE 300
    END)$_$;


ALTER FUNCTION trigger_rule."node/1h/highpowerusage_weight"(trigger_rule."node/1h/highpowerusage_details") OWNER TO minerva_admin;

--
-- Name: node/1h/highpowerusage_with_threshold(timestamp with time zone); Type: FUNCTION; Schema: trigger_rule; Owner: minerva_admin
--

CREATE FUNCTION trigger_rule."node/1h/highpowerusage_with_threshold"(timestamp with time zone) RETURNS SETOF trigger_rule."node/1h/highpowerusage_details"
    LANGUAGE sql STABLE
    AS $_$
SELECT kpi.*,COALESCE(exc.max_power, threshold.max_power) AS max_power
FROM trigger_rule."node/1h/highpowerusage_threshold" AS threshold, trigger_rule."node/1h/highpowerusage_kpi"($1) AS kpi
LEFT JOIN trigger_rule."node/1h/highpowerusage_exception_threshold" exc ON
    exc.entity_id = kpi.entity_id AND
    exc.start <= timestamp AND
    exc.expires > timestamp
$_$;


ALTER FUNCTION trigger_rule."node/1h/highpowerusage_with_threshold"(timestamp with time zone) OWNER TO minerva_admin;

--
-- Name: node/1w/highpowerusage(timestamp with time zone); Type: FUNCTION; Schema: trigger_rule; Owner: minerva_admin
--

CREATE FUNCTION trigger_rule."node/1w/highpowerusage"(timestamp with time zone) RETURNS SETOF trigger_rule."node/1w/highpowerusage_details"
    LANGUAGE sql STABLE
    AS $_$ SELECT * FROM trigger_rule."node/1w/highpowerusage_with_threshold"($1) WHERE power_kwh > max_power; $_$;


ALTER FUNCTION trigger_rule."node/1w/highpowerusage"(timestamp with time zone) OWNER TO minerva_admin;

--
-- Name: node/1w/highpowerusage_add_or_change_threshold_exception(integer, timestamp with time zone, timestamp with time zone, numeric); Type: FUNCTION; Schema: trigger_rule; Owner: postgres
--

CREATE FUNCTION trigger_rule."node/1w/highpowerusage_add_or_change_threshold_exception"(entity integer, new_start timestamp with time zone, new_expires timestamp with time zone, max_power_new numeric) RETURNS void
    LANGUAGE sql
    AS $$SELECT trigger_rule."node/1w/highpowerusage_get_or_create_exception_threshold"(entity); UPDATE trigger_rule."node/1w/highpowerusage_exception_threshold" SET (start, expires, max_power) = (new_start, new_expires, max_power_new) WHERE entity_id = entity;$$;


ALTER FUNCTION trigger_rule."node/1w/highpowerusage_add_or_change_threshold_exception"(entity integer, new_start timestamp with time zone, new_expires timestamp with time zone, max_power_new numeric) OWNER TO postgres;

--
-- Name: node/1w/highpowerusage_exception_threshold; Type: TABLE; Schema: trigger_rule; Owner: minerva_admin
--

CREATE TABLE trigger_rule."node/1w/highpowerusage_exception_threshold" (
    id integer NOT NULL,
    entity_id integer,
    created timestamp with time zone DEFAULT now(),
    start timestamp with time zone,
    expires timestamp with time zone,
    remark text,
    max_power numeric
);


ALTER TABLE trigger_rule."node/1w/highpowerusage_exception_threshold" OWNER TO minerva_admin;

--
-- Name: node/1w/highpowerusage_create_exception_threshold(integer); Type: FUNCTION; Schema: trigger_rule; Owner: postgres
--

CREATE FUNCTION trigger_rule."node/1w/highpowerusage_create_exception_threshold"(entity integer) RETURNS trigger_rule."node/1w/highpowerusage_exception_threshold"
    LANGUAGE sql
    AS $_$INSERT INTO trigger_rule."node/1w/highpowerusage_exception_threshold"(entity_id) VALUES ($1) RETURNING *;$_$;


ALTER FUNCTION trigger_rule."node/1w/highpowerusage_create_exception_threshold"(entity integer) OWNER TO postgres;

--
-- Name: node/1w/highpowerusage_create_notification(timestamp with time zone); Type: FUNCTION; Schema: trigger_rule; Owner: minerva_admin
--

CREATE FUNCTION trigger_rule."node/1w/highpowerusage_create_notification"(timestamp with time zone) RETURNS SETOF trigger.notification
    LANGUAGE sql STABLE
    AS $_$
SELECT
    n.entity_id,
    n.timestamp,
    COALESCE(exc.weight, trigger_rule."node/1w/highpowerusage_weight"(n)) AS weight,
    trigger_rule."node/1w/highpowerusage_notification_message"(n) AS details,
    trigger_rule."node/1w/highpowerusage_notification_data"(n) AS data
FROM trigger_rule."node/1w/highpowerusage"($1) AS n
LEFT JOIN trigger_rule."node/1w/highpowerusage_exception_weight" AS exc ON
    exc.entity_id = n.entity_id AND
    exc.start <= n.timestamp AND
    exc.expires > n.timestamp $_$;


ALTER FUNCTION trigger_rule."node/1w/highpowerusage_create_notification"(timestamp with time zone) OWNER TO minerva_admin;

--
-- Name: node/1w/highpowerusage_fingerprint(timestamp with time zone); Type: FUNCTION; Schema: trigger_rule; Owner: postgres
--

CREATE FUNCTION trigger_rule."node/1w/highpowerusage_fingerprint"(timestamp with time zone) RETURNS text
    LANGUAGE sql STABLE
    AS $$
      SELECT now()::text;
    $$;


ALTER FUNCTION trigger_rule."node/1w/highpowerusage_fingerprint"(timestamp with time zone) OWNER TO postgres;

--
-- Name: node/1w/highpowerusage_get_exception_threshold(integer); Type: FUNCTION; Schema: trigger_rule; Owner: postgres
--

CREATE FUNCTION trigger_rule."node/1w/highpowerusage_get_exception_threshold"(entity integer) RETURNS trigger_rule."node/1w/highpowerusage_exception_threshold"
    LANGUAGE sql
    AS $$SELECT * FROM trigger_rule."node/1w/highpowerusage_exception_threshold" WHERE entity_id = entity;$$;


ALTER FUNCTION trigger_rule."node/1w/highpowerusage_get_exception_threshold"(entity integer) OWNER TO postgres;

--
-- Name: node/1w/highpowerusage_get_or_create_exception_threshold(integer); Type: FUNCTION; Schema: trigger_rule; Owner: postgres
--

CREATE FUNCTION trigger_rule."node/1w/highpowerusage_get_or_create_exception_threshold"(entity integer) RETURNS trigger_rule."node/1w/highpowerusage_exception_threshold"
    LANGUAGE sql
    AS $_$SELECT COALESCE(trigger_rule."node/1w/highpowerusage_get_exception_threshold"($1), trigger_rule."node/1w/highpowerusage_create_exception_threshold"($1));$_$;


ALTER FUNCTION trigger_rule."node/1w/highpowerusage_get_or_create_exception_threshold"(entity integer) OWNER TO postgres;

--
-- Name: node/1w/highpowerusage_kpi(timestamp with time zone); Type: FUNCTION; Schema: trigger_rule; Owner: postgres
--

CREATE FUNCTION trigger_rule."node/1w/highpowerusage_kpi"(timestamp with time zone) RETURNS SETOF trigger_rule."node/1w/highpowerusage_kpi"
    LANGUAGE plpgsql STABLE
    AS $_$BEGIN
    RETURN QUERY EXECUTE $query$
    SELECT
        t.entity_id,
        t.timestamp,
        t.power_kwh
    FROM trend."hub_node_main_15m" AS t
    WHERE
        t.timestamp = $1
    $query$ USING $1;
END;$_$;


ALTER FUNCTION trigger_rule."node/1w/highpowerusage_kpi"(timestamp with time zone) OWNER TO postgres;

--
-- Name: node/1w/highpowerusage_notification_data(trigger_rule."node/1w/highpowerusage_details"); Type: FUNCTION; Schema: trigger_rule; Owner: minerva_admin
--

CREATE FUNCTION trigger_rule."node/1w/highpowerusage_notification_data"(trigger_rule."node/1w/highpowerusage_details") RETURNS json
    LANGUAGE plpgsql STABLE
    AS $_$
DECLARE
  data json;
BEGIN
SELECT (SELECT json_build_object(
  'power_kwh', $1.power_kwh
)) INTO data;
RETURN data;
END;
$_$;


ALTER FUNCTION trigger_rule."node/1w/highpowerusage_notification_data"(trigger_rule."node/1w/highpowerusage_details") OWNER TO minerva_admin;

--
-- Name: node/1w/highpowerusage_notification_message(trigger_rule."node/1w/highpowerusage_details"); Type: FUNCTION; Schema: trigger_rule; Owner: minerva_admin
--

CREATE FUNCTION trigger_rule."node/1w/highpowerusage_notification_message"(trigger_rule."node/1w/highpowerusage_details") RETURNS text
    LANGUAGE sql IMMUTABLE
    AS $_$
SELECT (SELECT array_to_string(
    ARRAY[
        'HighPowerUsage',
        format('%s > %s', $1.power_kwh, $1.max_power)
    ],
    E'\n'
))::text
$_$;


ALTER FUNCTION trigger_rule."node/1w/highpowerusage_notification_message"(trigger_rule."node/1w/highpowerusage_details") OWNER TO minerva_admin;

--
-- Name: node/1w/highpowerusage_runnable(timestamp with time zone); Type: FUNCTION; Schema: trigger_rule; Owner: postgres
--

CREATE FUNCTION trigger_rule."node/1w/highpowerusage_runnable"(timestamp with time zone) RETURNS boolean
    LANGUAGE sql STABLE
    AS $$
    SELECT TRUE;
    $$;


ALTER FUNCTION trigger_rule."node/1w/highpowerusage_runnable"(timestamp with time zone) OWNER TO postgres;

--
-- Name: node/1w/highpowerusage_set_thresholds(numeric); Type: FUNCTION; Schema: trigger_rule; Owner: postgres
--

CREATE FUNCTION trigger_rule."node/1w/highpowerusage_set_thresholds"(max_power numeric) RETURNS integer
    LANGUAGE plpgsql SECURITY DEFINER
    AS $_$
    BEGIN
        EXECUTE format('CREATE OR REPLACE VIEW trigger_rule."node/1w/highpowerusage_threshold" AS SELECT %L::numeric AS max_power', $1);
        RETURN 42;
    END;
    $_$;


ALTER FUNCTION trigger_rule."node/1w/highpowerusage_set_thresholds"(max_power numeric) OWNER TO postgres;

--
-- Name: node/1w/highpowerusage_weight(trigger_rule."node/1w/highpowerusage_details"); Type: FUNCTION; Schema: trigger_rule; Owner: minerva_admin
--

CREATE FUNCTION trigger_rule."node/1w/highpowerusage_weight"(trigger_rule."node/1w/highpowerusage_details") RETURNS integer
    LANGUAGE sql IMMUTABLE
    AS $_$SELECT (SELECT
    CASE
        WHEN $1.power_kwh > 1 THEN 500
        WHEN $1.power_kwh > 2 THEN 800
        ELSE 300
    END)$_$;


ALTER FUNCTION trigger_rule."node/1w/highpowerusage_weight"(trigger_rule."node/1w/highpowerusage_details") OWNER TO minerva_admin;

--
-- Name: node/1w/highpowerusage_with_threshold(timestamp with time zone); Type: FUNCTION; Schema: trigger_rule; Owner: minerva_admin
--

CREATE FUNCTION trigger_rule."node/1w/highpowerusage_with_threshold"(timestamp with time zone) RETURNS SETOF trigger_rule."node/1w/highpowerusage_details"
    LANGUAGE sql STABLE
    AS $_$
SELECT kpi.*,COALESCE(exc.max_power, threshold.max_power) AS max_power
FROM trigger_rule."node/1w/highpowerusage_threshold" AS threshold, trigger_rule."node/1w/highpowerusage_kpi"($1) AS kpi
LEFT JOIN trigger_rule."node/1w/highpowerusage_exception_threshold" exc ON
    exc.entity_id = kpi.entity_id AND
    exc.start <= timestamp AND
    exc.expires > timestamp
$_$;


ALTER FUNCTION trigger_rule."node/1w/highpowerusage_with_threshold"(timestamp with time zone) OWNER TO minerva_admin;

--
-- Name: update(name); Type: FUNCTION; Schema: virtual_entity; Owner: postgres
--

CREATE FUNCTION virtual_entity.update(name name) RETURNS integer
    LANGUAGE plpgsql
    AS $$
DECLARE
    result integer;
BEGIN
    EXECUTE format(
        'SELECT count(entity.%I(v.name)) FROM virtual_entity.%I v LEFT JOIN entity.%I e ON e.name = v.name WHERE e.name IS NULL',
        format('to_%s', name), name, name
    ) INTO result;

    RETURN result;
END;
$$;


ALTER FUNCTION virtual_entity.update(name name) OWNER TO postgres;

--
-- Name: first(anyelement); Type: AGGREGATE; Schema: public; Owner: postgres
--

CREATE AGGREGATE public.first(anyelement) (
    SFUNC = public.fst,
    STYPE = anyelement
);


ALTER AGGREGATE public.first(anyelement) OWNER TO postgres;

--
-- Name: last(anyelement); Type: AGGREGATE; Schema: public; Owner: postgres
--

CREATE AGGREGATE public.last(anyelement) (
    SFUNC = public.snd,
    STYPE = anyelement
);


ALTER AGGREGATE public.last(anyelement) OWNER TO postgres;

--
-- Name: sum_array(anyarray); Type: AGGREGATE; Schema: public; Owner: postgres
--

CREATE AGGREGATE public.sum_array(anyarray) (
    SFUNC = public.add_array,
    STYPE = anyarray
);


ALTER AGGREGATE public.sum_array(anyarray) OWNER TO postgres;

--
-- Name: max_data_type(text); Type: AGGREGATE; Schema: trend_directory; Owner: postgres
--

CREATE AGGREGATE trend_directory.max_data_type(text) (
    SFUNC = trend_directory.greatest_data_type,
    STYPE = text
);


ALTER AGGREGATE trend_directory.max_data_type(text) OWNER TO postgres;

--
-- Name: alias_type_id_seq; Type: SEQUENCE; Schema: alias_directory; Owner: postgres
--

CREATE SEQUENCE alias_directory.alias_type_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE alias_directory.alias_type_id_seq OWNER TO postgres;

--
-- Name: alias_type_id_seq; Type: SEQUENCE OWNED BY; Schema: alias_directory; Owner: postgres
--

ALTER SEQUENCE alias_directory.alias_type_id_seq OWNED BY alias_directory.alias_type.id;


--
-- Name: hub_node_curr_ptr; Type: TABLE; Schema: attribute_history; Owner: minerva_writer
--

CREATE TABLE attribute_history.hub_node_curr_ptr (
    id integer NOT NULL
);


ALTER TABLE attribute_history.hub_node_curr_ptr OWNER TO minerva_writer;

--
-- Name: hub_node; Type: VIEW; Schema: attribute; Owner: minerva_writer
--

CREATE VIEW attribute.hub_node AS
 SELECT h.id,
    h.first_appearance,
    h.modified,
    h.hash,
    h.entity_id,
    h."timestamp",
    h."end",
    h.name,
    h.equipment_type,
    h.equipment_serial,
    h.longitude,
    h.latitude
   FROM (attribute_history.hub_node h
     JOIN attribute_history.hub_node_curr_ptr c ON ((h.id = c.id)));


ALTER VIEW attribute.hub_node OWNER TO minerva_writer;

--
-- Name: hub_node; Type: TABLE; Schema: attribute_base; Owner: minerva_writer
--

CREATE TABLE attribute_base.hub_node (
    entity_id integer NOT NULL,
    "timestamp" timestamp with time zone NOT NULL,
    "end" timestamp with time zone,
    name text,
    equipment_type text,
    equipment_serial text,
    longitude real,
    latitude real
);


ALTER TABLE attribute_base.hub_node OWNER TO minerva_writer;

--
-- Name: minerva_entity_set; Type: TABLE; Schema: attribute_base; Owner: minerva_writer
--

CREATE TABLE attribute_base.minerva_entity_set (
    entity_id integer NOT NULL,
    "timestamp" timestamp with time zone NOT NULL,
    "end" timestamp with time zone,
    name text,
    fullname text,
    "group" text,
    source_entity_type text,
    owner text,
    description text,
    last_update text
);


ALTER TABLE attribute_base.minerva_entity_set OWNER TO minerva_writer;

--
-- Name: attribute_id_seq; Type: SEQUENCE; Schema: attribute_directory; Owner: postgres
--

CREATE SEQUENCE attribute_directory.attribute_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE attribute_directory.attribute_id_seq OWNER TO postgres;

--
-- Name: attribute_id_seq; Type: SEQUENCE OWNED BY; Schema: attribute_directory; Owner: postgres
--

ALTER SEQUENCE attribute_directory.attribute_id_seq OWNED BY attribute_directory.attribute.id;


--
-- Name: attribute_store_compacted; Type: TABLE; Schema: attribute_directory; Owner: postgres
--

CREATE TABLE attribute_directory.attribute_store_compacted (
    attribute_store_id integer NOT NULL,
    compacted timestamp with time zone
);


ALTER TABLE attribute_directory.attribute_store_compacted OWNER TO postgres;

--
-- Name: attribute_store_id_seq; Type: SEQUENCE; Schema: attribute_directory; Owner: postgres
--

CREATE SEQUENCE attribute_directory.attribute_store_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE attribute_directory.attribute_store_id_seq OWNER TO postgres;

--
-- Name: attribute_store_id_seq; Type: SEQUENCE OWNED BY; Schema: attribute_directory; Owner: postgres
--

ALTER SEQUENCE attribute_directory.attribute_store_id_seq OWNED BY attribute_directory.attribute_store.id;


--
-- Name: attribute_tag_link; Type: TABLE; Schema: attribute_directory; Owner: postgres
--

CREATE TABLE attribute_directory.attribute_tag_link (
    attribute_id integer NOT NULL,
    tag_id integer NOT NULL
);


ALTER TABLE attribute_directory.attribute_tag_link OWNER TO postgres;

--
-- Name: dependencies; Type: VIEW; Schema: attribute_directory; Owner: postgres
--

CREATE VIEW attribute_directory.dependencies AS
 SELECT dependent.relname AS src,
    pg_attribute.attname AS column_name,
    dependee.relname AS dst
   FROM (((((pg_depend
     JOIN pg_rewrite ON ((pg_depend.objid = pg_rewrite.oid)))
     JOIN pg_class dependee ON ((pg_rewrite.ev_class = dependee.oid)))
     JOIN pg_class dependent ON ((pg_depend.refobjid = dependent.oid)))
     JOIN pg_namespace n ON ((dependent.relnamespace = n.oid)))
     JOIN pg_attribute ON (((pg_depend.refobjid = pg_attribute.attrelid) AND (pg_depend.refobjsubid = pg_attribute.attnum))))
  WHERE ((n.nspname = 'attribute_directory'::name) AND (pg_attribute.attnum > 0));


ALTER VIEW attribute_directory.dependencies OWNER TO postgres;

--
-- Name: sampled_view_materialization_id_seq; Type: SEQUENCE; Schema: attribute_directory; Owner: postgres
--

CREATE SEQUENCE attribute_directory.sampled_view_materialization_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE attribute_directory.sampled_view_materialization_id_seq OWNER TO postgres;

--
-- Name: sampled_view_materialization_id_seq; Type: SEQUENCE OWNED BY; Schema: attribute_directory; Owner: postgres
--

ALTER SEQUENCE attribute_directory.sampled_view_materialization_id_seq OWNED BY attribute_directory.sampled_view_materialization.id;


--
-- Name: hub_node_curr_selection; Type: VIEW; Schema: attribute_history; Owner: minerva_writer
--

CREATE VIEW attribute_history.hub_node_curr_selection AS
 SELECT DISTINCT ON (a.entity_id) a.id
   FROM attribute_history.hub_node a
  ORDER BY a.entity_id, a."timestamp" DESC;


ALTER VIEW attribute_history.hub_node_curr_selection OWNER TO minerva_writer;

--
-- Name: hub_node_id_seq; Type: SEQUENCE; Schema: attribute_history; Owner: minerva_writer
--

CREATE SEQUENCE attribute_history.hub_node_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE attribute_history.hub_node_id_seq OWNER TO minerva_writer;

--
-- Name: hub_node_id_seq; Type: SEQUENCE OWNED BY; Schema: attribute_history; Owner: minerva_writer
--

ALTER SEQUENCE attribute_history.hub_node_id_seq OWNED BY attribute_history.hub_node.id;


--
-- Name: minerva_entity_set_changes; Type: VIEW; Schema: attribute_history; Owner: minerva_writer
--

CREATE VIEW attribute_history.minerva_entity_set_changes AS
 SELECT minerva_entity_set.entity_id,
    minerva_entity_set."timestamp",
    COALESCE(((minerva_entity_set.hash)::text <> (lag(minerva_entity_set.hash) OVER w)::text), true) AS change
   FROM attribute_history.minerva_entity_set
  WINDOW w AS (PARTITION BY minerva_entity_set.entity_id ORDER BY minerva_entity_set."timestamp");


ALTER VIEW attribute_history.minerva_entity_set_changes OWNER TO minerva_writer;

--
-- Name: minerva_entity_set_curr_selection; Type: VIEW; Schema: attribute_history; Owner: minerva_writer
--

CREATE VIEW attribute_history.minerva_entity_set_curr_selection AS
 SELECT DISTINCT ON (a.entity_id) a.id
   FROM attribute_history.minerva_entity_set a
  ORDER BY a.entity_id, a."timestamp" DESC;


ALTER VIEW attribute_history.minerva_entity_set_curr_selection OWNER TO minerva_writer;

--
-- Name: minerva_entity_set_id_seq; Type: SEQUENCE; Schema: attribute_history; Owner: minerva_writer
--

CREATE SEQUENCE attribute_history.minerva_entity_set_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE attribute_history.minerva_entity_set_id_seq OWNER TO minerva_writer;

--
-- Name: minerva_entity_set_id_seq; Type: SEQUENCE OWNED BY; Schema: attribute_history; Owner: minerva_writer
--

ALTER SEQUENCE attribute_history.minerva_entity_set_id_seq OWNED BY attribute_history.minerva_entity_set.id;


--
-- Name: hub_node; Type: TABLE; Schema: attribute_staging; Owner: minerva_writer
--

CREATE UNLOGGED TABLE attribute_staging.hub_node (
    entity_id integer NOT NULL,
    "timestamp" timestamp with time zone NOT NULL,
    "end" timestamp with time zone,
    name text,
    equipment_type text,
    equipment_serial text,
    longitude real,
    latitude real
);


ALTER TABLE attribute_staging.hub_node OWNER TO minerva_writer;

--
-- Name: hub_node_modified; Type: VIEW; Schema: attribute_staging; Owner: minerva_writer
--

CREATE VIEW attribute_staging.hub_node_modified AS
 SELECT s.entity_id,
    s."timestamp",
    s."end",
    s.name,
    s.equipment_type,
    s.equipment_serial,
    s.longitude,
    s.latitude
   FROM (attribute_staging.hub_node s
     JOIN attribute_history.hub_node a ON (((a.entity_id = s.entity_id) AND (a."timestamp" = s."timestamp"))));


ALTER VIEW attribute_staging.hub_node_modified OWNER TO minerva_writer;

--
-- Name: hub_node_new; Type: VIEW; Schema: attribute_staging; Owner: minerva_writer
--

CREATE VIEW attribute_staging.hub_node_new AS
 SELECT s.entity_id,
    s."timestamp",
    public.last(s."end") AS "end",
    public.last(s.name) AS name,
    public.last(s.equipment_type) AS equipment_type,
    public.last(s.equipment_serial) AS equipment_serial,
    public.last(s.longitude) AS longitude,
    public.last(s.latitude) AS latitude
   FROM (attribute_staging.hub_node s
     LEFT JOIN attribute_history.hub_node a ON (((a.entity_id = s.entity_id) AND (a."timestamp" = s."timestamp"))))
  WHERE (a.entity_id IS NULL)
  GROUP BY s.entity_id, s."timestamp";


ALTER VIEW attribute_staging.hub_node_new OWNER TO minerva_writer;

--
-- Name: minerva_entity_set; Type: TABLE; Schema: attribute_staging; Owner: minerva_writer
--

CREATE UNLOGGED TABLE attribute_staging.minerva_entity_set (
    entity_id integer NOT NULL,
    "timestamp" timestamp with time zone NOT NULL,
    "end" timestamp with time zone,
    name text,
    fullname text,
    "group" text,
    source_entity_type text,
    owner text,
    description text,
    last_update text
);


ALTER TABLE attribute_staging.minerva_entity_set OWNER TO minerva_writer;

--
-- Name: minerva_entity_set_modified; Type: VIEW; Schema: attribute_staging; Owner: minerva_writer
--

CREATE VIEW attribute_staging.minerva_entity_set_modified AS
 SELECT s.entity_id,
    s."timestamp",
    s."end",
    s.name,
    s.fullname,
    s."group",
    s.source_entity_type,
    s.owner,
    s.description,
    s.last_update
   FROM (attribute_staging.minerva_entity_set s
     JOIN attribute_history.minerva_entity_set a ON (((a.entity_id = s.entity_id) AND (a."timestamp" = s."timestamp"))));


ALTER VIEW attribute_staging.minerva_entity_set_modified OWNER TO minerva_writer;

--
-- Name: minerva_entity_set_new; Type: VIEW; Schema: attribute_staging; Owner: minerva_writer
--

CREATE VIEW attribute_staging.minerva_entity_set_new AS
 SELECT s.entity_id,
    s."timestamp",
    public.last(s."end") AS "end",
    public.last(s.name) AS name,
    public.last(s.fullname) AS fullname,
    public.last(s."group") AS "group",
    public.last(s.source_entity_type) AS source_entity_type,
    public.last(s.owner) AS owner,
    public.last(s.description) AS description,
    public.last(s.last_update) AS last_update
   FROM (attribute_staging.minerva_entity_set s
     LEFT JOIN attribute_history.minerva_entity_set a ON (((a.entity_id = s.entity_id) AND (a."timestamp" = s."timestamp"))))
  WHERE (a.entity_id IS NULL)
  GROUP BY s.entity_id, s."timestamp";


ALTER VIEW attribute_staging.minerva_entity_set_new OWNER TO minerva_writer;

--
-- Name: data_source_id_seq; Type: SEQUENCE; Schema: directory; Owner: postgres
--

CREATE SEQUENCE directory.data_source_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE directory.data_source_id_seq OWNER TO postgres;

--
-- Name: data_source_id_seq; Type: SEQUENCE OWNED BY; Schema: directory; Owner: postgres
--

ALTER SEQUENCE directory.data_source_id_seq OWNED BY directory.data_source.id;


--
-- Name: entity_type_id_seq; Type: SEQUENCE; Schema: directory; Owner: postgres
--

CREATE SEQUENCE directory.entity_type_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE directory.entity_type_id_seq OWNER TO postgres;

--
-- Name: entity_type_id_seq; Type: SEQUENCE OWNED BY; Schema: directory; Owner: postgres
--

ALTER SEQUENCE directory.entity_type_id_seq OWNED BY directory.entity_type.id;


--
-- Name: tag; Type: TABLE; Schema: directory; Owner: postgres
--

CREATE TABLE directory.tag (
    id integer NOT NULL,
    name character varying NOT NULL,
    tag_group_id integer NOT NULL,
    description character varying
);


ALTER TABLE directory.tag OWNER TO postgres;

--
-- Name: TABLE tag; Type: COMMENT; Schema: directory; Owner: postgres
--

COMMENT ON TABLE directory.tag IS 'Stores all tags. A tag is a simple label that can be attached to a number of object types in the database, such as entities and trends.';


--
-- Name: tag_group; Type: TABLE; Schema: directory; Owner: postgres
--

CREATE TABLE directory.tag_group (
    id integer NOT NULL,
    name character varying NOT NULL,
    complementary boolean NOT NULL
);


ALTER TABLE directory.tag_group OWNER TO postgres;

--
-- Name: TABLE tag_group; Type: COMMENT; Schema: directory; Owner: postgres
--

COMMENT ON TABLE directory.tag_group IS 'Stores groups that can be related to by tags.';


--
-- Name: tag_group_id_seq; Type: SEQUENCE; Schema: directory; Owner: postgres
--

CREATE SEQUENCE directory.tag_group_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE directory.tag_group_id_seq OWNER TO postgres;

--
-- Name: tag_group_id_seq; Type: SEQUENCE OWNED BY; Schema: directory; Owner: postgres
--

ALTER SEQUENCE directory.tag_group_id_seq OWNED BY directory.tag_group.id;


--
-- Name: tag_id_seq; Type: SEQUENCE; Schema: directory; Owner: postgres
--

CREATE SEQUENCE directory.tag_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE directory.tag_id_seq OWNER TO postgres;

--
-- Name: tag_id_seq; Type: SEQUENCE OWNED BY; Schema: directory; Owner: postgres
--

ALTER SEQUENCE directory.tag_id_seq OWNED BY directory.tag.id;


--
-- Name: entity_set_id_seq; Type: SEQUENCE; Schema: entity; Owner: postgres
--

CREATE SEQUENCE entity.entity_set_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE entity.entity_set_id_seq OWNER TO postgres;

--
-- Name: entity_set_id_seq; Type: SEQUENCE OWNED BY; Schema: entity; Owner: postgres
--

ALTER SEQUENCE entity.entity_set_id_seq OWNED BY entity.entity_set.id;


--
-- Name: node_id_seq; Type: SEQUENCE; Schema: entity; Owner: postgres
--

CREATE SEQUENCE entity.node_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE entity.node_id_seq OWNER TO postgres;

--
-- Name: node_id_seq; Type: SEQUENCE OWNED BY; Schema: entity; Owner: postgres
--

ALTER SEQUENCE entity.node_id_seq OWNED BY entity.node.id;


--
-- Name: v-network_id_seq; Type: SEQUENCE; Schema: entity; Owner: postgres
--

CREATE SEQUENCE entity."v-network_id_seq"
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE entity."v-network_id_seq" OWNER TO postgres;

--
-- Name: v-network_id_seq; Type: SEQUENCE OWNED BY; Schema: entity; Owner: postgres
--

ALTER SEQUENCE entity."v-network_id_seq" OWNED BY entity."v-network".id;


--
-- Name: job; Type: TABLE; Schema: logging; Owner: postgres
--

CREATE TABLE logging.job (
    id bigint NOT NULL,
    action jsonb NOT NULL,
    started timestamp with time zone NOT NULL,
    finished timestamp with time zone
);


ALTER TABLE logging.job OWNER TO postgres;

--
-- Name: job_id_seq; Type: SEQUENCE; Schema: logging; Owner: postgres
--

CREATE SEQUENCE logging.job_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE logging.job_id_seq OWNER TO postgres;

--
-- Name: job_id_seq; Type: SEQUENCE OWNED BY; Schema: logging; Owner: postgres
--

ALTER SEQUENCE logging.job_id_seq OWNED BY logging.job.id;


--
-- Name: trigger-notification; Type: TABLE; Schema: notification; Owner: minerva_writer
--

CREATE TABLE notification."trigger-notification" (
    id integer NOT NULL,
    entity_id integer NOT NULL,
    "timestamp" timestamp with time zone NOT NULL,
    rule_id integer,
    details text,
    data json,
    weight integer,
    created timestamp with time zone
);


ALTER TABLE notification."trigger-notification" OWNER TO minerva_writer;

--
-- Name: trigger-notification_id_seq; Type: SEQUENCE; Schema: notification; Owner: minerva_writer
--

CREATE SEQUENCE notification."trigger-notification_id_seq"
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE notification."trigger-notification_id_seq" OWNER TO minerva_writer;

--
-- Name: trigger-notification_id_seq; Type: SEQUENCE OWNED BY; Schema: notification; Owner: minerva_writer
--

ALTER SEQUENCE notification."trigger-notification_id_seq" OWNED BY notification."trigger-notification".id;


--
-- Name: trigger-notification_staging; Type: TABLE; Schema: notification; Owner: minerva_writer
--

CREATE TABLE notification."trigger-notification_staging" (
    entity_id integer NOT NULL,
    "timestamp" timestamp with time zone NOT NULL,
    rule_id integer,
    details text,
    data json,
    weight integer,
    created timestamp with time zone
);


ALTER TABLE notification."trigger-notification_staging" OWNER TO minerva_writer;

--
-- Name: attribute_id_seq; Type: SEQUENCE; Schema: notification_directory; Owner: postgres
--

CREATE SEQUENCE notification_directory.attribute_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE notification_directory.attribute_id_seq OWNER TO postgres;

--
-- Name: attribute_id_seq; Type: SEQUENCE OWNED BY; Schema: notification_directory; Owner: postgres
--

ALTER SEQUENCE notification_directory.attribute_id_seq OWNED BY notification_directory.attribute.id;


--
-- Name: last_notification; Type: TABLE; Schema: notification_directory; Owner: postgres
--

CREATE TABLE notification_directory.last_notification (
    name text NOT NULL,
    notification_store text NOT NULL,
    last_notification integer NOT NULL
);


ALTER TABLE notification_directory.last_notification OWNER TO postgres;

--
-- Name: TABLE last_notification; Type: COMMENT; Schema: notification_directory; Owner: postgres
--

COMMENT ON TABLE notification_directory.last_notification IS 'Specifies the id of the last notification seen by a client of
the notification service';


--
-- Name: notification_set_store_id_seq; Type: SEQUENCE; Schema: notification_directory; Owner: postgres
--

CREATE SEQUENCE notification_directory.notification_set_store_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE notification_directory.notification_set_store_id_seq OWNER TO postgres;

--
-- Name: notification_set_store_id_seq; Type: SEQUENCE OWNED BY; Schema: notification_directory; Owner: postgres
--

ALTER SEQUENCE notification_directory.notification_set_store_id_seq OWNED BY notification_directory.notification_set_store.id;


--
-- Name: notification_store_id_seq; Type: SEQUENCE; Schema: notification_directory; Owner: postgres
--

CREATE SEQUENCE notification_directory.notification_store_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE notification_directory.notification_store_id_seq OWNER TO postgres;

--
-- Name: notification_store_id_seq1; Type: SEQUENCE; Schema: notification_directory; Owner: postgres
--

CREATE SEQUENCE notification_directory.notification_store_id_seq1
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE notification_directory.notification_store_id_seq1 OWNER TO postgres;

--
-- Name: notification_store_id_seq1; Type: SEQUENCE OWNED BY; Schema: notification_directory; Owner: postgres
--

ALTER SEQUENCE notification_directory.notification_store_id_seq1 OWNED BY notification_directory.notification_store.id;


--
-- Name: set_attribute; Type: TABLE; Schema: notification_directory; Owner: postgres
--

CREATE TABLE notification_directory.set_attribute (
    id integer NOT NULL,
    notification_set_store_id integer,
    name name NOT NULL,
    data_type name NOT NULL,
    description character varying NOT NULL
);


ALTER TABLE notification_directory.set_attribute OWNER TO postgres;

--
-- Name: TABLE set_attribute; Type: COMMENT; Schema: notification_directory; Owner: postgres
--

COMMENT ON TABLE notification_directory.set_attribute IS 'Describes attributes of notification_set_stores. A set_attribute of a
notification_set_store is an attribute that each notification set has. A
set_attribute corresponds directly to a column in the main
notification_set_store table.';


--
-- Name: set_attribute_id_seq; Type: SEQUENCE; Schema: notification_directory; Owner: postgres
--

CREATE SEQUENCE notification_directory.set_attribute_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE notification_directory.set_attribute_id_seq OWNER TO postgres;

--
-- Name: set_attribute_id_seq; Type: SEQUENCE OWNED BY; Schema: notification_directory; Owner: postgres
--

ALTER SEQUENCE notification_directory.set_attribute_id_seq OWNED BY notification_directory.set_attribute.id;


--
-- Name: refinery_schema_history; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.refinery_schema_history (
    version integer NOT NULL,
    name character varying(255),
    applied_on character varying(255),
    checksum character varying(255)
);


ALTER TABLE public.refinery_schema_history OWNER TO postgres;

--
-- Name: node->v-network; Type: TABLE; Schema: relation; Owner: postgres
--

CREATE TABLE relation."node->v-network" (
    source_id integer,
    target_id integer
);


ALTER TABLE relation."node->v-network" OWNER TO postgres;

--
-- Name: node->v-network; Type: VIEW; Schema: relation_def; Owner: postgres
--

CREATE VIEW relation_def."node->v-network" AS
 SELECT n.id AS source_id,
    v.id AS target_id
   FROM entity.node n,
    entity."v-network" v
  GROUP BY n.id, v.id;


ALTER VIEW relation_def."node->v-network" OWNER TO postgres;

--
-- Name: type_id_seq; Type: SEQUENCE; Schema: relation_directory; Owner: postgres
--

CREATE SEQUENCE relation_directory.type_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE relation_directory.type_id_seq OWNER TO postgres;

--
-- Name: type_id_seq; Type: SEQUENCE OWNED BY; Schema: relation_directory; Owner: postgres
--

ALTER SEQUENCE relation_directory.type_id_seq OWNED BY relation_directory.type.id;


--
-- Name: setting_id_seq; Type: SEQUENCE; Schema: system; Owner: postgres
--

CREATE SEQUENCE system.setting_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE system.setting_id_seq OWNER TO postgres;

--
-- Name: setting_id_seq; Type: SEQUENCE OWNED BY; Schema: system; Owner: postgres
--

ALTER SEQUENCE system.setting_id_seq OWNED BY system.setting.id;


--
-- Name: hub_node_main_15m; Type: TABLE; Schema: trend; Owner: postgres
--

CREATE TABLE trend.hub_node_main_15m (
    entity_id integer NOT NULL,
    "timestamp" timestamp with time zone NOT NULL,
    created timestamp with time zone NOT NULL,
    job_id bigint NOT NULL,
    outside_temp numeric,
    inside_temp numeric,
    power_kwh numeric,
    freq_power numeric,
    "power_Mwh" numeric GENERATED ALWAYS AS ((power_kwh / (1000)::numeric)) STORED
)
PARTITION BY RANGE ("timestamp");


ALTER TABLE trend.hub_node_main_15m OWNER TO postgres;

--
-- Name: _hub-kpi_node_main_15m; Type: VIEW; Schema: trend; Owner: postgres
--

CREATE VIEW trend."_hub-kpi_node_main_15m" AS
 SELECT hub_node_main_15m."timestamp",
    hub_node_main_15m.entity_id,
    (hub_node_main_15m.power_kwh * (1000)::numeric) AS power_mwh
   FROM trend.hub_node_main_15m;


ALTER VIEW trend."_hub-kpi_node_main_15m" OWNER TO postgres;

--
-- Name: hub-kpi_node_main_15m; Type: TABLE; Schema: trend; Owner: postgres
--

CREATE TABLE trend."hub-kpi_node_main_15m" (
    entity_id integer NOT NULL,
    "timestamp" timestamp with time zone NOT NULL,
    created timestamp with time zone NOT NULL,
    job_id bigint NOT NULL,
    power_mwh bigint,
    power_max numeric
)
PARTITION BY RANGE ("timestamp");


ALTER TABLE trend."hub-kpi_node_main_15m" OWNER TO postgres;

--
-- Name: hub-kpi_node_main_15m_staging; Type: TABLE; Schema: trend; Owner: postgres
--

CREATE UNLOGGED TABLE trend."hub-kpi_node_main_15m_staging" (
    entity_id integer NOT NULL,
    "timestamp" timestamp with time zone NOT NULL,
    created timestamp with time zone,
    job_id bigint,
    power_mwh bigint,
    power_max numeric
);


ALTER TABLE trend."hub-kpi_node_main_15m_staging" OWNER TO postgres;

--
-- Name: hub_node_main_15m_staging; Type: TABLE; Schema: trend; Owner: postgres
--

CREATE UNLOGGED TABLE trend.hub_node_main_15m_staging (
    entity_id integer NOT NULL,
    "timestamp" timestamp with time zone NOT NULL,
    created timestamp with time zone,
    job_id bigint,
    outside_temp numeric,
    inside_temp numeric,
    power_kwh numeric,
    freq_power numeric
);


ALTER TABLE trend.hub_node_main_15m_staging OWNER TO postgres;

--
-- Name: hub_node_main_1d; Type: TABLE; Schema: trend; Owner: postgres
--

CREATE TABLE trend.hub_node_main_1d (
    entity_id integer NOT NULL,
    "timestamp" timestamp with time zone NOT NULL,
    created timestamp with time zone NOT NULL,
    job_id bigint NOT NULL,
    freq_power numeric,
    inside_temp numeric,
    outside_temp numeric,
    power_kwh numeric,
    samples integer,
    "power_Mwh" numeric GENERATED ALWAYS AS ((power_kwh / (1000)::numeric)) STORED
)
PARTITION BY RANGE ("timestamp");


ALTER TABLE trend.hub_node_main_1d OWNER TO postgres;

--
-- Name: hub_node_main_1d_staging; Type: TABLE; Schema: trend; Owner: postgres
--

CREATE UNLOGGED TABLE trend.hub_node_main_1d_staging (
    entity_id integer NOT NULL,
    "timestamp" timestamp with time zone NOT NULL,
    created timestamp with time zone,
    job_id bigint,
    freq_power numeric,
    inside_temp numeric,
    outside_temp numeric,
    power_kwh numeric,
    samples integer
);


ALTER TABLE trend.hub_node_main_1d_staging OWNER TO postgres;

--
-- Name: hub_node_main_1h; Type: TABLE; Schema: trend; Owner: postgres
--

CREATE TABLE trend.hub_node_main_1h (
    entity_id integer NOT NULL,
    "timestamp" timestamp with time zone NOT NULL,
    created timestamp with time zone NOT NULL,
    job_id bigint NOT NULL,
    freq_power numeric,
    inside_temp numeric,
    outside_temp numeric,
    power_kwh numeric,
    samples integer,
    "power_Mwh" numeric GENERATED ALWAYS AS ((power_kwh / (1000)::numeric)) STORED
)
PARTITION BY RANGE ("timestamp");


ALTER TABLE trend.hub_node_main_1h OWNER TO postgres;

--
-- Name: hub_node_main_1h_staging; Type: TABLE; Schema: trend; Owner: postgres
--

CREATE UNLOGGED TABLE trend.hub_node_main_1h_staging (
    entity_id integer NOT NULL,
    "timestamp" timestamp with time zone NOT NULL,
    created timestamp with time zone,
    job_id bigint,
    freq_power numeric,
    inside_temp numeric,
    outside_temp numeric,
    power_kwh numeric,
    samples integer
);


ALTER TABLE trend.hub_node_main_1h_staging OWNER TO postgres;

--
-- Name: hub_node_main_1month; Type: TABLE; Schema: trend; Owner: postgres
--

CREATE TABLE trend.hub_node_main_1month (
    entity_id integer NOT NULL,
    "timestamp" timestamp with time zone NOT NULL,
    created timestamp with time zone NOT NULL,
    job_id bigint NOT NULL,
    freq_power numeric,
    inside_temp numeric,
    outside_temp numeric,
    power_kwh numeric,
    samples bigint,
    "power_Mwh" numeric GENERATED ALWAYS AS ((power_kwh / (1000)::numeric)) STORED
)
PARTITION BY RANGE ("timestamp");


ALTER TABLE trend.hub_node_main_1month OWNER TO postgres;

--
-- Name: hub_node_main_1month_staging; Type: TABLE; Schema: trend; Owner: postgres
--

CREATE UNLOGGED TABLE trend.hub_node_main_1month_staging (
    entity_id integer NOT NULL,
    "timestamp" timestamp with time zone NOT NULL,
    created timestamp with time zone,
    job_id bigint,
    freq_power numeric,
    inside_temp numeric,
    outside_temp numeric,
    power_kwh numeric,
    samples bigint
);


ALTER TABLE trend.hub_node_main_1month_staging OWNER TO postgres;

--
-- Name: hub_node_main_1w; Type: TABLE; Schema: trend; Owner: postgres
--

CREATE TABLE trend.hub_node_main_1w (
    entity_id integer NOT NULL,
    "timestamp" timestamp with time zone NOT NULL,
    created timestamp with time zone NOT NULL,
    job_id bigint NOT NULL,
    freq_power numeric,
    inside_temp numeric,
    outside_temp numeric,
    power_kwh numeric,
    samples bigint,
    "power_Mwh" numeric GENERATED ALWAYS AS ((power_kwh / (1000)::numeric)) STORED
)
PARTITION BY RANGE ("timestamp");


ALTER TABLE trend.hub_node_main_1w OWNER TO postgres;

--
-- Name: hub_node_main_1w_staging; Type: TABLE; Schema: trend; Owner: postgres
--

CREATE UNLOGGED TABLE trend.hub_node_main_1w_staging (
    entity_id integer NOT NULL,
    "timestamp" timestamp with time zone NOT NULL,
    created timestamp with time zone,
    job_id bigint,
    freq_power numeric,
    inside_temp numeric,
    outside_temp numeric,
    power_kwh numeric,
    samples bigint
);


ALTER TABLE trend.hub_node_main_1w_staging OWNER TO postgres;

--
-- Name: hub_v-network_main_15m; Type: VIEW; Schema: trend; Owner: postgres
--

CREATE VIEW trend."hub_v-network_main_15m" AS
 SELECT r.target_id AS entity_id,
    t."timestamp",
    count(*) AS samples,
    sum(t.outside_temp) AS outside_temp,
    sum(t.inside_temp) AS inside_temp,
    sum(t.power_kwh) AS power_kwh,
    sum(t.freq_power) AS freq_power
   FROM (trend.hub_node_main_15m t
     JOIN relation."node->v-network" r ON ((t.entity_id = r.source_id)))
  GROUP BY t."timestamp", r.target_id;


ALTER VIEW trend."hub_v-network_main_15m" OWNER TO postgres;

--
-- Name: hub_v-network_main_1d; Type: VIEW; Schema: trend; Owner: postgres
--

CREATE VIEW trend."hub_v-network_main_1d" AS
 SELECT r.target_id AS entity_id,
    t."timestamp",
    sum(t.freq_power) AS freq_power,
    sum(t.inside_temp) AS inside_temp,
    sum(t.outside_temp) AS outside_temp,
    sum(t.power_kwh) AS power_kwh,
    sum(t.samples) AS samples
   FROM (trend.hub_node_main_1d t
     JOIN relation."node->v-network" r ON ((t.entity_id = r.source_id)))
  GROUP BY t."timestamp", r.target_id;


ALTER VIEW trend."hub_v-network_main_1d" OWNER TO postgres;

--
-- Name: hub_v-network_main_1h; Type: VIEW; Schema: trend; Owner: postgres
--

CREATE VIEW trend."hub_v-network_main_1h" AS
 SELECT r.target_id AS entity_id,
    t."timestamp",
    sum(t.freq_power) AS freq_power,
    sum(t.inside_temp) AS inside_temp,
    sum(t.outside_temp) AS outside_temp,
    sum(t.power_kwh) AS power_kwh,
    sum(t.samples) AS samples
   FROM (trend.hub_node_main_1h t
     JOIN relation."node->v-network" r ON ((t.entity_id = r.source_id)))
  GROUP BY t."timestamp", r.target_id;


ALTER VIEW trend."hub_v-network_main_1h" OWNER TO postgres;

--
-- Name: hub_v-network_main_1month; Type: VIEW; Schema: trend; Owner: postgres
--

CREATE VIEW trend."hub_v-network_main_1month" AS
 SELECT r.target_id AS entity_id,
    t."timestamp",
    sum(t.freq_power) AS freq_power,
    sum(t.inside_temp) AS inside_temp,
    sum(t.outside_temp) AS outside_temp,
    sum(t.power_kwh) AS power_kwh,
    sum(t.samples) AS samples
   FROM (trend.hub_node_main_1month t
     JOIN relation."node->v-network" r ON ((t.entity_id = r.source_id)))
  GROUP BY t."timestamp", r.target_id;


ALTER VIEW trend."hub_v-network_main_1month" OWNER TO postgres;

--
-- Name: hub_v-network_main_1w; Type: VIEW; Schema: trend; Owner: postgres
--

CREATE VIEW trend."hub_v-network_main_1w" AS
 SELECT r.target_id AS entity_id,
    t."timestamp",
    sum(t.freq_power) AS freq_power,
    sum(t.inside_temp) AS inside_temp,
    sum(t.outside_temp) AS outside_temp,
    sum(t.power_kwh) AS power_kwh,
    sum(t.samples) AS samples
   FROM (trend.hub_node_main_1w t
     JOIN relation."node->v-network" r ON ((t.entity_id = r.source_id)))
  GROUP BY t."timestamp", r.target_id;


ALTER VIEW trend."hub_v-network_main_1w" OWNER TO postgres;

--
-- Name: power_report; Type: VIEW; Schema: trend; Owner: postgres
--

CREATE VIEW trend.power_report AS
 SELECT date(hub_node_main_15m."timestamp") AS date,
    max(hub_node_main_15m.power_kwh) AS max_power_kwh
   FROM trend.hub_node_main_15m
  GROUP BY (date(hub_node_main_15m."timestamp"));


ALTER VIEW trend.power_report OWNER TO postgres;

--
-- Name: function_materialization_id_seq; Type: SEQUENCE; Schema: trend_directory; Owner: postgres
--

CREATE SEQUENCE trend_directory.function_materialization_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE trend_directory.function_materialization_id_seq OWNER TO postgres;

--
-- Name: function_materialization_id_seq; Type: SEQUENCE OWNED BY; Schema: trend_directory; Owner: postgres
--

ALTER SEQUENCE trend_directory.function_materialization_id_seq OWNED BY trend_directory.function_materialization.id;


--
-- Name: function_materialization_state; Type: TABLE; Schema: trend_directory; Owner: postgres
--

CREATE TABLE trend_directory.function_materialization_state (
    materialization_id integer NOT NULL,
    "timestamp" timestamp with time zone NOT NULL,
    source_fingerprint jsonb,
    processed_fingerprint jsonb,
    job_id bigint
);


ALTER TABLE trend_directory.function_materialization_state OWNER TO postgres;

--
-- Name: TABLE function_materialization_state; Type: COMMENT; Schema: trend_directory; Owner: postgres
--

COMMENT ON TABLE trend_directory.function_materialization_state IS 'Stores the relation between the state of the sources used for the materialization and the state of the materialized data, so that from this table, it can be decided if a new materialization should be done.
';


--
-- Name: COLUMN function_materialization_state.materialization_id; Type: COMMENT; Schema: trend_directory; Owner: postgres
--

COMMENT ON COLUMN trend_directory.function_materialization_state.materialization_id IS 'The ID of the materialization type';


--
-- Name: COLUMN function_materialization_state."timestamp"; Type: COMMENT; Schema: trend_directory; Owner: postgres
--

COMMENT ON COLUMN trend_directory.function_materialization_state."timestamp" IS 'The timestamp of the materialized (materialization result) data';


--
-- Name: COLUMN function_materialization_state.source_fingerprint; Type: COMMENT; Schema: trend_directory; Owner: postgres
--

COMMENT ON COLUMN trend_directory.function_materialization_state.source_fingerprint IS 'Aggregate state of all sources';


--
-- Name: COLUMN function_materialization_state.processed_fingerprint; Type: COMMENT; Schema: trend_directory; Owner: postgres
--

COMMENT ON COLUMN trend_directory.function_materialization_state.processed_fingerprint IS 'Snapshot of the source_fingerprint at the time of the most recent materialization
';


--
-- Name: COLUMN function_materialization_state.job_id; Type: COMMENT; Schema: trend_directory; Owner: postgres
--

COMMENT ON COLUMN trend_directory.function_materialization_state.job_id IS 'ID of the most recent job for this materialization';


--
-- Name: generated_table_trend_id_seq; Type: SEQUENCE; Schema: trend_directory; Owner: postgres
--

CREATE SEQUENCE trend_directory.generated_table_trend_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE trend_directory.generated_table_trend_id_seq OWNER TO postgres;

--
-- Name: generated_table_trend_id_seq; Type: SEQUENCE OWNED BY; Schema: trend_directory; Owner: postgres
--

ALTER SEQUENCE trend_directory.generated_table_trend_id_seq OWNED BY trend_directory.generated_table_trend.id;


--
-- Name: materialization_id_seq; Type: SEQUENCE; Schema: trend_directory; Owner: postgres
--

CREATE SEQUENCE trend_directory.materialization_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE trend_directory.materialization_id_seq OWNER TO postgres;

--
-- Name: materialization_id_seq; Type: SEQUENCE OWNED BY; Schema: trend_directory; Owner: postgres
--

ALTER SEQUENCE trend_directory.materialization_id_seq OWNED BY trend_directory.materialization.id;


--
-- Name: materialization_metrics; Type: TABLE; Schema: trend_directory; Owner: postgres
--

CREATE TABLE trend_directory.materialization_metrics (
    materialization_id integer NOT NULL,
    execution_count integer DEFAULT 0 NOT NULL,
    total_duration interval DEFAULT '00:00:00'::interval NOT NULL
);


ALTER TABLE trend_directory.materialization_metrics OWNER TO postgres;

--
-- Name: TABLE materialization_metrics; Type: COMMENT; Schema: trend_directory; Owner: postgres
--

COMMENT ON TABLE trend_directory.materialization_metrics IS 'Metrics on individual materializations.';


--
-- Name: COLUMN materialization_metrics.materialization_id; Type: COMMENT; Schema: trend_directory; Owner: postgres
--

COMMENT ON COLUMN trend_directory.materialization_metrics.materialization_id IS 'The ID of the materialization';


--
-- Name: materialization_state; Type: TABLE; Schema: trend_directory; Owner: postgres
--

CREATE TABLE trend_directory.materialization_state (
    materialization_id integer NOT NULL,
    "timestamp" timestamp with time zone NOT NULL,
    source_fingerprint jsonb,
    processed_fingerprint jsonb,
    max_modified timestamp with time zone,
    job_id bigint
);


ALTER TABLE trend_directory.materialization_state OWNER TO postgres;

--
-- Name: TABLE materialization_state; Type: COMMENT; Schema: trend_directory; Owner: postgres
--

COMMENT ON TABLE trend_directory.materialization_state IS 'Stores the relation between the state of the sources used for the materialization and the state of the materialized data, so that from this table, it can be decided if a new materialization should be done.
';


--
-- Name: COLUMN materialization_state.materialization_id; Type: COMMENT; Schema: trend_directory; Owner: postgres
--

COMMENT ON COLUMN trend_directory.materialization_state.materialization_id IS 'The ID of the materialization type';


--
-- Name: COLUMN materialization_state."timestamp"; Type: COMMENT; Schema: trend_directory; Owner: postgres
--

COMMENT ON COLUMN trend_directory.materialization_state."timestamp" IS 'The timestamp of the materialized (materialization result) data';


--
-- Name: COLUMN materialization_state.source_fingerprint; Type: COMMENT; Schema: trend_directory; Owner: postgres
--

COMMENT ON COLUMN trend_directory.materialization_state.source_fingerprint IS 'Aggregate state of all sources';


--
-- Name: COLUMN materialization_state.processed_fingerprint; Type: COMMENT; Schema: trend_directory; Owner: postgres
--

COMMENT ON COLUMN trend_directory.materialization_state.processed_fingerprint IS 'Snapshot of the source_fingerprint at the time of the most recent materialization
';


--
-- Name: COLUMN materialization_state.max_modified; Type: COMMENT; Schema: trend_directory; Owner: postgres
--

COMMENT ON COLUMN trend_directory.materialization_state.max_modified IS 'Date of last data received';


--
-- Name: COLUMN materialization_state.job_id; Type: COMMENT; Schema: trend_directory; Owner: postgres
--

COMMENT ON COLUMN trend_directory.materialization_state.job_id IS 'ID of the most recent job for this materialization';


--
-- Name: materialization_tag_link; Type: TABLE; Schema: trend_directory; Owner: postgres
--

CREATE TABLE trend_directory.materialization_tag_link (
    materialization_id integer NOT NULL,
    tag_id integer NOT NULL
);


ALTER TABLE trend_directory.materialization_tag_link OWNER TO postgres;

--
-- Name: TABLE materialization_tag_link; Type: COMMENT; Schema: trend_directory; Owner: postgres
--

COMMENT ON TABLE trend_directory.materialization_tag_link IS 'Links tags to materializations. Examples of tags to link to a materialization
might be: online, offline, aggregation, kpi, etc.';


--
-- Name: materialization_trend_store_link; Type: TABLE; Schema: trend_directory; Owner: postgres
--

CREATE TABLE trend_directory.materialization_trend_store_link (
    materialization_id integer NOT NULL,
    trend_store_part_id integer NOT NULL,
    timestamp_mapping_func regprocedure NOT NULL
);


ALTER TABLE trend_directory.materialization_trend_store_link OWNER TO postgres;

--
-- Name: TABLE materialization_trend_store_link; Type: COMMENT; Schema: trend_directory; Owner: postgres
--

COMMENT ON TABLE trend_directory.materialization_trend_store_link IS 'Stores the dependencies between a materialization and its source table trend store parts. Multiple levels of views and functions may exist between a materialization and its source table trend stores. These intermediate views and functions are not registered here, but only the table trend stores containing the actual source data used in the view.
The timestamp_mapping_func column stores the function to map a timestamp of the source (trend_store_part) to a timestamp of the target (the view and target trend_store_part).
';


--
-- Name: COLUMN materialization_trend_store_link.materialization_id; Type: COMMENT; Schema: trend_directory; Owner: postgres
--

COMMENT ON COLUMN trend_directory.materialization_trend_store_link.materialization_id IS 'Reference to a materialization.';


--
-- Name: COLUMN materialization_trend_store_link.trend_store_part_id; Type: COMMENT; Schema: trend_directory; Owner: postgres
--

COMMENT ON COLUMN trend_directory.materialization_trend_store_link.trend_store_part_id IS 'Reference to a trend_store_part that is a source of the materialization referenced by materialization_id.
';


--
-- Name: COLUMN materialization_trend_store_link.timestamp_mapping_func; Type: COMMENT; Schema: trend_directory; Owner: postgres
--

COMMENT ON COLUMN trend_directory.materialization_trend_store_link.timestamp_mapping_func IS 'The function that maps timestamps in the source table to timestamps in the materialized data. For example, for a view for an hour aggregation from 15 minute granularity data will need to map 4 timestamps in the source to 1 timestamp in the resulting data.
';


--
-- Name: modified; Type: TABLE; Schema: trend_directory; Owner: postgres
--

CREATE TABLE trend_directory.modified (
    trend_store_part_id integer NOT NULL,
    "timestamp" timestamp with time zone NOT NULL,
    first timestamp with time zone NOT NULL,
    last timestamp with time zone NOT NULL
);


ALTER TABLE trend_directory.modified OWNER TO postgres;

--
-- Name: TABLE modified; Type: COMMENT; Schema: trend_directory; Owner: postgres
--

COMMENT ON TABLE trend_directory.modified IS 'Stores information on when trend store parts have changed and for what timestamp. The information in this table is updated when the data has changed and any actions like materialization-state-updating can be triggered (using insert, update or delete triggers) from this table, because it is decoupled from the data loading processes.
';


--
-- Name: COLUMN modified.first; Type: COMMENT; Schema: trend_directory; Owner: postgres
--

COMMENT ON COLUMN trend_directory.modified.first IS 'Time of the first modification';


--
-- Name: COLUMN modified.last; Type: COMMENT; Schema: trend_directory; Owner: postgres
--

COMMENT ON COLUMN trend_directory.modified.last IS 'Time of the last modification';


--
-- Name: modified_log; Type: TABLE; Schema: trend_directory; Owner: postgres
--

CREATE TABLE trend_directory.modified_log (
    id bigint NOT NULL,
    trend_store_part_id integer NOT NULL,
    "timestamp" timestamp with time zone NOT NULL,
    modified timestamp with time zone NOT NULL
);


ALTER TABLE trend_directory.modified_log OWNER TO postgres;

--
-- Name: TABLE modified_log; Type: COMMENT; Schema: trend_directory; Owner: postgres
--

COMMENT ON TABLE trend_directory.modified_log IS 'The ``modified_log`` table stores records of when what ``trend_store_part`` is modified and for what timestamp. This table is typically populated by data loading tools that call the ``trend_directory.mark_modified`` function. It is not populated automatically when inserting into the trend_store_part tables. The main purpose is to decouple the logging of data changes from actions triggered by those changes. There are no triggers on this table, and any actions should be triggered by changes on the ``trend_directory.modified`` table, which is updated by a separate processed based on the contents of this table.
';


--
-- Name: COLUMN modified_log.id; Type: COMMENT; Schema: trend_directory; Owner: postgres
--

COMMENT ON COLUMN trend_directory.modified_log.id IS 'Unique identifier for the log entry';


--
-- Name: COLUMN modified_log.trend_store_part_id; Type: COMMENT; Schema: trend_directory; Owner: postgres
--

COMMENT ON COLUMN trend_directory.modified_log.trend_store_part_id IS 'Reference to the trend_store_part';


--
-- Name: COLUMN modified_log."timestamp"; Type: COMMENT; Schema: trend_directory; Owner: postgres
--

COMMENT ON COLUMN trend_directory.modified_log."timestamp" IS 'Timestamp of the data in the trend_store_part';


--
-- Name: COLUMN modified_log.modified; Type: COMMENT; Schema: trend_directory; Owner: postgres
--

COMMENT ON COLUMN trend_directory.modified_log.modified IS 'Timestamp of the moment of modification';


--
-- Name: modified_log_id_seq; Type: SEQUENCE; Schema: trend_directory; Owner: postgres
--

CREATE SEQUENCE trend_directory.modified_log_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE trend_directory.modified_log_id_seq OWNER TO postgres;

--
-- Name: modified_log_id_seq; Type: SEQUENCE OWNED BY; Schema: trend_directory; Owner: postgres
--

ALTER SEQUENCE trend_directory.modified_log_id_seq OWNED BY trend_directory.modified_log.id;


--
-- Name: modified_log_processing_state; Type: TABLE; Schema: trend_directory; Owner: postgres
--

CREATE TABLE trend_directory.modified_log_processing_state (
    name text NOT NULL,
    last_processed_id bigint NOT NULL,
    updated timestamp with time zone DEFAULT now() NOT NULL
);


ALTER TABLE trend_directory.modified_log_processing_state OWNER TO postgres;

--
-- Name: partition_id_seq; Type: SEQUENCE; Schema: trend_directory; Owner: postgres
--

CREATE SEQUENCE trend_directory.partition_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE trend_directory.partition_id_seq OWNER TO postgres;

--
-- Name: partition_id_seq; Type: SEQUENCE OWNED BY; Schema: trend_directory; Owner: postgres
--

ALTER SEQUENCE trend_directory.partition_id_seq OWNED BY trend_directory.partition.id;


--
-- Name: table_trend_id_seq; Type: SEQUENCE; Schema: trend_directory; Owner: postgres
--

CREATE SEQUENCE trend_directory.table_trend_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE trend_directory.table_trend_id_seq OWNER TO postgres;

--
-- Name: table_trend_id_seq; Type: SEQUENCE OWNED BY; Schema: trend_directory; Owner: postgres
--

ALTER SEQUENCE trend_directory.table_trend_id_seq OWNED BY trend_directory.table_trend.id;


--
-- Name: table_trend_tag_link; Type: TABLE; Schema: trend_directory; Owner: postgres
--

CREATE TABLE trend_directory.table_trend_tag_link (
    table_trend_id integer NOT NULL,
    tag_id integer NOT NULL
);


ALTER TABLE trend_directory.table_trend_tag_link OWNER TO postgres;

--
-- Name: trend_store_id_seq; Type: SEQUENCE; Schema: trend_directory; Owner: postgres
--

CREATE SEQUENCE trend_directory.trend_store_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE trend_directory.trend_store_id_seq OWNER TO postgres;

--
-- Name: trend_store_id_seq; Type: SEQUENCE OWNED BY; Schema: trend_directory; Owner: postgres
--

ALTER SEQUENCE trend_directory.trend_store_id_seq OWNED BY trend_directory.trend_store.id;


--
-- Name: trend_store_part_id_seq; Type: SEQUENCE; Schema: trend_directory; Owner: postgres
--

CREATE SEQUENCE trend_directory.trend_store_part_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE trend_directory.trend_store_part_id_seq OWNER TO postgres;

--
-- Name: trend_store_part_id_seq; Type: SEQUENCE OWNED BY; Schema: trend_directory; Owner: postgres
--

ALTER SEQUENCE trend_directory.trend_store_part_id_seq OWNED BY trend_directory.trend_store_part.id;


--
-- Name: trend_store_part_stats; Type: TABLE; Schema: trend_directory; Owner: postgres
--

CREATE TABLE trend_directory.trend_store_part_stats (
    trend_store_part_id integer NOT NULL,
    "timestamp" timestamp with time zone NOT NULL,
    modified timestamp with time zone NOT NULL,
    count integer NOT NULL
);


ALTER TABLE trend_directory.trend_store_part_stats OWNER TO postgres;

--
-- Name: COLUMN trend_store_part_stats.modified; Type: COMMENT; Schema: trend_directory; Owner: postgres
--

COMMENT ON COLUMN trend_directory.trend_store_part_stats.modified IS 'Time of the last modification';


--
-- Name: trend_store_part_stats_to_update; Type: VIEW; Schema: trend_directory; Owner: postgres
--

CREATE VIEW trend_directory.trend_store_part_stats_to_update AS
 SELECT tsps.trend_store_part_id,
    tsps."timestamp"
   FROM (trend_directory.trend_store_part_stats tsps
     JOIN trend_directory.modified m ON (((tsps.trend_store_part_id = m.trend_store_part_id) AND (tsps."timestamp" = m."timestamp"))))
  WHERE (tsps.modified < (m.last + '00:00:01'::interval));


ALTER VIEW trend_directory.trend_store_part_stats_to_update OWNER TO postgres;

--
-- Name: trend_view_id_seq; Type: SEQUENCE; Schema: trend_directory; Owner: postgres
--

CREATE SEQUENCE trend_directory.trend_view_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE trend_directory.trend_view_id_seq OWNER TO postgres;

--
-- Name: trend_view_id_seq; Type: SEQUENCE OWNED BY; Schema: trend_directory; Owner: postgres
--

ALTER SEQUENCE trend_directory.trend_view_id_seq OWNED BY trend_directory.trend_view.id;


--
-- Name: trend_view_part_id_seq; Type: SEQUENCE; Schema: trend_directory; Owner: postgres
--

CREATE SEQUENCE trend_directory.trend_view_part_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE trend_directory.trend_view_part_id_seq OWNER TO postgres;

--
-- Name: trend_view_part_id_seq; Type: SEQUENCE OWNED BY; Schema: trend_directory; Owner: postgres
--

ALTER SEQUENCE trend_directory.trend_view_part_id_seq OWNED BY trend_directory.trend_view_part.id;


--
-- Name: view_materialization_id_seq; Type: SEQUENCE; Schema: trend_directory; Owner: postgres
--

CREATE SEQUENCE trend_directory.view_materialization_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE trend_directory.view_materialization_id_seq OWNER TO postgres;

--
-- Name: view_materialization_id_seq; Type: SEQUENCE OWNED BY; Schema: trend_directory; Owner: postgres
--

ALTER SEQUENCE trend_directory.view_materialization_id_seq OWNED BY trend_directory.view_materialization.id;


--
-- Name: view_trend; Type: TABLE; Schema: trend_directory; Owner: postgres
--

CREATE TABLE trend_directory.view_trend (
    id integer NOT NULL,
    trend_view_part_id integer NOT NULL,
    name name NOT NULL,
    data_type text NOT NULL,
    extra_data jsonb DEFAULT '{}'::jsonb NOT NULL,
    description text NOT NULL,
    time_aggregation text NOT NULL,
    entity_aggregation text NOT NULL
);


ALTER TABLE trend_directory.view_trend OWNER TO postgres;

--
-- Name: view_trend_id_seq; Type: SEQUENCE; Schema: trend_directory; Owner: postgres
--

ALTER TABLE trend_directory.view_trend ALTER COLUMN id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME trend_directory.view_trend_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: exception_base; Type: TABLE; Schema: trigger; Owner: postgres
--

CREATE TABLE trigger.exception_base (
    id integer NOT NULL,
    entity_id integer,
    start timestamp with time zone,
    expires timestamp with time zone,
    created timestamp with time zone DEFAULT now()
);


ALTER TABLE trigger.exception_base OWNER TO postgres;

--
-- Name: exception_base_id_seq; Type: SEQUENCE; Schema: trigger; Owner: postgres
--

CREATE SEQUENCE trigger.exception_base_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE trigger.exception_base_id_seq OWNER TO postgres;

--
-- Name: exception_base_id_seq; Type: SEQUENCE OWNED BY; Schema: trigger; Owner: postgres
--

ALTER SEQUENCE trigger.exception_base_id_seq OWNED BY trigger.exception_base.id;


--
-- Name: rule_id_seq; Type: SEQUENCE; Schema: trigger; Owner: postgres
--

CREATE SEQUENCE trigger.rule_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE trigger.rule_id_seq OWNER TO postgres;

--
-- Name: rule_id_seq; Type: SEQUENCE OWNED BY; Schema: trigger; Owner: postgres
--

ALTER SEQUENCE trigger.rule_id_seq OWNED BY trigger.rule.id;


--
-- Name: rule_trend_store_link; Type: TABLE; Schema: trigger; Owner: postgres
--

CREATE TABLE trigger.rule_trend_store_link (
    rule_id integer NOT NULL,
    trend_store_part_id integer NOT NULL,
    timestamp_mapping_func regprocedure NOT NULL
);


ALTER TABLE trigger.rule_trend_store_link OWNER TO postgres;

--
-- Name: TABLE rule_trend_store_link; Type: COMMENT; Schema: trigger; Owner: postgres
--

COMMENT ON TABLE trigger.rule_trend_store_link IS 'Stores the dependencies between a trigger rule and its source table trend store parts. Multiple levels of views and functions may exist between a materialization and its source table trend stores. These intermediate views and functions are not registered here, but only the table trend stores containing the actual source data used in the trigger rule.
The timestamp_mapping_func column stores the function to map a timestamp of the source (trend_store_part) to a timestamp of the target notification.
';


--
-- Name: COLUMN rule_trend_store_link.rule_id; Type: COMMENT; Schema: trigger; Owner: postgres
--

COMMENT ON COLUMN trigger.rule_trend_store_link.rule_id IS 'Reference to a trigger rule.';


--
-- Name: COLUMN rule_trend_store_link.trend_store_part_id; Type: COMMENT; Schema: trigger; Owner: postgres
--

COMMENT ON COLUMN trigger.rule_trend_store_link.trend_store_part_id IS 'Reference to a trend_store_part that is a source of the materialization referenced by materialization_id.
';


--
-- Name: COLUMN rule_trend_store_link.timestamp_mapping_func; Type: COMMENT; Schema: trigger; Owner: postgres
--

COMMENT ON COLUMN trigger.rule_trend_store_link.timestamp_mapping_func IS 'The function that maps timestamps in the source table to timestamps in the materialized data. For example, for a view for an hour aggregation from 15 minute granularity data will need to map 4 timestamps in the source to 1 timestamp in the resulting data.
';


--
-- Name: node/15m/highpowerusage_exception_threshold_id_seq; Type: SEQUENCE; Schema: trigger_rule; Owner: minerva_admin
--

CREATE SEQUENCE trigger_rule."node/15m/highpowerusage_exception_threshold_id_seq"
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE trigger_rule."node/15m/highpowerusage_exception_threshold_id_seq" OWNER TO minerva_admin;

--
-- Name: node/15m/highpowerusage_exception_threshold_id_seq; Type: SEQUENCE OWNED BY; Schema: trigger_rule; Owner: minerva_admin
--

ALTER SEQUENCE trigger_rule."node/15m/highpowerusage_exception_threshold_id_seq" OWNED BY trigger_rule."node/15m/highpowerusage_exception_threshold".id;


--
-- Name: node/15m/highpowerusage_exception_weight; Type: TABLE; Schema: trigger_rule; Owner: minerva_admin
--

CREATE TABLE trigger_rule."node/15m/highpowerusage_exception_weight" (
    id integer NOT NULL,
    entity_id integer,
    created timestamp with time zone DEFAULT now() NOT NULL,
    start timestamp with time zone DEFAULT now() NOT NULL,
    expires timestamp with time zone DEFAULT (now() + '3 mons'::interval) NOT NULL,
    weight integer NOT NULL
);


ALTER TABLE trigger_rule."node/15m/highpowerusage_exception_weight" OWNER TO minerva_admin;

--
-- Name: node/15m/highpowerusage_exception_weight_id_seq; Type: SEQUENCE; Schema: trigger_rule; Owner: minerva_admin
--

CREATE SEQUENCE trigger_rule."node/15m/highpowerusage_exception_weight_id_seq"
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE trigger_rule."node/15m/highpowerusage_exception_weight_id_seq" OWNER TO minerva_admin;

--
-- Name: node/15m/highpowerusage_exception_weight_id_seq; Type: SEQUENCE OWNED BY; Schema: trigger_rule; Owner: minerva_admin
--

ALTER SEQUENCE trigger_rule."node/15m/highpowerusage_exception_weight_id_seq" OWNED BY trigger_rule."node/15m/highpowerusage_exception_weight".id;


--
-- Name: node/15m/highpowerusage_threshold; Type: VIEW; Schema: trigger_rule; Owner: minerva_admin
--

CREATE VIEW trigger_rule."node/15m/highpowerusage_threshold" AS
 SELECT 0.05 AS max_power;


ALTER VIEW trigger_rule."node/15m/highpowerusage_threshold" OWNER TO minerva_admin;

--
-- Name: node/1d/highpowerusage_exception_threshold_id_seq; Type: SEQUENCE; Schema: trigger_rule; Owner: minerva_admin
--

CREATE SEQUENCE trigger_rule."node/1d/highpowerusage_exception_threshold_id_seq"
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE trigger_rule."node/1d/highpowerusage_exception_threshold_id_seq" OWNER TO minerva_admin;

--
-- Name: node/1d/highpowerusage_exception_threshold_id_seq; Type: SEQUENCE OWNED BY; Schema: trigger_rule; Owner: minerva_admin
--

ALTER SEQUENCE trigger_rule."node/1d/highpowerusage_exception_threshold_id_seq" OWNED BY trigger_rule."node/1d/highpowerusage_exception_threshold".id;


--
-- Name: node/1d/highpowerusage_exception_weight; Type: TABLE; Schema: trigger_rule; Owner: minerva_admin
--

CREATE TABLE trigger_rule."node/1d/highpowerusage_exception_weight" (
    id integer NOT NULL,
    entity_id integer,
    created timestamp with time zone DEFAULT now() NOT NULL,
    start timestamp with time zone DEFAULT now() NOT NULL,
    expires timestamp with time zone DEFAULT (now() + '3 mons'::interval) NOT NULL,
    weight integer NOT NULL
);


ALTER TABLE trigger_rule."node/1d/highpowerusage_exception_weight" OWNER TO minerva_admin;

--
-- Name: node/1d/highpowerusage_exception_weight_id_seq; Type: SEQUENCE; Schema: trigger_rule; Owner: minerva_admin
--

CREATE SEQUENCE trigger_rule."node/1d/highpowerusage_exception_weight_id_seq"
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE trigger_rule."node/1d/highpowerusage_exception_weight_id_seq" OWNER TO minerva_admin;

--
-- Name: node/1d/highpowerusage_exception_weight_id_seq; Type: SEQUENCE OWNED BY; Schema: trigger_rule; Owner: minerva_admin
--

ALTER SEQUENCE trigger_rule."node/1d/highpowerusage_exception_weight_id_seq" OWNED BY trigger_rule."node/1d/highpowerusage_exception_weight".id;


--
-- Name: node/1d/highpowerusage_threshold; Type: VIEW; Schema: trigger_rule; Owner: minerva_admin
--

CREATE VIEW trigger_rule."node/1d/highpowerusage_threshold" AS
 SELECT 0.05 AS max_power;


ALTER VIEW trigger_rule."node/1d/highpowerusage_threshold" OWNER TO minerva_admin;

--
-- Name: node/1h/highpowerusage_exception_threshold_id_seq; Type: SEQUENCE; Schema: trigger_rule; Owner: minerva_admin
--

CREATE SEQUENCE trigger_rule."node/1h/highpowerusage_exception_threshold_id_seq"
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE trigger_rule."node/1h/highpowerusage_exception_threshold_id_seq" OWNER TO minerva_admin;

--
-- Name: node/1h/highpowerusage_exception_threshold_id_seq; Type: SEQUENCE OWNED BY; Schema: trigger_rule; Owner: minerva_admin
--

ALTER SEQUENCE trigger_rule."node/1h/highpowerusage_exception_threshold_id_seq" OWNED BY trigger_rule."node/1h/highpowerusage_exception_threshold".id;


--
-- Name: node/1h/highpowerusage_exception_weight; Type: TABLE; Schema: trigger_rule; Owner: minerva_admin
--

CREATE TABLE trigger_rule."node/1h/highpowerusage_exception_weight" (
    id integer NOT NULL,
    entity_id integer,
    created timestamp with time zone DEFAULT now() NOT NULL,
    start timestamp with time zone DEFAULT now() NOT NULL,
    expires timestamp with time zone DEFAULT (now() + '3 mons'::interval) NOT NULL,
    weight integer NOT NULL
);


ALTER TABLE trigger_rule."node/1h/highpowerusage_exception_weight" OWNER TO minerva_admin;

--
-- Name: node/1h/highpowerusage_exception_weight_id_seq; Type: SEQUENCE; Schema: trigger_rule; Owner: minerva_admin
--

CREATE SEQUENCE trigger_rule."node/1h/highpowerusage_exception_weight_id_seq"
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE trigger_rule."node/1h/highpowerusage_exception_weight_id_seq" OWNER TO minerva_admin;

--
-- Name: node/1h/highpowerusage_exception_weight_id_seq; Type: SEQUENCE OWNED BY; Schema: trigger_rule; Owner: minerva_admin
--

ALTER SEQUENCE trigger_rule."node/1h/highpowerusage_exception_weight_id_seq" OWNED BY trigger_rule."node/1h/highpowerusage_exception_weight".id;


--
-- Name: node/1h/highpowerusage_threshold; Type: VIEW; Schema: trigger_rule; Owner: minerva_admin
--

CREATE VIEW trigger_rule."node/1h/highpowerusage_threshold" AS
 SELECT 0.05 AS max_power;


ALTER VIEW trigger_rule."node/1h/highpowerusage_threshold" OWNER TO minerva_admin;

--
-- Name: node/1w/highpowerusage_exception_threshold_id_seq; Type: SEQUENCE; Schema: trigger_rule; Owner: minerva_admin
--

CREATE SEQUENCE trigger_rule."node/1w/highpowerusage_exception_threshold_id_seq"
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE trigger_rule."node/1w/highpowerusage_exception_threshold_id_seq" OWNER TO minerva_admin;

--
-- Name: node/1w/highpowerusage_exception_threshold_id_seq; Type: SEQUENCE OWNED BY; Schema: trigger_rule; Owner: minerva_admin
--

ALTER SEQUENCE trigger_rule."node/1w/highpowerusage_exception_threshold_id_seq" OWNED BY trigger_rule."node/1w/highpowerusage_exception_threshold".id;


--
-- Name: node/1w/highpowerusage_exception_weight; Type: TABLE; Schema: trigger_rule; Owner: minerva_admin
--

CREATE TABLE trigger_rule."node/1w/highpowerusage_exception_weight" (
    id integer NOT NULL,
    entity_id integer,
    created timestamp with time zone DEFAULT now() NOT NULL,
    start timestamp with time zone DEFAULT now() NOT NULL,
    expires timestamp with time zone DEFAULT (now() + '3 mons'::interval) NOT NULL,
    weight integer NOT NULL
);


ALTER TABLE trigger_rule."node/1w/highpowerusage_exception_weight" OWNER TO minerva_admin;

--
-- Name: node/1w/highpowerusage_exception_weight_id_seq; Type: SEQUENCE; Schema: trigger_rule; Owner: minerva_admin
--

CREATE SEQUENCE trigger_rule."node/1w/highpowerusage_exception_weight_id_seq"
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE trigger_rule."node/1w/highpowerusage_exception_weight_id_seq" OWNER TO minerva_admin;

--
-- Name: node/1w/highpowerusage_exception_weight_id_seq; Type: SEQUENCE OWNED BY; Schema: trigger_rule; Owner: minerva_admin
--

ALTER SEQUENCE trigger_rule."node/1w/highpowerusage_exception_weight_id_seq" OWNED BY trigger_rule."node/1w/highpowerusage_exception_weight".id;


--
-- Name: node/1w/highpowerusage_threshold; Type: VIEW; Schema: trigger_rule; Owner: minerva_admin
--

CREATE VIEW trigger_rule."node/1w/highpowerusage_threshold" AS
 SELECT 0.05 AS max_power;


ALTER VIEW trigger_rule."node/1w/highpowerusage_threshold" OWNER TO minerva_admin;

--
-- Name: v-network; Type: VIEW; Schema: virtual_entity; Owner: postgres
--

CREATE VIEW virtual_entity."v-network" AS
 SELECT 'network'::text AS name;


ALTER VIEW virtual_entity."v-network" OWNER TO postgres;

--
-- Name: alias_type id; Type: DEFAULT; Schema: alias_directory; Owner: postgres
--

ALTER TABLE ONLY alias_directory.alias_type ALTER COLUMN id SET DEFAULT nextval('alias_directory.alias_type_id_seq'::regclass);


--
-- Name: attribute id; Type: DEFAULT; Schema: attribute_directory; Owner: postgres
--

ALTER TABLE ONLY attribute_directory.attribute ALTER COLUMN id SET DEFAULT nextval('attribute_directory.attribute_id_seq'::regclass);


--
-- Name: attribute_store id; Type: DEFAULT; Schema: attribute_directory; Owner: postgres
--

ALTER TABLE ONLY attribute_directory.attribute_store ALTER COLUMN id SET DEFAULT nextval('attribute_directory.attribute_store_id_seq'::regclass);


--
-- Name: sampled_view_materialization id; Type: DEFAULT; Schema: attribute_directory; Owner: postgres
--

ALTER TABLE ONLY attribute_directory.sampled_view_materialization ALTER COLUMN id SET DEFAULT nextval('attribute_directory.sampled_view_materialization_id_seq'::regclass);


--
-- Name: hub_node id; Type: DEFAULT; Schema: attribute_history; Owner: minerva_writer
--

ALTER TABLE ONLY attribute_history.hub_node ALTER COLUMN id SET DEFAULT nextval('attribute_history.hub_node_id_seq'::regclass);


--
-- Name: minerva_entity_set id; Type: DEFAULT; Schema: attribute_history; Owner: minerva_writer
--

ALTER TABLE ONLY attribute_history.minerva_entity_set ALTER COLUMN id SET DEFAULT nextval('attribute_history.minerva_entity_set_id_seq'::regclass);


--
-- Name: data_source id; Type: DEFAULT; Schema: directory; Owner: postgres
--

ALTER TABLE ONLY directory.data_source ALTER COLUMN id SET DEFAULT nextval('directory.data_source_id_seq'::regclass);


--
-- Name: entity_type id; Type: DEFAULT; Schema: directory; Owner: postgres
--

ALTER TABLE ONLY directory.entity_type ALTER COLUMN id SET DEFAULT nextval('directory.entity_type_id_seq'::regclass);


--
-- Name: tag id; Type: DEFAULT; Schema: directory; Owner: postgres
--

ALTER TABLE ONLY directory.tag ALTER COLUMN id SET DEFAULT nextval('directory.tag_id_seq'::regclass);


--
-- Name: tag_group id; Type: DEFAULT; Schema: directory; Owner: postgres
--

ALTER TABLE ONLY directory.tag_group ALTER COLUMN id SET DEFAULT nextval('directory.tag_group_id_seq'::regclass);


--
-- Name: entity_set id; Type: DEFAULT; Schema: entity; Owner: postgres
--

ALTER TABLE ONLY entity.entity_set ALTER COLUMN id SET DEFAULT nextval('entity.entity_set_id_seq'::regclass);


--
-- Name: node id; Type: DEFAULT; Schema: entity; Owner: postgres
--

ALTER TABLE ONLY entity.node ALTER COLUMN id SET DEFAULT nextval('entity.node_id_seq'::regclass);


--
-- Name: v-network id; Type: DEFAULT; Schema: entity; Owner: postgres
--

ALTER TABLE ONLY entity."v-network" ALTER COLUMN id SET DEFAULT nextval('entity."v-network_id_seq"'::regclass);


--
-- Name: job id; Type: DEFAULT; Schema: logging; Owner: postgres
--

ALTER TABLE ONLY logging.job ALTER COLUMN id SET DEFAULT nextval('logging.job_id_seq'::regclass);


--
-- Name: trigger-notification id; Type: DEFAULT; Schema: notification; Owner: minerva_writer
--

ALTER TABLE ONLY notification."trigger-notification" ALTER COLUMN id SET DEFAULT nextval('notification."trigger-notification_id_seq"'::regclass);


--
-- Name: attribute id; Type: DEFAULT; Schema: notification_directory; Owner: postgres
--

ALTER TABLE ONLY notification_directory.attribute ALTER COLUMN id SET DEFAULT nextval('notification_directory.attribute_id_seq'::regclass);


--
-- Name: notification_set_store id; Type: DEFAULT; Schema: notification_directory; Owner: postgres
--

ALTER TABLE ONLY notification_directory.notification_set_store ALTER COLUMN id SET DEFAULT nextval('notification_directory.notification_set_store_id_seq'::regclass);


--
-- Name: notification_store id; Type: DEFAULT; Schema: notification_directory; Owner: postgres
--

ALTER TABLE ONLY notification_directory.notification_store ALTER COLUMN id SET DEFAULT nextval('notification_directory.notification_store_id_seq1'::regclass);


--
-- Name: set_attribute id; Type: DEFAULT; Schema: notification_directory; Owner: postgres
--

ALTER TABLE ONLY notification_directory.set_attribute ALTER COLUMN id SET DEFAULT nextval('notification_directory.set_attribute_id_seq'::regclass);


--
-- Name: type id; Type: DEFAULT; Schema: relation_directory; Owner: postgres
--

ALTER TABLE ONLY relation_directory.type ALTER COLUMN id SET DEFAULT nextval('relation_directory.type_id_seq'::regclass);


--
-- Name: setting id; Type: DEFAULT; Schema: system; Owner: postgres
--

ALTER TABLE ONLY system.setting ALTER COLUMN id SET DEFAULT nextval('system.setting_id_seq'::regclass);


--
-- Name: function_materialization id; Type: DEFAULT; Schema: trend_directory; Owner: postgres
--

ALTER TABLE ONLY trend_directory.function_materialization ALTER COLUMN id SET DEFAULT nextval('trend_directory.function_materialization_id_seq'::regclass);


--
-- Name: generated_table_trend id; Type: DEFAULT; Schema: trend_directory; Owner: postgres
--

ALTER TABLE ONLY trend_directory.generated_table_trend ALTER COLUMN id SET DEFAULT nextval('trend_directory.generated_table_trend_id_seq'::regclass);


--
-- Name: materialization id; Type: DEFAULT; Schema: trend_directory; Owner: postgres
--

ALTER TABLE ONLY trend_directory.materialization ALTER COLUMN id SET DEFAULT nextval('trend_directory.materialization_id_seq'::regclass);


--
-- Name: modified_log id; Type: DEFAULT; Schema: trend_directory; Owner: postgres
--

ALTER TABLE ONLY trend_directory.modified_log ALTER COLUMN id SET DEFAULT nextval('trend_directory.modified_log_id_seq'::regclass);


--
-- Name: partition id; Type: DEFAULT; Schema: trend_directory; Owner: postgres
--

ALTER TABLE ONLY trend_directory.partition ALTER COLUMN id SET DEFAULT nextval('trend_directory.partition_id_seq'::regclass);


--
-- Name: table_trend id; Type: DEFAULT; Schema: trend_directory; Owner: postgres
--

ALTER TABLE ONLY trend_directory.table_trend ALTER COLUMN id SET DEFAULT nextval('trend_directory.table_trend_id_seq'::regclass);


--
-- Name: trend_store id; Type: DEFAULT; Schema: trend_directory; Owner: postgres
--

ALTER TABLE ONLY trend_directory.trend_store ALTER COLUMN id SET DEFAULT nextval('trend_directory.trend_store_id_seq'::regclass);


--
-- Name: trend_store_part id; Type: DEFAULT; Schema: trend_directory; Owner: postgres
--

ALTER TABLE ONLY trend_directory.trend_store_part ALTER COLUMN id SET DEFAULT nextval('trend_directory.trend_store_part_id_seq'::regclass);


--
-- Name: trend_view id; Type: DEFAULT; Schema: trend_directory; Owner: postgres
--

ALTER TABLE ONLY trend_directory.trend_view ALTER COLUMN id SET DEFAULT nextval('trend_directory.trend_view_id_seq'::regclass);


--
-- Name: trend_view_part id; Type: DEFAULT; Schema: trend_directory; Owner: postgres
--

ALTER TABLE ONLY trend_directory.trend_view_part ALTER COLUMN id SET DEFAULT nextval('trend_directory.trend_view_part_id_seq'::regclass);


--
-- Name: view_materialization id; Type: DEFAULT; Schema: trend_directory; Owner: postgres
--

ALTER TABLE ONLY trend_directory.view_materialization ALTER COLUMN id SET DEFAULT nextval('trend_directory.view_materialization_id_seq'::regclass);


--
-- Name: exception_base id; Type: DEFAULT; Schema: trigger; Owner: postgres
--

ALTER TABLE ONLY trigger.exception_base ALTER COLUMN id SET DEFAULT nextval('trigger.exception_base_id_seq'::regclass);


--
-- Name: rule id; Type: DEFAULT; Schema: trigger; Owner: postgres
--

ALTER TABLE ONLY trigger.rule ALTER COLUMN id SET DEFAULT nextval('trigger.rule_id_seq'::regclass);


--
-- Name: node/15m/highpowerusage_exception_threshold id; Type: DEFAULT; Schema: trigger_rule; Owner: minerva_admin
--

ALTER TABLE ONLY trigger_rule."node/15m/highpowerusage_exception_threshold" ALTER COLUMN id SET DEFAULT nextval('trigger_rule."node/15m/highpowerusage_exception_threshold_id_seq"'::regclass);


--
-- Name: node/15m/highpowerusage_exception_weight id; Type: DEFAULT; Schema: trigger_rule; Owner: minerva_admin
--

ALTER TABLE ONLY trigger_rule."node/15m/highpowerusage_exception_weight" ALTER COLUMN id SET DEFAULT nextval('trigger_rule."node/15m/highpowerusage_exception_weight_id_seq"'::regclass);


--
-- Name: node/1d/highpowerusage_exception_threshold id; Type: DEFAULT; Schema: trigger_rule; Owner: minerva_admin
--

ALTER TABLE ONLY trigger_rule."node/1d/highpowerusage_exception_threshold" ALTER COLUMN id SET DEFAULT nextval('trigger_rule."node/1d/highpowerusage_exception_threshold_id_seq"'::regclass);


--
-- Name: node/1d/highpowerusage_exception_weight id; Type: DEFAULT; Schema: trigger_rule; Owner: minerva_admin
--

ALTER TABLE ONLY trigger_rule."node/1d/highpowerusage_exception_weight" ALTER COLUMN id SET DEFAULT nextval('trigger_rule."node/1d/highpowerusage_exception_weight_id_seq"'::regclass);


--
-- Name: node/1h/highpowerusage_exception_threshold id; Type: DEFAULT; Schema: trigger_rule; Owner: minerva_admin
--

ALTER TABLE ONLY trigger_rule."node/1h/highpowerusage_exception_threshold" ALTER COLUMN id SET DEFAULT nextval('trigger_rule."node/1h/highpowerusage_exception_threshold_id_seq"'::regclass);


--
-- Name: node/1h/highpowerusage_exception_weight id; Type: DEFAULT; Schema: trigger_rule; Owner: minerva_admin
--

ALTER TABLE ONLY trigger_rule."node/1h/highpowerusage_exception_weight" ALTER COLUMN id SET DEFAULT nextval('trigger_rule."node/1h/highpowerusage_exception_weight_id_seq"'::regclass);


--
-- Name: node/1w/highpowerusage_exception_threshold id; Type: DEFAULT; Schema: trigger_rule; Owner: minerva_admin
--

ALTER TABLE ONLY trigger_rule."node/1w/highpowerusage_exception_threshold" ALTER COLUMN id SET DEFAULT nextval('trigger_rule."node/1w/highpowerusage_exception_threshold_id_seq"'::regclass);


--
-- Name: node/1w/highpowerusage_exception_weight id; Type: DEFAULT; Schema: trigger_rule; Owner: minerva_admin
--

ALTER TABLE ONLY trigger_rule."node/1w/highpowerusage_exception_weight" ALTER COLUMN id SET DEFAULT nextval('trigger_rule."node/1w/highpowerusage_exception_weight_id_seq"'::regclass);


--
-- Name: alias_type alias_type_pkey; Type: CONSTRAINT; Schema: alias_directory; Owner: postgres
--

ALTER TABLE ONLY alias_directory.alias_type
    ADD CONSTRAINT alias_type_pkey PRIMARY KEY (id);


--
-- Name: attribute attribute_pkey; Type: CONSTRAINT; Schema: attribute_directory; Owner: postgres
--

ALTER TABLE ONLY attribute_directory.attribute
    ADD CONSTRAINT attribute_pkey PRIMARY KEY (id);


--
-- Name: attribute_store_compacted attribute_store_compacted_pkey; Type: CONSTRAINT; Schema: attribute_directory; Owner: postgres
--

ALTER TABLE ONLY attribute_directory.attribute_store_compacted
    ADD CONSTRAINT attribute_store_compacted_pkey PRIMARY KEY (attribute_store_id);


--
-- Name: attribute_store_curr_materialized attribute_store_curr_materialized_pkey; Type: CONSTRAINT; Schema: attribute_directory; Owner: postgres
--

ALTER TABLE ONLY attribute_directory.attribute_store_curr_materialized
    ADD CONSTRAINT attribute_store_curr_materialized_pkey PRIMARY KEY (attribute_store_id);


--
-- Name: attribute_store_modified attribute_store_modified_pkey; Type: CONSTRAINT; Schema: attribute_directory; Owner: postgres
--

ALTER TABLE ONLY attribute_directory.attribute_store_modified
    ADD CONSTRAINT attribute_store_modified_pkey PRIMARY KEY (attribute_store_id);


--
-- Name: attribute_store attribute_store_pkey; Type: CONSTRAINT; Schema: attribute_directory; Owner: postgres
--

ALTER TABLE ONLY attribute_directory.attribute_store
    ADD CONSTRAINT attribute_store_pkey PRIMARY KEY (id);


--
-- Name: attribute_tag_link attribute_tag_link_pkey; Type: CONSTRAINT; Schema: attribute_directory; Owner: postgres
--

ALTER TABLE ONLY attribute_directory.attribute_tag_link
    ADD CONSTRAINT attribute_tag_link_pkey PRIMARY KEY (attribute_id, tag_id);


--
-- Name: hub_node_curr_ptr hub_node_curr_ptr_pkey; Type: CONSTRAINT; Schema: attribute_history; Owner: minerva_writer
--

ALTER TABLE ONLY attribute_history.hub_node_curr_ptr
    ADD CONSTRAINT hub_node_curr_ptr_pkey PRIMARY KEY (id);


--
-- Name: hub_node hub_node_pkey; Type: CONSTRAINT; Schema: attribute_history; Owner: minerva_writer
--

ALTER TABLE ONLY attribute_history.hub_node
    ADD CONSTRAINT hub_node_pkey PRIMARY KEY (id, entity_id);


--
-- Name: minerva_entity_set_curr_ptr minerva_entity_set_curr_ptr_pkey; Type: CONSTRAINT; Schema: attribute_history; Owner: minerva_writer
--

ALTER TABLE ONLY attribute_history.minerva_entity_set_curr_ptr
    ADD CONSTRAINT minerva_entity_set_curr_ptr_pkey PRIMARY KEY (id);


--
-- Name: minerva_entity_set minerva_entity_set_pkey; Type: CONSTRAINT; Schema: attribute_history; Owner: minerva_writer
--

ALTER TABLE ONLY attribute_history.minerva_entity_set
    ADD CONSTRAINT minerva_entity_set_pkey PRIMARY KEY (id, entity_id);


--
-- Name: data_source data_source_pkey; Type: CONSTRAINT; Schema: directory; Owner: postgres
--

ALTER TABLE ONLY directory.data_source
    ADD CONSTRAINT data_source_pkey PRIMARY KEY (id);


--
-- Name: entity_type entity_type_pkey; Type: CONSTRAINT; Schema: directory; Owner: postgres
--

ALTER TABLE ONLY directory.entity_type
    ADD CONSTRAINT entity_type_pkey PRIMARY KEY (id);


--
-- Name: tag_group tag_group_pkey; Type: CONSTRAINT; Schema: directory; Owner: postgres
--

ALTER TABLE ONLY directory.tag_group
    ADD CONSTRAINT tag_group_pkey PRIMARY KEY (id);


--
-- Name: tag tag_pkey; Type: CONSTRAINT; Schema: directory; Owner: postgres
--

ALTER TABLE ONLY directory.tag
    ADD CONSTRAINT tag_pkey PRIMARY KEY (id);


--
-- Name: entity_set entity_set_name_key; Type: CONSTRAINT; Schema: entity; Owner: postgres
--

ALTER TABLE ONLY entity.entity_set
    ADD CONSTRAINT entity_set_name_key UNIQUE (name);


--
-- Name: node node_name_key; Type: CONSTRAINT; Schema: entity; Owner: postgres
--

ALTER TABLE ONLY entity.node
    ADD CONSTRAINT node_name_key UNIQUE (name);


--
-- Name: v-network v-network_name_key; Type: CONSTRAINT; Schema: entity; Owner: postgres
--

ALTER TABLE ONLY entity."v-network"
    ADD CONSTRAINT "v-network_name_key" UNIQUE (name);


--
-- Name: job job_pkey; Type: CONSTRAINT; Schema: logging; Owner: postgres
--

ALTER TABLE ONLY logging.job
    ADD CONSTRAINT job_pkey PRIMARY KEY (id);


--
-- Name: trigger-notification trigger-notification_pkey; Type: CONSTRAINT; Schema: notification; Owner: minerva_writer
--

ALTER TABLE ONLY notification."trigger-notification"
    ADD CONSTRAINT "trigger-notification_pkey" PRIMARY KEY (id);


--
-- Name: attribute attribute_pkey; Type: CONSTRAINT; Schema: notification_directory; Owner: postgres
--

ALTER TABLE ONLY notification_directory.attribute
    ADD CONSTRAINT attribute_pkey PRIMARY KEY (id);


--
-- Name: last_notification last_notification_pkey; Type: CONSTRAINT; Schema: notification_directory; Owner: postgres
--

ALTER TABLE ONLY notification_directory.last_notification
    ADD CONSTRAINT last_notification_pkey PRIMARY KEY (name, notification_store);


--
-- Name: notification_set_store notification_set_store_pkey; Type: CONSTRAINT; Schema: notification_directory; Owner: postgres
--

ALTER TABLE ONLY notification_directory.notification_set_store
    ADD CONSTRAINT notification_set_store_pkey PRIMARY KEY (id);


--
-- Name: notification_store notification_store_pkey; Type: CONSTRAINT; Schema: notification_directory; Owner: postgres
--

ALTER TABLE ONLY notification_directory.notification_store
    ADD CONSTRAINT notification_store_pkey PRIMARY KEY (id);


--
-- Name: set_attribute set_attribute_pkey; Type: CONSTRAINT; Schema: notification_directory; Owner: postgres
--

ALTER TABLE ONLY notification_directory.set_attribute
    ADD CONSTRAINT set_attribute_pkey PRIMARY KEY (id);


--
-- Name: refinery_schema_history refinery_schema_history_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.refinery_schema_history
    ADD CONSTRAINT refinery_schema_history_pkey PRIMARY KEY (version);


--
-- Name: type type_pkey; Type: CONSTRAINT; Schema: relation_directory; Owner: postgres
--

ALTER TABLE ONLY relation_directory.type
    ADD CONSTRAINT type_pkey PRIMARY KEY (id);


--
-- Name: setting setting_pkey; Type: CONSTRAINT; Schema: system; Owner: postgres
--

ALTER TABLE ONLY system.setting
    ADD CONSTRAINT setting_pkey PRIMARY KEY (id);


--
-- Name: hub-kpi_node_main_15m hub-kpi_node_main_15m_pkey; Type: CONSTRAINT; Schema: trend; Owner: postgres
--

ALTER TABLE ONLY trend."hub-kpi_node_main_15m"
    ADD CONSTRAINT "hub-kpi_node_main_15m_pkey" PRIMARY KEY (entity_id, "timestamp");


--
-- Name: hub-kpi_node_main_15m_staging hub-kpi_node_main_15m_staging_pkey; Type: CONSTRAINT; Schema: trend; Owner: postgres
--

ALTER TABLE ONLY trend."hub-kpi_node_main_15m_staging"
    ADD CONSTRAINT "hub-kpi_node_main_15m_staging_pkey" PRIMARY KEY (entity_id, "timestamp");


--
-- Name: hub_node_main_15m hub_node_main_15m_pkey; Type: CONSTRAINT; Schema: trend; Owner: postgres
--

ALTER TABLE ONLY trend.hub_node_main_15m
    ADD CONSTRAINT hub_node_main_15m_pkey PRIMARY KEY (entity_id, "timestamp");


--
-- Name: hub_node_main_15m_staging hub_node_main_15m_staging_pkey; Type: CONSTRAINT; Schema: trend; Owner: postgres
--

ALTER TABLE ONLY trend.hub_node_main_15m_staging
    ADD CONSTRAINT hub_node_main_15m_staging_pkey PRIMARY KEY (entity_id, "timestamp");


--
-- Name: hub_node_main_1d hub_node_main_1d_pkey; Type: CONSTRAINT; Schema: trend; Owner: postgres
--

ALTER TABLE ONLY trend.hub_node_main_1d
    ADD CONSTRAINT hub_node_main_1d_pkey PRIMARY KEY (entity_id, "timestamp");


--
-- Name: hub_node_main_1d_staging hub_node_main_1d_staging_pkey; Type: CONSTRAINT; Schema: trend; Owner: postgres
--

ALTER TABLE ONLY trend.hub_node_main_1d_staging
    ADD CONSTRAINT hub_node_main_1d_staging_pkey PRIMARY KEY (entity_id, "timestamp");


--
-- Name: hub_node_main_1h hub_node_main_1h_pkey; Type: CONSTRAINT; Schema: trend; Owner: postgres
--

ALTER TABLE ONLY trend.hub_node_main_1h
    ADD CONSTRAINT hub_node_main_1h_pkey PRIMARY KEY (entity_id, "timestamp");


--
-- Name: hub_node_main_1h_staging hub_node_main_1h_staging_pkey; Type: CONSTRAINT; Schema: trend; Owner: postgres
--

ALTER TABLE ONLY trend.hub_node_main_1h_staging
    ADD CONSTRAINT hub_node_main_1h_staging_pkey PRIMARY KEY (entity_id, "timestamp");


--
-- Name: hub_node_main_1month hub_node_main_1month_pkey; Type: CONSTRAINT; Schema: trend; Owner: postgres
--

ALTER TABLE ONLY trend.hub_node_main_1month
    ADD CONSTRAINT hub_node_main_1month_pkey PRIMARY KEY (entity_id, "timestamp");


--
-- Name: hub_node_main_1month_staging hub_node_main_1month_staging_pkey; Type: CONSTRAINT; Schema: trend; Owner: postgres
--

ALTER TABLE ONLY trend.hub_node_main_1month_staging
    ADD CONSTRAINT hub_node_main_1month_staging_pkey PRIMARY KEY (entity_id, "timestamp");


--
-- Name: hub_node_main_1w hub_node_main_1w_pkey; Type: CONSTRAINT; Schema: trend; Owner: postgres
--

ALTER TABLE ONLY trend.hub_node_main_1w
    ADD CONSTRAINT hub_node_main_1w_pkey PRIMARY KEY (entity_id, "timestamp");


--
-- Name: hub_node_main_1w_staging hub_node_main_1w_staging_pkey; Type: CONSTRAINT; Schema: trend; Owner: postgres
--

ALTER TABLE ONLY trend.hub_node_main_1w_staging
    ADD CONSTRAINT hub_node_main_1w_staging_pkey PRIMARY KEY (entity_id, "timestamp");


--
-- Name: function_materialization function_materialization_pkey; Type: CONSTRAINT; Schema: trend_directory; Owner: postgres
--

ALTER TABLE ONLY trend_directory.function_materialization
    ADD CONSTRAINT function_materialization_pkey PRIMARY KEY (id);


--
-- Name: function_materialization_state function_materialization_state_pkey; Type: CONSTRAINT; Schema: trend_directory; Owner: postgres
--

ALTER TABLE ONLY trend_directory.function_materialization_state
    ADD CONSTRAINT function_materialization_state_pkey PRIMARY KEY (materialization_id, "timestamp");


--
-- Name: generated_table_trend generated_table_trend_pkey; Type: CONSTRAINT; Schema: trend_directory; Owner: postgres
--

ALTER TABLE ONLY trend_directory.generated_table_trend
    ADD CONSTRAINT generated_table_trend_pkey PRIMARY KEY (id);


--
-- Name: materialization_metrics materialization_metrics_pkey; Type: CONSTRAINT; Schema: trend_directory; Owner: postgres
--

ALTER TABLE ONLY trend_directory.materialization_metrics
    ADD CONSTRAINT materialization_metrics_pkey PRIMARY KEY (materialization_id);


--
-- Name: materialization materialization_pkey; Type: CONSTRAINT; Schema: trend_directory; Owner: postgres
--

ALTER TABLE ONLY trend_directory.materialization
    ADD CONSTRAINT materialization_pkey PRIMARY KEY (id);


--
-- Name: materialization_state materialization_state_pkey; Type: CONSTRAINT; Schema: trend_directory; Owner: postgres
--

ALTER TABLE ONLY trend_directory.materialization_state
    ADD CONSTRAINT materialization_state_pkey PRIMARY KEY (materialization_id, "timestamp");


--
-- Name: materialization_tag_link materialization_tag_link_pkey; Type: CONSTRAINT; Schema: trend_directory; Owner: postgres
--

ALTER TABLE ONLY trend_directory.materialization_tag_link
    ADD CONSTRAINT materialization_tag_link_pkey PRIMARY KEY (materialization_id, tag_id);


--
-- Name: materialization_trend_store_link materialization_trend_store_link_pkey; Type: CONSTRAINT; Schema: trend_directory; Owner: postgres
--

ALTER TABLE ONLY trend_directory.materialization_trend_store_link
    ADD CONSTRAINT materialization_trend_store_link_pkey PRIMARY KEY (materialization_id, trend_store_part_id);


--
-- Name: modified_log modified_log_pkey; Type: CONSTRAINT; Schema: trend_directory; Owner: postgres
--

ALTER TABLE ONLY trend_directory.modified_log
    ADD CONSTRAINT modified_log_pkey PRIMARY KEY (id);


--
-- Name: modified_log_processing_state modified_log_processing_state_pkey; Type: CONSTRAINT; Schema: trend_directory; Owner: postgres
--

ALTER TABLE ONLY trend_directory.modified_log_processing_state
    ADD CONSTRAINT modified_log_processing_state_pkey PRIMARY KEY (name);


--
-- Name: modified modified_pkey; Type: CONSTRAINT; Schema: trend_directory; Owner: postgres
--

ALTER TABLE ONLY trend_directory.modified
    ADD CONSTRAINT modified_pkey PRIMARY KEY (trend_store_part_id, "timestamp");


--
-- Name: partition partition_pkey; Type: CONSTRAINT; Schema: trend_directory; Owner: postgres
--

ALTER TABLE ONLY trend_directory.partition
    ADD CONSTRAINT partition_pkey PRIMARY KEY (id);


--
-- Name: table_trend table_trend_pkey; Type: CONSTRAINT; Schema: trend_directory; Owner: postgres
--

ALTER TABLE ONLY trend_directory.table_trend
    ADD CONSTRAINT table_trend_pkey PRIMARY KEY (id);


--
-- Name: table_trend_tag_link table_trend_tag_link_pkey; Type: CONSTRAINT; Schema: trend_directory; Owner: postgres
--

ALTER TABLE ONLY trend_directory.table_trend_tag_link
    ADD CONSTRAINT table_trend_tag_link_pkey PRIMARY KEY (table_trend_id, tag_id);


--
-- Name: trend_store_part trend_store_part_pkey; Type: CONSTRAINT; Schema: trend_directory; Owner: postgres
--

ALTER TABLE ONLY trend_directory.trend_store_part
    ADD CONSTRAINT trend_store_part_pkey PRIMARY KEY (id);


--
-- Name: trend_store_part_stats trend_store_part_stats_pkey; Type: CONSTRAINT; Schema: trend_directory; Owner: postgres
--

ALTER TABLE ONLY trend_directory.trend_store_part_stats
    ADD CONSTRAINT trend_store_part_stats_pkey PRIMARY KEY (trend_store_part_id, "timestamp");


--
-- Name: trend_store trend_store_pkey; Type: CONSTRAINT; Schema: trend_directory; Owner: postgres
--

ALTER TABLE ONLY trend_directory.trend_store
    ADD CONSTRAINT trend_store_pkey PRIMARY KEY (id);


--
-- Name: trend_view_part trend_view_part_pkey; Type: CONSTRAINT; Schema: trend_directory; Owner: postgres
--

ALTER TABLE ONLY trend_directory.trend_view_part
    ADD CONSTRAINT trend_view_part_pkey PRIMARY KEY (id);


--
-- Name: trend_view trend_view_pkey; Type: CONSTRAINT; Schema: trend_directory; Owner: postgres
--

ALTER TABLE ONLY trend_directory.trend_view
    ADD CONSTRAINT trend_view_pkey PRIMARY KEY (id);


--
-- Name: view_materialization view_materialization_pkey; Type: CONSTRAINT; Schema: trend_directory; Owner: postgres
--

ALTER TABLE ONLY trend_directory.view_materialization
    ADD CONSTRAINT view_materialization_pkey PRIMARY KEY (id);


--
-- Name: view_trend view_trend_pkey; Type: CONSTRAINT; Schema: trend_directory; Owner: postgres
--

ALTER TABLE ONLY trend_directory.view_trend
    ADD CONSTRAINT view_trend_pkey PRIMARY KEY (id);


--
-- Name: rule rule_pkey; Type: CONSTRAINT; Schema: trigger; Owner: postgres
--

ALTER TABLE ONLY trigger.rule
    ADD CONSTRAINT rule_pkey PRIMARY KEY (id);


--
-- Name: rule_tag_link rule_tag_link_pkey; Type: CONSTRAINT; Schema: trigger; Owner: postgres
--

ALTER TABLE ONLY trigger.rule_tag_link
    ADD CONSTRAINT rule_tag_link_pkey PRIMARY KEY (rule_id, tag_id);


--
-- Name: rule_trend_store_link rule_trend_store_link_pkey; Type: CONSTRAINT; Schema: trigger; Owner: postgres
--

ALTER TABLE ONLY trigger.rule_trend_store_link
    ADD CONSTRAINT rule_trend_store_link_pkey PRIMARY KEY (rule_id, trend_store_part_id);


--
-- Name: alias_type_name_lower_idx; Type: INDEX; Schema: alias_directory; Owner: postgres
--

CREATE UNIQUE INDEX alias_type_name_lower_idx ON alias_directory.alias_type USING btree (name, lower((name)::text));


--
-- Name: attribute_store_uniqueness; Type: INDEX; Schema: attribute_directory; Owner: postgres
--

CREATE UNIQUE INDEX attribute_store_uniqueness ON attribute_directory.attribute_store USING btree (data_source_id, entity_type_id);


--
-- Name: attribute_uniqueness; Type: INDEX; Schema: attribute_directory; Owner: postgres
--

CREATE UNIQUE INDEX attribute_uniqueness ON attribute_directory.attribute USING btree (attribute_store_id, name);


--
-- Name: hub_node_curr_ptr_id_idx; Type: INDEX; Schema: attribute_history; Owner: minerva_writer
--

CREATE INDEX hub_node_curr_ptr_id_idx ON attribute_history.hub_node_curr_ptr USING btree (id);


--
-- Name: hub_node_first_appearance_idx; Type: INDEX; Schema: attribute_history; Owner: minerva_writer
--

CREATE INDEX hub_node_first_appearance_idx ON attribute_history.hub_node USING btree (first_appearance);


--
-- Name: hub_node_id_idx; Type: INDEX; Schema: attribute_history; Owner: minerva_writer
--

CREATE INDEX hub_node_id_idx ON attribute_history.hub_node USING btree (id);


--
-- Name: hub_node_modified_idx; Type: INDEX; Schema: attribute_history; Owner: minerva_writer
--

CREATE INDEX hub_node_modified_idx ON attribute_history.hub_node USING btree (modified);


--
-- Name: minerva_entity_set_curr_ptr_id_idx; Type: INDEX; Schema: attribute_history; Owner: minerva_writer
--

CREATE INDEX minerva_entity_set_curr_ptr_id_idx ON attribute_history.minerva_entity_set_curr_ptr USING btree (id);


--
-- Name: minerva_entity_set_first_appearance_idx; Type: INDEX; Schema: attribute_history; Owner: minerva_writer
--

CREATE INDEX minerva_entity_set_first_appearance_idx ON attribute_history.minerva_entity_set USING btree (first_appearance);


--
-- Name: minerva_entity_set_id_idx; Type: INDEX; Schema: attribute_history; Owner: minerva_writer
--

CREATE INDEX minerva_entity_set_id_idx ON attribute_history.minerva_entity_set USING btree (id);


--
-- Name: minerva_entity_set_modified_idx; Type: INDEX; Schema: attribute_history; Owner: minerva_writer
--

CREATE INDEX minerva_entity_set_modified_idx ON attribute_history.minerva_entity_set USING btree (modified);


--
-- Name: hub_node_entity_id_timestamp_idx; Type: INDEX; Schema: attribute_staging; Owner: minerva_writer
--

CREATE INDEX hub_node_entity_id_timestamp_idx ON attribute_staging.hub_node USING btree (entity_id, "timestamp");


--
-- Name: minerva_entity_set_entity_id_timestamp_idx; Type: INDEX; Schema: attribute_staging; Owner: minerva_writer
--

CREATE INDEX minerva_entity_set_entity_id_timestamp_idx ON attribute_staging.minerva_entity_set USING btree (entity_id, "timestamp");


--
-- Name: ix_directory_data_source_name; Type: INDEX; Schema: directory; Owner: postgres
--

CREATE UNIQUE INDEX ix_directory_data_source_name ON directory.data_source USING btree (name);


--
-- Name: ix_directory_entity_type_name; Type: INDEX; Schema: directory; Owner: postgres
--

CREATE UNIQUE INDEX ix_directory_entity_type_name ON directory.entity_type USING btree (lower((name)::text));


--
-- Name: ix_directory_tag_group_name; Type: INDEX; Schema: directory; Owner: postgres
--

CREATE UNIQUE INDEX ix_directory_tag_group_name ON directory.tag_group USING btree (lower((name)::text));


--
-- Name: ix_directory_tag_name; Type: INDEX; Schema: directory; Owner: postgres
--

CREATE UNIQUE INDEX ix_directory_tag_name ON directory.tag USING btree (lower((name)::text));


--
-- Name: tag_lower_id_idx; Type: INDEX; Schema: directory; Owner: postgres
--

CREATE INDEX tag_lower_id_idx ON directory.tag USING btree (lower((name)::text), id);


--
-- Name: idx_notification_trigger-notification_timestamp; Type: INDEX; Schema: notification; Owner: minerva_writer
--

CREATE INDEX "idx_notification_trigger-notification_timestamp" ON notification."trigger-notification" USING btree ("timestamp");


--
-- Name: uniqueness; Type: INDEX; Schema: notification_directory; Owner: postgres
--

CREATE UNIQUE INDEX uniqueness ON notification_directory.notification_store USING btree (data_source_id);


--
-- Name: node->v-network_source_id_target_id_idx; Type: INDEX; Schema: relation; Owner: postgres
--

CREATE UNIQUE INDEX "node->v-network_source_id_target_id_idx" ON relation."node->v-network" USING btree (source_id, target_id);


--
-- Name: node->v-network_target_id_idx; Type: INDEX; Schema: relation; Owner: postgres
--

CREATE INDEX "node->v-network_target_id_idx" ON relation."node->v-network" USING btree (target_id);


--
-- Name: type_name_key; Type: INDEX; Schema: relation_directory; Owner: postgres
--

CREATE UNIQUE INDEX type_name_key ON relation_directory.type USING btree (name);


--
-- Name: hub-kpi_node_main_15m_job_id_idx; Type: INDEX; Schema: trend; Owner: postgres
--

CREATE INDEX "hub-kpi_node_main_15m_job_id_idx" ON ONLY trend."hub-kpi_node_main_15m" USING btree (job_id);


--
-- Name: hub-kpi_node_main_15m_timestamp_idx; Type: INDEX; Schema: trend; Owner: postgres
--

CREATE INDEX "hub-kpi_node_main_15m_timestamp_idx" ON ONLY trend."hub-kpi_node_main_15m" USING btree ("timestamp");


--
-- Name: hub_node_main_15m_job_id_idx; Type: INDEX; Schema: trend; Owner: postgres
--

CREATE INDEX hub_node_main_15m_job_id_idx ON ONLY trend.hub_node_main_15m USING btree (job_id);


--
-- Name: hub_node_main_15m_timestamp_idx; Type: INDEX; Schema: trend; Owner: postgres
--

CREATE INDEX hub_node_main_15m_timestamp_idx ON ONLY trend.hub_node_main_15m USING btree ("timestamp");


--
-- Name: hub_node_main_1d_job_id_idx; Type: INDEX; Schema: trend; Owner: postgres
--

CREATE INDEX hub_node_main_1d_job_id_idx ON ONLY trend.hub_node_main_1d USING btree (job_id);


--
-- Name: hub_node_main_1d_timestamp_idx; Type: INDEX; Schema: trend; Owner: postgres
--

CREATE INDEX hub_node_main_1d_timestamp_idx ON ONLY trend.hub_node_main_1d USING btree ("timestamp");


--
-- Name: hub_node_main_1h_job_id_idx; Type: INDEX; Schema: trend; Owner: postgres
--

CREATE INDEX hub_node_main_1h_job_id_idx ON ONLY trend.hub_node_main_1h USING btree (job_id);


--
-- Name: hub_node_main_1h_timestamp_idx; Type: INDEX; Schema: trend; Owner: postgres
--

CREATE INDEX hub_node_main_1h_timestamp_idx ON ONLY trend.hub_node_main_1h USING btree ("timestamp");


--
-- Name: hub_node_main_1month_job_id_idx; Type: INDEX; Schema: trend; Owner: postgres
--

CREATE INDEX hub_node_main_1month_job_id_idx ON ONLY trend.hub_node_main_1month USING btree (job_id);


--
-- Name: hub_node_main_1month_timestamp_idx; Type: INDEX; Schema: trend; Owner: postgres
--

CREATE INDEX hub_node_main_1month_timestamp_idx ON ONLY trend.hub_node_main_1month USING btree ("timestamp");


--
-- Name: hub_node_main_1w_job_id_idx; Type: INDEX; Schema: trend; Owner: postgres
--

CREATE INDEX hub_node_main_1w_job_id_idx ON ONLY trend.hub_node_main_1w USING btree (job_id);


--
-- Name: hub_node_main_1w_timestamp_idx; Type: INDEX; Schema: trend; Owner: postgres
--

CREATE INDEX hub_node_main_1w_timestamp_idx ON ONLY trend.hub_node_main_1w USING btree ("timestamp");


--
-- Name: ix_function_materialization_uniqueness; Type: INDEX; Schema: trend_directory; Owner: postgres
--

CREATE UNIQUE INDEX ix_function_materialization_uniqueness ON trend_directory.function_materialization USING btree (materialization_id);


--
-- Name: ix_materialization_uniqueness; Type: INDEX; Schema: trend_directory; Owner: postgres
--

CREATE UNIQUE INDEX ix_materialization_uniqueness ON trend_directory.materialization USING btree (dst_trend_store_part_id);


--
-- Name: ix_view_materialization_uniqueness; Type: INDEX; Schema: trend_directory; Owner: postgres
--

CREATE UNIQUE INDEX ix_view_materialization_uniqueness ON trend_directory.view_materialization USING btree (materialization_id);


--
-- Name: trend_store_unique_constraint; Type: INDEX; Schema: trend_directory; Owner: postgres
--

CREATE UNIQUE INDEX trend_store_unique_constraint ON trend_directory.trend_store USING btree (entity_type_id, data_source_id, granularity);


--
-- Name: trend_view_unique_constraint; Type: INDEX; Schema: trend_directory; Owner: postgres
--

CREATE UNIQUE INDEX trend_view_unique_constraint ON trend_directory.trend_view USING btree (entity_type_id, data_source_id, granularity);


--
-- Name: rule_name_key; Type: INDEX; Schema: trigger; Owner: postgres
--

CREATE UNIQUE INDEX rule_name_key ON trigger.rule USING btree (name);


--
-- Name: attribute update_attribute_type; Type: TRIGGER; Schema: attribute_directory; Owner: postgres
--

CREATE TRIGGER update_attribute_type AFTER UPDATE ON attribute_directory.attribute FOR EACH ROW EXECUTE FUNCTION attribute_directory.update_data_type_on_change();


--
-- Name: function_materialization cleanup_on_function_materialization_delete; Type: TRIGGER; Schema: trend_directory; Owner: postgres
--

CREATE TRIGGER cleanup_on_function_materialization_delete BEFORE DELETE ON trend_directory.function_materialization FOR EACH ROW EXECUTE FUNCTION trend_directory.cleanup_for_function_materialization();


--
-- Name: view_materialization cleanup_on_view_materialization_delete; Type: TRIGGER; Schema: trend_directory; Owner: postgres
--

CREATE TRIGGER cleanup_on_view_materialization_delete BEFORE DELETE ON trend_directory.view_materialization FOR EACH ROW EXECUTE FUNCTION trend_directory.cleanup_for_view_materialization();


--
-- Name: materialization create_metrics_on_new_materialization; Type: TRIGGER; Schema: trend_directory; Owner: postgres
--

CREATE TRIGGER create_metrics_on_new_materialization AFTER INSERT ON trend_directory.materialization FOR EACH ROW EXECUTE FUNCTION trend_directory.create_metrics_for_materialization();


--
-- Name: modified create_stats_on_creation; Type: TRIGGER; Schema: trend_directory; Owner: postgres
--

CREATE TRIGGER create_stats_on_creation AFTER INSERT ON trend_directory.modified FOR EACH ROW EXECUTE FUNCTION trend_directory.create_stats_on_creation();


--
-- Name: modified update_materialization_state_on_new_modified; Type: TRIGGER; Schema: trend_directory; Owner: postgres
--

CREATE TRIGGER update_materialization_state_on_new_modified AFTER INSERT OR UPDATE ON trend_directory.modified FOR EACH ROW EXECUTE FUNCTION trend_directory.new_modified();


--
-- Name: attribute attribute_attribute_attribute_store_id_fkey; Type: FK CONSTRAINT; Schema: attribute_directory; Owner: postgres
--

ALTER TABLE ONLY attribute_directory.attribute
    ADD CONSTRAINT attribute_attribute_attribute_store_id_fkey FOREIGN KEY (attribute_store_id) REFERENCES attribute_directory.attribute_store(id) ON DELETE CASCADE;


--
-- Name: attribute_store attribute_attribute_store_data_source_id_fkey; Type: FK CONSTRAINT; Schema: attribute_directory; Owner: postgres
--

ALTER TABLE ONLY attribute_directory.attribute_store
    ADD CONSTRAINT attribute_attribute_store_data_source_id_fkey FOREIGN KEY (data_source_id) REFERENCES directory.data_source(id);


--
-- Name: attribute_store attribute_attribute_store_entity_type_id_fkey; Type: FK CONSTRAINT; Schema: attribute_directory; Owner: postgres
--

ALTER TABLE ONLY attribute_directory.attribute_store
    ADD CONSTRAINT attribute_attribute_store_entity_type_id_fkey FOREIGN KEY (entity_type_id) REFERENCES directory.entity_type(id) ON DELETE CASCADE;


--
-- Name: attribute_store_compacted attribute_store_compacted_attribute_store_id_fkey; Type: FK CONSTRAINT; Schema: attribute_directory; Owner: postgres
--

ALTER TABLE ONLY attribute_directory.attribute_store_compacted
    ADD CONSTRAINT attribute_store_compacted_attribute_store_id_fkey FOREIGN KEY (attribute_store_id) REFERENCES attribute_directory.attribute_store(id) ON DELETE CASCADE;


--
-- Name: attribute_store_curr_materialized attribute_store_curr_materialized_attribute_store_id_fkey; Type: FK CONSTRAINT; Schema: attribute_directory; Owner: postgres
--

ALTER TABLE ONLY attribute_directory.attribute_store_curr_materialized
    ADD CONSTRAINT attribute_store_curr_materialized_attribute_store_id_fkey FOREIGN KEY (attribute_store_id) REFERENCES attribute_directory.attribute_store(id) ON DELETE CASCADE;


--
-- Name: attribute_store_modified attribute_store_modified_attribute_store_id_fkey; Type: FK CONSTRAINT; Schema: attribute_directory; Owner: postgres
--

ALTER TABLE ONLY attribute_directory.attribute_store_modified
    ADD CONSTRAINT attribute_store_modified_attribute_store_id_fkey FOREIGN KEY (attribute_store_id) REFERENCES attribute_directory.attribute_store(id) ON DELETE CASCADE;


--
-- Name: attribute_tag_link attribute_tag_link_attribute_id_fkey; Type: FK CONSTRAINT; Schema: attribute_directory; Owner: postgres
--

ALTER TABLE ONLY attribute_directory.attribute_tag_link
    ADD CONSTRAINT attribute_tag_link_attribute_id_fkey FOREIGN KEY (attribute_id) REFERENCES attribute_directory.attribute(id) ON DELETE CASCADE;


--
-- Name: attribute_tag_link attribute_tag_link_tag_id_fkey; Type: FK CONSTRAINT; Schema: attribute_directory; Owner: postgres
--

ALTER TABLE ONLY attribute_directory.attribute_tag_link
    ADD CONSTRAINT attribute_tag_link_tag_id_fkey FOREIGN KEY (tag_id) REFERENCES directory.tag(id) ON DELETE CASCADE;


--
-- Name: sampled_view_materialization sampled_view_materialization_attribute_store_id_fkey; Type: FK CONSTRAINT; Schema: attribute_directory; Owner: postgres
--

ALTER TABLE ONLY attribute_directory.sampled_view_materialization
    ADD CONSTRAINT sampled_view_materialization_attribute_store_id_fkey FOREIGN KEY (attribute_store_id) REFERENCES attribute_directory.attribute_store(id) ON DELETE CASCADE;


--
-- Name: tag tag_tag_group_id_fkey; Type: FK CONSTRAINT; Schema: directory; Owner: postgres
--

ALTER TABLE ONLY directory.tag
    ADD CONSTRAINT tag_tag_group_id_fkey FOREIGN KEY (tag_group_id) REFERENCES directory.tag_group(id) ON DELETE CASCADE;


--
-- Name: attribute attribute_notification_store_id_fkey; Type: FK CONSTRAINT; Schema: notification_directory; Owner: postgres
--

ALTER TABLE ONLY notification_directory.attribute
    ADD CONSTRAINT attribute_notification_store_id_fkey FOREIGN KEY (notification_store_id) REFERENCES notification_directory.notification_store(id) ON DELETE CASCADE;


--
-- Name: notification_set_store notification_set_store_notification_store_id_fkey; Type: FK CONSTRAINT; Schema: notification_directory; Owner: postgres
--

ALTER TABLE ONLY notification_directory.notification_set_store
    ADD CONSTRAINT notification_set_store_notification_store_id_fkey FOREIGN KEY (notification_store_id) REFERENCES notification_directory.notification_store(id) ON DELETE CASCADE;


--
-- Name: notification_store notification_store_data_source_id_fkey; Type: FK CONSTRAINT; Schema: notification_directory; Owner: postgres
--

ALTER TABLE ONLY notification_directory.notification_store
    ADD CONSTRAINT notification_store_data_source_id_fkey FOREIGN KEY (data_source_id) REFERENCES directory.data_source(id) ON DELETE CASCADE;


--
-- Name: notification_store notification_store_entity_type_id_fkey; Type: FK CONSTRAINT; Schema: notification_directory; Owner: postgres
--

ALTER TABLE ONLY notification_directory.notification_store
    ADD CONSTRAINT notification_store_entity_type_id_fkey FOREIGN KEY (entity_type_id) REFERENCES directory.entity_type(id) ON DELETE SET NULL;


--
-- Name: set_attribute set_attribute_notification_set_store_id_fkey; Type: FK CONSTRAINT; Schema: notification_directory; Owner: postgres
--

ALTER TABLE ONLY notification_directory.set_attribute
    ADD CONSTRAINT set_attribute_notification_set_store_id_fkey FOREIGN KEY (notification_set_store_id) REFERENCES notification_directory.notification_set_store(id) ON DELETE CASCADE;


--
-- Name: function_materialization function_materialization_materialization_id_fkey; Type: FK CONSTRAINT; Schema: trend_directory; Owner: postgres
--

ALTER TABLE ONLY trend_directory.function_materialization
    ADD CONSTRAINT function_materialization_materialization_id_fkey FOREIGN KEY (materialization_id) REFERENCES trend_directory.materialization(id) ON DELETE CASCADE;


--
-- Name: materialization materialization_dst_trend_store_id_fkey; Type: FK CONSTRAINT; Schema: trend_directory; Owner: postgres
--

ALTER TABLE ONLY trend_directory.materialization
    ADD CONSTRAINT materialization_dst_trend_store_id_fkey FOREIGN KEY (dst_trend_store_part_id) REFERENCES trend_directory.trend_store_part(id) ON DELETE CASCADE;


--
-- Name: materialization_metrics materialization_metrics_materialization_id_fkey; Type: FK CONSTRAINT; Schema: trend_directory; Owner: postgres
--

ALTER TABLE ONLY trend_directory.materialization_metrics
    ADD CONSTRAINT materialization_metrics_materialization_id_fkey FOREIGN KEY (materialization_id) REFERENCES trend_directory.materialization(id) ON DELETE CASCADE;


--
-- Name: materialization_state materialization_state_materialization_id_fkey; Type: FK CONSTRAINT; Schema: trend_directory; Owner: postgres
--

ALTER TABLE ONLY trend_directory.materialization_state
    ADD CONSTRAINT materialization_state_materialization_id_fkey FOREIGN KEY (materialization_id) REFERENCES trend_directory.materialization(id) ON DELETE CASCADE;


--
-- Name: function_materialization_state materialization_state_materialization_id_fkey; Type: FK CONSTRAINT; Schema: trend_directory; Owner: postgres
--

ALTER TABLE ONLY trend_directory.function_materialization_state
    ADD CONSTRAINT materialization_state_materialization_id_fkey FOREIGN KEY (materialization_id) REFERENCES trend_directory.function_materialization(id) ON DELETE CASCADE;


--
-- Name: materialization_tag_link materialization_tag_link_materialization_id_fkey; Type: FK CONSTRAINT; Schema: trend_directory; Owner: postgres
--

ALTER TABLE ONLY trend_directory.materialization_tag_link
    ADD CONSTRAINT materialization_tag_link_materialization_id_fkey FOREIGN KEY (materialization_id) REFERENCES trend_directory.materialization(id) ON DELETE CASCADE;


--
-- Name: materialization_tag_link materialization_tag_link_tag_id_fkey; Type: FK CONSTRAINT; Schema: trend_directory; Owner: postgres
--

ALTER TABLE ONLY trend_directory.materialization_tag_link
    ADD CONSTRAINT materialization_tag_link_tag_id_fkey FOREIGN KEY (tag_id) REFERENCES directory.tag(id) ON DELETE CASCADE;


--
-- Name: materialization_trend_store_link materialization_trend_store_link_materialization_id_fkey; Type: FK CONSTRAINT; Schema: trend_directory; Owner: postgres
--

ALTER TABLE ONLY trend_directory.materialization_trend_store_link
    ADD CONSTRAINT materialization_trend_store_link_materialization_id_fkey FOREIGN KEY (materialization_id) REFERENCES trend_directory.materialization(id) ON DELETE CASCADE;


--
-- Name: materialization_trend_store_link materialization_trend_store_link_trend_store_id_fkey; Type: FK CONSTRAINT; Schema: trend_directory; Owner: postgres
--

ALTER TABLE ONLY trend_directory.materialization_trend_store_link
    ADD CONSTRAINT materialization_trend_store_link_trend_store_id_fkey FOREIGN KEY (trend_store_part_id) REFERENCES trend_directory.trend_store_part(id) ON DELETE CASCADE;


--
-- Name: modified_log modified_trend_store_part_id_fkey; Type: FK CONSTRAINT; Schema: trend_directory; Owner: postgres
--

ALTER TABLE ONLY trend_directory.modified_log
    ADD CONSTRAINT modified_trend_store_part_id_fkey FOREIGN KEY (trend_store_part_id) REFERENCES trend_directory.trend_store_part(id) ON DELETE CASCADE;


--
-- Name: modified modified_trend_store_part_id_fkey; Type: FK CONSTRAINT; Schema: trend_directory; Owner: postgres
--

ALTER TABLE ONLY trend_directory.modified
    ADD CONSTRAINT modified_trend_store_part_id_fkey FOREIGN KEY (trend_store_part_id) REFERENCES trend_directory.trend_store_part(id) ON DELETE CASCADE;


--
-- Name: partition partition_trend_store_part_id_fkey; Type: FK CONSTRAINT; Schema: trend_directory; Owner: postgres
--

ALTER TABLE ONLY trend_directory.partition
    ADD CONSTRAINT partition_trend_store_part_id_fkey FOREIGN KEY (trend_store_part_id) REFERENCES trend_directory.trend_store_part(id) ON DELETE CASCADE;


--
-- Name: table_trend_tag_link table_trend_tag_link_table_trend_id_fkey; Type: FK CONSTRAINT; Schema: trend_directory; Owner: postgres
--

ALTER TABLE ONLY trend_directory.table_trend_tag_link
    ADD CONSTRAINT table_trend_tag_link_table_trend_id_fkey FOREIGN KEY (table_trend_id) REFERENCES trend_directory.table_trend(id) ON DELETE CASCADE;


--
-- Name: table_trend_tag_link table_trend_tag_link_tag_id_fkey; Type: FK CONSTRAINT; Schema: trend_directory; Owner: postgres
--

ALTER TABLE ONLY trend_directory.table_trend_tag_link
    ADD CONSTRAINT table_trend_tag_link_tag_id_fkey FOREIGN KEY (tag_id) REFERENCES directory.tag(id) ON DELETE CASCADE;


--
-- Name: table_trend table_trend_trend_store_part_id_fkey; Type: FK CONSTRAINT; Schema: trend_directory; Owner: postgres
--

ALTER TABLE ONLY trend_directory.table_trend
    ADD CONSTRAINT table_trend_trend_store_part_id_fkey FOREIGN KEY (trend_store_part_id) REFERENCES trend_directory.trend_store_part(id) ON DELETE CASCADE;


--
-- Name: generated_table_trend table_trend_trend_store_part_id_fkey; Type: FK CONSTRAINT; Schema: trend_directory; Owner: postgres
--

ALTER TABLE ONLY trend_directory.generated_table_trend
    ADD CONSTRAINT table_trend_trend_store_part_id_fkey FOREIGN KEY (trend_store_part_id) REFERENCES trend_directory.trend_store_part(id) ON DELETE CASCADE;


--
-- Name: view_trend table_trend_trend_view_part_id_fkey; Type: FK CONSTRAINT; Schema: trend_directory; Owner: postgres
--

ALTER TABLE ONLY trend_directory.view_trend
    ADD CONSTRAINT table_trend_trend_view_part_id_fkey FOREIGN KEY (trend_view_part_id) REFERENCES trend_directory.trend_view_part(id) ON DELETE CASCADE;


--
-- Name: trend_store trend_store_data_source_id_fkey; Type: FK CONSTRAINT; Schema: trend_directory; Owner: postgres
--

ALTER TABLE ONLY trend_directory.trend_store
    ADD CONSTRAINT trend_store_data_source_id_fkey FOREIGN KEY (data_source_id) REFERENCES directory.data_source(id);


--
-- Name: trend_store trend_store_entity_type_id_fkey; Type: FK CONSTRAINT; Schema: trend_directory; Owner: postgres
--

ALTER TABLE ONLY trend_directory.trend_store
    ADD CONSTRAINT trend_store_entity_type_id_fkey FOREIGN KEY (entity_type_id) REFERENCES directory.entity_type(id) ON DELETE CASCADE;


--
-- Name: trend_store_part_stats trend_store_part_stats_trend_store_part_id_fkey; Type: FK CONSTRAINT; Schema: trend_directory; Owner: postgres
--

ALTER TABLE ONLY trend_directory.trend_store_part_stats
    ADD CONSTRAINT trend_store_part_stats_trend_store_part_id_fkey FOREIGN KEY (trend_store_part_id) REFERENCES trend_directory.trend_store_part(id) ON DELETE CASCADE;


--
-- Name: trend_store_part trend_store_part_trend_store_id_fkey; Type: FK CONSTRAINT; Schema: trend_directory; Owner: postgres
--

ALTER TABLE ONLY trend_directory.trend_store_part
    ADD CONSTRAINT trend_store_part_trend_store_id_fkey FOREIGN KEY (trend_store_id) REFERENCES trend_directory.trend_store(id) ON DELETE CASCADE;


--
-- Name: trend_view trend_view_data_source_id_fkey; Type: FK CONSTRAINT; Schema: trend_directory; Owner: postgres
--

ALTER TABLE ONLY trend_directory.trend_view
    ADD CONSTRAINT trend_view_data_source_id_fkey FOREIGN KEY (data_source_id) REFERENCES directory.data_source(id);


--
-- Name: trend_view trend_view_entity_type_id_fkey; Type: FK CONSTRAINT; Schema: trend_directory; Owner: postgres
--

ALTER TABLE ONLY trend_directory.trend_view
    ADD CONSTRAINT trend_view_entity_type_id_fkey FOREIGN KEY (entity_type_id) REFERENCES directory.entity_type(id) ON DELETE CASCADE;


--
-- Name: trend_view_part trend_view_part_trend_view_id_fkey; Type: FK CONSTRAINT; Schema: trend_directory; Owner: postgres
--

ALTER TABLE ONLY trend_directory.trend_view_part
    ADD CONSTRAINT trend_view_part_trend_view_id_fkey FOREIGN KEY (trend_view_id) REFERENCES trend_directory.trend_view(id) ON DELETE CASCADE;


--
-- Name: view_materialization view_materialization_materialization_id_fkey; Type: FK CONSTRAINT; Schema: trend_directory; Owner: postgres
--

ALTER TABLE ONLY trend_directory.view_materialization
    ADD CONSTRAINT view_materialization_materialization_id_fkey FOREIGN KEY (materialization_id) REFERENCES trend_directory.materialization(id) ON DELETE CASCADE;


--
-- Name: rule rule_notification_store_id_fkey; Type: FK CONSTRAINT; Schema: trigger; Owner: postgres
--

ALTER TABLE ONLY trigger.rule
    ADD CONSTRAINT rule_notification_store_id_fkey FOREIGN KEY (notification_store_id) REFERENCES notification_directory.notification_store(id);


--
-- Name: rule_tag_link rule_tag_link_rule_id_fkey; Type: FK CONSTRAINT; Schema: trigger; Owner: postgres
--

ALTER TABLE ONLY trigger.rule_tag_link
    ADD CONSTRAINT rule_tag_link_rule_id_fkey FOREIGN KEY (rule_id) REFERENCES trigger.rule(id) ON DELETE CASCADE;


--
-- Name: rule_tag_link rule_tag_link_tag_id_fkey; Type: FK CONSTRAINT; Schema: trigger; Owner: postgres
--

ALTER TABLE ONLY trigger.rule_tag_link
    ADD CONSTRAINT rule_tag_link_tag_id_fkey FOREIGN KEY (tag_id) REFERENCES directory.tag(id) ON DELETE CASCADE;


--
-- Name: rule_trend_store_link rule_trend_store_link_rule_id_fkey; Type: FK CONSTRAINT; Schema: trigger; Owner: postgres
--

ALTER TABLE ONLY trigger.rule_trend_store_link
    ADD CONSTRAINT rule_trend_store_link_rule_id_fkey FOREIGN KEY (rule_id) REFERENCES trigger.rule(id) ON DELETE CASCADE;


--
-- Name: rule_trend_store_link rule_trend_store_link_trend_store_part_id_fkey; Type: FK CONSTRAINT; Schema: trigger; Owner: postgres
--

ALTER TABLE ONLY trigger.rule_trend_store_link
    ADD CONSTRAINT rule_trend_store_link_trend_store_part_id_fkey FOREIGN KEY (trend_store_part_id) REFERENCES trend_directory.trend_store_part(id) ON DELETE CASCADE;


--
-- Name: SCHEMA alias; Type: ACL; Schema: -; Owner: postgres
--

GRANT ALL ON SCHEMA alias TO minerva_writer;
GRANT USAGE ON SCHEMA alias TO minerva;


--
-- Name: SCHEMA alias_def; Type: ACL; Schema: -; Owner: postgres
--

GRANT ALL ON SCHEMA alias_def TO minerva_writer;
GRANT USAGE ON SCHEMA alias_def TO minerva;


--
-- Name: SCHEMA alias_directory; Type: ACL; Schema: -; Owner: postgres
--

GRANT ALL ON SCHEMA alias_directory TO minerva_writer;
GRANT USAGE ON SCHEMA alias_directory TO minerva;


--
-- Name: SCHEMA attribute; Type: ACL; Schema: -; Owner: postgres
--

GRANT ALL ON SCHEMA attribute TO minerva_writer;
GRANT USAGE ON SCHEMA attribute TO minerva;


--
-- Name: SCHEMA attribute_base; Type: ACL; Schema: -; Owner: postgres
--

GRANT ALL ON SCHEMA attribute_base TO minerva_writer;
GRANT USAGE ON SCHEMA attribute_base TO minerva;


--
-- Name: SCHEMA attribute_directory; Type: ACL; Schema: -; Owner: postgres
--

GRANT ALL ON SCHEMA attribute_directory TO minerva_writer;
GRANT USAGE ON SCHEMA attribute_directory TO minerva;


--
-- Name: SCHEMA attribute_history; Type: ACL; Schema: -; Owner: postgres
--

GRANT ALL ON SCHEMA attribute_history TO minerva_writer;
GRANT USAGE ON SCHEMA attribute_history TO minerva;


--
-- Name: SCHEMA attribute_staging; Type: ACL; Schema: -; Owner: postgres
--

GRANT ALL ON SCHEMA attribute_staging TO minerva_writer;
GRANT USAGE ON SCHEMA attribute_staging TO minerva;


--
-- Name: SCHEMA directory; Type: ACL; Schema: -; Owner: postgres
--

GRANT USAGE ON SCHEMA directory TO minerva;


--
-- Name: SCHEMA entity; Type: ACL; Schema: -; Owner: postgres
--

GRANT ALL ON SCHEMA entity TO minerva_writer;
GRANT USAGE ON SCHEMA entity TO minerva;


--
-- Name: SCHEMA logging; Type: ACL; Schema: -; Owner: postgres
--

GRANT USAGE ON SCHEMA logging TO minerva;


--
-- Name: SCHEMA metric; Type: ACL; Schema: -; Owner: postgres
--

GRANT USAGE ON SCHEMA metric TO minerva;


--
-- Name: SCHEMA notification; Type: ACL; Schema: -; Owner: postgres
--

GRANT ALL ON SCHEMA notification TO minerva_writer;
GRANT USAGE ON SCHEMA notification TO minerva;


--
-- Name: SCHEMA notification_directory; Type: ACL; Schema: -; Owner: postgres
--

GRANT ALL ON SCHEMA notification_directory TO minerva_writer;
GRANT USAGE ON SCHEMA notification_directory TO minerva;


--
-- Name: SCHEMA olap; Type: ACL; Schema: -; Owner: postgres
--

GRANT ALL ON SCHEMA olap TO minerva_writer;
GRANT USAGE ON SCHEMA olap TO minerva;


--
-- Name: SCHEMA relation; Type: ACL; Schema: -; Owner: postgres
--

GRANT ALL ON SCHEMA relation TO minerva_writer;
GRANT USAGE ON SCHEMA relation TO minerva;


--
-- Name: SCHEMA relation_def; Type: ACL; Schema: -; Owner: postgres
--

GRANT ALL ON SCHEMA relation_def TO minerva_writer;
GRANT USAGE ON SCHEMA relation_def TO minerva;


--
-- Name: SCHEMA trend; Type: ACL; Schema: -; Owner: postgres
--

GRANT ALL ON SCHEMA trend TO minerva_writer;
GRANT USAGE ON SCHEMA trend TO minerva;


--
-- Name: SCHEMA trend_directory; Type: ACL; Schema: -; Owner: postgres
--

GRANT USAGE ON SCHEMA trend_directory TO minerva_writer;
GRANT USAGE ON SCHEMA trend_directory TO minerva;


--
-- Name: SCHEMA trend_partition; Type: ACL; Schema: -; Owner: postgres
--

GRANT ALL ON SCHEMA trend_partition TO minerva_writer;
GRANT USAGE ON SCHEMA trend_partition TO minerva;


--
-- Name: SCHEMA trigger; Type: ACL; Schema: -; Owner: postgres
--

GRANT ALL ON SCHEMA trigger TO minerva_writer;
GRANT USAGE ON SCHEMA trigger TO minerva;


--
-- Name: SCHEMA trigger_rule; Type: ACL; Schema: -; Owner: postgres
--

GRANT ALL ON SCHEMA trigger_rule TO minerva_writer;
GRANT USAGE ON SCHEMA trigger_rule TO minerva;


--
-- Name: SCHEMA virtual_entity; Type: ACL; Schema: -; Owner: postgres
--

GRANT USAGE ON SCHEMA virtual_entity TO minerva;


--
-- Name: TABLE attribute_store; Type: ACL; Schema: attribute_directory; Owner: postgres
--

GRANT SELECT ON TABLE attribute_directory.attribute_store TO minerva;
GRANT INSERT,DELETE,UPDATE ON TABLE attribute_directory.attribute_store TO minerva_writer;


--
-- Name: TABLE materialization; Type: ACL; Schema: trend_directory; Owner: postgres
--

GRANT SELECT ON TABLE trend_directory.materialization TO minerva;
GRANT INSERT,DELETE,UPDATE ON TABLE trend_directory.materialization TO minerva_writer;


--
-- Name: TABLE notification_store; Type: ACL; Schema: notification_directory; Owner: postgres
--

GRANT SELECT ON TABLE notification_directory.notification_store TO minerva;
GRANT INSERT,DELETE,UPDATE ON TABLE notification_directory.notification_store TO minerva_writer;


--
-- Name: TABLE trend_store_part; Type: ACL; Schema: trend_directory; Owner: postgres
--

GRANT SELECT ON TABLE trend_directory.trend_store_part TO minerva;
GRANT INSERT,DELETE,UPDATE ON TABLE trend_directory.trend_store_part TO minerva_writer;


--
-- Name: TABLE attribute; Type: ACL; Schema: attribute_directory; Owner: postgres
--

GRANT SELECT ON TABLE attribute_directory.attribute TO minerva;
GRANT INSERT,DELETE,UPDATE ON TABLE attribute_directory.attribute TO minerva_writer;


--
-- Name: TABLE attribute_store_curr_materialized; Type: ACL; Schema: attribute_directory; Owner: postgres
--

GRANT SELECT ON TABLE attribute_directory.attribute_store_curr_materialized TO minerva;
GRANT INSERT,DELETE,UPDATE ON TABLE attribute_directory.attribute_store_curr_materialized TO minerva_writer;


--
-- Name: TABLE attribute_store_modified; Type: ACL; Schema: attribute_directory; Owner: postgres
--

GRANT SELECT ON TABLE attribute_directory.attribute_store_modified TO minerva;
GRANT INSERT,DELETE,UPDATE ON TABLE attribute_directory.attribute_store_modified TO minerva_writer;


--
-- Name: TABLE hub_node; Type: ACL; Schema: attribute_history; Owner: minerva_writer
--

GRANT SELECT ON TABLE attribute_history.hub_node TO minerva;


--
-- Name: TABLE minerva_entity_set; Type: ACL; Schema: attribute_history; Owner: minerva_writer
--

GRANT SELECT ON TABLE attribute_history.minerva_entity_set TO minerva;


--
-- Name: TABLE data_source; Type: ACL; Schema: directory; Owner: postgres
--

GRANT SELECT ON TABLE directory.data_source TO minerva;
GRANT INSERT,DELETE,UPDATE ON TABLE directory.data_source TO minerva_writer;


--
-- Name: TABLE entity_type; Type: ACL; Schema: directory; Owner: postgres
--

GRANT SELECT ON TABLE directory.entity_type TO minerva;
GRANT INSERT,DELETE,UPDATE ON TABLE directory.entity_type TO minerva_writer;


--
-- Name: TABLE entity_set; Type: ACL; Schema: entity; Owner: postgres
--

GRANT SELECT ON TABLE entity.entity_set TO minerva;
GRANT SELECT,INSERT,DELETE,UPDATE ON TABLE entity.entity_set TO minerva_writer;


--
-- Name: TABLE node; Type: ACL; Schema: entity; Owner: postgres
--

GRANT SELECT ON TABLE entity.node TO minerva;
GRANT SELECT,INSERT,DELETE,UPDATE ON TABLE entity.node TO minerva_writer;


--
-- Name: TABLE "v-network"; Type: ACL; Schema: entity; Owner: postgres
--

GRANT SELECT ON TABLE entity."v-network" TO minerva;
GRANT SELECT,INSERT,DELETE,UPDATE ON TABLE entity."v-network" TO minerva_writer;


--
-- Name: TABLE attribute; Type: ACL; Schema: notification_directory; Owner: postgres
--

GRANT SELECT ON TABLE notification_directory.attribute TO minerva;
GRANT INSERT,DELETE,UPDATE ON TABLE notification_directory.attribute TO minerva_writer;


--
-- Name: TABLE notification_set_store; Type: ACL; Schema: notification_directory; Owner: postgres
--

GRANT SELECT ON TABLE notification_directory.notification_set_store TO minerva;
GRANT INSERT,DELETE,UPDATE ON TABLE notification_directory.notification_set_store TO minerva_writer;


--
-- Name: TABLE minerva_entity_set_curr_ptr; Type: ACL; Schema: attribute_history; Owner: minerva_writer
--

GRANT SELECT ON TABLE attribute_history.minerva_entity_set_curr_ptr TO minerva;


--
-- Name: TABLE minerva_entity_set; Type: ACL; Schema: attribute; Owner: minerva_writer
--

GRANT SELECT ON TABLE attribute.minerva_entity_set TO minerva;


--
-- Name: TABLE type; Type: ACL; Schema: relation_directory; Owner: postgres
--

GRANT SELECT ON TABLE relation_directory.type TO minerva;
GRANT INSERT,DELETE,UPDATE ON TABLE relation_directory.type TO minerva_writer;


--
-- Name: TABLE setting; Type: ACL; Schema: system; Owner: postgres
--

GRANT SELECT ON TABLE system.setting TO minerva;
GRANT INSERT,DELETE,UPDATE ON TABLE system.setting TO minerva_writer;


--
-- Name: TABLE table_trend; Type: ACL; Schema: trend_directory; Owner: postgres
--

GRANT SELECT ON TABLE trend_directory.table_trend TO minerva;
GRANT INSERT,DELETE,UPDATE ON TABLE trend_directory.table_trend TO minerva_writer;


--
-- Name: TABLE generated_table_trend; Type: ACL; Schema: trend_directory; Owner: postgres
--

GRANT SELECT ON TABLE trend_directory.generated_table_trend TO minerva;
GRANT INSERT,DELETE,UPDATE ON TABLE trend_directory.generated_table_trend TO minerva_writer;


--
-- Name: TABLE trend_store; Type: ACL; Schema: trend_directory; Owner: postgres
--

GRANT SELECT ON TABLE trend_directory.trend_store TO minerva;
GRANT INSERT,DELETE,UPDATE ON TABLE trend_directory.trend_store TO minerva_writer;


--
-- Name: TABLE partition; Type: ACL; Schema: trend_directory; Owner: postgres
--

GRANT SELECT ON TABLE trend_directory.partition TO minerva;
GRANT INSERT,DELETE,UPDATE ON TABLE trend_directory.partition TO minerva_writer;


--
-- Name: TABLE function_materialization; Type: ACL; Schema: trend_directory; Owner: postgres
--

GRANT SELECT ON TABLE trend_directory.function_materialization TO minerva;
GRANT INSERT,DELETE,UPDATE ON TABLE trend_directory.function_materialization TO minerva_writer;


--
-- Name: TABLE view_materialization; Type: ACL; Schema: trend_directory; Owner: postgres
--

GRANT SELECT ON TABLE trend_directory.view_materialization TO minerva;
GRANT INSERT,DELETE,UPDATE ON TABLE trend_directory.view_materialization TO minerva_writer;


--
-- Name: TABLE rule; Type: ACL; Schema: trigger; Owner: postgres
--

GRANT SELECT ON TABLE trigger.rule TO minerva;
GRANT UPDATE ON TABLE trigger.rule TO minerva_writer;


--
-- Name: TABLE rule_tag_link; Type: ACL; Schema: trigger; Owner: postgres
--

GRANT SELECT ON TABLE trigger.rule_tag_link TO minerva;
GRANT UPDATE ON TABLE trigger.rule_tag_link TO minerva_writer;


--
-- Name: FUNCTION "node/15m/highpowerusage"(timestamp with time zone); Type: ACL; Schema: trigger_rule; Owner: minerva_admin
--

GRANT ALL ON FUNCTION trigger_rule."node/15m/highpowerusage"(timestamp with time zone) TO minerva;


--
-- Name: FUNCTION "node/15m/highpowerusage_add_or_change_threshold_exception"(entity integer, new_start timestamp with time zone, new_expires timestamp with time zone, max_power_new numeric); Type: ACL; Schema: trigger_rule; Owner: postgres
--

GRANT ALL ON FUNCTION trigger_rule."node/15m/highpowerusage_add_or_change_threshold_exception"(entity integer, new_start timestamp with time zone, new_expires timestamp with time zone, max_power_new numeric) TO minerva;


--
-- Name: TABLE "node/15m/highpowerusage_exception_threshold"; Type: ACL; Schema: trigger_rule; Owner: minerva_admin
--

GRANT SELECT ON TABLE trigger_rule."node/15m/highpowerusage_exception_threshold" TO minerva;
GRANT SELECT,INSERT,DELETE,UPDATE ON TABLE trigger_rule."node/15m/highpowerusage_exception_threshold" TO minerva_writer;


--
-- Name: FUNCTION "node/15m/highpowerusage_create_exception_threshold"(entity integer); Type: ACL; Schema: trigger_rule; Owner: postgres
--

GRANT ALL ON FUNCTION trigger_rule."node/15m/highpowerusage_create_exception_threshold"(entity integer) TO minerva;


--
-- Name: FUNCTION "node/15m/highpowerusage_create_notification"(timestamp with time zone); Type: ACL; Schema: trigger_rule; Owner: minerva_admin
--

GRANT ALL ON FUNCTION trigger_rule."node/15m/highpowerusage_create_notification"(timestamp with time zone) TO minerva;


--
-- Name: FUNCTION "node/15m/highpowerusage_fingerprint"(timestamp with time zone); Type: ACL; Schema: trigger_rule; Owner: postgres
--

GRANT ALL ON FUNCTION trigger_rule."node/15m/highpowerusage_fingerprint"(timestamp with time zone) TO minerva;


--
-- Name: FUNCTION "node/15m/highpowerusage_get_exception_threshold"(entity integer); Type: ACL; Schema: trigger_rule; Owner: postgres
--

GRANT ALL ON FUNCTION trigger_rule."node/15m/highpowerusage_get_exception_threshold"(entity integer) TO minerva;


--
-- Name: FUNCTION "node/15m/highpowerusage_get_or_create_exception_threshold"(entity integer); Type: ACL; Schema: trigger_rule; Owner: postgres
--

GRANT ALL ON FUNCTION trigger_rule."node/15m/highpowerusage_get_or_create_exception_threshold"(entity integer) TO minerva;


--
-- Name: FUNCTION "node/15m/highpowerusage_kpi"(timestamp with time zone); Type: ACL; Schema: trigger_rule; Owner: postgres
--

GRANT ALL ON FUNCTION trigger_rule."node/15m/highpowerusage_kpi"(timestamp with time zone) TO minerva;


--
-- Name: FUNCTION "node/15m/highpowerusage_notification_data"(trigger_rule."node/15m/highpowerusage_details"); Type: ACL; Schema: trigger_rule; Owner: minerva_admin
--

GRANT ALL ON FUNCTION trigger_rule."node/15m/highpowerusage_notification_data"(trigger_rule."node/15m/highpowerusage_details") TO minerva;


--
-- Name: FUNCTION "node/15m/highpowerusage_notification_message"(trigger_rule."node/15m/highpowerusage_details"); Type: ACL; Schema: trigger_rule; Owner: minerva_admin
--

GRANT ALL ON FUNCTION trigger_rule."node/15m/highpowerusage_notification_message"(trigger_rule."node/15m/highpowerusage_details") TO minerva;


--
-- Name: FUNCTION "node/15m/highpowerusage_runnable"(timestamp with time zone); Type: ACL; Schema: trigger_rule; Owner: postgres
--

GRANT ALL ON FUNCTION trigger_rule."node/15m/highpowerusage_runnable"(timestamp with time zone) TO minerva;


--
-- Name: FUNCTION "node/15m/highpowerusage_set_thresholds"(max_power numeric); Type: ACL; Schema: trigger_rule; Owner: postgres
--

GRANT ALL ON FUNCTION trigger_rule."node/15m/highpowerusage_set_thresholds"(max_power numeric) TO minerva;


--
-- Name: FUNCTION "node/15m/highpowerusage_weight"(trigger_rule."node/15m/highpowerusage_details"); Type: ACL; Schema: trigger_rule; Owner: minerva_admin
--

GRANT ALL ON FUNCTION trigger_rule."node/15m/highpowerusage_weight"(trigger_rule."node/15m/highpowerusage_details") TO minerva;


--
-- Name: FUNCTION "node/15m/highpowerusage_with_threshold"(timestamp with time zone); Type: ACL; Schema: trigger_rule; Owner: minerva_admin
--

GRANT ALL ON FUNCTION trigger_rule."node/15m/highpowerusage_with_threshold"(timestamp with time zone) TO minerva;


--
-- Name: FUNCTION "node/1d/highpowerusage"(timestamp with time zone); Type: ACL; Schema: trigger_rule; Owner: minerva_admin
--

GRANT ALL ON FUNCTION trigger_rule."node/1d/highpowerusage"(timestamp with time zone) TO minerva;


--
-- Name: FUNCTION "node/1d/highpowerusage_add_or_change_threshold_exception"(entity integer, new_start timestamp with time zone, new_expires timestamp with time zone, max_power_new numeric); Type: ACL; Schema: trigger_rule; Owner: postgres
--

GRANT ALL ON FUNCTION trigger_rule."node/1d/highpowerusage_add_or_change_threshold_exception"(entity integer, new_start timestamp with time zone, new_expires timestamp with time zone, max_power_new numeric) TO minerva;


--
-- Name: TABLE "node/1d/highpowerusage_exception_threshold"; Type: ACL; Schema: trigger_rule; Owner: minerva_admin
--

GRANT SELECT ON TABLE trigger_rule."node/1d/highpowerusage_exception_threshold" TO minerva;
GRANT SELECT,INSERT,DELETE,UPDATE ON TABLE trigger_rule."node/1d/highpowerusage_exception_threshold" TO minerva_writer;


--
-- Name: FUNCTION "node/1d/highpowerusage_create_exception_threshold"(entity integer); Type: ACL; Schema: trigger_rule; Owner: postgres
--

GRANT ALL ON FUNCTION trigger_rule."node/1d/highpowerusage_create_exception_threshold"(entity integer) TO minerva;


--
-- Name: FUNCTION "node/1d/highpowerusage_create_notification"(timestamp with time zone); Type: ACL; Schema: trigger_rule; Owner: minerva_admin
--

GRANT ALL ON FUNCTION trigger_rule."node/1d/highpowerusage_create_notification"(timestamp with time zone) TO minerva;


--
-- Name: FUNCTION "node/1d/highpowerusage_fingerprint"(timestamp with time zone); Type: ACL; Schema: trigger_rule; Owner: postgres
--

GRANT ALL ON FUNCTION trigger_rule."node/1d/highpowerusage_fingerprint"(timestamp with time zone) TO minerva;


--
-- Name: FUNCTION "node/1d/highpowerusage_get_exception_threshold"(entity integer); Type: ACL; Schema: trigger_rule; Owner: postgres
--

GRANT ALL ON FUNCTION trigger_rule."node/1d/highpowerusage_get_exception_threshold"(entity integer) TO minerva;


--
-- Name: FUNCTION "node/1d/highpowerusage_get_or_create_exception_threshold"(entity integer); Type: ACL; Schema: trigger_rule; Owner: postgres
--

GRANT ALL ON FUNCTION trigger_rule."node/1d/highpowerusage_get_or_create_exception_threshold"(entity integer) TO minerva;


--
-- Name: FUNCTION "node/1d/highpowerusage_kpi"(timestamp with time zone); Type: ACL; Schema: trigger_rule; Owner: postgres
--

GRANT ALL ON FUNCTION trigger_rule."node/1d/highpowerusage_kpi"(timestamp with time zone) TO minerva;


--
-- Name: FUNCTION "node/1d/highpowerusage_notification_data"(trigger_rule."node/1d/highpowerusage_details"); Type: ACL; Schema: trigger_rule; Owner: minerva_admin
--

GRANT ALL ON FUNCTION trigger_rule."node/1d/highpowerusage_notification_data"(trigger_rule."node/1d/highpowerusage_details") TO minerva;


--
-- Name: FUNCTION "node/1d/highpowerusage_notification_message"(trigger_rule."node/1d/highpowerusage_details"); Type: ACL; Schema: trigger_rule; Owner: minerva_admin
--

GRANT ALL ON FUNCTION trigger_rule."node/1d/highpowerusage_notification_message"(trigger_rule."node/1d/highpowerusage_details") TO minerva;


--
-- Name: FUNCTION "node/1d/highpowerusage_runnable"(timestamp with time zone); Type: ACL; Schema: trigger_rule; Owner: postgres
--

GRANT ALL ON FUNCTION trigger_rule."node/1d/highpowerusage_runnable"(timestamp with time zone) TO minerva;


--
-- Name: FUNCTION "node/1d/highpowerusage_set_thresholds"(max_power numeric); Type: ACL; Schema: trigger_rule; Owner: postgres
--

GRANT ALL ON FUNCTION trigger_rule."node/1d/highpowerusage_set_thresholds"(max_power numeric) TO minerva;


--
-- Name: FUNCTION "node/1d/highpowerusage_weight"(trigger_rule."node/1d/highpowerusage_details"); Type: ACL; Schema: trigger_rule; Owner: minerva_admin
--

GRANT ALL ON FUNCTION trigger_rule."node/1d/highpowerusage_weight"(trigger_rule."node/1d/highpowerusage_details") TO minerva;


--
-- Name: FUNCTION "node/1d/highpowerusage_with_threshold"(timestamp with time zone); Type: ACL; Schema: trigger_rule; Owner: minerva_admin
--

GRANT ALL ON FUNCTION trigger_rule."node/1d/highpowerusage_with_threshold"(timestamp with time zone) TO minerva;


--
-- Name: FUNCTION "node/1h/highpowerusage"(timestamp with time zone); Type: ACL; Schema: trigger_rule; Owner: minerva_admin
--

GRANT ALL ON FUNCTION trigger_rule."node/1h/highpowerusage"(timestamp with time zone) TO minerva;


--
-- Name: FUNCTION "node/1h/highpowerusage_add_or_change_threshold_exception"(entity integer, new_start timestamp with time zone, new_expires timestamp with time zone, max_power_new numeric); Type: ACL; Schema: trigger_rule; Owner: postgres
--

GRANT ALL ON FUNCTION trigger_rule."node/1h/highpowerusage_add_or_change_threshold_exception"(entity integer, new_start timestamp with time zone, new_expires timestamp with time zone, max_power_new numeric) TO minerva;


--
-- Name: TABLE "node/1h/highpowerusage_exception_threshold"; Type: ACL; Schema: trigger_rule; Owner: minerva_admin
--

GRANT SELECT ON TABLE trigger_rule."node/1h/highpowerusage_exception_threshold" TO minerva;
GRANT SELECT,INSERT,DELETE,UPDATE ON TABLE trigger_rule."node/1h/highpowerusage_exception_threshold" TO minerva_writer;


--
-- Name: FUNCTION "node/1h/highpowerusage_create_exception_threshold"(entity integer); Type: ACL; Schema: trigger_rule; Owner: postgres
--

GRANT ALL ON FUNCTION trigger_rule."node/1h/highpowerusage_create_exception_threshold"(entity integer) TO minerva;


--
-- Name: FUNCTION "node/1h/highpowerusage_create_notification"(timestamp with time zone); Type: ACL; Schema: trigger_rule; Owner: minerva_admin
--

GRANT ALL ON FUNCTION trigger_rule."node/1h/highpowerusage_create_notification"(timestamp with time zone) TO minerva;


--
-- Name: FUNCTION "node/1h/highpowerusage_fingerprint"(timestamp with time zone); Type: ACL; Schema: trigger_rule; Owner: postgres
--

GRANT ALL ON FUNCTION trigger_rule."node/1h/highpowerusage_fingerprint"(timestamp with time zone) TO minerva;


--
-- Name: FUNCTION "node/1h/highpowerusage_get_exception_threshold"(entity integer); Type: ACL; Schema: trigger_rule; Owner: postgres
--

GRANT ALL ON FUNCTION trigger_rule."node/1h/highpowerusage_get_exception_threshold"(entity integer) TO minerva;


--
-- Name: FUNCTION "node/1h/highpowerusage_get_or_create_exception_threshold"(entity integer); Type: ACL; Schema: trigger_rule; Owner: postgres
--

GRANT ALL ON FUNCTION trigger_rule."node/1h/highpowerusage_get_or_create_exception_threshold"(entity integer) TO minerva;


--
-- Name: FUNCTION "node/1h/highpowerusage_kpi"(timestamp with time zone); Type: ACL; Schema: trigger_rule; Owner: postgres
--

GRANT ALL ON FUNCTION trigger_rule."node/1h/highpowerusage_kpi"(timestamp with time zone) TO minerva;


--
-- Name: FUNCTION "node/1h/highpowerusage_notification_data"(trigger_rule."node/1h/highpowerusage_details"); Type: ACL; Schema: trigger_rule; Owner: minerva_admin
--

GRANT ALL ON FUNCTION trigger_rule."node/1h/highpowerusage_notification_data"(trigger_rule."node/1h/highpowerusage_details") TO minerva;


--
-- Name: FUNCTION "node/1h/highpowerusage_notification_message"(trigger_rule."node/1h/highpowerusage_details"); Type: ACL; Schema: trigger_rule; Owner: minerva_admin
--

GRANT ALL ON FUNCTION trigger_rule."node/1h/highpowerusage_notification_message"(trigger_rule."node/1h/highpowerusage_details") TO minerva;


--
-- Name: FUNCTION "node/1h/highpowerusage_runnable"(timestamp with time zone); Type: ACL; Schema: trigger_rule; Owner: postgres
--

GRANT ALL ON FUNCTION trigger_rule."node/1h/highpowerusage_runnable"(timestamp with time zone) TO minerva;


--
-- Name: FUNCTION "node/1h/highpowerusage_set_thresholds"(max_power numeric); Type: ACL; Schema: trigger_rule; Owner: postgres
--

GRANT ALL ON FUNCTION trigger_rule."node/1h/highpowerusage_set_thresholds"(max_power numeric) TO minerva;


--
-- Name: FUNCTION "node/1h/highpowerusage_weight"(trigger_rule."node/1h/highpowerusage_details"); Type: ACL; Schema: trigger_rule; Owner: minerva_admin
--

GRANT ALL ON FUNCTION trigger_rule."node/1h/highpowerusage_weight"(trigger_rule."node/1h/highpowerusage_details") TO minerva;


--
-- Name: FUNCTION "node/1h/highpowerusage_with_threshold"(timestamp with time zone); Type: ACL; Schema: trigger_rule; Owner: minerva_admin
--

GRANT ALL ON FUNCTION trigger_rule."node/1h/highpowerusage_with_threshold"(timestamp with time zone) TO minerva;


--
-- Name: FUNCTION "node/1w/highpowerusage"(timestamp with time zone); Type: ACL; Schema: trigger_rule; Owner: minerva_admin
--

GRANT ALL ON FUNCTION trigger_rule."node/1w/highpowerusage"(timestamp with time zone) TO minerva;


--
-- Name: FUNCTION "node/1w/highpowerusage_add_or_change_threshold_exception"(entity integer, new_start timestamp with time zone, new_expires timestamp with time zone, max_power_new numeric); Type: ACL; Schema: trigger_rule; Owner: postgres
--

GRANT ALL ON FUNCTION trigger_rule."node/1w/highpowerusage_add_or_change_threshold_exception"(entity integer, new_start timestamp with time zone, new_expires timestamp with time zone, max_power_new numeric) TO minerva;


--
-- Name: TABLE "node/1w/highpowerusage_exception_threshold"; Type: ACL; Schema: trigger_rule; Owner: minerva_admin
--

GRANT SELECT ON TABLE trigger_rule."node/1w/highpowerusage_exception_threshold" TO minerva;
GRANT SELECT,INSERT,DELETE,UPDATE ON TABLE trigger_rule."node/1w/highpowerusage_exception_threshold" TO minerva_writer;


--
-- Name: FUNCTION "node/1w/highpowerusage_create_exception_threshold"(entity integer); Type: ACL; Schema: trigger_rule; Owner: postgres
--

GRANT ALL ON FUNCTION trigger_rule."node/1w/highpowerusage_create_exception_threshold"(entity integer) TO minerva;


--
-- Name: FUNCTION "node/1w/highpowerusage_create_notification"(timestamp with time zone); Type: ACL; Schema: trigger_rule; Owner: minerva_admin
--

GRANT ALL ON FUNCTION trigger_rule."node/1w/highpowerusage_create_notification"(timestamp with time zone) TO minerva;


--
-- Name: FUNCTION "node/1w/highpowerusage_fingerprint"(timestamp with time zone); Type: ACL; Schema: trigger_rule; Owner: postgres
--

GRANT ALL ON FUNCTION trigger_rule."node/1w/highpowerusage_fingerprint"(timestamp with time zone) TO minerva;


--
-- Name: FUNCTION "node/1w/highpowerusage_get_exception_threshold"(entity integer); Type: ACL; Schema: trigger_rule; Owner: postgres
--

GRANT ALL ON FUNCTION trigger_rule."node/1w/highpowerusage_get_exception_threshold"(entity integer) TO minerva;


--
-- Name: FUNCTION "node/1w/highpowerusage_get_or_create_exception_threshold"(entity integer); Type: ACL; Schema: trigger_rule; Owner: postgres
--

GRANT ALL ON FUNCTION trigger_rule."node/1w/highpowerusage_get_or_create_exception_threshold"(entity integer) TO minerva;


--
-- Name: FUNCTION "node/1w/highpowerusage_kpi"(timestamp with time zone); Type: ACL; Schema: trigger_rule; Owner: postgres
--

GRANT ALL ON FUNCTION trigger_rule."node/1w/highpowerusage_kpi"(timestamp with time zone) TO minerva;


--
-- Name: FUNCTION "node/1w/highpowerusage_notification_data"(trigger_rule."node/1w/highpowerusage_details"); Type: ACL; Schema: trigger_rule; Owner: minerva_admin
--

GRANT ALL ON FUNCTION trigger_rule."node/1w/highpowerusage_notification_data"(trigger_rule."node/1w/highpowerusage_details") TO minerva;


--
-- Name: FUNCTION "node/1w/highpowerusage_notification_message"(trigger_rule."node/1w/highpowerusage_details"); Type: ACL; Schema: trigger_rule; Owner: minerva_admin
--

GRANT ALL ON FUNCTION trigger_rule."node/1w/highpowerusage_notification_message"(trigger_rule."node/1w/highpowerusage_details") TO minerva;


--
-- Name: FUNCTION "node/1w/highpowerusage_runnable"(timestamp with time zone); Type: ACL; Schema: trigger_rule; Owner: postgres
--

GRANT ALL ON FUNCTION trigger_rule."node/1w/highpowerusage_runnable"(timestamp with time zone) TO minerva;


--
-- Name: FUNCTION "node/1w/highpowerusage_set_thresholds"(max_power numeric); Type: ACL; Schema: trigger_rule; Owner: postgres
--

GRANT ALL ON FUNCTION trigger_rule."node/1w/highpowerusage_set_thresholds"(max_power numeric) TO minerva;


--
-- Name: FUNCTION "node/1w/highpowerusage_weight"(trigger_rule."node/1w/highpowerusage_details"); Type: ACL; Schema: trigger_rule; Owner: minerva_admin
--

GRANT ALL ON FUNCTION trigger_rule."node/1w/highpowerusage_weight"(trigger_rule."node/1w/highpowerusage_details") TO minerva;


--
-- Name: FUNCTION "node/1w/highpowerusage_with_threshold"(timestamp with time zone); Type: ACL; Schema: trigger_rule; Owner: minerva_admin
--

GRANT ALL ON FUNCTION trigger_rule."node/1w/highpowerusage_with_threshold"(timestamp with time zone) TO minerva;


--
-- Name: SEQUENCE alias_type_id_seq; Type: ACL; Schema: alias_directory; Owner: postgres
--

GRANT SELECT,USAGE ON SEQUENCE alias_directory.alias_type_id_seq TO minerva_writer;


--
-- Name: TABLE hub_node_curr_ptr; Type: ACL; Schema: attribute_history; Owner: minerva_writer
--

GRANT SELECT ON TABLE attribute_history.hub_node_curr_ptr TO minerva;


--
-- Name: TABLE hub_node; Type: ACL; Schema: attribute; Owner: minerva_writer
--

GRANT SELECT ON TABLE attribute.hub_node TO minerva;


--
-- Name: TABLE hub_node; Type: ACL; Schema: attribute_base; Owner: minerva_writer
--

GRANT SELECT ON TABLE attribute_base.hub_node TO minerva;


--
-- Name: TABLE minerva_entity_set; Type: ACL; Schema: attribute_base; Owner: minerva_writer
--

GRANT SELECT ON TABLE attribute_base.minerva_entity_set TO minerva;


--
-- Name: SEQUENCE attribute_id_seq; Type: ACL; Schema: attribute_directory; Owner: postgres
--

GRANT SELECT,USAGE ON SEQUENCE attribute_directory.attribute_id_seq TO minerva_writer;


--
-- Name: TABLE attribute_store_compacted; Type: ACL; Schema: attribute_directory; Owner: postgres
--

GRANT SELECT ON TABLE attribute_directory.attribute_store_compacted TO minerva;
GRANT INSERT,DELETE,UPDATE ON TABLE attribute_directory.attribute_store_compacted TO minerva_writer;


--
-- Name: SEQUENCE attribute_store_id_seq; Type: ACL; Schema: attribute_directory; Owner: postgres
--

GRANT SELECT,USAGE ON SEQUENCE attribute_directory.attribute_store_id_seq TO minerva_writer;


--
-- Name: TABLE attribute_tag_link; Type: ACL; Schema: attribute_directory; Owner: postgres
--

GRANT SELECT ON TABLE attribute_directory.attribute_tag_link TO minerva;
GRANT INSERT,DELETE,UPDATE ON TABLE attribute_directory.attribute_tag_link TO minerva_writer;


--
-- Name: TABLE dependencies; Type: ACL; Schema: attribute_directory; Owner: postgres
--

GRANT SELECT ON TABLE attribute_directory.dependencies TO minerva;


--
-- Name: SEQUENCE sampled_view_materialization_id_seq; Type: ACL; Schema: attribute_directory; Owner: postgres
--

GRANT SELECT,USAGE ON SEQUENCE attribute_directory.sampled_view_materialization_id_seq TO minerva_writer;


--
-- Name: TABLE hub_node_curr_selection; Type: ACL; Schema: attribute_history; Owner: minerva_writer
--

GRANT SELECT ON TABLE attribute_history.hub_node_curr_selection TO minerva;


--
-- Name: TABLE minerva_entity_set_changes; Type: ACL; Schema: attribute_history; Owner: minerva_writer
--

GRANT SELECT ON TABLE attribute_history.minerva_entity_set_changes TO minerva;


--
-- Name: TABLE minerva_entity_set_curr_selection; Type: ACL; Schema: attribute_history; Owner: minerva_writer
--

GRANT SELECT ON TABLE attribute_history.minerva_entity_set_curr_selection TO minerva;


--
-- Name: TABLE hub_node; Type: ACL; Schema: attribute_staging; Owner: minerva_writer
--

GRANT SELECT ON TABLE attribute_staging.hub_node TO minerva;


--
-- Name: TABLE hub_node_modified; Type: ACL; Schema: attribute_staging; Owner: minerva_writer
--

GRANT SELECT ON TABLE attribute_staging.hub_node_modified TO minerva;


--
-- Name: TABLE hub_node_new; Type: ACL; Schema: attribute_staging; Owner: minerva_writer
--

GRANT SELECT ON TABLE attribute_staging.hub_node_new TO minerva;


--
-- Name: TABLE minerva_entity_set; Type: ACL; Schema: attribute_staging; Owner: minerva_writer
--

GRANT SELECT ON TABLE attribute_staging.minerva_entity_set TO minerva;


--
-- Name: TABLE minerva_entity_set_modified; Type: ACL; Schema: attribute_staging; Owner: minerva_writer
--

GRANT SELECT ON TABLE attribute_staging.minerva_entity_set_modified TO minerva;


--
-- Name: TABLE minerva_entity_set_new; Type: ACL; Schema: attribute_staging; Owner: minerva_writer
--

GRANT SELECT ON TABLE attribute_staging.minerva_entity_set_new TO minerva;


--
-- Name: SEQUENCE data_source_id_seq; Type: ACL; Schema: directory; Owner: postgres
--

GRANT SELECT,USAGE ON SEQUENCE directory.data_source_id_seq TO minerva_writer;


--
-- Name: SEQUENCE entity_type_id_seq; Type: ACL; Schema: directory; Owner: postgres
--

GRANT SELECT,USAGE ON SEQUENCE directory.entity_type_id_seq TO minerva_writer;


--
-- Name: TABLE tag; Type: ACL; Schema: directory; Owner: postgres
--

GRANT SELECT ON TABLE directory.tag TO minerva;
GRANT INSERT,DELETE,UPDATE ON TABLE directory.tag TO minerva_writer;


--
-- Name: TABLE tag_group; Type: ACL; Schema: directory; Owner: postgres
--

GRANT SELECT ON TABLE directory.tag_group TO minerva;
GRANT INSERT,DELETE,UPDATE ON TABLE directory.tag_group TO minerva_writer;


--
-- Name: SEQUENCE tag_group_id_seq; Type: ACL; Schema: directory; Owner: postgres
--

GRANT SELECT,USAGE ON SEQUENCE directory.tag_group_id_seq TO minerva_writer;


--
-- Name: SEQUENCE tag_id_seq; Type: ACL; Schema: directory; Owner: postgres
--

GRANT SELECT,USAGE ON SEQUENCE directory.tag_id_seq TO minerva_writer;


--
-- Name: SEQUENCE entity_set_id_seq; Type: ACL; Schema: entity; Owner: postgres
--

GRANT SELECT,USAGE ON SEQUENCE entity.entity_set_id_seq TO minerva_writer;


--
-- Name: SEQUENCE node_id_seq; Type: ACL; Schema: entity; Owner: postgres
--

GRANT SELECT,USAGE ON SEQUENCE entity.node_id_seq TO minerva_writer;


--
-- Name: SEQUENCE "v-network_id_seq"; Type: ACL; Schema: entity; Owner: postgres
--

GRANT SELECT,USAGE ON SEQUENCE entity."v-network_id_seq" TO minerva_writer;


--
-- Name: TABLE job; Type: ACL; Schema: logging; Owner: postgres
--

GRANT SELECT ON TABLE logging.job TO minerva;
GRANT INSERT,DELETE,UPDATE ON TABLE logging.job TO minerva_writer;


--
-- Name: SEQUENCE job_id_seq; Type: ACL; Schema: logging; Owner: postgres
--

GRANT SELECT,USAGE ON SEQUENCE logging.job_id_seq TO minerva_writer;


--
-- Name: TABLE "trigger-notification"; Type: ACL; Schema: notification; Owner: minerva_writer
--

GRANT SELECT ON TABLE notification."trigger-notification" TO minerva;


--
-- Name: TABLE "trigger-notification_staging"; Type: ACL; Schema: notification; Owner: minerva_writer
--

GRANT SELECT ON TABLE notification."trigger-notification_staging" TO minerva;


--
-- Name: SEQUENCE attribute_id_seq; Type: ACL; Schema: notification_directory; Owner: postgres
--

GRANT SELECT,USAGE ON SEQUENCE notification_directory.attribute_id_seq TO minerva_writer;


--
-- Name: SEQUENCE notification_set_store_id_seq; Type: ACL; Schema: notification_directory; Owner: postgres
--

GRANT SELECT,USAGE ON SEQUENCE notification_directory.notification_set_store_id_seq TO minerva_writer;


--
-- Name: SEQUENCE notification_store_id_seq; Type: ACL; Schema: notification_directory; Owner: postgres
--

GRANT SELECT,USAGE ON SEQUENCE notification_directory.notification_store_id_seq TO minerva_writer;


--
-- Name: SEQUENCE notification_store_id_seq1; Type: ACL; Schema: notification_directory; Owner: postgres
--

GRANT SELECT,USAGE ON SEQUENCE notification_directory.notification_store_id_seq1 TO minerva_writer;


--
-- Name: TABLE set_attribute; Type: ACL; Schema: notification_directory; Owner: postgres
--

GRANT SELECT ON TABLE notification_directory.set_attribute TO minerva;
GRANT INSERT,DELETE,UPDATE ON TABLE notification_directory.set_attribute TO minerva_writer;


--
-- Name: SEQUENCE set_attribute_id_seq; Type: ACL; Schema: notification_directory; Owner: postgres
--

GRANT SELECT,USAGE ON SEQUENCE notification_directory.set_attribute_id_seq TO minerva_writer;


--
-- Name: TABLE "node->v-network"; Type: ACL; Schema: relation; Owner: postgres
--

GRANT SELECT ON TABLE relation."node->v-network" TO minerva;
GRANT SELECT,INSERT,DELETE,UPDATE ON TABLE relation."node->v-network" TO minerva_writer;


--
-- Name: TABLE "node->v-network"; Type: ACL; Schema: relation_def; Owner: postgres
--

GRANT SELECT ON TABLE relation_def."node->v-network" TO minerva;
GRANT SELECT,INSERT,DELETE,UPDATE ON TABLE relation_def."node->v-network" TO minerva_writer;


--
-- Name: SEQUENCE type_id_seq; Type: ACL; Schema: relation_directory; Owner: postgres
--

GRANT SELECT,USAGE ON SEQUENCE relation_directory.type_id_seq TO minerva_writer;


--
-- Name: TABLE hub_node_main_15m; Type: ACL; Schema: trend; Owner: postgres
--

GRANT SELECT ON TABLE trend.hub_node_main_15m TO minerva;
GRANT SELECT,INSERT,DELETE,UPDATE ON TABLE trend.hub_node_main_15m TO minerva_writer;


--
-- Name: TABLE "_hub-kpi_node_main_15m"; Type: ACL; Schema: trend; Owner: postgres
--

GRANT SELECT ON TABLE trend."_hub-kpi_node_main_15m" TO minerva;
GRANT SELECT,INSERT,DELETE,UPDATE ON TABLE trend."_hub-kpi_node_main_15m" TO minerva_writer;


--
-- Name: TABLE "hub-kpi_node_main_15m"; Type: ACL; Schema: trend; Owner: postgres
--

GRANT SELECT ON TABLE trend."hub-kpi_node_main_15m" TO minerva;
GRANT SELECT,INSERT,DELETE,UPDATE ON TABLE trend."hub-kpi_node_main_15m" TO minerva_writer;


--
-- Name: TABLE "hub-kpi_node_main_15m_staging"; Type: ACL; Schema: trend; Owner: postgres
--

GRANT SELECT ON TABLE trend."hub-kpi_node_main_15m_staging" TO minerva;
GRANT SELECT,INSERT,DELETE,UPDATE ON TABLE trend."hub-kpi_node_main_15m_staging" TO minerva_writer;


--
-- Name: TABLE hub_node_main_15m_staging; Type: ACL; Schema: trend; Owner: postgres
--

GRANT SELECT ON TABLE trend.hub_node_main_15m_staging TO minerva;
GRANT SELECT,INSERT,DELETE,UPDATE ON TABLE trend.hub_node_main_15m_staging TO minerva_writer;


--
-- Name: TABLE hub_node_main_1d; Type: ACL; Schema: trend; Owner: postgres
--

GRANT SELECT ON TABLE trend.hub_node_main_1d TO minerva;
GRANT SELECT,INSERT,DELETE,UPDATE ON TABLE trend.hub_node_main_1d TO minerva_writer;


--
-- Name: TABLE hub_node_main_1d_staging; Type: ACL; Schema: trend; Owner: postgres
--

GRANT SELECT ON TABLE trend.hub_node_main_1d_staging TO minerva;
GRANT SELECT,INSERT,DELETE,UPDATE ON TABLE trend.hub_node_main_1d_staging TO minerva_writer;


--
-- Name: TABLE hub_node_main_1h; Type: ACL; Schema: trend; Owner: postgres
--

GRANT SELECT ON TABLE trend.hub_node_main_1h TO minerva;
GRANT SELECT,INSERT,DELETE,UPDATE ON TABLE trend.hub_node_main_1h TO minerva_writer;


--
-- Name: TABLE hub_node_main_1h_staging; Type: ACL; Schema: trend; Owner: postgres
--

GRANT SELECT ON TABLE trend.hub_node_main_1h_staging TO minerva;
GRANT SELECT,INSERT,DELETE,UPDATE ON TABLE trend.hub_node_main_1h_staging TO minerva_writer;


--
-- Name: TABLE hub_node_main_1month; Type: ACL; Schema: trend; Owner: postgres
--

GRANT SELECT ON TABLE trend.hub_node_main_1month TO minerva;
GRANT SELECT,INSERT,DELETE,UPDATE ON TABLE trend.hub_node_main_1month TO minerva_writer;


--
-- Name: TABLE hub_node_main_1month_staging; Type: ACL; Schema: trend; Owner: postgres
--

GRANT SELECT ON TABLE trend.hub_node_main_1month_staging TO minerva;
GRANT SELECT,INSERT,DELETE,UPDATE ON TABLE trend.hub_node_main_1month_staging TO minerva_writer;


--
-- Name: TABLE hub_node_main_1w; Type: ACL; Schema: trend; Owner: postgres
--

GRANT SELECT ON TABLE trend.hub_node_main_1w TO minerva;
GRANT SELECT,INSERT,DELETE,UPDATE ON TABLE trend.hub_node_main_1w TO minerva_writer;


--
-- Name: TABLE hub_node_main_1w_staging; Type: ACL; Schema: trend; Owner: postgres
--

GRANT SELECT ON TABLE trend.hub_node_main_1w_staging TO minerva;
GRANT SELECT,INSERT,DELETE,UPDATE ON TABLE trend.hub_node_main_1w_staging TO minerva_writer;


--
-- Name: TABLE "hub_v-network_main_15m"; Type: ACL; Schema: trend; Owner: postgres
--

GRANT SELECT ON TABLE trend."hub_v-network_main_15m" TO minerva;
GRANT SELECT,INSERT,DELETE,UPDATE ON TABLE trend."hub_v-network_main_15m" TO minerva_writer;


--
-- Name: TABLE "hub_v-network_main_1d"; Type: ACL; Schema: trend; Owner: postgres
--

GRANT SELECT ON TABLE trend."hub_v-network_main_1d" TO minerva;
GRANT SELECT,INSERT,DELETE,UPDATE ON TABLE trend."hub_v-network_main_1d" TO minerva_writer;


--
-- Name: TABLE "hub_v-network_main_1h"; Type: ACL; Schema: trend; Owner: postgres
--

GRANT SELECT ON TABLE trend."hub_v-network_main_1h" TO minerva;
GRANT SELECT,INSERT,DELETE,UPDATE ON TABLE trend."hub_v-network_main_1h" TO minerva_writer;


--
-- Name: TABLE "hub_v-network_main_1month"; Type: ACL; Schema: trend; Owner: postgres
--

GRANT SELECT ON TABLE trend."hub_v-network_main_1month" TO minerva;
GRANT SELECT,INSERT,DELETE,UPDATE ON TABLE trend."hub_v-network_main_1month" TO minerva_writer;


--
-- Name: TABLE "hub_v-network_main_1w"; Type: ACL; Schema: trend; Owner: postgres
--

GRANT SELECT ON TABLE trend."hub_v-network_main_1w" TO minerva;
GRANT SELECT,INSERT,DELETE,UPDATE ON TABLE trend."hub_v-network_main_1w" TO minerva_writer;


--
-- Name: TABLE power_report; Type: ACL; Schema: trend; Owner: postgres
--

GRANT SELECT ON TABLE trend.power_report TO minerva;
GRANT SELECT,INSERT,DELETE,UPDATE ON TABLE trend.power_report TO minerva_writer;


--
-- Name: SEQUENCE function_materialization_id_seq; Type: ACL; Schema: trend_directory; Owner: postgres
--

GRANT SELECT,USAGE ON SEQUENCE trend_directory.function_materialization_id_seq TO minerva_writer;


--
-- Name: TABLE function_materialization_state; Type: ACL; Schema: trend_directory; Owner: postgres
--

GRANT SELECT ON TABLE trend_directory.function_materialization_state TO minerva;
GRANT INSERT,DELETE,UPDATE ON TABLE trend_directory.function_materialization_state TO minerva_writer;


--
-- Name: SEQUENCE generated_table_trend_id_seq; Type: ACL; Schema: trend_directory; Owner: postgres
--

GRANT SELECT,USAGE ON SEQUENCE trend_directory.generated_table_trend_id_seq TO minerva_writer;


--
-- Name: SEQUENCE materialization_id_seq; Type: ACL; Schema: trend_directory; Owner: postgres
--

GRANT SELECT,USAGE ON SEQUENCE trend_directory.materialization_id_seq TO minerva_writer;


--
-- Name: TABLE materialization_metrics; Type: ACL; Schema: trend_directory; Owner: postgres
--

GRANT SELECT ON TABLE trend_directory.materialization_metrics TO minerva;
GRANT INSERT,DELETE,UPDATE ON TABLE trend_directory.materialization_metrics TO minerva_writer;


--
-- Name: TABLE materialization_state; Type: ACL; Schema: trend_directory; Owner: postgres
--

GRANT SELECT ON TABLE trend_directory.materialization_state TO minerva;
GRANT INSERT,DELETE,UPDATE ON TABLE trend_directory.materialization_state TO minerva_writer;


--
-- Name: TABLE materialization_tag_link; Type: ACL; Schema: trend_directory; Owner: postgres
--

GRANT SELECT ON TABLE trend_directory.materialization_tag_link TO minerva;
GRANT INSERT,DELETE,UPDATE ON TABLE trend_directory.materialization_tag_link TO minerva_writer;


--
-- Name: TABLE materialization_trend_store_link; Type: ACL; Schema: trend_directory; Owner: postgres
--

GRANT SELECT ON TABLE trend_directory.materialization_trend_store_link TO minerva;
GRANT INSERT,DELETE,UPDATE ON TABLE trend_directory.materialization_trend_store_link TO minerva_writer;


--
-- Name: TABLE modified; Type: ACL; Schema: trend_directory; Owner: postgres
--

GRANT SELECT ON TABLE trend_directory.modified TO minerva;
GRANT INSERT,DELETE,UPDATE ON TABLE trend_directory.modified TO minerva_writer;


--
-- Name: TABLE modified_log; Type: ACL; Schema: trend_directory; Owner: postgres
--

GRANT SELECT ON TABLE trend_directory.modified_log TO minerva;
GRANT INSERT,DELETE,UPDATE ON TABLE trend_directory.modified_log TO minerva_writer;


--
-- Name: SEQUENCE modified_log_id_seq; Type: ACL; Schema: trend_directory; Owner: postgres
--

GRANT SELECT,USAGE ON SEQUENCE trend_directory.modified_log_id_seq TO minerva_writer;


--
-- Name: TABLE modified_log_processing_state; Type: ACL; Schema: trend_directory; Owner: postgres
--

GRANT SELECT ON TABLE trend_directory.modified_log_processing_state TO minerva;
GRANT INSERT,DELETE,UPDATE ON TABLE trend_directory.modified_log_processing_state TO minerva_writer;


--
-- Name: SEQUENCE partition_id_seq; Type: ACL; Schema: trend_directory; Owner: postgres
--

GRANT SELECT,USAGE ON SEQUENCE trend_directory.partition_id_seq TO minerva_writer;


--
-- Name: SEQUENCE table_trend_id_seq; Type: ACL; Schema: trend_directory; Owner: postgres
--

GRANT SELECT,USAGE ON SEQUENCE trend_directory.table_trend_id_seq TO minerva_writer;


--
-- Name: SEQUENCE trend_store_id_seq; Type: ACL; Schema: trend_directory; Owner: postgres
--

GRANT SELECT,USAGE ON SEQUENCE trend_directory.trend_store_id_seq TO minerva_writer;


--
-- Name: SEQUENCE trend_store_part_id_seq; Type: ACL; Schema: trend_directory; Owner: postgres
--

GRANT SELECT,USAGE ON SEQUENCE trend_directory.trend_store_part_id_seq TO minerva_writer;


--
-- Name: TABLE trend_store_part_stats; Type: ACL; Schema: trend_directory; Owner: postgres
--

GRANT SELECT ON TABLE trend_directory.trend_store_part_stats TO minerva;
GRANT INSERT,DELETE,UPDATE ON TABLE trend_directory.trend_store_part_stats TO minerva_writer;


--
-- Name: SEQUENCE trend_view_id_seq; Type: ACL; Schema: trend_directory; Owner: postgres
--

GRANT SELECT,USAGE ON SEQUENCE trend_directory.trend_view_id_seq TO minerva_writer;


--
-- Name: SEQUENCE trend_view_part_id_seq; Type: ACL; Schema: trend_directory; Owner: postgres
--

GRANT SELECT,USAGE ON SEQUENCE trend_directory.trend_view_part_id_seq TO minerva_writer;


--
-- Name: SEQUENCE view_materialization_id_seq; Type: ACL; Schema: trend_directory; Owner: postgres
--

GRANT SELECT,USAGE ON SEQUENCE trend_directory.view_materialization_id_seq TO minerva_writer;


--
-- Name: TABLE view_trend; Type: ACL; Schema: trend_directory; Owner: postgres
--

GRANT SELECT ON TABLE trend_directory.view_trend TO minerva;
GRANT INSERT,DELETE,UPDATE ON TABLE trend_directory.view_trend TO minerva_writer;


--
-- Name: SEQUENCE view_trend_id_seq; Type: ACL; Schema: trend_directory; Owner: postgres
--

GRANT SELECT,USAGE ON SEQUENCE trend_directory.view_trend_id_seq TO minerva_writer;


--
-- Name: TABLE exception_base; Type: ACL; Schema: trigger; Owner: postgres
--

GRANT SELECT ON TABLE trigger.exception_base TO minerva;
GRANT UPDATE ON TABLE trigger.exception_base TO minerva_writer;


--
-- Name: SEQUENCE exception_base_id_seq; Type: ACL; Schema: trigger; Owner: postgres
--

GRANT SELECT,USAGE ON SEQUENCE trigger.exception_base_id_seq TO minerva_writer;


--
-- Name: SEQUENCE rule_id_seq; Type: ACL; Schema: trigger; Owner: postgres
--

GRANT SELECT,USAGE ON SEQUENCE trigger.rule_id_seq TO minerva_writer;


--
-- Name: SEQUENCE "node/15m/highpowerusage_exception_threshold_id_seq"; Type: ACL; Schema: trigger_rule; Owner: minerva_admin
--

GRANT SELECT,USAGE ON SEQUENCE trigger_rule."node/15m/highpowerusage_exception_threshold_id_seq" TO minerva_writer;


--
-- Name: TABLE "node/15m/highpowerusage_exception_weight"; Type: ACL; Schema: trigger_rule; Owner: minerva_admin
--

GRANT SELECT ON TABLE trigger_rule."node/15m/highpowerusage_exception_weight" TO minerva;
GRANT SELECT,INSERT,DELETE,UPDATE ON TABLE trigger_rule."node/15m/highpowerusage_exception_weight" TO minerva_writer;


--
-- Name: SEQUENCE "node/15m/highpowerusage_exception_weight_id_seq"; Type: ACL; Schema: trigger_rule; Owner: minerva_admin
--

GRANT SELECT,USAGE ON SEQUENCE trigger_rule."node/15m/highpowerusage_exception_weight_id_seq" TO minerva_writer;


--
-- Name: TABLE "node/15m/highpowerusage_threshold"; Type: ACL; Schema: trigger_rule; Owner: minerva_admin
--

GRANT SELECT ON TABLE trigger_rule."node/15m/highpowerusage_threshold" TO minerva;
GRANT SELECT,INSERT,DELETE,UPDATE ON TABLE trigger_rule."node/15m/highpowerusage_threshold" TO minerva_writer;


--
-- Name: SEQUENCE "node/1d/highpowerusage_exception_threshold_id_seq"; Type: ACL; Schema: trigger_rule; Owner: minerva_admin
--

GRANT SELECT,USAGE ON SEQUENCE trigger_rule."node/1d/highpowerusage_exception_threshold_id_seq" TO minerva_writer;


--
-- Name: TABLE "node/1d/highpowerusage_exception_weight"; Type: ACL; Schema: trigger_rule; Owner: minerva_admin
--

GRANT SELECT ON TABLE trigger_rule."node/1d/highpowerusage_exception_weight" TO minerva;
GRANT SELECT,INSERT,DELETE,UPDATE ON TABLE trigger_rule."node/1d/highpowerusage_exception_weight" TO minerva_writer;


--
-- Name: SEQUENCE "node/1d/highpowerusage_exception_weight_id_seq"; Type: ACL; Schema: trigger_rule; Owner: minerva_admin
--

GRANT SELECT,USAGE ON SEQUENCE trigger_rule."node/1d/highpowerusage_exception_weight_id_seq" TO minerva_writer;


--
-- Name: TABLE "node/1d/highpowerusage_threshold"; Type: ACL; Schema: trigger_rule; Owner: minerva_admin
--

GRANT SELECT ON TABLE trigger_rule."node/1d/highpowerusage_threshold" TO minerva;
GRANT SELECT,INSERT,DELETE,UPDATE ON TABLE trigger_rule."node/1d/highpowerusage_threshold" TO minerva_writer;


--
-- Name: SEQUENCE "node/1h/highpowerusage_exception_threshold_id_seq"; Type: ACL; Schema: trigger_rule; Owner: minerva_admin
--

GRANT SELECT,USAGE ON SEQUENCE trigger_rule."node/1h/highpowerusage_exception_threshold_id_seq" TO minerva_writer;


--
-- Name: TABLE "node/1h/highpowerusage_exception_weight"; Type: ACL; Schema: trigger_rule; Owner: minerva_admin
--

GRANT SELECT ON TABLE trigger_rule."node/1h/highpowerusage_exception_weight" TO minerva;
GRANT SELECT,INSERT,DELETE,UPDATE ON TABLE trigger_rule."node/1h/highpowerusage_exception_weight" TO minerva_writer;


--
-- Name: SEQUENCE "node/1h/highpowerusage_exception_weight_id_seq"; Type: ACL; Schema: trigger_rule; Owner: minerva_admin
--

GRANT SELECT,USAGE ON SEQUENCE trigger_rule."node/1h/highpowerusage_exception_weight_id_seq" TO minerva_writer;


--
-- Name: TABLE "node/1h/highpowerusage_threshold"; Type: ACL; Schema: trigger_rule; Owner: minerva_admin
--

GRANT SELECT ON TABLE trigger_rule."node/1h/highpowerusage_threshold" TO minerva;
GRANT SELECT,INSERT,DELETE,UPDATE ON TABLE trigger_rule."node/1h/highpowerusage_threshold" TO minerva_writer;


--
-- Name: SEQUENCE "node/1w/highpowerusage_exception_threshold_id_seq"; Type: ACL; Schema: trigger_rule; Owner: minerva_admin
--

GRANT SELECT,USAGE ON SEQUENCE trigger_rule."node/1w/highpowerusage_exception_threshold_id_seq" TO minerva_writer;


--
-- Name: TABLE "node/1w/highpowerusage_exception_weight"; Type: ACL; Schema: trigger_rule; Owner: minerva_admin
--

GRANT SELECT ON TABLE trigger_rule."node/1w/highpowerusage_exception_weight" TO minerva;
GRANT SELECT,INSERT,DELETE,UPDATE ON TABLE trigger_rule."node/1w/highpowerusage_exception_weight" TO minerva_writer;


--
-- Name: SEQUENCE "node/1w/highpowerusage_exception_weight_id_seq"; Type: ACL; Schema: trigger_rule; Owner: minerva_admin
--

GRANT SELECT,USAGE ON SEQUENCE trigger_rule."node/1w/highpowerusage_exception_weight_id_seq" TO minerva_writer;


--
-- Name: TABLE "node/1w/highpowerusage_threshold"; Type: ACL; Schema: trigger_rule; Owner: minerva_admin
--

GRANT SELECT ON TABLE trigger_rule."node/1w/highpowerusage_threshold" TO minerva;
GRANT SELECT,INSERT,DELETE,UPDATE ON TABLE trigger_rule."node/1w/highpowerusage_threshold" TO minerva_writer;


--
-- Name: DEFAULT PRIVILEGES FOR SEQUENCES; Type: DEFAULT ACL; Schema: alias; Owner: postgres
--

ALTER DEFAULT PRIVILEGES FOR ROLE postgres IN SCHEMA alias GRANT SELECT,USAGE ON SEQUENCES TO minerva_writer;


--
-- Name: DEFAULT PRIVILEGES FOR TABLES; Type: DEFAULT ACL; Schema: alias; Owner: postgres
--

ALTER DEFAULT PRIVILEGES FOR ROLE postgres IN SCHEMA alias GRANT SELECT ON TABLES TO minerva;
ALTER DEFAULT PRIVILEGES FOR ROLE postgres IN SCHEMA alias GRANT SELECT,DELETE,UPDATE ON TABLES TO minerva_writer;


--
-- Name: DEFAULT PRIVILEGES FOR SEQUENCES; Type: DEFAULT ACL; Schema: alias_def; Owner: postgres
--

ALTER DEFAULT PRIVILEGES FOR ROLE postgres IN SCHEMA alias_def GRANT SELECT,USAGE ON SEQUENCES TO minerva_writer;


--
-- Name: DEFAULT PRIVILEGES FOR TABLES; Type: DEFAULT ACL; Schema: alias_def; Owner: postgres
--

ALTER DEFAULT PRIVILEGES FOR ROLE postgres IN SCHEMA alias_def GRANT SELECT ON TABLES TO minerva;
ALTER DEFAULT PRIVILEGES FOR ROLE postgres IN SCHEMA alias_def GRANT SELECT,DELETE,UPDATE ON TABLES TO minerva_writer;


--
-- Name: DEFAULT PRIVILEGES FOR SEQUENCES; Type: DEFAULT ACL; Schema: alias_directory; Owner: postgres
--

ALTER DEFAULT PRIVILEGES FOR ROLE postgres IN SCHEMA alias_directory GRANT SELECT,USAGE ON SEQUENCES TO minerva_writer;


--
-- Name: DEFAULT PRIVILEGES FOR TABLES; Type: DEFAULT ACL; Schema: attribute; Owner: postgres
--

ALTER DEFAULT PRIVILEGES FOR ROLE postgres IN SCHEMA attribute GRANT SELECT ON TABLES TO minerva;
ALTER DEFAULT PRIVILEGES FOR ROLE postgres IN SCHEMA attribute GRANT ALL ON TABLES TO minerva_writer;


--
-- Name: DEFAULT PRIVILEGES FOR TABLES; Type: DEFAULT ACL; Schema: attribute_base; Owner: postgres
--

ALTER DEFAULT PRIVILEGES FOR ROLE postgres IN SCHEMA attribute_base GRANT SELECT ON TABLES TO minerva;
ALTER DEFAULT PRIVILEGES FOR ROLE postgres IN SCHEMA attribute_base GRANT ALL ON TABLES TO minerva_writer;


--
-- Name: DEFAULT PRIVILEGES FOR SEQUENCES; Type: DEFAULT ACL; Schema: attribute_directory; Owner: postgres
--

ALTER DEFAULT PRIVILEGES FOR ROLE postgres IN SCHEMA attribute_directory GRANT SELECT,USAGE ON SEQUENCES TO minerva_writer;


--
-- Name: DEFAULT PRIVILEGES FOR SEQUENCES; Type: DEFAULT ACL; Schema: attribute_history; Owner: postgres
--

ALTER DEFAULT PRIVILEGES FOR ROLE postgres IN SCHEMA attribute_history GRANT SELECT,USAGE ON SEQUENCES TO minerva_writer;


--
-- Name: DEFAULT PRIVILEGES FOR TABLES; Type: DEFAULT ACL; Schema: attribute_history; Owner: postgres
--

ALTER DEFAULT PRIVILEGES FOR ROLE postgres IN SCHEMA attribute_history GRANT SELECT ON TABLES TO minerva;
ALTER DEFAULT PRIVILEGES FOR ROLE postgres IN SCHEMA attribute_history GRANT ALL ON TABLES TO minerva_writer;


--
-- Name: DEFAULT PRIVILEGES FOR TABLES; Type: DEFAULT ACL; Schema: attribute_staging; Owner: postgres
--

ALTER DEFAULT PRIVILEGES FOR ROLE postgres IN SCHEMA attribute_staging GRANT SELECT ON TABLES TO minerva;
ALTER DEFAULT PRIVILEGES FOR ROLE postgres IN SCHEMA attribute_staging GRANT SELECT,INSERT,DELETE,UPDATE ON TABLES TO minerva_writer;


--
-- Name: DEFAULT PRIVILEGES FOR SEQUENCES; Type: DEFAULT ACL; Schema: directory; Owner: postgres
--

ALTER DEFAULT PRIVILEGES FOR ROLE postgres IN SCHEMA directory GRANT SELECT,USAGE ON SEQUENCES TO minerva_writer;


--
-- Name: DEFAULT PRIVILEGES FOR SEQUENCES; Type: DEFAULT ACL; Schema: entity; Owner: postgres
--

ALTER DEFAULT PRIVILEGES FOR ROLE postgres IN SCHEMA entity GRANT SELECT,USAGE ON SEQUENCES TO minerva_writer;


--
-- Name: DEFAULT PRIVILEGES FOR TABLES; Type: DEFAULT ACL; Schema: entity; Owner: postgres
--

ALTER DEFAULT PRIVILEGES FOR ROLE postgres IN SCHEMA entity GRANT SELECT ON TABLES TO minerva;
ALTER DEFAULT PRIVILEGES FOR ROLE postgres IN SCHEMA entity GRANT SELECT,INSERT,DELETE,UPDATE ON TABLES TO minerva_writer;


--
-- Name: DEFAULT PRIVILEGES FOR SEQUENCES; Type: DEFAULT ACL; Schema: logging; Owner: postgres
--

ALTER DEFAULT PRIVILEGES FOR ROLE postgres IN SCHEMA logging GRANT SELECT,USAGE ON SEQUENCES TO minerva_writer;


--
-- Name: DEFAULT PRIVILEGES FOR SEQUENCES; Type: DEFAULT ACL; Schema: notification; Owner: postgres
--

ALTER DEFAULT PRIVILEGES FOR ROLE postgres IN SCHEMA notification GRANT SELECT,USAGE ON SEQUENCES TO minerva_writer;


--
-- Name: DEFAULT PRIVILEGES FOR TABLES; Type: DEFAULT ACL; Schema: notification; Owner: postgres
--

ALTER DEFAULT PRIVILEGES FOR ROLE postgres IN SCHEMA notification GRANT SELECT ON TABLES TO minerva;
ALTER DEFAULT PRIVILEGES FOR ROLE postgres IN SCHEMA notification GRANT ALL ON TABLES TO minerva_writer;


--
-- Name: DEFAULT PRIVILEGES FOR SEQUENCES; Type: DEFAULT ACL; Schema: notification_directory; Owner: postgres
--

ALTER DEFAULT PRIVILEGES FOR ROLE postgres IN SCHEMA notification_directory GRANT SELECT,USAGE ON SEQUENCES TO minerva_writer;


--
-- Name: DEFAULT PRIVILEGES FOR TABLES; Type: DEFAULT ACL; Schema: relation; Owner: postgres
--

ALTER DEFAULT PRIVILEGES FOR ROLE postgres IN SCHEMA relation GRANT SELECT ON TABLES TO minerva;
ALTER DEFAULT PRIVILEGES FOR ROLE postgres IN SCHEMA relation GRANT SELECT,INSERT,DELETE,UPDATE ON TABLES TO minerva_writer;


--
-- Name: DEFAULT PRIVILEGES FOR SEQUENCES; Type: DEFAULT ACL; Schema: relation_def; Owner: postgres
--

ALTER DEFAULT PRIVILEGES FOR ROLE postgres IN SCHEMA relation_def GRANT SELECT,USAGE ON SEQUENCES TO minerva_writer;


--
-- Name: DEFAULT PRIVILEGES FOR TABLES; Type: DEFAULT ACL; Schema: relation_def; Owner: postgres
--

ALTER DEFAULT PRIVILEGES FOR ROLE postgres IN SCHEMA relation_def GRANT SELECT ON TABLES TO minerva;
ALTER DEFAULT PRIVILEGES FOR ROLE postgres IN SCHEMA relation_def GRANT SELECT,INSERT,DELETE,UPDATE ON TABLES TO minerva_writer;


--
-- Name: DEFAULT PRIVILEGES FOR SEQUENCES; Type: DEFAULT ACL; Schema: relation_directory; Owner: postgres
--

ALTER DEFAULT PRIVILEGES FOR ROLE postgres IN SCHEMA relation_directory GRANT SELECT,USAGE ON SEQUENCES TO minerva_writer;


--
-- Name: DEFAULT PRIVILEGES FOR TABLES; Type: DEFAULT ACL; Schema: trend; Owner: postgres
--

ALTER DEFAULT PRIVILEGES FOR ROLE postgres IN SCHEMA trend GRANT SELECT ON TABLES TO minerva;
ALTER DEFAULT PRIVILEGES FOR ROLE postgres IN SCHEMA trend GRANT SELECT,INSERT,DELETE,UPDATE ON TABLES TO minerva_writer;


--
-- Name: DEFAULT PRIVILEGES FOR SEQUENCES; Type: DEFAULT ACL; Schema: trend_directory; Owner: postgres
--

ALTER DEFAULT PRIVILEGES FOR ROLE postgres IN SCHEMA trend_directory GRANT SELECT,USAGE ON SEQUENCES TO minerva_writer;


--
-- Name: DEFAULT PRIVILEGES FOR TABLES; Type: DEFAULT ACL; Schema: trend_partition; Owner: postgres
--

ALTER DEFAULT PRIVILEGES FOR ROLE postgres IN SCHEMA trend_partition GRANT SELECT ON TABLES TO minerva;
ALTER DEFAULT PRIVILEGES FOR ROLE postgres IN SCHEMA trend_partition GRANT SELECT,INSERT,DELETE,UPDATE ON TABLES TO minerva_writer;


--
-- Name: DEFAULT PRIVILEGES FOR SEQUENCES; Type: DEFAULT ACL; Schema: trigger; Owner: postgres
--

ALTER DEFAULT PRIVILEGES FOR ROLE postgres IN SCHEMA trigger GRANT SELECT,USAGE ON SEQUENCES TO minerva_writer;


--
-- Name: DEFAULT PRIVILEGES FOR SEQUENCES; Type: DEFAULT ACL; Schema: trigger_rule; Owner: postgres
--

ALTER DEFAULT PRIVILEGES FOR ROLE postgres IN SCHEMA trigger_rule GRANT SELECT,USAGE ON SEQUENCES TO minerva_writer;


--
-- Name: DEFAULT PRIVILEGES FOR FUNCTIONS; Type: DEFAULT ACL; Schema: trigger_rule; Owner: postgres
--

ALTER DEFAULT PRIVILEGES FOR ROLE postgres IN SCHEMA trigger_rule GRANT ALL ON FUNCTIONS TO minerva;


--
-- Name: DEFAULT PRIVILEGES FOR TABLES; Type: DEFAULT ACL; Schema: trigger_rule; Owner: postgres
--

ALTER DEFAULT PRIVILEGES FOR ROLE postgres IN SCHEMA trigger_rule GRANT SELECT ON TABLES TO minerva;
ALTER DEFAULT PRIVILEGES FOR ROLE postgres IN SCHEMA trigger_rule GRANT SELECT,INSERT,DELETE,UPDATE ON TABLES TO minerva_writer;
ALTER DEFAULT PRIVILEGES FOR ROLE postgres IN SCHEMA trigger_rule GRANT ALL ON TABLES TO minerva_admin;


--
-- PostgreSQL database dump complete
--

