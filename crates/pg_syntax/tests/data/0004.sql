CREATE VIEW myview AS
    SELECT name, location
        FROM weather, cities
        WHERE city = name;
