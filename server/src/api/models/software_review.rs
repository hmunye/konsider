use serde::{Deserialize, Serialize};
use unicode_segmentation::UnicodeSegmentation;
use uuid::Uuid;

use crate::{Error, Result};

use super::{SoftwareRequestDTO, UserDTO};

#[derive(Debug, Deserialize)]
pub struct SoftwareReview {
    pub id: Option<Uuid>,
    pub software_request_id: Uuid,
    pub reviewer_id: Uuid,
    pub exported: bool,
    pub review_notes: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

// Data Transfer Object (DTO) for SoftwareReview
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct SoftwareReviewDTO {
    pub id: Option<Uuid>,
    pub software_request: SoftwareRequestDTO,
    pub reviewer: UserDTO,
    pub exported: bool,
    pub review_notes: String,
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
            exported: review.exported,
            review_notes: review.review_notes.clone(),
            created_at: review.created_at,
        }
    }
}

impl SoftwareReview {
    pub fn parse(&self) -> Result<()> {
        if !Self::validate_review_notes(&self.review_notes) {
            return Err(Error::ValidationError(format!(
                "software review payload: '{}' is invaild review notes",
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
    use super::SoftwareReview;

    // Returns true is field is vaild, false if invalid

    #[test]
    fn a_255_grapheme_review_notes_is_vaild() {
        let review_notes = "a".repeat(255);
        assert_eq!(SoftwareReview::validate_review_notes(&review_notes), true);
    }

    #[test]
    fn a_256_grapheme_review_notes_is_invaild() {
        let review_notes = "a".repeat(256);
        assert_eq!(SoftwareReview::validate_review_notes(&review_notes), false);
    }

    #[test]
    fn whitespace_only_review_notes_is_invalid() {
        let review_notes = " ".to_string();
        assert_eq!(SoftwareReview::validate_review_notes(&review_notes), false);
    }

    #[test]
    fn empty_review_notes_is_invalid() {
        let review_notes = "".to_string();
        assert_eq!(SoftwareReview::validate_review_notes(&review_notes), false);
    }

    #[test]
    fn forbidden_characters_in_review_notes_are_invalid() {
        for chars in &['/', '(', ')', '"', '<', '>', '\\', '{', '}', '$', '\'', '-'] {
            let review_notes = chars.to_string();
            assert_eq!(SoftwareReview::validate_review_notes(&review_notes), false);
        }
    }

    #[test]
    fn valid_review_notes_is_accepted() {
        let review_notes = "Notes for the software review".to_string();
        assert_eq!(SoftwareReview::validate_review_notes(&review_notes), true);
    }
}
