CREATE PUBLICATION mypublication FOR TABLE users, departments;
-- CREATE PUBLICATION active_departments FOR TABLE departments WHERE (active IS TRUE);
CREATE PUBLICATION alltables FOR ALL TABLES;
-- CREATE PUBLICATION insert_only FOR TABLE mydata WITH (publish = 'insert');
-- CREATE PUBLICATION production_publication FOR TABLE users, departments, TABLES IN SCHEMA production;
CREATE PUBLICATION sales_publication FOR TABLES IN SCHEMA marketing, sales;
-- CREATE PUBLICATION users_filtered FOR TABLE users (user_id, firstname);
