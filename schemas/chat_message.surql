DEFINE TABLE chat_message TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD id ON chat_message TYPE record<chat_message>;
DEFINE FIELD user ON chat_message TYPE record<user>;
DEFINE FIELD message ON chat_message TYPE option<string>;
DEFINE FIELD reaction ON chat_message TYPE option<string>;
DEFINE FIELD is_read ON chat_message TYPE bool DEFAULT false;
DEFINE FIELD is_deleted ON chat_message TYPE bool DEFAULT false;
DEFINE FIELD read_at ON chat_message TYPE option<datetime>;
DEFINE FIELD created_at ON chat_message TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON chat_message TYPE datetime VALUE time::now();
