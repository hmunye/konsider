use sqlx::PgPool;

use crate::api::services::get_valid_tokens;
use crate::api::TokenCache;
use crate::config::DatabaseConfig;
use crate::server::get_db_pool;
use crate::Result;

pub async fn poll_and_update_token_cache(cache: TokenCache, config: DatabaseConfig) -> Result<()> {
    let db_pool = get_db_pool(&config)?;

    worker_loop(&cache, &db_pool).await
}

async fn worker_loop(cache: &TokenCache, db_pool: &PgPool) -> Result<()> {
    let polling_interval = tokio::time::Duration::from_secs(600); // 10 minutes

    loop {
        tracing::info!("[WORKER LOOP RUNNING]");

        let valid_tokens = get_valid_tokens(db_pool).await?;

        {
            for (jti, user_id) in valid_tokens {
                // Pass a reference to the jti when calling remove_token
                if !cache.is_token_valid(jti, user_id).await {
                    cache.remove_token(jti, user_id).await;
                }
            }
        }

        tokio::time::sleep(polling_interval).await;
    }
}
