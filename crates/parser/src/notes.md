refactor:

- parser only has very basic controls
- directory with parsers
- source
- statement
- valid_statement

- each parser has a parser() fn that passes the &mut parser
- check if we can pass the &mut parser to the constructor
- for statement, pass kind in constructor
- for valid_statement, pass node and until in constructor
