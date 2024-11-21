use serde::{Deserialize, Serialize};

use crate::{Error, Result};

use super::{RequesterDTO, SoftwareDTO};

#[derive(Debug, Deserialize)]
pub struct SoftwareRequest {
    pub id: Option<uuid::Uuid>,
    pub td_request_id: String,
    pub software_id: uuid::Uuid,
    pub requester_id: uuid::Uuid,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    pub version: Option<i32>,
}

// Data Transfer Object (DTO) for SoftwareRequest
#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct SoftwareRequestDTO {
    pub id: Option<uuid::Uuid>,
    pub td_request_id: String,
    pub software: SoftwareDTO,
    pub requester: RequesterDTO,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl From<(&SoftwareRequest, SoftwareDTO, RequesterDTO)> for SoftwareRequestDTO {
    fn from((request, software, requester): (&SoftwareRequest, SoftwareDTO, RequesterDTO)) -> Self {
        SoftwareRequestDTO {
            id: request.id,
            td_request_id: request.td_request_id.clone(),
            software,
            requester,
            created_at: request.created_at,
        }
    }
}

impl SoftwareRequest {
    pub fn parse(&self) -> Result<()> {
        if !Self::validate_td_request_id(&self.td_request_id) {
            return Err(Error::ValidationError(format!(
                "requests payload: '{}' is an invaild td_request_id for requests",
                self.td_request_id
            )));
        }

        Ok(())
    }

    fn validate_td_request_id(id: &str) -> bool {
        let forbidden_chars = ['/', '(', ')', '"', '<', '>', '\\', '{', '}', '$', '\'', '-'];

        if id.trim().is_empty() {
            return false;
        }

        if id.chars().any(|c| forbidden_chars.contains(&c)) {
            return false;
        }

        id.len() == 8 && id.chars().all(|c| c.is_ascii_digit())
    }
}

impl SoftwareRequestDTO {
    pub fn parse(&self) -> Result<()> {
        if !Self::validate_td_request_id(&self.td_request_id) {
            return Err(Error::ValidationError(format!(
                "requests payload: '{}' is an invaild td_request_id for requests",
                self.td_request_id
            )));
        }

        Ok(())
    }

    fn validate_td_request_id(id: &str) -> bool {
        let forbidden_chars = ['/', '(', ')', '"', '<', '>', '\\', '{', '}', '$', '\'', '-'];

        if id.trim().is_empty() {
            return false;
        }

        if id.chars().any(|c| forbidden_chars.contains(&c)) {
            return false;
        }

        id.len() == 8 && id.chars().all(|c| c.is_ascii_digit())
    }
}

// Unit tests
#[cfg(test)]
mod request_id_tests {
    use super::SoftwareRequest;

    #[test]
    fn valid_request_id_is_accepted() {
        let id = String::from("12345678");
        assert!(SoftwareRequest::validate_td_request_id(&id));
    }

    #[test]
    fn invalid_request_id_too_short() {
        let id = String::from("123");
        assert!(!SoftwareRequest::validate_td_request_id(&id));
    }

    #[test]
    fn invalid_request_id_too_long() {
        let id = String::from("123456789");
        assert!(!SoftwareRequest::validate_td_request_id(&id));
    }

    #[test]
    fn invalid_request_id_non_digit() {
        let id = String::from("1234abcd");
        assert!(!SoftwareRequest::validate_td_request_id(&id));
    }

    #[test]
    fn empty_request_id_is_invalid() {
        let id = String::from("");
        assert!(!SoftwareRequest::validate_td_request_id(&id));
    }
}
