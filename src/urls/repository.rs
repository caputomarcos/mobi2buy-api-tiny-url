#![allow(proc_macro_derive_resolution_fallback)]
use crate::urls::{Url, InsertableUrl};
use crate::mongo_connection::Conn;
use crate::r2d2_mongodb::mongodb::db::ThreadedDatabase;
use mongodb::{bson, coll::results::DeleteResult, doc, error::Error, oid::ObjectId};
use chrono::{Duration, Utc};

const COLLECTION: &str = "urls";

pub fn all(connection: &Conn) -> Result<Vec<Url>, Error> {
    let cursor = connection.collection(COLLECTION).find(None, None).unwrap();

    cursor
        .map(|result| match result {
            Ok(doc) => match bson::from_bson(bson::Bson::Document(doc)) {
                Ok(result_model) => Ok(result_model),
                Err(_) => Err(Error::DefaultError(String::from(""))),
            },
            Err(err) => Err(err),
        })
        .collect::<Result<Vec<Url>, Error>>()
}

pub fn get(tiny_url: String, connection: &Conn) -> Result<Option<Url>, Error> {
    match connection
        .collection(COLLECTION)
        .find_one(Some(doc! {"tiny_url": tiny_url}), None)
    {
        Ok(db_result) => match db_result {
            Some(result_doc) => match bson::from_bson(bson::Bson::Document(result_doc)) {
                Ok(result_model) => Ok(Some(result_model)),
                Err(_) => Err(Error::DefaultError(String::from(
                    "Failed to create reverse BSON",
                ))),
            },
            None => Ok(None),
        },
        Err(err) => Err(err),
    }
}

pub fn insert(urls: Url, connection: &Conn) -> Result<ObjectId, Error> {
    let insertable = InsertableUrl::from_url(urls.clone());
    match bson::to_bson(&insertable) {
        Ok(model_bson) => match model_bson {
            bson::Bson::Document(model_doc) => {
                match connection
                    .collection(COLLECTION)
                    .insert_one(model_doc, None)
                {
                    Ok(res) => match res.inserted_id {
                        Some(res) => match bson::from_bson(res) {
                            Ok(res) => Ok(res),
                            Err(_) => Err(Error::DefaultError(String::from("Failed to read BSON"))),
                        },
                        None => Err(Error::DefaultError(String::from("None"))),
                    },
                    Err(err) => Err(err),
                }
            }
            _ => Err(Error::DefaultError(String::from(
                "Failed to create Document",
            ))),
        },
        Err(_) => Err(Error::DefaultError(String::from("Failed to create BSON"))),
    }
}

pub fn update(id: ObjectId, urls: Url, connection: &Conn) -> Result<Url, Error> {
    let mut new_url = urls.clone();
    new_url.id = Some(id.clone());
    match bson::to_bson(&new_url) {
        Ok(model_bson) => match model_bson {
            bson::Bson::Document(model_doc) => {
                match connection.collection(COLLECTION).replace_one(
                    doc! {"_id": id},
                    model_doc,
                    None,
                ) {
                    Ok(_) => Ok(new_url),
                    Err(err) => Err(err),
                }
            }
            _ => Err(Error::DefaultError(String::from(
                "Failed to create Document",
            ))),
        },
        Err(_) => Err(Error::DefaultError(String::from("Failed to create BSON"))),
    }
}

pub fn delete_old(connection: &Conn) -> Result<DeleteResult, Error> {
    let filter = doc!{
        "created": {
            "$lt": Utc::now() - Duration::days(7)
        }
    };
    connection
        .collection(COLLECTION)
        .delete_many(filter, None)
}

pub fn delete_all(connection: &Conn) -> Result<(), Error> {
    connection.collection(COLLECTION).drop()
}
