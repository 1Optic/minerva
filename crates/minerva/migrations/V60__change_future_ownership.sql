-- Because it involves an event trigger, this migration has to be run by a superuser.

CREATE OR REPLACE FUNCTION change_ownership_on_future_objects()
  RETURNS event_trigger AS $$
DECLARE
    obj record;
BEGIN
    FOR obj IN SELECT * FROM pg_event_trigger_ddl_commands() LOOP
        IF obj.command_tag IN ('CREATE TABLE', 'CREATE VIEW', 'CREATE TYPE') THEN
            PERFORM run_command_on_workers('ALTER ' || obj.object_type || ' ' || obj.object_identity || ' OWNER TO postgres');
            EXECUTE 'ALTER ' || obj.object_type || ' ' || obj.object_identity || ' OWNER TO postgres';
        END IF;
    END LOOP;
END;
$$ LANGUAGE plpgsql VOLATILE;

CREATE EVENT TRIGGER change_ownership
    ON ddl_command_end
    WHEN tag IN ('CREATE TABLE', 'CREATE VIEW', 'CREATE TYPE')
    EXECUTE FUNCTION change_ownership_on_future_objects();
