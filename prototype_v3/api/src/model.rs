use serde::Deserialize;
use unicode_segmentation::UnicodeSegmentation;

use crate::{Error, Result};

//----------------------------------------------------------------------
#[derive(Debug, Deserialize)]
pub struct User {
    pub name: String,

    pub email: String,

    pub password: String,

    pub role: UserRole,
}

#[derive(Clone, Debug, Deserialize, sqlx::Type)]
#[sqlx(type_name = "user_role")]
pub enum UserRole {
    Reviewer,
    Admin,
}

impl User {
    pub fn parse(&self) -> Result<()> {
        if Self::validate_name(&self.name)
            || Self::validate_email(&self.email)
            || Self::validate_password(&self.password)
        {
            Err(Error::CreateUserFail)
        } else {
            Ok(())
        }
    }

    fn validate_name(name: &String) -> bool {
        let forbidden_chars = ['/', '(', ')', '"', '<', '>', '\\', '{', '}', '$'];

        let name_is_empty_or_whitespace = name.trim().is_empty();

        // A grapheme is defined by the Unicode standard as a "user-perceived"
        // character: `å` is a single grapheme, but it is composed of two characters
        // (`a` and `̊`)
        //
        // `graphemes` returns an iterator over the graphemes in the input
        // `true` specifies that we want to use the extended grapheme definition set
        let name_too_long = name.graphemes(true).count() > 256;
        let name_contains_forbidden_chars = name.chars().any(|c| forbidden_chars.contains(&c));

        name_is_empty_or_whitespace || name_too_long || name_contains_forbidden_chars
    }

    fn validate_email(email: &String) -> bool {
        let forbidden_chars = ['/', '(', ')', '"', '<', '>', '\\', '{', '}', '$'];

        let email_is_empty_or_whitespace = email.trim().is_empty();
        let email_too_long = email.graphemes(true).count() > 256;
        let email_contains_forbidden_chars = email.chars().any(|s| forbidden_chars.contains(&s));

        email_is_empty_or_whitespace || email_too_long || email_contains_forbidden_chars
    }

    fn validate_password(password: &String) -> bool {
        let forbidden_chars = ['/', '(', ')', '"', '<', '>', '\\', '{', '}', '$'];

        let password_is_empty_or_whitespace = password.trim().is_empty();
        let password_too_long = password.graphemes(true).count() > 256;
        let password_too_short = password.graphemes(true).count() < 8;
        let password_contains_forbidden_chars =
            password.chars().any(|s| forbidden_chars.contains(&s));

        password_is_empty_or_whitespace
            || password_too_long
            || password_too_short
            || password_contains_forbidden_chars
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
// Unit Tests

#[cfg(test)]
mod tests {
    use crate::model::User;

    // Returns true is field is invalid, false if valid

    #[test]
    fn a_256_grapheme_field_is_vaild() {
        let value = "a".repeat(256);
        assert_eq!(User::validate_name(&value), false);
        assert_eq!(User::validate_email(&value), false);
        assert_eq!(User::validate_password(&value), false);
    }

    #[test]
    fn a_257_grapheme_field_is_invaild() {
        let value = "a".repeat(257);
        assert_eq!(User::validate_name(&value), true);
        assert_eq!(User::validate_email(&value), true);
        assert_eq!(User::validate_password(&value), true);
    }

    #[test]
    fn whitespace_only_is_rejected() {
        let value = " ".to_string();
        assert_eq!(User::validate_name(&value), true);
        assert_eq!(User::validate_email(&value), true);
        assert_eq!(User::validate_password(&value), true);
    }

    #[test]
    fn empty_string_is_rejected() {
        let value = "".to_string();
        assert_eq!(User::validate_name(&value), true);
        assert_eq!(User::validate_email(&value), true);
        assert_eq!(User::validate_password(&value), true);
    }

    #[test]
    fn forbidden_characters_are_rejected() {
        for value in &['/', '(', ')', '"', '<', '>', '\\', '{', '}'] {
            let value = value.to_string();
            assert_eq!(User::validate_name(&value), true);
            assert_eq!(User::validate_email(&value), true);
            assert_eq!(User::validate_password(&value), true);
        }
    }

    #[test]
    fn valid_field_return_false() {
        let name = "John".to_string();
        let email = "john@gmail.com".to_string();
        let password = "password@123".to_string();

        assert_eq!(User::validate_name(&name), false);
        assert_eq!(User::validate_email(&email), false);
        assert_eq!(User::validate_password(&password), false);
    }
}
