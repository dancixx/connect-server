DEFINE TABLE user TYPE ANY SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD email ON user TYPE string ASSERT string::is::email($value) PERMISSIONS FULL;
DEFINE FIELD username ON user TYPE option<string> PERMISSIONS FULL;
DEFINE FIELD last_name ON user TYPE option<string> PERMISSIONS FULL;
DEFINE FIELD first_name ON user TYPE option<string> PERMISSIONS FULL;
DEFINE FIELD birth_date ON user TYPE option<datetime> PERMISSIONS FULL;
DEFINE FIELD age ON user TYPE option<number> PERMISSIONS FULL;
DEFINE FIELD gender ON user TYPE option<string> PERMISSIONS FULL;
DEFINE FIELD preference ON user TYPE option<string> PERMISSIONS FULL;
DEFINE FIELD goal ON user TYPE option<string> PERMISSIONS FULL;
DEFINE FIELD interests ON user TYPE option<array<string>> PERMISSIONS FULL;
DEFINE FIELD metric ON user TYPE bool DEFAULT true PERMISSIONS FULL;
DEFINE FIELD height ON user TYPE option<number> PERMISSIONS FULL;
DEFINE FIELD weight ON user TYPE option<number> PERMISSIONS FULL;
DEFINE FIELD bio ON user TYPE option<string> PERMISSIONS FULL;
DEFINE FIELD current_location ON user TYPE option<point> PERMISSIONS FULL;
DEFINE FIELD max_distance ON user TYPE number DEFAULT 25 PERMISSIONS FULL;
DEFINE FIELD age_range ON user TYPE option<array<number>> PERMISSIONS FULL;
DEFINE FIELD max_match_count ON user TYPE number DEFAULT 10 PERMISSIONS FULL;
###########################
DEFINE FIELD education ON user TYPE option<string> PERMISSIONS FULL;
DEFINE FIELD job ON user TYPE option<string> PERMISSIONS FULL;
DEFINE FIELD alcohol ON user TYPE option<string> PERMISSIONS FULL;
DEFINE FIELD smoking ON user TYPE bool DEFAULT false PERMISSIONS FULL;
DEFINE FIELD political_views ON user TYPE option<string> PERMISSIONS FULL;
DEFINE FIELD religion ON user TYPE option<string> PERMISSIONS FULL;
DEFINE FIELD spoken_languages ON user TYPE option<array<string>> PERMISSIONS FULL;
DEFINE FIELD app_language ON user TYPE option<string> PERMISSIONS FULL;
DEFINE FIELD zodiac ON user TYPE option<string> PERMISSIONS FULL;
###########################
DEFINE FIELD has_pet ON user TYPE bool DEFAULT false PERMISSIONS FULL;
DEFINE FIELD has_car ON user TYPE bool DEFAULT false PERMISSIONS FULL;
DEFINE FIELD has_house ON user TYPE bool DEFAULT false PERMISSIONS FULL;
DEFINE FIELD need_pet ON user TYPE bool DEFAULT false PERMISSIONS FULL;
DEFINE FIELD need_car ON user TYPE bool DEFAULT false PERMISSIONS FULL;
DEFINE FIELD need_house ON user TYPE bool DEFAULT false PERMISSIONS FULL;
###########################
DEFINE FIELD has_kids ON user TYPE bool DEFAULT false PERMISSIONS FULL;
DEFINE FIELD want_kids ON user TYPE option<bool> PERMISSIONS FULL;
###########################
DEFINE FIELD spontaneous_travel ON user TYPE bool DEFAULT false PERMISSIONS FULL;
###########################
DEFINE FIELD is_registration_completed ON user TYPE bool DEFAULT false PERMISSIONS FULL;
DEFINE FIELD is_verified ON user TYPE bool DEFAULT false PERMISSIONS FULL;
DEFINE FIELD is_banned ON user TYPE bool DEFAULT false PERMISSIONS FULL;
###########################
DEFINE FIELD created_at ON user TYPE datetime DEFAULT time::now() PERMISSIONS FULL;
DEFINE FIELD updated_at ON user TYPE datetime VALUE time::now() PERMISSIONS FULL;

DEFINE INDEX user_email_index ON user FIELDS email UNIQUE;
DEFINE INDEX user_name_index ON user FIELDS username UNIQUE;