#![allow(proc_macro_derive_resolution_fallback)]

pub mod handler;
pub mod repository;
use mongodb::bson;
use bson::{UtcDateTime};
use chrono::prelude::*;
use harsh::Harsh;
use rand::Rng;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Url {
    #[serde(rename = "_id")] // Use MongoDB's special primary key field name when serializing
    pub id: Option<bson::oid::ObjectId>,
    pub url: Option<String>,
    pub tiny_url: Option<String>,
    pub created: Option<UtcDateTime>,
}

#[derive(Serialize, Debug, Clone)]
pub struct InsertableUrl {
    pub url: Option<String>,
    pub tiny_url: Option<String>,
    pub created: Option<UtcDateTime>,
}

impl InsertableUrl {
    fn from_url(urls: Url) -> InsertableUrl {
        let mut harsh = Harsh::default().encode(&[rand::thread_rng().gen_range(0, 99)]);

        if urls.tiny_url.is_some() {
            harsh = urls.tiny_url.unwrap().to_string();
        }

        InsertableUrl {
            url: urls.url,
            tiny_url: Some(harsh),
            created: Some(bson::UtcDateTime(Utc::now())),
        }
    }
}
