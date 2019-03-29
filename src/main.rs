extern crate actix;
extern crate actix_web;

extern crate futures;
extern crate serde_json;
extern crate env_logger;

extern crate dotenv;
extern crate chrono;

#[macro_use] extern crate diesel;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate json;

extern crate r2d2;

mod modules;
mod schema;
mod db_executor;

use actix_web::{
    error, http, middleware, server, App, AsyncResponder, Error, HttpMessage,
    HttpRequest, HttpResponse, Json
};
use actix::{
    Addr, SyncArbiter
};
use futures::{Future, Stream};
use json::JsonValue;
use chrono::*;
use dotenv::dotenv;
use std::env;

use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager};

use modules::*;
use db_executor::DbExecutor;

struct State {
    db: Addr<DbExecutor>
}

#[derive(Debug, Serialize, Deserialize)]
struct Echo {
    state: String,
}

fn establish_connection() -> SqliteConnection {
    dotenv().ok();
    let url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set to .env.");
            
    SqliteConnection::establish(&url)
            .expect("Cannot connect to database.")
}

fn index(req: &HttpRequest) -> Box<Future<Item = HttpResponse, Error = Error>> {
    req.body()
        .from_err()
        .and_then(|_| {
            Ok(HttpResponse::Ok().json(Echo{ state: "Hello?".to_string() }))
        })
        .responder()
}

fn main() {
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let sys = actix::System::new("minakanushi");
    
    dotenv().ok();
    let url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set to .env.");
            
    let manager = ConnectionManager::<SqliteConnection>::new(url);
    let pool = r2d2::Pool::builder()
            .max_size(3)
            .build(manager)
            .expect("Failed to create connection pool.");
 
    
    let addr = SyncArbiter::start(3, move || {
        DbExecutor(pool.clone())
    });
    
    server::new(|| {
        App::new()
            .middleware(middleware::Logger::default())
            .resource("/", |r| r.method(http::Method::GET).f(index))
    }).bind("127.0.0.1:8080")
        .unwrap()
        .shutdown_timeout(1)
        .start();
        
    println!("Start http server");
    let _ = sys.run();

}
