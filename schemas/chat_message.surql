DEFINE TABLE chat_message SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD message ON chat_message TYPE string;
DEFINE FIELD reaction ON chat_message TYPE string;
DEFINE FIELD created_at ON chat_message TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON chat_message TYPE datetime VALUE time::now();