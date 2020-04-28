table! {
    likes (id) {
        id -> Uuid,
        created_at -> Timestamp,
        tweet_id -> Uuid,
    }
}

table! {
    tweets (id) {
        id -> Uuid,
        created_at -> Timestamp,
        message -> Text,
    }
}

joinable!(likes -> tweets (tweet_id));

allow_tables_to_appear_in_same_query!(
    likes,
    tweets,
);
