use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, Eq, Ord, PartialEq, PartialOrd)]
pub struct User {
    pub id: String,
    pub country: String,
    pub avatar: String,
    pub age: i32,
    pub name: String,
    pub rank: i32,
    pub points: i32,
}

impl User {
    pub fn new(id: String, country: String, avatar: String,
               age: i32, name: String, points: i32, rank: i32) -> Self {
        Self {
            id,
            country,
            avatar,
            age,
            name,
            rank,
            points,
        }
    }
}
