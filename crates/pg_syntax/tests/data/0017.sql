SELECT name, (SELECT max(pop) FROM cities WHERE cities.state = states.name)
    FROM states;
