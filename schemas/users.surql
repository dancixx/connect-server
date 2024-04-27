DEFINE TABLE users SCHEMAFULL PERMISSIONS NONE;
    -- PERMISSIONS
    --     FOR create NONE
    --     FOR select, update, delete WHERE id = $auth.id;

DEFINE FIELD name ON users TYPE string;
DEFINE FIELD email ON users TYPE string
    ASSERT string::is::email($value);
DEFINE INDEX user_email_index ON TABLE users COLUMNS email UNIQUE;
DEFINE FIELD created_at ON users TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON users TYPE datetime VALUE time::now();

-- DEFINE SCOPE account
--     SESSION 24h
--     SIGNIN ( SELECT * FROM users WHERE id = $auth.id )
-- ;