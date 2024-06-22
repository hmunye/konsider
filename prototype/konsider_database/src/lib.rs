use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
pub mod models;
pub mod schema;
use self::models::{NewPost, Post};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn insert_post(conn: &mut PgConnection, title: &str, content: &str) -> Post {
    use crate::schema::posts;

    let new_post: NewPost = NewPost { title, content };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .returning(Post::as_returning())
        .get_result(conn)
        .expect("Error saving new post")
}

pub fn fetch_posts(conn: &mut PgConnection) -> Vec<Post> {
    use self::schema::posts::dsl::*;

    posts
        .select(Post::as_select())
        .load(conn)
        .expect("Error loading posts")
}
