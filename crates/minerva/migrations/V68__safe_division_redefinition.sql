DROP FUNCTION IF EXISTS safe_division(anyelement, anyelement);
DROP FUNCTION IF EXISTS safe_division(anyelement, anyelement, anyelement);
DROP FUNCTION IF EXISTS safe_division(bigint, numeric);

CREATE OR REPLACE FUNCTION public.safe_division(numerator anyelement, denominator anycompatible)
RETURNS numeric AS $$
  SELECT CASE
    WHEN denominator = 0 THEN 0
    ELSE numerator / denominator
  END;
$$ LANGUAGE sql IMMUTABLE;

CREATE OR REPLACE FUNCTION public.safe_division(numerator anyelement, denominator anycompatible, division_by_zero_indicator numeric)
RETURNS numeric AS $$
  SELECT CASE
    WHEN denominator = 0 THEN division_by_zero_indicator
    ELSE numerator / denominator
  END;
$$ LANGUAGE sql IMMUTABLE;
