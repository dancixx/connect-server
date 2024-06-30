DEFINE TABLE match TYPE RELATION FROM user TO user SCHEMAFULL PERMISSIONS none;

DEFINE FIELD id ON match TYPE record<match>;
DEFINE FIELD in ON match TYPE record<user>;
DEFINE FIELD out ON match TYPE record<user>;
DEFINE FIELD in_swipe ON match TYPE option<bool>;
DEFINE FIELD out_swipe ON match TYPE option<bool>;
DEFINE FIELD is_super_swipe ON match TYPE bool DEFAULT false;
DEFINE FIELD is_unmatched ON match TYPE bool DEFAULT false;
DEFINE FIELD unmatched_at ON match TYPE option<datetime>;
DEFINE FIELD message_count ON match
    VALUE <future> {array::len(SELECT count() FROM message WHERE $this.in = $parent.id)};
DEFINE FIELD unread_message_count_by_in ON match
    VALUE <future> {array::len(SELECT count() FROM message WHERE $this.out = $parent.out && !$this.is_read)};
DEFINE FIELD unread_message_count_by_out ON match
    VALUE <future> {array::len(SELECT count() FROM message WHERE $this.out = $parent.in && !$this.is_read)};
DEFINE FIELD disabled_by_in ON match TYPE bool DEFAULT false;
DEFINE FIELD disabled_by_out ON match TYPE bool DEFAULT false;
DEFINE FIELD created_at ON match TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON match TYPE datetime VALUE time::now();
DEFINE FIELD last_message on match
    VALUE <future> {
        LET $message = SELECT id, created_at FROM message WHERE $this.in = $parent.id ORDER BY created_at DESC LIMIT 1;
        LET $id = array::first($message).id;
        RETURN $id;
    };