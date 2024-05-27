DEFINE TABLE users SCHEMAFULL PERMISSIONS NONE;
    -- PERMISSIONS
    --     FOR create NONE
    --     FOR select, update, delete WHERE id = $auth.id;

DEFINE FIELD email ON users TYPE string
    ASSERT string::is::email($value);
DEFINE FIELD username ON users TYPE string | null DEFAULT null;
DEFINE FIELD last_name ON users TYPE string | null DEFAULT null;
DEFINE FIELD first_name ON users TYPE string | null DEFAULT null;
DEFINE FIELD birth_date ON users TYPE datetime | null DEFAULT null; 
DEFINE FIELD age ON users TYPE int | null DEFAULT null;
DEFINE FIELD gender ON users TYPE string | null DEFAULT null;
DEFINE FIELD looking_for ON users TYPE string | null DEFAULT null;
DEFINE FIELD metric ON users TYPE string | null DEFAULT null;
DEFINE FIELD height ON users TYPE int | null DEFAULT null;
DEFINE FIELD weight ON users TYPE int | null DEFAULT null;
DEFINE FIELD bio ON users TYPE string | null DEFAULT null;
DEFINE FIELD interest ON users TYPE array<string> | null DEFAULT null;
DEFINE FIELD country ON users TYPE string | null DEFAULT null;
DEFINE FIELD city ON users TYPE string | null DEFAULT null;
DEFINE FIELD max_distance ON users TYPE int DEFAULT 25;
DEFINE FIELD age_range ON users TYPE array<int> | null DEFAULT null;
DEFINE FIELD max_match_count ON users TYPE int DEFAULT 10;
DEFINE FIELD active_hours ON users TYPE array<int> | null DEFAULT null;
DEFINE FIELD has_dog ON users TYPE bool DEFAULT false;
DEFINE FIELD has_cat ON users TYPE bool DEFAULT false;
DEFINE FIELD has_car ON users TYPE bool DEFAULT false;
DEFINE FIELD has_house ON users TYPE bool DEFAULT false;
DEFINE FIELD need_pet ON users TYPE bool DEFAULT false;
DEFINE FIELD need_car ON users TYPE bool DEFAULT false;
DEFINE FIELD need_house ON users TYPE bool DEFAULT false;
DEFINE FIELD open_for_travel_together ON users TYPE bool DEFAULT false;
DEFINE FIELD is_paid_travel ON users TYPE bool DEFAULT false;
DEFINE FIELD is_verified ON users TYPE bool DEFAULT false;
DEFINE FIELD is_banned ON users TYPE bool DEFAULT false;
DEFINE FIELD created_at ON users TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON users TYPE datetime VALUE time::now();

DEFINE INDEX user_email_index ON TABLE users COLUMNS email UNIQUE;
DEFINE INDEX user_name_index ON TABLE users COLUMNS username UNIQUE;
