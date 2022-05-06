use actix_web::{web};
use actix_web::HttpResponse;
use chrono::{Duration, Timelike, Utc};
use redis::Client;
use crate::AppClients;
use crate::errors::CustomError;
use crate::redis::{fetch_total_users, fetch_user_points, fetch_user_rank};
use crate::user::User;


const APPLICATION_JSON: &str = "application/json";

pub async fn users(app_clients: web::Data<AppClients>,
                   path: web::Path<u64>) -> Result<HttpResponse, CustomError> {
    let page = path.into_inner();
    let users = app_clients.mongodb_client
        .fetch_users(page)
        .await?;

    let mut users = get_user_rank_and_points(&app_clients.redis_client, users).await?;
    let total_user_count = get_total_uses_count(&app_clients.redis_client).await?;

    users.sort_by(|x, y| y.points.cmp(&x.points));

    Ok(HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .append_header(("TOTAL-USERS", total_user_count))
        .append_header(("TIME-REMAINING", time_remaining_until_next_leaderboard()))
        .json(users))
}

async fn get_user_rank_and_points(redis_client: &Client, users: Vec<User>) -> Result<Vec<User>, CustomError> {
    let mut updated_users = Vec::new();
    for mut user in users {
        let points = fetch_user_points(redis_client, &user.id).await?;
        user.points = points;
        let rank = fetch_user_rank(redis_client, &user.id).await?;
        user.rank = rank;
        updated_users.push(user);
    }

    Ok(updated_users)
}

async fn get_total_uses_count(redis_client: &Client) -> Result<i32, CustomError> {
    let total_users_count = fetch_total_users(redis_client).await?;
    Ok(total_users_count)
}

fn time_remaining_until_next_leaderboard() -> String {
    let now = Utc::now();
    let reset_time = (now + Duration::hours(1)).date().and_hms(now.hour() + 1, 0, 0);
    let remaining_time = reset_time.signed_duration_since(now).num_minutes();
    let format_time = format!("{} mins", remaining_time);
    return format_time;
}
