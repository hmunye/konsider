// Manages the in-memory cache of JWTs stored in Postgres
#![allow(dead_code)]

#[derive(Clone, Debug)]
struct TokenCache {
    revoked_tokens: std::sync::Arc<std::sync::RwLock<Vec<uuid::Uuid>>>,
}

impl TokenCache {
    fn new() -> Self {
        TokenCache {
            revoked_tokens: std::sync::Arc::new(std::sync::RwLock::new(Vec::new())),
        }
    }

    fn revoke_token(&self, jti: uuid::Uuid) {
        let mut cache = self.revoked_tokens.write().unwrap();
        if !cache.contains(&jti) {
            cache.push(jti);
        }
    }

    fn is_token_revoked(&self, jti: &uuid::Uuid) -> bool {
        let cache = self.revoked_tokens.read().unwrap();
        cache.contains(jti)
    }

    fn clear_cache(&self) {
        let mut cache = self.revoked_tokens.write().unwrap();
        cache.clear();
    }
}

// Background task to update the cache
async fn worker_loop(cache: TokenCache) {
    let polling_interval = tokio::time::Duration::from_secs(600); // 10 minutes

    let db_fetch_revoked_tokens = || {
        // Simulate fetching revoked tokens from the database
        vec![uuid::Uuid::new_v4()]
    };

    loop {
        let revoked_tokens = db_fetch_revoked_tokens();

        {
            let mut cache_write = cache.revoked_tokens.write().unwrap();
            for jti in revoked_tokens {
                if !cache_write.contains(&jti) {
                    cache_write.push(jti);
                }
            }
        }

        tokio::time::sleep(polling_interval).await;
    }
}
