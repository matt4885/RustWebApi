use uuid::Uuid;

pub struct User {
    pub id: Uuid,
    pub given_name: String,
    pub family_name: String,
    pub email: String,
}
