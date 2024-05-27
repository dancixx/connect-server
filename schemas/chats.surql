DEFINE TABLE chats SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD in ON TABLE chats;
DEFINE FIELD out ON TABLE chats;
DEFINE FIELD disabled_by_in ON chats TYPE bool DEFAULT false;
DEFINE FIELD disabled_by_out ON chats TYPE bool DEFAULT false;
DEFINE FIELD created_at ON chats TYPE datetime DEFAULT time::now();
// TODO: Trigger if new messags
DEFINE FIELD updated_at ON chats TYPE datetime VALUE time::now();

DEFINE INDEX unique_relationship 
    ON TABLE chats 
    COLUMNS in, out UNIQUE;

// TODO: Messages count