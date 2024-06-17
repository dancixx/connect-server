DEFINE TABLE chat_edge SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD in ON TABLE chat_edge;
DEFINE FIELD out ON TABLE chat_edge;

DEFINE INDEX unique_relationship 
    ON TABLE chat_edge 
    COLUMNS in, out UNIQUE;