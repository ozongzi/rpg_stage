use crate::domains::{Email, UserName};
use serde::Serialize;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize)]
pub struct User {
    id: Uuid,
    name: UserName,
    email: Email,
    password_hash: String,
}
impl User {
    pub fn new(id: Uuid, name: UserName, email: Email, password_hash: String) -> Self {
        Self {
            id,
            name,
            email,
            password_hash,
        }
    }

    pub fn password_hash(&self) -> &str {
        &self.password_hash
    }

    pub fn id(&self) -> Uuid {
        self.id
    }
    pub fn name(&self) -> &UserName {
        &self.name
    }
    pub fn email(&self) -> &Email {
        &self.email
    }
}
