ALTER TABLE "trigger"."template_parameter"
    ADD COLUMN "has_lookback" boolean NOT NULL DEFAULT false,
    ADD COLUMN "lookback_parameter" text;
