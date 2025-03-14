CREATE TABLE IF NOT EXISTS relation."node->entity_set"(
  source_id integer,
  target_id integer,
  PRIMARY KEY (source_id, target_id));

SELECT relation_directory.name_to_type('node->entity_set');
