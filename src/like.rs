use actix_web::web::Path;
use actix_web::HttpResponse;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::constants::APPLICATION_JSON;
use crate::response::Response;

pub type Likes = Response<Like>;

#[derive(Debug, Deserialize, Serialize)]
pub struct Like {
    pub id: String,
    pub created_at: DateTime<Utc>,
}

impl Like {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            created_at: Utc::now(),
        }
    }
}

/// list last 50 likes from a tweet `/tweets/{id}/likes`
#[get("/tweets/{id}/likes")]
pub async fn list(path: Path<(String,)>) -> HttpResponse {
    // TODO find likes by tweet ID and return them
    let likes = Likes { results: vec![] };

    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(likes)
}

/// add one like to a tweet `/tweets/{id}/likes`
#[post("/tweets/{id}/likes")]
pub async fn plus_one(path: Path<(String,)>) -> HttpResponse {
    // TODO add one like to a tweet
    let like = Like::new();

    HttpResponse::Created()
        .content_type(APPLICATION_JSON)
        .json(like)
}

/// remove one like from a tweet `/tweets/{id}/likes`
#[delete("/tweets/{id}/likes")]
pub async fn minus_one(path: Path<(String,)>) -> HttpResponse {
    // TODO remove one like to a tweet
    HttpResponse::NoContent()
        .content_type(APPLICATION_JSON)
        .await
        .unwrap()
}
