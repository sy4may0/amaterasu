use actix_web::{Error, error, web, HttpResponse};
use futures::{Future, Stream};
use futures::future::{err, Either};
use bytes::BytesMut;

use diesel::prelude::{SqliteConnection};
use diesel::r2d2::{ConnectionManager};
use crate::{dao};
use crate::dao::achievement::{NewAchievement};

use crate::handler::{read_bytes};

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

pub fn add(
    payload: web::Payload,
    pool: web::Data<Pool>,
) -> impl Future<Item = HttpResponse, Error = Error> {

    payload.from_err()
        .fold(BytesMut::new(), move |body, chunk| {
            read_bytes(body, chunk)
        })
        .and_then(move |body| {
            let r_obj = serde_json::from_slice::<NewAchievement>(&body);
            match r_obj{
                Ok(obj) => {
                    Either::A(web::block(move || { 
                        dao::achievement::insert(
                            pool.get_ref(), 
                            obj,
                        )
                    }).then(|res| match res {
                        Ok(_) => Ok(HttpResponse::Ok().finish()),
                        Err(_) => Ok(HttpResponse::InternalServerError().into()),
                    })
                )},
                
                Err(_) => Either::B(err(error::ErrorBadRequest("Json Decode Failed"))),
                
            }
        })
}

pub fn update(
    id: web::Path<String>,
    payload: web::Payload,
    pool: web::Data<Pool>,
) -> impl Future<Item = HttpResponse, Error = Error> {

    payload.from_err()
        .fold(BytesMut::new(), move |body, chunk| {
            read_bytes(body, chunk)
        })
        .and_then(move |body| {
            let r_obj = serde_json::from_slice::<NewAchievement>(&body);
            match r_obj{
                Ok(obj) => {
                    Either::A(web::block(move || { 
                        dao::achievement::update(
                            pool.get_ref(), 
                            obj,
                            id.into_inner().parse().unwrap(),
                        )
                    }).then(|res| match res {
                        Ok(_) => Ok(HttpResponse::Ok().finish()),
                        Err(_) => Ok(HttpResponse::InternalServerError().into()),
                    })
                )},
                
                Err(_) => Either::B(err(error::ErrorBadRequest("Json Decode Failed"))),
                
            }
        })
}

