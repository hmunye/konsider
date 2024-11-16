use secrecy::SecretString;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct User {
    pub id: Option<uuid::Uuid>,
    pub name: String,
    pub email: String,
    pub password: SecretString,
    pub role: UserRole,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Clone, Debug, Deserialize, Serialize, sqlx::Type)]
#[sqlx(type_name = "user_role")]
#[allow(non_camel_case_types)]
pub enum UserRole {
    REVIEWER,
    ADMIN,
}

impl std::fmt::Display for UserRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserRole::REVIEWER => write!(f, "REVIEWER"),
            UserRole::ADMIN => write!(f, "ADMIN"),
        }
    }
}
