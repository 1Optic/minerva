CREATE OR REPLACE VIEW virtual_entity."v-network" AS
SELECT 'network' AS name;

SELECT directory.create_entity_type('v-network');

INSERT INTO entity."v-network" (name)
SELECT
  name
FROM virtual_entity."v-network"
ON CONFLICT DO NOTHING;
