CREATE TABLESPACE x LOCATION 'a';
CREATE TABLESPACE x OWNER a LOCATION 'b' WITH (random_page_cost=42, seq_page_cost=3);
