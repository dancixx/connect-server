pub mod surreal_datetime;
pub mod surreal_id;

pub struct UserID(pub String);

impl AsRef<String> for UserID {
    fn as_ref(&self) -> &String {
        &self.0
    }
}

impl Into<String> for UserID {
    fn into(self) -> String {
        self.0
    }
}
