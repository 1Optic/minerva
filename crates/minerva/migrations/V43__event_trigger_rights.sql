GRANT SELECT ON TABLE "trigger"."template" TO minerva;
GRANT SELECT ON TABLE "trigger"."template_parameter" TO minerva;

GRANT INSERT,UPDATE,DELETE ON TABLE "trigger"."template" TO minerva_writer;
GRANT INSERT,UPDATE,DELETE ON TABLE "trigger"."template_parameter" TO minerva_writer;
