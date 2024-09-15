use secrecy::{ExposeSecret, Secret};
use serde::{Deserialize, Serialize};
use unicode_segmentation::UnicodeSegmentation;
use validator::ValidateEmail;

use crate::{Error, Result};

// ---------------------------------------------------------------------------------------------------------------
#[derive(Debug, Deserialize)]
pub struct User {
    pub name: String,
    pub email: String,
    pub password: Secret<String>,
    pub role: UserRole,
    pub version: Option<i32>,
}

#[derive(Clone, Debug, Deserialize, Serialize, sqlx::Type)]
#[sqlx(type_name = "user_role")]
pub enum UserRole {
    Reviewer,
    Admin,
}

impl std::fmt::Display for UserRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserRole::Reviewer => write!(f, "Reviewer"),
            UserRole::Admin => write!(f, "Admin"),
        }
    }
}

impl User {
    pub fn parse(&self) -> Result<()> {
        if !Self::validate_name(&self.name) {
            return Err(Error::UserValidationError(format!(
                "'{}' is an invaild name",
                &self.name
            )));
        }

        if !Self::validate_email(&self.email) {
            return Err(Error::UserValidationError(format!(
                "'{}' is an invaild email",
                &self.email
            )));
        }

        if !Self::validate_password(self.password.expose_secret()) {
            return Err(Error::UserValidationError(
                "invaild password provided".into(),
            ));
        }

        Ok(())
    }

    // When handling partial updates on users, if password is not changed, their password hash is
    // parsed, resulting in a failure because of `forbidden_chars` contained within it
    pub fn parse_without_password(&self) -> Result<()> {
        if !Self::validate_name(&self.name) {
            return Err(Error::UserValidationError(format!(
                "'{}' is an invaild name",
                &self.name
            )));
        }

        if !Self::validate_email(&self.email) {
            return Err(Error::UserValidationError(format!(
                "'{}' is an invaild email",
                &self.email
            )));
        }

        Ok(())
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
        let name_too_long = name.graphemes(true).count() > 128;
        let name_contains_forbidden_chars = name.chars().any(|c| forbidden_chars.contains(&c));

        !(name_is_empty_or_whitespace || name_too_long || name_contains_forbidden_chars)
    }

    fn validate_email(email: &String) -> bool {
        let split = email.split("@").nth(1);

        // `ValidateEmail` validates email based on HTML5 spec
        split.is_some_and(|str| !str.is_empty()) && ValidateEmail::validate_email(&email)
    }

    fn validate_password(password: &String) -> bool {
        let forbidden_chars = ['/', '(', ')', '"', '<', '>', '\\', '{', '}', '$'];

        // TODO: Make sure multiple spaces are handled (Replace multiple spaces with single space)
        //
        // TODO: Possibly check password against breached passwords such as the top 1,000 or 10,000
        // most common passwords
        let password_is_empty_or_whitespace = password.trim().is_empty();

        let password_too_short = password.graphemes(true).count() < 12;
        let password_too_long = password.graphemes(true).count() > 128;

        let password_contains_forbidden_chars =
            password.chars().any(|s| forbidden_chars.contains(&s));

        !(password_is_empty_or_whitespace
            || password_too_short
            || password_too_long
            || password_contains_forbidden_chars)
    }
}
// ---------------------------------------------------------------------------------------------------------------
// Unit Tests
#[cfg(test)]
mod name_tests {
    use crate::model::User;

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
        for chars in &['/', '(', ')', '"', '<', '>', '\\', '{', '}'] {
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

    use crate::model::User;

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
    use crate::model::User;

    // Returns true is field is vaild, false if invalid

    #[test]
    fn a_12_grapheme_password_is_vaild() {
        let password = "a".repeat(12);
        assert_eq!(User::validate_password(&password), true);
    }

    #[test]
    fn a_11_grapheme_password_is_invaild() {
        let password = "a".repeat(11);
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
        for chars in &['/', '(', ')', '"', '<', '>', '\\', '{', '}'] {
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
