#[macro_use]
extern crate diesel;

mod schema {
  table! {
      posts (id) {
          id -> Int4,
          title -> Varchar,
          body -> Text,
          published -> Bool,
      }
  }
}

mod models {
  use schema::posts;
  #[derive(Queryable)]
  pub struct Post {
      pub id: i32,
      pub title: String,
      pub body: String,
      pub published: bool,
  }

  // apparently this can be done without heap storage, but lifetimes spread far..
  #[derive(Insertable)]
  #[table_name="posts"]
  pub struct NewPost {
      pub title: String,
      pub body: String,
  }
}

use diesel::prelude::*;
use diesel::pg::PgConnection;

fn main() {
    let database_url = std::env::var("DATABASE_URL").unwrap_or("postgres://localhost?connect_timeout=1".into());
    match PgConnection::establish(&database_url) {
      Err(e) => {
        println!("Should fail to connect here:");
        println!("{}", e);
      }
      Ok(_) => {
        unreachable!();
      }
    }
}
