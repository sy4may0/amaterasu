use actix_web::{Error, error, web, HttpResponse};
use futures::{Future, Stream};
use futures::future::{err, Either};
use bytes::BytesMut;

use diesel::prelude::{MysqlConnection};
use diesel::r2d2::{ConnectionManager};
use crate::{dao};
use crate::dao::category::{NewCategory};

use crate::handler::{MAX_SIZE};

type Pool = 
        r2d2::Pool<ConnectionManager<MysqlConnection>>;

pub fn get_all(
    pool: web::Data<Pool>,
) -> impl Future<Item = HttpResponse, Error = Error> {

    web::block(move || 
        dao::category::select_all(pool.get_ref())
    ).then(|res| match res {
        Ok(categories) => Ok(HttpResponse::Ok().json(categories)),
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    })
    
}

pub fn get_by_id(
    id: web::Path<String>,
    pool: web::Data<Pool>,
) -> impl Future<Item = HttpResponse, Error = Error> {
  
    web::block(move || 
        dao::category::select_by_id(
            pool.get_ref(), 
            id.into_inner().parse().unwrap(),
        )
    ).then(|res| match res {
        Ok(categories) => Ok(HttpResponse::Ok().json(categories)),
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    })
}

pub fn add(
    payload: web::Payload,
    pool: web::Data<Pool>,
) -> impl Future<Item = HttpResponse, Error = Error> {

    payload.from_err()
        .fold(BytesMut::new(), move |mut body, chunk| {
            if (body.len() + chunk.len()) > MAX_SIZE {
                Err(error::ErrorBadRequest("overflow"))
            } else {
                body.extend_from_slice(&chunk);
                Ok(body)
            }
        })
        .and_then(move |body| {
            let r_obj = serde_json::from_slice::<NewCategory>(&body);
            match r_obj{
                Ok(obj) => {
                    Either::A(web::block(move || { 
                        dao::category::insert(
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

pub fn modify(
    id: web::Path<String>,
    payload: web::Payload,
    pool: web::Data<Pool>,
) -> impl Future<Item = HttpResponse, Error = Error> {

    payload.from_err()
        .fold(BytesMut::new(), move |mut body, chunk| {
            if (body.len() + chunk.len()) > MAX_SIZE {
                Err(error::ErrorBadRequest("overflow"))
            } else {
                body.extend_from_slice(&chunk);
                Ok(body)
            }
        })
        .and_then(move |body| {
            let r_obj = serde_json::from_slice::<NewCategory>(&body);
            match r_obj{
                Ok(obj) => {
                    Either::A(web::block(move || { 
                        dao::category::update(
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

pub fn remove(
    id: web::Path<String>,
    pool: web::Data<Pool>,
) -> impl Future<Item = HttpResponse, Error = Error> {
  
    web::block(move || 
        dao::category::delete(
            pool.get_ref(), 
            id.into_inner().parse().unwrap(),
        )
    ).then(|res| match res {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    })
}
