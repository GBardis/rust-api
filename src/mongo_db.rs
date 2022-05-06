use futures::stream::{StreamExt};
use mongodb::bson::{document::Document};
use mongodb::{Client, Collection};
use mongodb::options::FindOptions;
use crate::errors::CustomError;
use crate::user::User;

const DB_NAME: &str = "wp_users";
const COLL: &str = "users";

const ID: &str = "_id";
const NAME: &str = "name";
const COUNTRY: &str = "country";
const AVATAR: &str = "avatar";
const AGE: &str = "age";

const PAGE_SIZE: u64 = 25;

#[derive(Clone, Debug)]
pub struct MongoDbClient {
    pub client: Client,
}

impl MongoDbClient {
    pub async fn new(mongodb_uri: String) -> Self {
        let mongodb_client = Client::with_uri_str(mongodb_uri)
            .await
            .expect("Failed to create MongoDB client");

        MongoDbClient {
            client: mongodb_client,
        }
    }

    pub async fn fetch_users(&self, page_num: u64) -> Result<Vec<User>, CustomError> {
        let options = self.pagination(page_num);
        let collection = self.get_users_collection();
        let mut cursor = collection.find(None, options).await?;

        let mut result: Vec<User> = Vec::new();
        while let Some(doc) = cursor.next().await {
            result.push(self.doc_to_user(&doc?)?);
        }

        Ok(result)
    }

    pub fn doc_to_user(&self, doc: &Document) -> Result<User, CustomError> {
        let id = doc.get_object_id(ID)?;
        let name = doc.get_str(NAME)?;
        let country = doc.get_str(COUNTRY)?;
        let avatar = doc.get_str(AVATAR)?;
        let age = doc.get_i32(AGE)?;

        let user = User {
            id: id.to_hex(),
            name: name.to_owned(),
            country: country.to_owned(),
            avatar: avatar.to_owned(),
            age: age.to_owned(),
            points: 0,
            rank: 0
        };
        Ok(user)
    }

    fn get_users_collection(&self) -> Collection<Document> {
        self.client.database(DB_NAME).collection(COLL)
    }

    fn pagination(&self, page_num: u64) -> FindOptions {
        let builder = FindOptions::builder();
        return if page_num != 0 {
            let skips: u64 = (PAGE_SIZE * (page_num - 1)) as u64;
            builder.skip(skips).limit((PAGE_SIZE) as i64).build()
        } else {
            builder.build()
        }
    }
}
