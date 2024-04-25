DEFINE TABLE users SCHEMAFULL;

DEFINE FIELD name ON users TYPE string;
DEFINE FIELD email ON users TYPE string;
DEFINE FIELD created_at ON users TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON users TYPE datetime VALUE time::now();

