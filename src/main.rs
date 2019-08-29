extern crate actix;
extern crate actix_web;

extern crate futures;
extern crate serde_json;
extern crate env_logger;
extern crate bytes;

extern crate dotenv;
extern crate chrono;

#[macro_use] extern crate diesel;
#[macro_use] extern crate serde_derive;

extern crate r2d2;

mod modules;
mod schema;
mod handler;
mod dao;

use actix_web::{
    web, middleware, App, HttpServer
};

use dotenv::dotenv;
use std::env;
use diesel::prelude::{SqliteConnection};
use diesel::r2d2::{ConnectionManager};
use r2d2::{Pool};

fn main() {
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    dotenv().ok();
            
    let url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set to .env.");
    let max_db_conn: u32 = env::var("MAX_DB_CONNECTION")
            .expect("MAX_DB_CONNECTION must be set to .env.")
            .parse()
            .unwrap();
    let listen = env::var("LISTEN")
            .expect("LISTEN must be set to .env");
            
            
    let manager = ConnectionManager::<SqliteConnection>::new(url); 
    let pool = Pool::builder()
            .max_size(max_db_conn)
            .build(manager)
            .unwrap();
 
    let server = HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            
            // API service List
            .service(web::resource("/tasks")
                .route(web::get().to_async(handler::task::get_all)))
            .service(web::resource("/task/{id}")
                .route(web::get().to_async(handler::task::get_by_id)))
            .service(web::resource("/task/add")
                .route(web::post().to_async(handler::task::add)))
            .service(web::resource("/task/modify/{id}")
                .route(web::post().to_async(handler::task::modify)))
            .service(web::resource("/task/remove/{id}")
                .route(web::post().to_async(handler::task::remove)))
 
                
            .service(web::resource("/categories")
                .route(web::get().to_async(handler::category::get_all)))
            .service(web::resource("/category/{id}")
                .route(web::get().to_async(handler::category::get_by_id)))
            .service(web::resource("/category/add")
                .route(web::post().to_async(handler::category::add)))               
            .service(web::resource("/category/modify/{id}")
                .route(web::post().to_async(handler::category::modify)))               
            .service(web::resource("/category/remove/{id}")
                .route(web::post().to_async(handler::category::remove)))               
                
            .service(web::resource("/tasktypes")
                .route(web::get().to_async(handler::tasktype::get_all)))
            .service(web::resource("/tasktype/{id}")
                .route(web::get().to_async(handler::tasktype::get_by_id)))
            .service(web::resource("/tasktype/add")
                .route(web::post().to_async(handler::tasktype::add)))
            .service(web::resource("/tasktype/modify/{id}")
                .route(web::post().to_async(handler::tasktype::modify)))
            .service(web::resource("/tasktype/remove/{id}")
                .route(web::post().to_async(handler::tasktype::remove)))
                
            .service(web::resource("/achievements")
                .route(web::get().to_async(handler::achievement::get_all)))
            .service(web::resource("/achievement/{id}")
                .route(web::get().to_async(handler::achievement::get_by_id)))             
            .service(web::resource("/achievements/bydate")
                .route(web::get().to_async(handler::achievement::get_by_date)))
            .service(web::resource("/achievement/add")
                .route(web::post().to_async(handler::achievement::add)))
            .service(web::resource("/achievement/modify/{id}")
                .route(web::post().to_async(handler::achievement::modify)))
            .service(web::resource("/achievement/remove/{id}")
                .route(web::post().to_async(handler::achievement::remove)))
 
                
    }).bind(listen).unwrap();
    
    server.run().unwrap();    
        
    println!("Start http server");
}
