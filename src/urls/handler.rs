use crate::urls;
use crate::mongo_connection::Conn;
use urls::Url;
use mongodb::{doc, error::Error, oid::ObjectId};
use rocket::http::Status;
use rocket_contrib::json::Json;
use rocket::response::Redirect;

fn error_status(error: Error) -> Status {
    match error {
        Error::CursorNotFoundError => Status::NotFound,
        _ => Status::InternalServerError,
    }
}

#[get("/list")]
pub fn all(connection: Conn) -> Result<Json<Vec<Url>>, Status> {
    match urls::repository::all(&connection) {
        Ok(res) => Ok(Json(res)),
        Err(err) => Err(error_status(err)),
    }
}

#[get("/<tiny_url>")]
pub fn get(tiny_url: String, connection: Conn) -> Result<Redirect, &'static str> {
    let _tiny_url = String::from(&tiny_url);
    match urls::repository::get(_tiny_url, &connection) {   
        Ok(res) => Ok(Redirect::permanent(format!("{}", res.unwrap().url.unwrap().to_string()))),
        Err(_) => Err("Requested tiny_url was not found."),
    }
}

#[post("/", format = "application/json", data = "<urls>")]
pub fn post(urls: Json<Url>, connection: Conn) -> Result<Json<ObjectId>, Status> {
    match urls::repository::insert(urls.into_inner(), &connection) {
        Ok(res) => Ok(Json(res)),
        Err(err) => Err(error_status(err)),
    }
}

#[delete("/old")]
pub fn delete(connection: Conn) -> Result<Json<bool>, Status> {
    match urls::repository::delete_old(&connection) {
        Ok(_) => Ok(Json(true)),
        Err(err) => Err(error_status(err)),
    }
}

#[delete("/")]
pub fn delete_all(connection: Conn) -> Result<Json<bool>, Status> {
    match urls::repository::delete_all(&connection) {
        Ok(_) => Ok(Json(true)),
        Err(err) => Err(error_status(err)),
    }
}
