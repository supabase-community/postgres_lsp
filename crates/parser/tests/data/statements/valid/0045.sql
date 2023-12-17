CREATE OPERATOR + (procedure = plusfunc);
CREATE OPERATOR + (procedure = plusfunc, leftarg = int4, rightarg = int4);
CREATE OPERATOR + (procedure = plusfunc, hashes, merges);
