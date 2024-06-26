DEFINE TABLE message TYPE RELATION FROM match TO user SCHEMAFULL PERMISSIONS none;

DEFINE FIELD id ON message TYPE record<message>;
DEFINE FIELD in ON message TYPE record<match>;
DEFINE FIELD out ON message TYPE record<user>;
DEFINE FIELD text ON message TYPE option<string>;
DEFINE FIELD reaction ON message TYPE option<string>;
DEFINE FIELD is_read ON message TYPE bool DEFAULT false;
DEFINE FIELD is_deleted ON message TYPE bool DEFAULT false;
DEFINE FIELD read_at ON message TYPE option<datetime>;
DEFINE FIELD created_at ON message TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON message TYPE datetime VALUE time::now();
