DEFINE TABLE chat_message_user_edge SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD in ON TABLE chat_message_user_edge;
DEFINE FIELD out ON TABLE chat_message_user_edge;

DEFINE INDEX unique_relationship 
    ON TABLE chat_message_user_edge 
    COLUMNS in, out UNIQUE;