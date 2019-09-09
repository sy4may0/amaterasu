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

use actix_cors::Cors;
use actix_web::{
    http::header, web, middleware, App, HttpServer
};

use dotenv::dotenv;
use std::env;
use diesel::prelude::{MysqlConnection};
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

    let manager = ConnectionManager::<MysqlConnection>::new(url);
    let pool = Pool::builder()
                    .max_size(max_db_conn)
                    .build(manager)
                    .unwrap();


    let server = HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(
                Cors::new()
                    .allowed_origin("http://tk2-239-29306.vs.sakura.ne.jp:8080")
                    .allowed_methods(vec!["POST", "GET",])
                    .allowed_headers(vec![header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .max_age(3600),
            )
            .wrap(middleware::Logger::default())
            
            // API service List
            .service(web::resource("/get/task")
                .route(web::get().to_async(handler::task::get_all)))
            .service(web::resource("/get/task/{id}")
                .route(web::get().to_async(handler::task::get_by_id)))
            .service(web::resource("/add/task")
                .route(web::post().to_async(handler::task::add)))
            .service(web::resource("/modify/task/{id}")
                .route(web::post().to_async(handler::task::modify)))
            .service(web::resource("/remove/task/{id}")
                .route(web::post().to_async(handler::task::remove)))
 
                
            .service(web::resource("/get/category")
                .route(web::get().to_async(handler::category::get_all)))
            .service(web::resource("/get/category/{id}")
                .route(web::get().to_async(handler::category::get_by_id)))
            .service(web::resource("/add/category")
                .route(web::post().to_async(handler::category::add)))               
            .service(web::resource("/modify/category/{id}")
                .route(web::post().to_async(handler::category::modify)))               
            .service(web::resource("/remove/category/{id}")
                .route(web::post().to_async(handler::category::remove)))               
                
            .service(web::resource("/get/tasktype")
                .route(web::get().to_async(handler::tasktype::get_all)))
            .service(web::resource("/get/tasktype/{id}")
                .route(web::get().to_async(handler::tasktype::get_by_id)))
            .service(web::resource("/add/tasktype")
                .route(web::post().to_async(handler::tasktype::add)))
            .service(web::resource("/modify/tasktype/{id}")
                .route(web::post().to_async(handler::tasktype::modify)))
            .service(web::resource("/remove/tasktype/{id}")
                .route(web::post().to_async(handler::tasktype::remove)))
                
            .service(web::resource("/get/achievement")
                .route(web::get().to_async(handler::achievement::get_all)))
            .service(web::resource("/get/achievement/{id}")
                .route(web::get().to_async(handler::achievement::get_by_id)))             
            .service(web::resource("/get/achievement/bydate")
                .route(web::get().to_async(handler::achievement::get_by_date)))
            .service(web::resource("/add/achievement")
                .route(web::post().to_async(handler::achievement::add)))
            .service(web::resource("/modify/achievement/{id}")
                .route(web::post().to_async(handler::achievement::modify)))
            .service(web::resource("/remove/achievement/{id}")
                .route(web::post().to_async(handler::achievement::remove)))
 
                
    }).bind(listen).unwrap();
    
    server.run().unwrap();    
        
    println!("Start http server");
}
