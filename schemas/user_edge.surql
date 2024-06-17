DEFINE TABLE user_edge TYPE ANY SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD in ON user_edge PERMISSIONS FULL;
DEFINE FIELD out ON user_edge PERMISSIONS FULL;
DEFINE FIELD in_swipe ON user_edge TYPE option<bool> PERMISSIONS FULL;
DEFINE FIELD out_swipe ON user_edge TYPE option<bool> PERMISSIONS FULL;
DEFINE FIELD is_super_swipe ON user_edge TYPE bool DEFAULT false PERMISSIONS FULL;
DEFINE FIELD is_unmatched ON user_edge TYPE bool DEFAULT false PERMISSIONS FULL;
DEFINE FIELD unmatched_at ON user_edge TYPE option<datetime> PERMISSIONS FULL;
DEFINE FIELD messages_count ON user_edge TYPE int DEFAULT 0;
DEFINE FIELD unread_messages_count ON user_edge TYPE int DEFAULT 0;
DEFINE FIELD chat_disabled_by_in ON user_edge TYPE bool DEFAULT false;
DEFINE FIELD chat_disabled_by_out ON user_edge TYPE bool DEFAULT false;
DEFINE FIELD created_at ON user_edge TYPE datetime DEFAULT time::now() PERMISSIONS FULL;
DEFINE FIELD updated_at ON user_edge TYPE datetime VALUE time::now() PERMISSIONS FULL;

DEFINE INDEX unique_relationship ON user_edge FIELDS in, out UNIQUE;
