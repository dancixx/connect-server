DEFINE TABLE user_image TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD id ON user_image TYPE record<user_image>;
DEFINE FIELD height ON user_image TYPE int;
DEFINE FIELD width ON user_image TYPE int;
DEFINE FIELD url ON user_image TYPE string;
DEFINE FIELD mime_type ON user_image TYPE string;
DEFINE FIELD blurhash ON user_image TYPE string;
DEFINE FIELD file_name ON user_image TYPE string;
DEFINE FIELD is_default ON user_image TYPE bool DEFAULT false;
DEFINE FIELD created_at ON user_image TYPE datetime DEFAULT time::now();