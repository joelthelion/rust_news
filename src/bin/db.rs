#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;

use self::diesel::prelude::*;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

use rust_playground::schema::posts;

#[derive(Queryable, Debug)]
struct Post {
    id : i32,
    title : String
}

#[derive(Insertable, Debug)]
#[table_name="posts"]
struct IPost {
    title : String
}

fn main() {
    let con = establish_connection();
    use rust_playground::schema::posts::dsl::*;
    let scoop = IPost{title:"Trump steps down!!!".to_string()};
    diesel::insert_into(posts)
        .values(&scoop)
        .execute(&con)
        .expect("insertion");
    let results = posts
        .limit(3)
        .load::<Post>(&con)
        .expect("error loading posts");
    for res in results {
        println!("{:?}", res);
    }
    println!("hello");
}
