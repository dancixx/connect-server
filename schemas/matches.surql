DEFINE TABLE matches SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD in ON TABLE matches;
DEFINE FIELD out ON TABLE matches;
DEFINE FIELD created_at ON matches TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON matches TYPE datetime VALUE time::now();

DEFINE INDEX unique_relationship 
    ON TABLE matches 
    COLUMNS in, out UNIQUE;
