use actix_web::{Error, web, HttpResponse};
use futures::{Future};

use diesel::prelude::{SqliteConnection};
use diesel::r2d2::{ConnectionManager};
use crate::{dao};

type Pool = 
        r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub fn get_all(
    pool: web::Data<Pool>,
) -> impl Future<Item = HttpResponse, Error = Error> {

    web::block(move || 
        dao::achievement::select_all(pool.get_ref())
    ).then(|res| match res {
        Ok(achievements) => Ok(HttpResponse::Ok().json(achievements)),
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    })
    
}

pub fn get_by_id(
    id: web::Path<String>,
    pool: web::Data<Pool>,
) -> impl Future<Item = HttpResponse, Error = Error> {
  
    web::block(move || 
        dao::achievement::select_by_id(
            pool.get_ref(), 
            id.into_inner().parse().unwrap(),
        )
    ).then(|res| match res {
        Ok(achievements) => Ok(HttpResponse::Ok().json(achievements)),
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    })
}