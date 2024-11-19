use serde::{Deserialize, Serialize};
use unicode_segmentation::UnicodeSegmentation;
use validator::ValidateEmail;

use crate::{Error, Result};

#[derive(Debug, Deserialize)]
pub struct Requester {
    pub id: Option<uuid::Uuid>,
    pub name: String,
    pub email: String,
    pub department: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    pub version: Option<i32>,
}

// Data Transfer Object (DTO) for Requester
#[derive(Debug, Serialize, sqlx::FromRow, sqlx::Type)]
pub struct RequesterDTO {
    pub id: Option<uuid::Uuid>,
    pub name: String,
    pub email: String,
    pub department: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl From<&Requester> for RequesterDTO {
    fn from(request: &Requester) -> Self {
        RequesterDTO {
            id: request.id,
            name: request.name.clone(),
            email: request.email.clone(),
            department: request.department.clone(),
            created_at: request.created_at,
        }
    }
}

impl Requester {
    pub fn parse(&self) -> Result<()> {
        if !Self::validate_name(&self.name) {
            return Err(Error::ValidationError(format!(
                "requests payload: '{}' is an invaild name for requests",
                &self.name
            )));
        }

        if !Self::validate_email(&self.email) {
            return Err(Error::ValidationError(format!(
                "requests payload: '{}' is an invaild email for requests",
                &self.email
            )));
        }

        if !Self::validate_department(&self.department) {
            return Err(Error::ValidationError(format!(
                "requests payload: '{}' is an invaild department for requests",
                &self.department
            )));
        }

        Ok(())
    }

    fn validate_name(name: &String) -> bool {
        let forbidden_chars = ['/', '(', ')', '"', '<', '>', '\\', '{', '}', '$', '\'', '-'];

        let name_is_empty_or_whitespace = name.trim().is_empty();

        let name_too_long = name.graphemes(true).count() > 100;
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

    fn validate_department(department: &String) -> bool {
        let forbidden_chars = ['/', '(', ')', '"', '<', '>', '\\', '{', '}', '$', '\'', '-'];

        let department_is_empty_or_whitespace = department.trim().is_empty();

        let department_too_long = department.graphemes(true).count() > 100;

        let department_contains_forbidden_chars =
            department.chars().any(|c| forbidden_chars.contains(&c));

        // Return false if any of the above conditions are met
        !(department_is_empty_or_whitespace
            || department_too_long
            || department_contains_forbidden_chars)
    }
}

// Unit tests
#[cfg(test)]
mod name_tests {
    use super::Requester;

    // Returns true if field is valid, false if invalid

    #[test]
    fn a_100_grapheme_name_is_valid() {
        let name = "a".repeat(100);
        assert_eq!(Requester::validate_name(&name), true);
    }

    #[test]
    fn a_101_grapheme_name_is_invalid() {
        let name = "a".repeat(101);
        assert_eq!(Requester::validate_name(&name), false);
    }

    #[test]
    fn whitespace_only_name_is_invalid() {
        let name = "   ".to_string();
        assert_eq!(Requester::validate_name(&name), false);
    }

    #[test]
    fn empty_name_is_invalid() {
        let name = "".to_string();
        assert_eq!(Requester::validate_name(&name), false);
    }

    #[test]
    fn forbidden_characters_in_name_are_invalid() {
        for chars in &['/', '(', ')', '"', '<', '>', '\\', '{', '}', '$', '\'', '-'] {
            let name = format!("Invalid Name {}", chars);
            assert_eq!(Requester::validate_name(&name), false);
        }
    }

    #[test]
    fn valid_name_is_accepted() {
        let name = "John Doe".to_string();
        assert_eq!(Requester::validate_name(&name), true);
    }
}

#[cfg(test)]
mod email_tests {
    use fake::faker::internet::en::SafeEmail;
    use fake::Fake;

    use super::Requester;

    // Returns true if field is valid, false if invalid

    #[test]
    fn empty_email_is_rejected() {
        let email = "".to_string();
        assert_eq!(Requester::validate_email(&email), false);
    }

    #[test]
    fn email_missing_symbol_is_rejected() {
        let email = "johngmail.com".to_string();
        assert_eq!(Requester::validate_email(&email), false);
    }

    #[test]
    fn email_missing_subject_is_rejected() {
        let email = "@gmail.com".to_string();
        assert_eq!(Requester::validate_email(&email), false);
    }

    #[test]
    fn email_missing_domain_is_rejected() {
        let email = "test@".to_string();
        assert_eq!(Requester::validate_email(&email), false);
    }

    #[test]
    fn valid_email_is_accepted() {
        let email = SafeEmail().fake();
        assert_eq!(Requester::validate_email(&email), true);
    }
}

#[cfg(test)]
mod department_tests {
    use super::Requester;

    // Returns true is field is vaild, false if invalid

    #[test]
    fn valid_department_is_accepted() {
        let department = "Engineering".to_string();
        assert_eq!(Requester::validate_department(&department), true);
    }

    #[test]
    fn department_with_forbidden_chars_is_invalid() {
        for chars in &['/', '(', ')', '"', '<', '>', '\\', '{', '}', '$', '\'', '-'] {
            let department = format!("Invalid Department {}", chars);
            assert_eq!(Requester::validate_department(&department), false);
        }
    }

    #[test]
    fn empty_department_is_invalid() {
        let department = "".to_string();
        assert_eq!(Requester::validate_department(&department), false);
    }

    #[test]
    fn whitespace_only_department_is_invalid() {
        let department = "   ".to_string();
        assert_eq!(Requester::validate_department(&department), false);
    }

    #[test]
    fn department_longer_than_100_graphemes_is_invalid() {
        let department = "a".repeat(101);
        assert_eq!(Requester::validate_department(&department), false);
    }
}
