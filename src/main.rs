extern crate core;

use std::env;
use ::redis::Client;
use crate::mongo_db::{MongoDbClient};
use actix_web::{App, HttpServer, web};
use actix_web::middleware::Logger;

mod leaderboard;
mod mongo_db;
mod user;
mod redis;
mod errors;
mod tests;
// mod tests;

pub struct AppClients {
    pub mongodb_client: MongoDbClient,
    pub redis_client: Client,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::from_filename(".env.local").ok();
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();
    let mongodb_uri = env::var("MONGODB_URI").expect("MONGODB_URI env var should be specified");
    let mongodb_client = MongoDbClient::new(mongodb_uri).await;
    let redis_uri = env::var("REDIS_URI").expect("REDIS_URI env var should be specified");
    let redis_client = redis::create_client(redis_uri).await.expect("Can't create Redis client");


    let clients = web::Data::new(AppClients {
        mongodb_client,
        redis_client,
    });

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(clients.clone())
            .service(
                web::scope("/api")
                    .route("/leaderboard/{page}",
                           web::get().to(leaderboard::users)),
            )
    })
        .bind(("127.0.0.1", 3000))?
        .run()
        .await
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use actix_web::{test, web, App};
//     use actix_web::test::TestRequest;
//     use crate::user::User;
//
//     #[actix_web::test]
//     async fn test_index() {
//         dotenv::from_filename(".env.local").ok();
//         env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
//         env_logger::init();
//         let mongodb_uri = env::var("MONGODB_URI").expect("MONGODB_URI env var should be specified");
//         let mongodb_client = MongoDbClient::new(mongodb_uri).await;
//         let redis_uri = env::var("REDIS_URI").expect("REDIS_URI env var should be specified");
//         let redis_client = redis::create_client(redis_uri).await.expect("Can't create Redis client");
//
//
//         let clients = web::Data::new(AppClients {
//             mongodb_client,
//             redis_client,
//         });
//
//         let mut app =
//             test::init_service(App::new()
//                 .app_data(clients.clone())
//                 .service(
//                     web::scope("/api")
//                         .route("/leaderboard/{page}",
//                                web::get().to(leaderboard::users)),
//                 ))
//                 .await;
//         let resp = TestRequest::get()
//             .uri(&format!("/api/leaderboard/{}", 1))
//             .send_request(&mut app).await;
//
//         assert!(resp.status().is_success(), "Failed to get user leaderboards");
//
//         let users: Vec<User> = test::read_body_json(resp).await;
//
//         assert_eq!(users.len(), 25, "Pagination not working");
//     }
// }