extern crate failure;
extern crate dotenv;

#[macro_use] extern crate diesel;

mod db_object;
mod task;
mod schema;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;
use task::{Task, NewTask};

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();
    
    let url = env::var("DATABASE_URL")
                .expect("DATABASE_URL must be set");
                
    SqliteConnection::establish(&url)
                .expect(&format!("Error connecting to {}", url))
} 

fn main() {
    let conn = establish_connection();
    println!("Hello, world!");
    
    use schema::tasks;
    let new_task = NewTask {
        name: "test".to_string(),
        description: "test".to_string(),
        category: 1,
        tasktype: 2,
        status: 0,
        unplanned: 0,
    };
    
    diesel::insert_into(tasks::table)
        .values(&new_task)
        .execute(&conn)
        .expect("Error");
}
