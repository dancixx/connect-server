DEFINE TABLE goals_relations SCHEMAFULL PERMISSIONS NONE;

# This is the user who has the goal
DEFINE FIELD in ON goals_relations;
DEFINE FIELD out ON goals_relations;

DEFINE INDEX unique_relationship 
    ON TABLE goals_relations 
    COLUMNS in, out UNIQUE;