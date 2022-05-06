#[cfg(test)]
mod tests {
    use std::env;
    use actix_web::{test, web, App};
    use actix_web::test::TestRequest;
    use mongodb::bson::{Bson, doc};
    use crate::{leaderboard, MongoDbClient};
    use crate::user::User;
    use crate::redis::create_client;

    #[actix_web::test]
    async fn test_user_leaderboards() {
        dotenv::from_filename(".env.local").ok();
        env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
        env_logger::init();
        let mongodb_uri = env::var("MONGODB_URI").expect("MONGODB_URI env var should be specified");
        let mongodb_client = MongoDbClient::new(mongodb_uri).await;
        let redis_uri = env::var("REDIS_URI").expect("REDIS_URI env var should be specified");
        let redis_client = create_client(redis_uri).await.expect("Can't create Redis client");


        let clients = web::Data::new(crate::AppClients {
            mongodb_client,
            redis_client,
        });

        let mut app =
            test::init_service(App::new()
                .app_data(clients.clone())
                .service(
                    web::scope("/api")
                        .route("/leaderboard/{page}",
                               web::get().to(leaderboard::users)),
                ))
                .await;
        let resp = TestRequest::get()
            .uri(&format!("/api/leaderboard/{}", 1))
            .send_request(&mut app).await;

        assert!(resp.status().is_success(), "Failed to get user leaderboards");

        let users: Vec<User> = test::read_body_json(resp).await;

        assert_eq!(users.len(), 25, "Pagination not working");
    }

    #[test]
    async fn test_document_convert_to_user() {
        dotenv::from_filename(".env.local").ok();
        let mongodb_uri = env::var("MONGODB_URI").expect("MONGODB_URI env var should be specified");
        let mongodb_client = MongoDbClient::new(mongodb_uri).await;
        let id = Bson::ObjectId("6274ffb0e6ebb9fdfddd1a12".parse().unwrap());

        let document = doc! {
            "_id": id,
            "country": "Qatar",
            "avatar": "https://via.placeholder.com/150.jpeg/281D60/CBFDC8/?text=Nova%20Yundt",
            "age": 12 as i32,
            "name": "Emmanuel Wolff"
            };

        let doc_user_name = document.get_str("name").unwrap();
        let doc_user_country = document.get_str("country").unwrap();
        let doc_user_avatar = document.get_str("avatar").unwrap();
        let doc_user_age = document.get_i32("age").unwrap();
        let doc_user_id = document.get_object_id("_id").unwrap().to_hex();

        let user = mongodb_client.doc_to_user(&document).unwrap();
        assert_eq!(doc_user_name, user.name);
        assert_eq!(doc_user_country, user.country);
        assert_eq!(doc_user_avatar, user.avatar);
        assert_eq!(doc_user_age, user.age);
        assert_eq!(doc_user_id, user.id);
    }
}