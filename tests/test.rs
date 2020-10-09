// See https://rocket.rs/v0.4/guide/testing/#local-dispatching
#[cfg(test)]
mod test {
    use rocket::http::{ContentType, Status};
    use rocket::local::Client;
    use rustlang_rocket_mongodb::rocket;

    #[test]
    fn get_urls() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let response = client.get("/list").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn post_url() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let response = client
            .post("/")
            .header(ContentType::JSON)
            .body(r#"{ "url": "http://google.com", "tiny_url":"caos"}"#)
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        client.delete("/").dispatch();
    }

    #[test]
    fn post_url_delete_url() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let mut response = client
            .post("/")
            .header(ContentType::JSON)
            .body(r#"{ "url": "http://google.com" }"#)
            .dispatch();
        assert_eq!(response.status(), Status::Ok);

        let id = response.body_string().unwrap();
        let id: Vec<&str> = id.split("\"").collect();
        let mut response = client.delete(format!("/{}", id[3])).dispatch();
        assert!(response.body().is_some());
        assert!(response.body_string().unwrap().contains(&id[3]));
        client.delete("/").dispatch();
    }

    #[test]
    fn delete_all() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        client.delete("/").dispatch();
        let response = client
            .post("/")
            .header(ContentType::JSON)
            .body(r#"{ "url": "http://google.com" }"#)
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        let response = client.delete("/").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }
}
