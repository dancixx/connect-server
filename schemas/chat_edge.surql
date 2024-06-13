DEFINE TABLE chat_edge SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD in ON TABLE chat_edge;
DEFINE FIELD out ON TABLE chat_edge;
// TODO: Messages count, autoincrement
DEFINE FIELD messages_count ON chat_edge TYPE int DEFAULT 0;
// TODO: Unread messages count, autoincrement
DEFINE FIELD unread_messages_count ON chat_edge TYPE int DEFAULT 0;
DEFINE FIELD disabled_by_in ON chat_edge TYPE bool DEFAULT false;
DEFINE FIELD disabled_by_out ON chat_edge TYPE bool DEFAULT false;
DEFINE FIELD created_at ON chat_edge TYPE datetime DEFAULT time::now();
// TODO: Trigger if new messags
DEFINE FIELD updated_at ON chat_edge TYPE datetime VALUE time::now();

DEFINE INDEX unique_relationship 
    ON TABLE chat_edge 
    COLUMNS in, out UNIQUE;