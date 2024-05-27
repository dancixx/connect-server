DEFINE TABLE chat_messages SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD message ON chat_messages TYPE string;
DEFINE FIELD created_at ON users TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON users TYPE datetime VALUE time::now();