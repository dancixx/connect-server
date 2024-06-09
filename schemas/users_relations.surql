DEFINE TABLE users_relations TYPE ANY SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD in ON users_relations PERMISSIONS FULL;
DEFINE FIELD out ON users_relations PERMISSIONS FULL;
DEFINE FIELD in_swipe ON users_relations TYPE bool | null DEFAULT NULL PERMISSIONS FULL;
DEFINE FIELD out_swipe ON users_relations TYPE bool | null DEFAULT NULL PERMISSIONS FULL;
DEFINE FIELD is_super_swipe ON users_relations TYPE bool DEFAULT false PERMISSIONS FULL;
DEFINE FIELD created_at ON users_relations TYPE datetime DEFAULT time::now() PERMISSIONS FULL;
DEFINE FIELD updated_at ON users_relations TYPE datetime VALUE time::now() PERMISSIONS FULL;

DEFINE INDEX unique_relationship ON users_relations FIELDS in, out UNIQUE;
