use actix_web::web::{Data, Json, Path};
use actix_web::{web, HttpResponse};
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use diesel::result::Error;
use diesel::{ExpressionMethods, Insertable, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::constants::{APPLICATION_JSON, CONNECTION_POOL_ERROR};
use crate::like::Like;
use crate::response::Response;
use crate::{DBPool, DBPooledConnection};

use super::schema::tweets;
use diesel::query_dsl::methods::{FilterDsl, LimitDsl, OrderDsl};
use std::str::FromStr;

pub type Tweets = Response<Tweet>;

#[derive(Debug, Deserialize, Serialize)]
pub struct Tweet {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub message: String,
    pub likes: Vec<Like>,
}

impl Tweet {
    pub fn new(message: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            created_at: Utc::now(),
            message,
            likes: vec![],
        }
    }

    pub fn to_tweet_db(&self) -> TweetDB {
        TweetDB {
            id: Uuid::new_v4(),
            created_at: Utc::now().naive_utc(),
            message: self.message.clone(),
        }
    }
}

#[table_name = "tweets"]
#[derive(Queryable, Insertable)]
pub struct TweetDB {
    pub id: Uuid,
    pub created_at: NaiveDateTime,
    pub message: String,
}

impl TweetDB {
    fn to_tweet(&self) -> Tweet {
        Tweet {
            id: self.id.to_string(),
            created_at: Utc.from_utc_datetime(&self.created_at),
            message: self.message.clone(),
            likes: vec![],
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TweetRequest {
    pub message: Option<String>,
}

impl TweetRequest {
    pub fn to_tweet(&self) -> Option<Tweet> {
        match &self.message {
            Some(message) => Some(Tweet::new(message.to_string())),
            None => None,
        }
    }
}

fn list_tweets(total_tweets: i64, conn: &DBPooledConnection) -> Result<Tweets, Error> {
    use crate::schema::tweets::dsl::*;

    let _tweets = match tweets
        .order(created_at.desc())
        .limit(total_tweets)
        .load::<TweetDB>(conn)
    {
        Ok(tws) => tws,
        Err(err) => vec![],
    };

    Ok(Tweets {
        results: _tweets
            .into_iter()
            .map(|t| t.to_tweet())
            .collect::<Vec<Tweet>>(),
    })
}

fn find_tweet(_id: Uuid, conn: &DBPooledConnection) -> Result<Tweet, Error> {
    use crate::schema::tweets::dsl::*;

    let res = tweets.filter(id.eq(_id)).load::<TweetDB>(conn);
    match res {
        Ok(tweets_db) => match tweets_db.first() {
            Some(tweet_db) => Ok(tweet_db.to_tweet()),
            _ => Err(Error::NotFound),
        },
        Err(err) => Err(err),
    }
}

fn create_tweet(tweet: Tweet, conn: &DBPooledConnection) -> Result<Tweet, Error> {
    use crate::schema::tweets::dsl::*;

    let tweet_db = tweet.to_tweet_db();
    diesel::insert_into(tweets).values(&tweet_db).execute(conn);

    Ok(tweet_db.to_tweet())
}

/// list 50 last tweets `/tweets`
#[get("/tweets")]
pub async fn list(pool: Data<DBPool>) -> HttpResponse {
    let conn = pool.get().expect(CONNECTION_POOL_ERROR);

    let tweets = web::block(move || list_tweets(50, &conn)).await;

    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(tweets.unwrap())
}

fn delete_tweet(_id: Uuid, conn: &DBPooledConnection) -> Result<(), Error> {
    use crate::schema::tweets::dsl::*;

    let res = diesel::delete(tweets.filter(id.eq(_id))).execute(conn);
    match res {
        Ok(tweet_db) => Ok(()),
        Err(err) => Err(err),
    }
}

/// create a tweet `/tweets`
#[post("/tweets")]
pub async fn create(tweet_req: Json<TweetRequest>, pool: Data<DBPool>) -> HttpResponse {
    let conn = pool.get().expect(CONNECTION_POOL_ERROR);

    match create_tweet(tweet_req.to_tweet().unwrap(), &conn) {
        Ok(tweet) => HttpResponse::Created()
            .content_type(APPLICATION_JSON)
            .json(tweet),
        Err(_) => HttpResponse::NoContent().await.unwrap(),
    }
}

/// find a tweet by its id `/tweets/{id}`
#[get("/tweets/{id}")]
pub async fn get(path: Path<(String,)>, pool: Data<DBPool>) -> HttpResponse {
    let conn = pool.get().expect(CONNECTION_POOL_ERROR);

    let tweet = find_tweet(Uuid::from_str(path.0.as_str()).unwrap(), &conn);

    match tweet {
        Ok(tweet) => HttpResponse::Ok()
            .content_type(APPLICATION_JSON)
            .json(tweet),
        Err(_) => HttpResponse::NoContent()
            .content_type(APPLICATION_JSON)
            .await
            .unwrap(),
    }
}

/// delete a tweet by its id `/tweets/{id}`
#[delete("/tweets/{id}")]
pub async fn delete(path: Path<(String,)>, pool: Data<DBPool>) -> HttpResponse {
    // in any case return status 204
    let conn = pool.get().expect(CONNECTION_POOL_ERROR);

    delete_tweet(Uuid::from_str(path.0.as_str()).unwrap(), &conn);

    HttpResponse::NoContent()
        .content_type(APPLICATION_JSON)
        .await
        .unwrap()
}
