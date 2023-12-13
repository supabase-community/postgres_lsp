CREATE AGGREGATE aggregate1 (int4) (sfunc = sfunc1, stype = stype1);
CREATE AGGREGATE aggregate1 (int4, bool) (sfunc = sfunc1, stype = stype1);
CREATE AGGREGATE aggregate1 (*) (sfunc = sfunc1, stype = stype1);
CREATE AGGREGATE aggregate1 (int4) (sfunc = sfunc1, stype = stype1, finalfunc_extra, mfinalfuncextra);
CREATE AGGREGATE aggregate1 (int4) (sfunc = sfunc1, stype = stype1, finalfunc_modify = read_only, parallel = restricted);
CREATE AGGREGATE percentile_disc (float8 ORDER BY anyelement) (sfunc = ordered_set_transition, stype = internal, finalfunc = percentile_disc_final, finalfunc_extra);
