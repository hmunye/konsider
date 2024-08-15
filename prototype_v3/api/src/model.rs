use serde::{Deserialize, Serialize};

use crate::{Error, Result};

//----------------------------------------------------------------------
#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub name: String,

    pub email: String,

    pub password: String,

    pub role: UserRole,
}

#[derive(Debug, Deserialize, Serialize, sqlx::Type)]
#[sqlx(type_name = "user_role")]
pub enum UserRole {
    Reviewer,
    Admin,
}

impl User {
    pub fn validate_new_user(&self) -> Result<()> {
        if self.name.is_empty() || self.name.len() > 50 {
            return Err(Error::CreateUserFail);
        }

        if self.email.is_empty() || self.email.len() > 50 {
            return Err(Error::CreateUserFail);
        }

        // Validates password before it is hashed
        if self.password.is_empty() || self.password.len() < 8 {
            return Err(Error::CreateUserFail);
        }

        Ok(())
    }
}
//----------------------------------------------------------------------
#[derive(Debug)]
pub struct Requester {
    pub request_id: i32,

    pub name: String,

    pub email: String,

    pub department: String,
}
//----------------------------------------------------------------------
#[derive(Debug)]
pub struct Software {
    pub name: String,

    pub version: f32,

    pub description: String,

    pub developer: String,
}
//----------------------------------------------------------------------
#[derive(Debug)]
pub struct Review {
    pub software_name: String,

    pub request_id: i32,

    pub reviewer_name: String,

    pub is_supported: bool,

    pub is_current_version: bool,

    pub is_reputation_good: bool,

    pub is_installation_from_developer: bool,

    pub is_local_admin_required: bool,

    pub is_connected_to_brockport_cloud: bool,

    pub is_connected_to_cloud_services_or_client: bool,

    pub is_security_or_optimization_software: bool,

    pub is_supported_by_current_os: bool,

    pub exported: bool,

    pub notes: String,

    pub status: ReviewStatus,
}

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "review_status")]
pub enum ReviewStatus {
    UnderReview,
    Complete,
}
//----------------------------------------------------------------------
