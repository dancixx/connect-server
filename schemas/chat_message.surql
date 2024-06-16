DEFINE TABLE chat_message TYPE ANY SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD message ON chat_message TYPE option<string> PERMISSIONS FULL;
DEFINE FIELD reaction ON chat_message TYPE option<string> PERMISSIONS FULL;
DEFINE FIELD is_read ON chat_message TYPE bool DEFAULT false PERMISSIONS FULL;
DEFINE FIELD is_deleted ON chat_message TYPE bool DEFAULT false PERMISSIONS FULL;
DEFINE FIELD read_at ON chat_message TYPE option<datetime> PERMISSIONS FULL;
DEFINE FIELD created_at ON chat_message TYPE datetime DEFAULT time::now() PERMISSIONS FULL;
DEFINE FIELD updated_at ON chat_message TYPE datetime VALUE time::now() PERMISSIONS FULL;
