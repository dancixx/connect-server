DEFINE TABLE interests_relations SCHEMAFULL PERMISSIONS NONE;

# This is the user who has the interest
DEFINE FIELD in ON interests_relations;
DEFINE FIELD out ON interests_relations;

DEFINE INDEX unique_relationship 
    ON TABLE interests_relations 
    COLUMNS in, out UNIQUE;