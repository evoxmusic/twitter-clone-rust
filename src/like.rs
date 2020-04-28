use actix_web::web::{Data, Path};
use actix_web::{web, HttpResponse};
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use diesel::{ExpressionMethods, Insertable, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::constants::{APPLICATION_JSON, CONNECTION_POOL_ERROR};
use crate::response::Response;
use crate::{DBPool, DBPooledConnection};

use super::schema::likes;
use diesel::query_dsl::methods::{FilterDsl, OrderDsl};
use diesel::result::Error;
use std::str::FromStr;

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

    pub fn to_like_db(&self, tweet_id: Uuid) -> LikeDB {
        LikeDB {
            id: Uuid::from_str(self.id.as_str()).unwrap(),
            created_at: self.created_at.naive_utc(),
            tweet_id,
        }
    }
}

#[table_name = "likes"]
#[derive(Queryable, Insertable)]
pub struct LikeDB {
    pub id: Uuid,
    pub created_at: NaiveDateTime,
    pub tweet_id: Uuid,
}

impl LikeDB {
    pub fn to_like(&self) -> Like {
        Like {
            id: self.id.to_string(),
            created_at: Utc.from_utc_datetime(&self.created_at),
        }
    }
}

pub fn list_likes(_tweet_id: Uuid, conn: &DBPooledConnection) -> Result<Likes, Error> {
    use crate::schema::likes::dsl::*;

    let _likes: Vec<LikeDB> = match likes
        .filter(tweet_id.eq(_tweet_id))
        .order(created_at.desc())
        .load::<LikeDB>(conn)
    {
        Ok(lks) => lks,
        Err(_) => vec![],
    };

    Ok(Likes {
        results: _likes
            .into_iter()
            .map(|l| l.to_like())
            .collect::<Vec<Like>>(),
    })
}

pub fn create_like(_tweet_id: Uuid, conn: &DBPooledConnection) -> Result<Like, Error> {
    use crate::schema::likes::dsl::*;

    let like = Like::new();
    let _ = diesel::insert_into(likes)
        .values(like.to_like_db(_tweet_id))
        .execute(conn);

    Ok(like)
}

pub fn delete_like(_tweet_id: Uuid, conn: &DBPooledConnection) -> Result<(), Error> {
    use crate::schema::likes::dsl::*;

    let _likes = list_likes(_tweet_id, conn);

    let like = match &_likes {
        Ok(_likes) if !_likes.results.is_empty() => _likes.results.first(),
        _ => None,
    };

    if like.is_none() {
        return Ok(());
    }

    let like_id = Uuid::from_str(like.unwrap().id.as_str()).unwrap();

    let res = diesel::delete(likes.filter(id.eq(like_id))).execute(conn);
    match res {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}

/// list last 50 likes from a tweet `/tweets/{id}/likes`
#[get("/tweets/{id}/likes")]
pub async fn list(path: Path<(String,)>, pool: Data<DBPool>) -> HttpResponse {
    let conn = pool.get().expect(CONNECTION_POOL_ERROR);

    let likes =
        web::block(move || list_likes(Uuid::from_str(path.0.as_str()).unwrap(), &conn)).await;

    match likes {
        Ok(likes) => HttpResponse::Ok()
            .content_type(APPLICATION_JSON)
            .json(likes),
        _ => HttpResponse::Ok()
            .content_type(APPLICATION_JSON)
            .json(Likes::new()),
    }
}

/// add one like to a tweet `/tweets/{id}/likes`
#[post("/tweets/{id}/likes")]
pub async fn plus_one(path: Path<(String,)>, pool: Data<DBPool>) -> HttpResponse {
    let conn = pool.get().expect(CONNECTION_POOL_ERROR);

    let like =
        web::block(move || create_like(Uuid::from_str(path.0.as_str()).unwrap(), &conn)).await;

    match like {
        Ok(like) => HttpResponse::Ok().content_type(APPLICATION_JSON).json(like),
        _ => HttpResponse::NoContent().await.unwrap(),
    }
}

/// remove one like from a tweet `/tweets/{id}/likes`
#[delete("/tweets/{id}/likes")]
pub async fn minus_one(path: Path<(String,)>, pool: Data<DBPool>) -> HttpResponse {
    // in any case return status 204
    let conn = pool.get().expect(CONNECTION_POOL_ERROR);

    let _ = web::block(move || delete_like(Uuid::from_str(path.0.as_str()).unwrap(), &conn)).await;

    HttpResponse::NoContent()
        .content_type(APPLICATION_JSON)
        .await
        .unwrap()
}
