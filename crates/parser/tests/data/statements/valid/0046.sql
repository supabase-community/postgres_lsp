CREATE TYPE type1;
CREATE TYPE type1 AS (attr1 int4, attr2 bool);
CREATE TYPE type1 AS (attr1 int4 COLLATE collation1, attr2 bool);
CREATE TYPE type1 AS ENUM ('value1', 'value2', 'value3');
CREATE TYPE type1 AS RANGE (subtype = int4);
CREATE TYPE type1 AS RANGE (subtype = int4, receive = receive_func, passedbyvalue);
CREATE TYPE type1 (input = input1, output = output1);
CREATE TYPE type1 (input = input1, output = output1, passedbyvalue);
