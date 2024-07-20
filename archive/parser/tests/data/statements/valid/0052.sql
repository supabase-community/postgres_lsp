CREATE PROCEDURE insert_data(a integer, b integer) LANGUAGE SQL AS $$INSERT INTO tbl VALUES (a); INSERT INTO tbl VALUES (b);$$;
CREATE PROCEDURE insert_data(a integer, b integer) LANGUAGE SQL BEGIN ATOMIC INSERT INTO tbl VALUES (a); INSERT INTO tbl VALUES (b); END;
