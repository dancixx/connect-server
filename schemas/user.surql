DEFINE TABLE user TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD id ON user TYPE record<user>;
DEFINE FIELD email ON user TYPE string ASSERT string::is::email($value);
DEFINE FIELD username ON user TYPE option<string>;
DEFINE FIELD last_name ON user TYPE option<string>;
DEFINE FIELD first_name ON user TYPE option<string>;
DEFINE FIELD birth_date ON user TYPE option<datetime>;
DEFINE FIELD age ON user TYPE option<number>;
DEFINE FIELD gender ON user TYPE option<string>;
DEFINE FIELD preference ON user TYPE option<string>;
DEFINE FIELD goal ON user TYPE option<string>;
DEFINE FIELD interests ON user TYPE option<array<string>>;
DEFINE FIELD metric ON user TYPE bool DEFAULT true;
DEFINE FIELD height ON user TYPE option<number>;
DEFINE FIELD weight ON user TYPE option<number>;
DEFINE FIELD bio ON user TYPE option<string>;
DEFINE FIELD current_location ON user TYPE option<point>;
DEFINE FIELD max_distance ON user TYPE number DEFAULT 25;
DEFINE FIELD age_range ON user TYPE option<array<number>>;
DEFINE FIELD max_match_count ON user TYPE number DEFAULT 10;
###########################
DEFINE FIELD education ON user TYPE option<string>;
DEFINE FIELD job ON user TYPE option<string>;
DEFINE FIELD alcohol ON user TYPE option<string>;
DEFINE FIELD smoking ON user TYPE bool DEFAULT false;
DEFINE FIELD political_views ON user TYPE option<string>;
DEFINE FIELD religion ON user TYPE option<string>;
DEFINE FIELD spoken_languages ON user TYPE option<array<string>>;
DEFINE FIELD app_language ON user TYPE option<string>;
DEFINE FIELD zodiac ON user TYPE option<string>;
###########################
DEFINE FIELD has_pet ON user TYPE bool DEFAULT false;
DEFINE FIELD has_car ON user TYPE bool DEFAULT false;
DEFINE FIELD has_house ON user TYPE bool DEFAULT false;
DEFINE FIELD need_pet ON user TYPE bool DEFAULT false;
DEFINE FIELD need_car ON user TYPE bool DEFAULT false;
DEFINE FIELD need_house ON user TYPE bool DEFAULT false;
###########################
DEFINE FIELD has_kids ON user TYPE bool DEFAULT false;
DEFINE FIELD want_kids ON user TYPE option<bool>;
###########################
DEFINE FIELD spontaneous_travel ON user TYPE bool DEFAULT false;
###########################
DEFINE FIELD is_registration_completed ON user TYPE bool DEFAULT false;
DEFINE FIELD is_verified ON user TYPE bool DEFAULT false;
DEFINE FIELD is_banned ON user TYPE bool DEFAULT false;
###########################
DEFINE FIELD created_at ON user TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON user TYPE datetime VALUE time::now();

DEFINE INDEX user_email_index ON user FIELDS email UNIQUE;
DEFINE INDEX user_name_index ON user FIELDS username UNIQUE;