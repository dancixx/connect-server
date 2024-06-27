DEFINE TABLE user_image TYPE RELATION FROM user TO image SCHEMAFULL PERMISSIONS none;

DEFINE FIELD id ON user_image TYPE record<user_image>;
DEFINE FIELD in ON user_image TYPE record<user>;
DEFINE FIELD out ON user_image TYPE record<image>;
DEFINE FIELD is_default ON image TYPE bool DEFAULT false;
DEFINE FIELD created_at ON image TYPE datetime DEFAULT time::now();