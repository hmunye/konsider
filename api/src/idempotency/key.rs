use serde::Deserialize;

use crate::Error;

// ---------------------------------------------------------------------------------------------------------------
#[derive(Debug, Deserialize)]
pub struct IdempotencyKey(String);

impl TryFrom<String> for IdempotencyKey {
    type Error = Error;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        if s.is_empty() {
            return Err(Error::UnexpectedError(
                std::sync::Arc::new(Error::IdempotencyKeyError),
                "The idempotency key cannot be empty".into(),
            ));
        }

        let max_length = 50;

        if s.len() >= max_length {
            return Err(Error::UnexpectedError(
                std::sync::Arc::new(Error::IdempotencyKeyError),
                format!(
                    "The idempotency key must be shorter than {} characters",
                    max_length
                ),
            ));
        }

        let forbidden_chars = ['/', '(', ')', '"', '<', '>', '\\', '{', '}', '$'];

        if s.chars().any(|s| forbidden_chars.contains(&s)) {
            return Err(Error::UnexpectedError(
                std::sync::Arc::new(Error::IdempotencyKeyError),
                "The idempotency key contains forbidden characters".into(),
            ));
        }

        Ok(Self(s))
    }
}

impl From<IdempotencyKey> for String {
    fn from(key: IdempotencyKey) -> Self {
        key.0
    }
}

impl AsRef<str> for IdempotencyKey {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
