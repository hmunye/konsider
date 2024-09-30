use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------------------------------------------
#[derive(Debug, Deserialize)]
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

#[derive(Clone, Debug, Deserialize, Serialize, sqlx::Type)]
#[sqlx(type_name = "review_status")]
pub enum ReviewStatus {
    UnderReview,
    Complete,
}
