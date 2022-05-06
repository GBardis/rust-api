extern crate redis;

use chrono::{Utc};
use redis::{Client, Commands, RedisError};
use crate::errors::CustomError;

pub async fn create_client(redis_uri: String) -> Result<Client, RedisError> {
    Ok(Client::open(redis_uri)?)
}

pub async fn fetch_user_points(redis_client: &Client, user_id: &String) -> Result<i32, CustomError> {
    let mut redis_con = redis_client.get_connection()?;

    let user_points: i32 = redis_con
        .zscore(build_redis_key(), user_id).unwrap_or(0);

    Ok(user_points)
}

pub async fn fetch_user_rank(redis_client: &Client, user_id: &String) -> Result<i32, CustomError> {
    let mut redis_con = redis_client.get_connection()?;

    let user_rank: i32 = redis_con
        .zrank(build_redis_key(), user_id).unwrap_or(0);

    Ok(user_rank)
}

pub async fn fetch_total_users(redis_client: &Client) -> Result<i32, CustomError> {
    let mut redis_con = redis_client.get_connection()?;

    let total_users: i32 = redis_con.zcount(build_redis_key(), "-inf", " +inf")
        .unwrap_or(0);
    Ok(total_users)
}

fn build_redis_key() -> String {
    let date_time: String = Utc::now().format("%Y%m%d-%H").to_string();
    let redis_key: String = format!("lb_{}", date_time);
    return redis_key;
}