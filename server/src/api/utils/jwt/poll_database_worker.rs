use sqlx::PgPool;

use crate::api::services::get_valid_tokens;
use crate::api::utils::jwt::TokenCache;
use crate::config::DatabaseConfig;
use crate::server::get_db_pool;
use crate::Result;

pub async fn poll_and_update_token_cache(cache: TokenCache, config: DatabaseConfig) -> Result<()> {
    let db_pool = get_db_pool(&config)?;

    poll_database_task(&cache, &db_pool).await
}

#[tracing::instrument(name = "poll database running", skip(cache, db_pool))]
async fn poll_database_task(cache: &TokenCache, db_pool: &PgPool) -> Result<()> {
    let polling_interval = tokio::time::Duration::from_secs(600); // 10 minutes

    loop {
        let valid_tokens = get_valid_tokens(db_pool).await?;

        {
            for (jti, user_id) in valid_tokens {
                if !cache.is_token_valid(jti, user_id).await {
                    cache.remove_token(jti, user_id).await;
                }
            }
        }

        tokio::time::sleep(polling_interval).await;
    }
}
