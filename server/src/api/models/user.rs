use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};
use unicode_segmentation::UnicodeSegmentation;
use validator::ValidateEmail;

use crate::{Error, Result};

#[derive(Debug, Deserialize)]
pub struct User {
    pub id: Option<uuid::Uuid>,
    pub name: String,
    pub email: String,
    pub password: SecretString,
    pub role: UserRole,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    pub version: Option<i32>,
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

// Data Transfer Object (DTO) for Users
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct UserDTO {
    pub id: Option<uuid::Uuid>,
    pub name: String,
    pub email: String,
    pub role: UserRole,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl From<&User> for UserDTO {
    fn from(user: &User) -> Self {
        UserDTO {
            id: user.id,
            name: user.name.clone(),
            email: user.email.clone(),
            role: user.role.clone(),
            created_at: user.created_at,
        }
    }
}

impl User {
    pub fn parse(&self) -> Result<()> {
        if !Self::validate_name(&self.name) {
            return Err(Error::ValidationError(format!(
                "user payload: '{}' is an invaild name for user",
                &self.name
            )));
        }

        if !Self::validate_email(&self.email) {
            return Err(Error::ValidationError(format!(
                "user payload: '{}' is an invaild email for user",
                &self.email
            )));
        }

        if !Self::validate_password(&self.password.expose_secret().to_string()) {
            return Err(Error::ValidationError(
                "user payload: invaild password provided for user".into(),
            ));
        }

        Ok(())
    }

    // When handling partial updates on users, their password hash is
    // parsed, resulting in a failure because of `forbidden_chars` contained within it
    pub fn parse_without_password(&self) -> Result<()> {
        if !Self::validate_name(&self.name) {
            return Err(Error::ValidationError(format!(
                "user payload: '{}' is an invaild name for user",
                &self.name
            )));
        }

        if !Self::validate_email(&self.email) {
            return Err(Error::ValidationError(format!(
                "user payload: '{}' is an invaild email for user",
                &self.email
            )));
        }

        Ok(())
    }

    fn validate_name(name: &String) -> bool {
        let forbidden_chars = ['/', '(', ')', '"', '<', '>', '\\', '{', '}', '$', '\'', '-'];

        let name_is_empty_or_whitespace = name.trim().is_empty();

        // A grapheme is defined by the Unicode standard as a "user-perceived"
        // character: `å` is a single grapheme, but it is composed of two characters
        // (`a` and `̊`)
        //
        // `graphemes` returns an iterator over the graphemes in the input
        // `true` specifies that we want to use the extended grapheme definition set
        let name_too_long = name.graphemes(true).count() > 128;
        let name_contains_forbidden_chars = name.chars().any(|c| forbidden_chars.contains(&c));

        // Return false if any of the above conditions are met
        !(name_is_empty_or_whitespace || name_too_long || name_contains_forbidden_chars)
    }

    fn validate_email(email: &String) -> bool {
        // Check if the email contains exactly one '@' symbol and has a domain
        let split = email.split('@').collect::<Vec<&str>>();

        if split.len() != 2 || split[1].is_empty() {
            return false;
        }

        // `ValidateEmail` validates email based on HTML5 spec
        ValidateEmail::validate_email(&email)
    }

    pub fn validate_password(password: &String) -> bool {
        let forbidden_chars = ['/', '(', ')', '"', '<', '>', '\\', '{', '}', '$', '\'', '-'];

        // TODO: Make sure multiple spaces are handled (Replace multiple spaces with single space)
        //
        // TODO: Possibly check password against breached passwords such as the top 1,000 or 10,000
        // most common passwords
        let password_is_empty_or_whitespace = password.trim().is_empty();

        let password_too_short = password.graphemes(true).count() < 8;
        let password_too_long = password.graphemes(true).count() > 128;

        let password_contains_forbidden_chars =
            password.chars().any(|s| forbidden_chars.contains(&s));

        // Return false if any of the above conditions are met
        !(password_is_empty_or_whitespace
            || password_too_short
            || password_too_long
            || password_contains_forbidden_chars)
    }
}

// Unit Tests
#[cfg(test)]
mod name_tests {
    use super::User;

    // Returns true is field is vaild, false if invalid

    #[test]
    fn a_128_grapheme_name_is_vaild() {
        let name = "a".repeat(128);
        assert_eq!(User::validate_name(&name), true);
    }

    #[test]
    fn a_129_grapheme_name_is_invaild() {
        let name = "a".repeat(129);
        assert_eq!(User::validate_name(&name), false);
    }

    #[test]
    fn whitespace_only_name_is_invalid() {
        let name = " ".to_string();
        assert_eq!(User::validate_name(&name), false);
    }

    #[test]
    fn empty_name_is_invalid() {
        let name = "".to_string();
        assert_eq!(User::validate_name(&name), false);
    }

    #[test]
    fn forbidden_characters_in_name_are_invalid() {
        for chars in &['/', '(', ')', '"', '<', '>', '\\', '{', '}', '$', '\'', '-'] {
            let name = chars.to_string();
            assert_eq!(User::validate_name(&name), false);
        }
    }

    #[test]
    fn valid_name_is_accepted() {
        let name = "John".to_string();
        assert_eq!(User::validate_name(&name), true);
    }
}

#[cfg(test)]
mod email_tests {
    use fake::faker::internet::en::SafeEmail;
    use fake::Fake;

    use super::User;

    // Returns true is field is vaild, false if invalid

    #[test]
    fn empty_email_is_rejected() {
        let email = "".to_string();
        assert_eq!(User::validate_email(&email), false);
    }

    #[test]
    fn email_missing_symbol_is_rejected() {
        let email = "johngmail.com".to_string();
        assert_eq!(User::validate_email(&email), false);
    }

    #[test]
    fn email_missing_subject_is_rejected() {
        let email = "@gmail.com".to_string();
        assert_eq!(User::validate_email(&email), false);
    }

    #[test]
    fn email_missing_domain_is_rejected() {
        let email = "test@".to_string();
        assert_eq!(User::validate_email(&email), false);
    }

    #[test]
    fn vaild_email_is_accepted() {
        let email = SafeEmail().fake();
        assert_eq!(User::validate_email(&email), true)
    }
}

#[cfg(test)]
mod password_tests {
    use super::User;

    // Returns true is field is vaild, false if invalid

    #[test]
    fn a_12_grapheme_password_is_vaild() {
        let password = "a".repeat(8);
        assert_eq!(User::validate_password(&password), true);
    }

    #[test]
    fn a_11_grapheme_password_is_invaild() {
        let password = "a".repeat(7);
        assert_eq!(User::validate_password(&password), false);
    }

    #[test]
    fn a_128_grapheme_password_is_vaild() {
        let password = "a".repeat(128);
        assert_eq!(User::validate_password(&password), true);
    }

    #[test]
    fn a_129_grapheme_password_is_invaild() {
        let password = "a".repeat(129);
        assert_eq!(User::validate_password(&password), false);
    }

    #[test]
    fn whitespace_only_password_is_rejected() {
        let password = " ".to_string();
        assert_eq!(User::validate_password(&password), false);
    }

    #[test]
    fn empty_password_is_rejected() {
        let password = "".to_string();
        assert_eq!(User::validate_password(&password), false);
    }

    #[test]
    fn forbidden_characters_in_password_are_rejected() {
        for chars in &['/', '(', ')', '"', '<', '>', '\\', '{', '}', '$', '\'', '-'] {
            let password = chars.to_string();
            assert_eq!(User::validate_password(&password), false);
        }
    }

    #[test]
    fn valid_password_is_accepted() {
        let password = "pdsfbhb#2kjL".to_string();
        assert_eq!(User::validate_password(&password), true);
    }
}
