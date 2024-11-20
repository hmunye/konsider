use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{ReviewFactor, SoftwareReviewDTO};

#[derive(Debug, Deserialize)]
pub struct SoftwareReviewResponse {
    pub id: Option<Uuid>,
    pub software_review_id: Uuid,
    pub review_factor_id: Uuid,
    pub response: ReviewOptions,
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

// Data Transfer Object (DTO) for SoftwareReviewResponse
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct SoftwareReviewResponseDTO {
    pub id: Option<Uuid>,
    pub software_review: SoftwareReviewDTO,
    pub review_factor: ReviewFactor,
    pub response: ReviewOptions,
}

impl From<(&SoftwareReviewResponse, SoftwareReviewDTO, ReviewFactor)>
    for SoftwareReviewResponseDTO
{
    fn from(
        (review_response, software_review, review_factor): (
            &SoftwareReviewResponse,
            SoftwareReviewDTO,
            ReviewFactor,
        ),
    ) -> Self {
        SoftwareReviewResponseDTO {
            id: review_response.id,
            software_review,
            review_factor,
            response: review_response.response.clone(),
        }
    }
}
