use actix_web::{Form, HttpResponse, Result};
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct MyParams {
    name: String,
}

/// Simple handle POST request
pub fn handle_post_1(params: Form<MyParams>) -> Result<HttpResponse> {
    let mut hash = HashMap::new();
    hash.insert("name", format!("{}", params.name));
    Ok(HttpResponse::Ok().json(hash))

    // Ok(HttpResponse::build(http::StatusCode::OK)
    //     .content_type("text/plain")
    //     .body(format!("Your name is {}", params.name)))
}

pub fn handle_get_1(_params: Form<MyParams>) -> Result<HttpResponse> {
    let mut hash = HashMap::new();
    hash.insert("status", "200");
    Ok(HttpResponse::Ok().json(hash))
}
