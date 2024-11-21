use serde::{Deserialize, Serialize};
use unicode_segmentation::UnicodeSegmentation;
use uuid::Uuid;

use crate::{Error, Result};

use super::{SoftwareRequestDTO, UserDTO};

#[derive(Debug, Deserialize)]
pub struct SoftwareReview {
    pub id: Uuid,
    pub software_request_id: Uuid,
    pub reviewer_id: Uuid,
    pub is_supported: ReviewOptions,
    pub is_current_version: ReviewOptions,
    pub is_reputation_good: ReviewOptions,
    pub is_installation_from_developer: ReviewOptions,
    pub is_local_admin_required: ReviewOptions,
    pub is_connected_to_brockport_cloud: ReviewOptions,
    pub is_connected_to_cloud_services_or_client: ReviewOptions,
    pub is_security_or_optimization_software: ReviewOptions,
    pub is_supported_by_current_os: ReviewOptions,
    pub exported: Option<bool>,
    pub review_notes: Option<String>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    pub version: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct SoftwareReviewPayload {
    pub id: Option<Uuid>,
    pub software_request: SoftwareRequestDTO,
    pub reviewer_id: Option<Uuid>,
    pub is_supported: ReviewOptions,
    pub is_current_version: ReviewOptions,
    pub is_reputation_good: ReviewOptions,
    pub is_installation_from_developer: ReviewOptions,
    pub is_local_admin_required: ReviewOptions,
    pub is_connected_to_brockport_cloud: ReviewOptions,
    pub is_connected_to_cloud_services_or_client: ReviewOptions,
    pub is_security_or_optimization_software: ReviewOptions,
    pub is_supported_by_current_os: ReviewOptions,
    pub exported: Option<bool>,
    pub review_notes: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    pub version: Option<i32>,
}

#[derive(Clone, Debug, Deserialize, Serialize, sqlx::Type)]
#[sqlx(type_name = "review_options")]
#[allow(non_camel_case_types)]
pub enum ReviewOptions {
    TRUE,
    FALSE,
    NOT_SURE,
}

impl std::fmt::Display for ReviewOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReviewOptions::TRUE => write!(f, "TRUE"),
            ReviewOptions::FALSE => write!(f, "FALSE"),
            ReviewOptions::NOT_SURE => write!(f, "NOT_SURE"),
        }
    }
}

// Data Transfer Object (DTO) for SoftwareReview
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct SoftwareReviewDTO {
    pub id: Uuid,
    pub software_request: SoftwareRequestDTO,
    pub reviewer: UserDTO,
    pub is_supported: ReviewOptions,
    pub is_current_version: ReviewOptions,
    pub is_reputation_good: ReviewOptions,
    pub is_installation_from_developer: ReviewOptions,
    pub is_local_admin_required: ReviewOptions,
    pub is_connected_to_brockport_cloud: ReviewOptions,
    pub is_connected_to_cloud_services_or_client: ReviewOptions,
    pub is_security_or_optimization_software: ReviewOptions,
    pub is_supported_by_current_os: ReviewOptions,
    pub exported: Option<bool>,
    pub review_notes: Option<String>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl From<(&SoftwareReview, SoftwareRequestDTO, UserDTO)> for SoftwareReviewDTO {
    fn from(
        (review, software_request, reviewer): (&SoftwareReview, SoftwareRequestDTO, UserDTO),
    ) -> Self {
        SoftwareReviewDTO {
            id: review.id,
            software_request,
            reviewer,
            is_supported: review.is_supported.clone(),
            is_current_version: review.is_current_version.clone(),
            is_reputation_good: review.is_reputation_good.clone(),
            is_installation_from_developer: review.is_installation_from_developer.clone(),
            is_local_admin_required: review.is_local_admin_required.clone(),
            is_connected_to_brockport_cloud: review.is_connected_to_brockport_cloud.clone(),
            is_connected_to_cloud_services_or_client: review
                .is_connected_to_cloud_services_or_client
                .clone(),
            is_security_or_optimization_software: review
                .is_security_or_optimization_software
                .clone(),
            is_supported_by_current_os: review.is_supported_by_current_os.clone(),
            exported: review.exported,
            review_notes: review.review_notes.clone(),
            created_at: review.created_at,
        }
    }
}

impl SoftwareReview {
    pub fn parse(&self) -> Result<()> {
        if let Some(review_notes) = &self.review_notes {
            if !Self::validate_review_notes(review_notes) {
                return Err(Error::ValidationError(format!(
                    "software review payload: '{}' is invalid review notes",
                    review_notes
                )));
            }
        }

        Ok(())
    }

    fn validate_review_notes(notes: &String) -> bool {
        let forbidden_chars = ['/', '(', ')', '"', '<', '>', '\\', '{', '}', '$', '\'', '-'];

        let notes_is_empty_or_whitespace = notes.trim().is_empty();

        let notes_too_long = notes.graphemes(true).count() > 255;
        let notes_contains_forbidden_chars = notes.chars().any(|c| forbidden_chars.contains(&c));

        // Return false if any of the above conditions are met
        !(notes_is_empty_or_whitespace || notes_too_long || notes_contains_forbidden_chars)
    }
}

impl SoftwareReviewPayload {
    pub fn parse(&self) -> Result<()> {
        if !Self::validate_review_notes(&self.review_notes) {
            return Err(Error::ValidationError(format!(
                "software review payload: '{}' is invalid review notes",
                &self.review_notes
            )));
        }

        Ok(())
    }

    fn validate_review_notes(notes: &String) -> bool {
        let forbidden_chars = ['/', '(', ')', '"', '<', '>', '\\', '{', '}', '$', '\'', '-'];

        let notes_is_empty_or_whitespace = notes.trim().is_empty();

        let notes_too_long = notes.graphemes(true).count() > 255;
        let notes_contains_forbidden_chars = notes.chars().any(|c| forbidden_chars.contains(&c));

        // Return false if any of the above conditions are met
        !(notes_is_empty_or_whitespace || notes_too_long || notes_contains_forbidden_chars)
    }
}

// Unit Tests
#[cfg(test)]
mod review_notes_tests {
    use super::SoftwareReviewPayload;

    // Returns true is field is vaild, false if invalid

    #[test]
    fn a_255_grapheme_review_notes_is_vaild() {
        let review_notes = "a".repeat(255);
        assert_eq!(
            SoftwareReviewPayload::validate_review_notes(&review_notes),
            true
        );
    }

    #[test]
    fn a_256_grapheme_review_notes_is_invaild() {
        let review_notes = "a".repeat(256);
        assert_eq!(
            SoftwareReviewPayload::validate_review_notes(&review_notes),
            false
        );
    }

    #[test]
    fn whitespace_only_review_notes_is_invalid() {
        let review_notes = " ".to_string();
        assert_eq!(
            SoftwareReviewPayload::validate_review_notes(&review_notes),
            false
        );
    }

    #[test]
    fn empty_review_notes_is_invalid() {
        let review_notes = "".to_string();
        assert_eq!(
            SoftwareReviewPayload::validate_review_notes(&review_notes),
            false
        );
    }

    #[test]
    fn forbidden_characters_in_review_notes_are_invalid() {
        for chars in &['/', '(', ')', '"', '<', '>', '\\', '{', '}', '$', '\'', '-'] {
            let review_notes = chars.to_string();
            assert_eq!(
                SoftwareReviewPayload::validate_review_notes(&review_notes),
                false
            );
        }
    }

    #[test]
    fn valid_review_notes_is_accepted() {
        let review_notes = "Notes for the software review".to_string();
        assert_eq!(
            SoftwareReviewPayload::validate_review_notes(&review_notes),
            true
        );
    }
}
