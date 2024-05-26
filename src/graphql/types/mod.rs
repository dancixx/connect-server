pub mod surreal_datetime;
pub mod surreal_id;

pub struct UserID(pub String);

impl AsRef<String> for UserID {
    fn as_ref(&self) -> &String {
        &self.0
    }
}

impl From<UserID> for String {
    fn from(value: UserID) -> Self {
        value.0
    }
}
