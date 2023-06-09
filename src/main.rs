#![allow(unused)]
#[macro_use]
extern crate diesel;

pub mod schema;
pub mod models;

use dotenvy::dotenv;
use std::env;
use std::thread::sleep;
use std::time::Duration;

use diesel::prelude::*;
use diesel::pg::PgConnection;

// Migration to DB tables creation
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

fn main(){
  // Load .env file+
  dotenv().ok();
  // Database connection
  let db_url = env::var("DATABASE_URL").expect("DB URL not found!");
  let mut conn = PgConnection::establish(&db_url)
    .unwrap_or_else(|_| panic!("Error connecting to {}", db_url));
 
  // Run migrations
  conn.run_pending_migrations(MIGRATIONS).unwrap();;
  let sleep_time: Duration;

  // Imports structs and schemas
  use self::models::{Post, NewPost};
  use self::schema::posts::dsl::*;
  use self::schema::posts;

  // Load all posts
  let posts_list = posts.load::<Post>(&mut conn).expect("Failed to load posts");
  
  // If there are no posts, create a new one
  if posts_list.len() == 0 {
    let new_post = NewPost {
      title: "My firs post",
      body: "This is body of my first post",
      published: false
    };

    // Insert new post
    diesel::insert_into(posts::table)
      .values(new_post)
      .get_result::<Post>(&mut conn)
      .expect("Failed to create new post");
  }

  // Prints all post every 5 seconds
  sleep_time = Duration::new(5, 0);
  
  loop {
    posts_list.iter().for_each(|post| {
      println!("{:?}", post);
    });
    
    sleep(sleep_time);
  }

}