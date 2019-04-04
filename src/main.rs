extern crate actix;
extern crate actix_web;

extern crate futures;
extern crate serde_json;
extern crate env_logger;

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

use handler::*;
use dao::{DAO};

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
 
    let dao = DAO::new(url, max_db_conn).unwrap();
    
    let server = HttpServer::new(move || {
        App::new()
            .data(dao.clone())
            .wrap(middleware::Logger::default())
            
            .service(web::resource("/tasks")
                .route(web::get().to_async(handle_get_task_all)))
            .service(web::resource("/task/{id}")
                .route(web::get().to_async(handle_get_task_by_id)))
                
            .service(web::resource("/categories")
                .route(web::get().to_async(handle_get_category_all)))
            .service(web::resource("/category/{id}")
                .route(web::get().to_async(handle_get_category_by_id)))
                
            .service(web::resource("/tasktypes")
                .route(web::get().to_async(handle_get_tasktype_all)))
            .service(web::resource("/tasktype/{id}")
                .route(web::get().to_async(handle_get_tasktype_by_id)))
                
            .service(web::resource("/achievements")
                .route(web::get().to_async(handle_get_achievement_all)))
            .service(web::resource("/achievement/{id}")
                .route(web::get().to_async(handle_get_achievement_by_id)))             
                
    }).bind("127.0.0.1:8080").unwrap();
    
    server.run().unwrap();    
        
    println!("Start http server");
}
