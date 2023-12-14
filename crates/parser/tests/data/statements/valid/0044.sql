CREATE VIEW comedies AS SELECT * FROM films WHERE kind = 'Comedy';
CREATE VIEW universal_comedies AS SELECT * FROM comedies WHERE classification = 'U' WITH LOCAL CHECK OPTION;
CREATE VIEW pg_comedies AS SELECT * FROM comedies WHERE classification = 'PG' WITH CASCADED CHECK OPTION;
CREATE VIEW comedies AS SELECT f.*, country_code_to_name(f.country_code) AS country, (SELECT avg(r.rating) FROM user_ratings r WHERE r.film_id = f.id) AS avg_rating FROM films f WHERE f.kind = 'Comedy';
CREATE RECURSIVE VIEW public.nums_1_100 (n) AS VALUES (1) UNION ALL SELECT n+1 FROM nums_1_100 WHERE n < 100;
