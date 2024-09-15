use tower_sessions_redis_store::fred::prelude::{KeysInterface, RedisPool};
use tower_sessions_redis_store::fred::types::{Expiration, SetOptions};
use uuid::Uuid;

use crate::idempotency::IdempotencyKey;
use crate::Result;

// ---------------------------------------------------------------------------------------------------------------
#[derive(Debug)]
pub enum IdempotencyStatus {
    Processed,
    NotProcessed,
}
// ---------------------------------------------------------------------------------------------------------------
pub async fn get_key_status(
    redis_pool: &RedisPool,
    idempotency_key: &IdempotencyKey,
    user_id: Uuid,
) -> Result<IdempotencyStatus> {
    let redis_key = format!("idempotency:{}:{}", user_id, idempotency_key.as_ref());

    let processed: Option<String> = redis_pool.get(&redis_key).await?;

    // If a key exists, it means the request is already processed
    if processed.is_some() {
        return Ok(IdempotencyStatus::Processed);
    }

    // Request has not been processed yet
    Ok(IdempotencyStatus::NotProcessed)
}
// ---------------------------------------------------------------------------------------------------------------
pub async fn save_key_status(
    redis_pool: &RedisPool,
    idempotency_key: &IdempotencyKey,
    user_id: Uuid,
) -> Result<IdempotencyStatus> {
    let redis_key = format!("idempotency:{}:{}", user_id, idempotency_key.as_ref());

    let result: Option<String> = redis_pool
        .set(
            redis_key,
            "processed",
            Some(Expiration::EX(300)), // Set TTL to 5 minutes
            Some(SetOptions::NX),      // NX: Only set the key if it does not already exist
            true,
        )
        .await?;

    // If the key already existed (NX condition failed)
    if result.is_none() {
        return Ok(IdempotencyStatus::NotProcessed);
    }

    // Key has been saved
    Ok(IdempotencyStatus::Processed)
}
