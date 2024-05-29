DEFINE TABLE user_images SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD height ON user_images TYPE int;
DEFINE FIELD width ON user_images TYPE int;
DEFINE FIELD url ON user_images TYPE string;
DEFINE FIELD mime_type ON user_images TYPE string;
DEFINE FIELD blurhash ON user_images TYPE string;
DEFINE FIELD file_name ON user_images TYPE string;
DEFINE FIELD is_default ON user_images TYPE bool DEFAULT false;
DEFINE FIELD created_at ON users TYPE datetime DEFAULT time::now();