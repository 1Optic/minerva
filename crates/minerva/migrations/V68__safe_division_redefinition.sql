DROP FUNCTION IF EXISTS safe_division(anyelement, anyelement);
DROP FUNCTION IF EXISTS safe_division(anyelement, anyelement, anyelement);
DROP FUNCTION IF EXISTS safe_division(bigint, numeric);

CREATE OR REPLACE FUNCTION public.safe_division(numerator anyelement, denominator anycompatible)
RETURNS anycompatible AS $$
  SELECT CASE
    WHEN denominator = 0 THEN NULL
    ELSE numerator / denominator
  END;
$$ LANGUAGE sql IMMUTABLE;

CREATE OR REPLACE FUNCTION public.safe_division(numerator anyelement, denominator anycompatible, division_by_zero_indicator anycompatible)
RETURNS anycompatible AS $$
  SELECT CASE
    WHEN denominator = 0 THEN division_by_zero_indicator
    ELSE numerator / denominator
  END;
$$ LANGUAGE sql IMMUTABLE;
