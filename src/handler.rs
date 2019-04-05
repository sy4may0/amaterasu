//use actix_web::{Error, web, HttpResponse};
//use futures::{Future};
//
//use crate::dao::{DAO};
//
pub mod task;
pub mod category;
pub mod tasktype;
pub mod achievement;
//
//pub fn handle_get_task_all(
//    dao: web::Data<DAO>,
//) -> impl Future<Item = HttpResponse, Error = Error> {
//
//    web::block(move || dao.select_task_all())
//            .then(|res| match res {
//        Ok(tasks) => Ok(HttpResponse::Ok().json(tasks)),
//        Err(_) => Ok(HttpResponse::InternalServerError().into()),
//    })
//    
//}
//
//pub fn handle_get_task_by_id(
//    id: web::Path<String>,
//    dao: web::Data<DAO>,
//) -> impl Future<Item = HttpResponse, Error = Error> {
//  
//    web::block(move || dao.select_task_by_id(id.into_inner().parse().unwrap())) 
//                .then(|res| match res {
//        Ok(tasks) => Ok(HttpResponse::Ok().json(tasks)),
//        Err(_) => Ok(HttpResponse::InternalServerError().into()),
//    })
//}
//
//pub fn handle_get_category_all(
//    dao: web::Data<DAO>,
//) -> impl Future<Item = HttpResponse, Error = Error> {
//
//    web::block(move || dao.select_category_all())
//            .then(|res| match res {
//        Ok(categories) => Ok(HttpResponse::Ok().json(categories)),
//        Err(_) => Ok(HttpResponse::InternalServerError().into()),
//    })
//    
//}
//
//pub fn handle_get_category_by_id(
//    id: web::Path<String>,
//    dao: web::Data<DAO>,
//) -> impl Future<Item = HttpResponse, Error = Error> {
//  
//    web::block(move || dao.select_category_by_id(id.into_inner().parse().unwrap())) 
//                .then(|res| match res {
//        Ok(categories) => Ok(HttpResponse::Ok().json(categories)),
//        Err(_) => Ok(HttpResponse::InternalServerError().into()),
//    })
//}
//
//pub fn handle_get_tasktype_all(
//    dao: web::Data<DAO>,
//) -> impl Future<Item = HttpResponse, Error = Error> {
//
//    web::block(move || dao.select_tasktype_all())
//            .then(|res| match res {
//        Ok(tasktypes) => Ok(HttpResponse::Ok().json(tasktypes)),
//        Err(_) => Ok(HttpResponse::InternalServerError().into()),
//    })
//    
//}
//
//pub fn handle_get_tasktype_by_id(
//    id: web::Path<String>,
//    dao: web::Data<DAO>,
//) -> impl Future<Item = HttpResponse, Error = Error> {
//
//    web::block(move || dao.select_tasktype_by_id(id.into_inner().parse().unwrap()))
//            .then(|res| match res {
//        Ok(tasktypes) => Ok(HttpResponse::Ok().json(tasktypes)),
//        Err(_) => Ok(HttpResponse::InternalServerError().into()),
//    })
//    
//}
//
//pub fn handle_get_achievement_all(
//    dao: web::Data<DAO>,
//) -> impl Future<Item = HttpResponse, Error = Error> {
//
//    web::block(move || dao.select_achievement_all())
//            .then(|res| match res {
//        Ok(achievements) => Ok(HttpResponse::Ok().json(achievements)),
//        Err(_) => Ok(HttpResponse::InternalServerError().into()),
//    })
//    
//}
//
//pub fn handle_get_achievement_by_id(
//    id: web::Path<String>,
//    dao: web::Data<DAO>,
//) -> impl Future<Item = HttpResponse, Error = Error> {
//
//    web::block(move || dao.select_achievement_by_id(id.into_inner().parse().unwrap()))
//            .then(|res| match res {
//        Ok(achievements) => Ok(HttpResponse::Ok().json(achievements)),
//        Err(_) => Ok(HttpResponse::InternalServerError().into()),
//    })
//    
//}