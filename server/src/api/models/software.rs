use serde::{Deserialize, Serialize};
use unicode_segmentation::UnicodeSegmentation;

use crate::{Error, Result};

#[derive(Debug, Deserialize)]
pub struct Software {
    pub id: Option<uuid::Uuid>,
    pub software_name: String,
    pub software_version: String,
    pub developer_name: String,
    pub description: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    pub version: Option<i32>,
}

// Data Transfer Object (DTO) for Software
#[derive(Debug, Serialize, sqlx::FromRow, sqlx::Type)]
pub struct SoftwareDTO {
    pub id: Option<uuid::Uuid>,
    pub software_name: String,
    pub software_version: String,
    pub developer_name: String,
    pub description: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl From<&Software> for SoftwareDTO {
    fn from(software: &Software) -> Self {
        SoftwareDTO {
            id: software.id,
            software_name: software.software_name.clone(),
            software_version: software.software_version.clone(),
            developer_name: software.developer_name.clone(),
            description: software.description.clone(),
            created_at: software.created_at,
        }
    }
}

impl Software {
    pub fn parse(&self) -> Result<()> {
        if !Self::validate_software_name(&self.software_name) {
            return Err(Error::ValidationError(format!(
                "software payload: '{}' is an invaild software_name for software",
                &self.software_name
            )));
        }

        if !Self::validate_software_version(&self.software_version) {
            return Err(Error::ValidationError(format!(
                "software payload: '{}' is an invaild software version for software",
                &self.software_version
            )));
        }

        if !Self::validate_developer_name(&self.developer_name) {
            return Err(Error::ValidationError(format!(
                "software payload: '{}' is an invaild developer_name for software",
                &self.developer_name
            )));
        }

        if !Self::validate_description(&self.description) {
            return Err(Error::ValidationError(format!(
                "software payload: '{}' is an invaild description for software",
                &self.description
            )));
        }

        Ok(())
    }

    fn validate_software_name(name: &String) -> bool {
        let forbidden_chars = ['/', '(', ')', '"', '<', '>', '\\', '{', '}', '$', '\'', '-'];

        let name_is_empty_or_whitespace = name.trim().is_empty();

        let name_too_long = name.graphemes(true).count() > 100;
        let name_contains_forbidden_chars = name.chars().any(|c| forbidden_chars.contains(&c));

        // Return false if any of the above conditions are met
        !(name_is_empty_or_whitespace || name_too_long || name_contains_forbidden_chars)
    }

    fn validate_software_version(version: &str) -> bool {
        let parts: Vec<&str> = version.split('.').collect();

        // Ensure exactly 3 parts (x.y.z)
        if parts.len() != 3 {
            return false;
        }

        // Return false if any of the above conditions are met
        parts
            .iter()
            .all(|&part| !part.is_empty() && part.len() <= 4 && part.chars().all(char::is_numeric))
    }

    fn validate_developer_name(developer: &String) -> bool {
        let forbidden_chars = ['/', '(', ')', '"', '<', '>', '\\', '{', '}', '$', '\'', '-'];

        let developer_is_empty_or_whitespace = developer.trim().is_empty();

        let developer_too_long = developer.graphemes(true).count() > 100;
        let developer_contains_forbidden_chars =
            developer.chars().any(|c| forbidden_chars.contains(&c));

        // Return false if any of the above conditions are met
        !(developer_is_empty_or_whitespace
            || developer_too_long
            || developer_contains_forbidden_chars)
    }

    fn validate_description(description: &String) -> bool {
        let forbidden_chars = ['/', '(', ')', '"', '<', '>', '\\', '{', '}', '$', '\'', '-'];

        let description_is_empty_or_whitespace = description.trim().is_empty();

        let description_too_long = description.graphemes(true).count() > 255;
        let description_contains_forbidden_chars =
            description.chars().any(|c| forbidden_chars.contains(&c));

        // Return false if any of the above conditions are met
        !(description_is_empty_or_whitespace
            || description_too_long
            || description_contains_forbidden_chars)
    }
}

// Unit Tests
#[cfg(test)]
mod name_tests {
    use super::Software;

    // Returns true if field is valid, false if invalid

    #[test]
    fn a_100_grapheme_name_is_valid() {
        let name = "a".repeat(100);
        assert_eq!(Software::validate_software_name(&name), true);
    }

    #[test]
    fn a_101_grapheme_name_is_invalid() {
        let name = "a".repeat(101);
        assert_eq!(Software::validate_software_name(&name), false);
    }

    #[test]
    fn whitespace_only_name_is_invalid() {
        let name = " ".to_string();
        assert_eq!(Software::validate_software_name(&name), false);
    }

    #[test]
    fn empty_name_is_invalid() {
        let name = "".to_string();
        assert_eq!(Software::validate_software_name(&name), false);
    }

    #[test]
    fn forbidden_characters_in_name_are_invalid() {
        for chars in &['/', '(', ')', '"', '<', '>', '\\', '{', '}', '$', '\'', '-'] {
            let name = chars.to_string();
            assert_eq!(Software::validate_software_name(&name), false);
        }
    }

    #[test]
    fn valid_name_is_accepted() {
        let name = "Valid Software".to_string();
        assert_eq!(Software::validate_software_name(&name), true);
    }
}

#[cfg(test)]
mod version_tests {
    use super::Software;

    // Returns true if field is valid, false if invalid

    #[test]
    fn software_version_with_non_numeric_parts_is_invalid() {
        let versions = ["1.0.a", "1..0", "1.0.0.", ".1.0", "1.0.0.1"];
        for &version in &versions {
            assert_eq!(
                Software::validate_software_version(&version.to_string()),
                false
            );
        }
    }

    #[test]
    fn empty_software_version_is_invalid() {
        let version = "".to_string();
        assert_eq!(Software::validate_software_version(&version), false);
    }

    #[test]
    fn software_version_with_invalid_format_is_invalid() {
        let versions = ["1.0", "1.0.0.0", "1..0", "1.1.a"];
        for &version in &versions {
            assert_eq!(
                Software::validate_software_version(&version.to_string()),
                false
            );
        }
    }

    #[test]
    fn valid_software_version_is_accepted() {
        let version = "1.0.0".to_string();
        assert_eq!(Software::validate_software_version(&version), true);
    }
}

#[cfg(test)]
mod developer_tests {
    use super::Software;

    // Returns true if field is valid, false if invalid

    #[test]
    fn valid_developer_is_accepted() {
        let developer = "John Doe".to_string();
        assert_eq!(Software::validate_developer_name(&developer), true);
    }

    #[test]
    fn empty_developer_is_invalid() {
        let developer = "".to_string();
        assert_eq!(Software::validate_developer_name(&developer), false);
    }

    #[test]
    fn developer_longer_than_100_graphemes_is_invalid() {
        let developer = "a".repeat(101);
        assert_eq!(Software::validate_developer_name(&developer), false);
    }

    #[test]
    fn developer_with_forbidden_chars_is_invalid() {
        for chars in &['/', '(', ')', '"', '<', '>', '\\', '{', '}', '$', '\'', '-'] {
            let developer = format!("Developer {}", chars);
            assert_eq!(Software::validate_developer_name(&developer), false);
        }
    }

    #[test]
    fn valid_developer_name_is_accepted() {
        let developer = "Jane Doe".to_string();
        assert_eq!(Software::validate_developer_name(&developer), true);
    }
}

#[cfg(test)]
mod description_tests {
    use super::Software;

    // Returns true if field is valid, false if invalid

    #[test]
    fn description_longer_than_255_graphemes_is_invalid() {
        let description = "a".repeat(256);
        assert_eq!(Software::validate_description(&description), false);
    }

    #[test]
    fn empty_description_is_invalid() {
        let description = "".to_string();
        assert_eq!(Software::validate_description(&description), false);
    }

    #[test]
    fn whitespace_only_description_is_invalid() {
        let description = "   ".to_string();
        assert_eq!(Software::validate_description(&description), false);
    }

    #[test]
    fn description_with_forbidden_chars_is_invalid() {
        for chars in &['/', '(', ')', '"', '<', '>', '\\', '{', '}', '$', '\'', '-'] {
            let description = format!("Invalid Software {}", chars);
            assert_eq!(Software::validate_description(&description), false);
        }
    }

    #[test]
    fn valid_description_is_accepted() {
        let description = "This software helps with validation.".to_string();
        assert_eq!(Software::validate_description(&description), true);
    }
}
