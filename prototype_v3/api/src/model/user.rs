use serde::Deserialize;
use unicode_segmentation::UnicodeSegmentation;
use validator::ValidateEmail;

use crate::{Error, Result};

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
        if !Self::validate_name(&self.name)
            || !Self::validate_email(&self.email)
            || !Self::validate_password(&self.password)
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

        !(name_is_empty_or_whitespace || name_too_long || name_contains_forbidden_chars)
    }

    fn validate_email(email: &String) -> bool {
        ValidateEmail::validate_email(&email)
    }

    // TODO: Update password validation
    fn validate_password(password: &String) -> bool {
        let forbidden_chars = ['/', '(', ')', '"', '<', '>', '\\', '{', '}', '$'];

        let password_is_empty_or_whitespace = password.trim().is_empty();
        let password_too_long = password.graphemes(true).count() > 256;
        let password_too_short = password.graphemes(true).count() < 8;
        let password_contains_forbidden_chars =
            password.chars().any(|s| forbidden_chars.contains(&s));

        !(password_is_empty_or_whitespace
            || password_too_long
            || password_too_short
            || password_contains_forbidden_chars)
    }
}

// Unit Tests

#[cfg(test)]
mod name_tests {
    use crate::model::User;

    // Returns true is field is vaild, false if invalid

    #[test]
    fn a_256_grapheme_name_is_vaild() {
        let name = "a".repeat(256);
        assert_eq!(User::validate_name(&name), true);
    }

    #[test]
    fn a_257_grapheme_name_is_invaild() {
        let name = "a".repeat(257);
        assert_eq!(User::validate_name(&name), false);
    }

    #[test]
    fn whitespace_only_name_is_rejected() {
        let name = " ".to_string();
        assert_eq!(User::validate_name(&name), false);
    }

    #[test]
    fn empty_name_is_rejected() {
        let name = "".to_string();
        assert_eq!(User::validate_name(&name), false);
    }

    #[test]
    fn forbidden_characters_in_name_are_rejected() {
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
    fn vaild_email_is_accepted() {
        let email = SafeEmail().fake();
        assert_eq!(User::validate_email(&email), true)
    }
}

#[cfg(test)]
mod password_tests {
    // Returns true is field is vaild, false if invalid
}
