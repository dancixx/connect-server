DEFINE TABLE image TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD id ON image TYPE record<image>;
DEFINE FIELD height ON image TYPE int;
DEFINE FIELD width ON image TYPE int;
DEFINE FIELD url ON image TYPE string ASSERT string::is::url($value);
DEFINE FIELD mime_type ON image TYPE string;
DEFINE FIELD blurhash ON image TYPE string;
DEFINE FIELD file_name ON image TYPE string;
