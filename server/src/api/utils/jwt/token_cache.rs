// Manages the in-memory cache of JWTs stored in Postgres

use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct TokenCache {
    valid_tokens: std::sync::Arc<tokio::sync::RwLock<HashSet<(uuid::Uuid, uuid::Uuid)>>>,
}

impl Default for TokenCache {
    fn default() -> Self {
        Self::new()
    }
}

impl TokenCache {
    pub fn new() -> Self {
        TokenCache {
            valid_tokens: std::sync::Arc::new(tokio::sync::RwLock::new(HashSet::new())),
        }
    }

    // Insert a token with the associated user_id into the cache
    pub async fn insert_token(&self, jti: uuid::Uuid, user_id: uuid::Uuid) {
        let mut cache = self.valid_tokens.write().await;

        cache.insert((jti, user_id));
    }

    // Remove a token from the cache by its jti and associated user_id
    pub async fn remove_token(&self, jti: uuid::Uuid, user_id: uuid::Uuid) {
        let mut cache = self.valid_tokens.write().await;

        cache.remove(&(jti, user_id));
    }

    // Check if the token with a specific jti and user_id is in cache
    pub async fn is_token_valid(&self, jti: uuid::Uuid, user_id: uuid::Uuid) -> bool {
        let cache = self.valid_tokens.read().await;

        cache.contains(&(jti, user_id))
    }

    pub async fn clear_cache(&self) {
        let mut cache = self.valid_tokens.write().await;

        cache.clear();
    }
}
