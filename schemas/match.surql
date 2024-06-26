DEFINE TABLE match TYPE RELATION FROM user TO user SCHEMAFULL PERMISSIONS none;

DEFINE FIELD id ON match TYPE record<match> PERMISSIONS FULL;
DEFINE FIELD in ON match TYPE record<user> PERMISSIONS FULL;
DEFINE FIELD out ON match TYPE record<user> PERMISSIONS FULL;
DEFINE FIELD in_swipe ON match TYPE option<bool> PERMISSIONS FULL;
DEFINE FIELD out_swipe ON match TYPE option<bool> PERMISSIONS FULL;
DEFINE FIELD is_super_swipe ON match TYPE bool DEFAULT false PERMISSIONS FULL;
DEFINE FIELD is_unmatched ON match TYPE bool DEFAULT false PERMISSIONS FULL;
DEFINE FIELD unmatched_at ON match TYPE option<datetime> PERMISSIONS FULL;
DEFINE FIELD messages_count ON match TYPE int DEFAULT 0;
DEFINE FIELD unread_messages_count_by_in ON match TYPE int DEFAULT 0;
DEFINE FIELD unread_messages_count_by_out ON match TYPE int DEFAULT 0;
DEFINE FIELD chat_disabled_by_in ON match TYPE bool DEFAULT false;
DEFINE FIELD chat_disabled_by_out ON match TYPE bool DEFAULT false;
DEFINE FIELD messages ON match
DEFINE FIELD created_at ON match TYPE datetime DEFAULT time::now() PERMISSIONS FULL;
DEFINE FIELD updated_at ON match TYPE datetime VALUE time::now() PERMISSIONS FULL;