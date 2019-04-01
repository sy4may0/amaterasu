//use actix::prelude::*;
//
//use actix_web::Error;
//
//use diesel;
//use diesel::prelude::*;
//use diesel::r2d2::{ConnectionManager, Pool};
//
//use chrono::{NaiveDate};
//
//use crate::modules::*;
//
//pub struct DbExecutor(pub Pool<ConnectionManager<SqliteConnection>>);
//
//impl Actor for DbExecutor {
//    type Context = SyncContext<Self>;
//}
//
//pub struct GetTask {}
//pub struct GetTaskById { id: i32 }
//pub struct InsertTask { task: NewTask }
//pub struct UpdateTask { id: i32, task: NewTask }
//
//pub struct GetCategory {}
//pub struct GetCategoryById { id: i32}
//pub struct InsertCategory { category: NewCategory }
//pub struct UpdateCategory { id: i32, category: NewCategory}
//
//pub struct GetTaskType {}
//pub struct GetTaskTypeById { id: i32 }
//pub struct InsertTaskType { category: NewCategory }
//pub struct UpdateTaskType { id: i32, category: NewCategory }
//
//pub struct GetAchievement {}
//pub struct GetAchievementById { id: i32 }
//pub struct GetAchievementByDate { date: NaiveDate }
//pub struct InsertAchievement { achievement: NewAchievement }
//pub struct UpdateAchievement { id: i32, achievement: NewAchievement }
//
//
//impl Message for GetTask {
//    type Result = Result<Vec<Task>, Error>;
//}
//impl Message for GetTaskById {
//    type Result = Result<Task, Error>;
//}
//impl Message for InsertTask {
//    type Result = Result<(), Error>;
//}
//impl Message for UpdateTask {
//    type Result = Result<(), Error>;
//}
//
//impl Message for GetCategory {
//    type Result = Result<Vec<Category>, Error>;
//}
//impl Message for GetCategoryById {
//    type Result = Result<Category, Error>;
//}
//impl Message for InsertCategory {
//    type Result = Result<(), Error>;
//}
//impl Message for UpdateCategory {
//    type Result = Result<(), Error>;
//}
//
//impl Message for GetTaskType {
//    type Result = Result<Vec<TaskType>, Error>;
//}
//impl Message for GetTaskTypeById {
//    type Result = Result<TaskType, Error>;
//}
//impl Message for InsertTaskType {
//    type Result = Result<(), Error>;
//}
//impl Message for UpdateTaskType {
//    type Result = Result<(), Error>;
//}
//
//impl Message for GetAchievement {
//    type Result = Result<Vec<Achievement>, Error>;
//}
//impl Message for GetAchievementById {
//    type Result = Result<Achievement, Error>;
//}
//impl Message for GetAchievementByDate {
//    type Result = Result<Vec<Achievement>, Error>;
//}
//impl Message for InsertAchievement {
//    type Result = Result<(), Error>;
//}
//impl Message for UpdateAchievement {
//    type Result = Result<(), Error>;
//}
//
//
//impl Handler<GetTask> for DbExecutor {
//    type Result = Result<Vec<Task>, Error>;
//    
//    fn handle(&mut self, msg: GetTask, _: &mut Self::Context) -> Self::Result {
//        use crate::schema::tasks::dsl::*;
//        
//        let result = tasks
//                 .load::<Task>(&self.get().expect("Cannot get connection Pool."))
//                 .expect("Cannot handle Database Execution: GetTask");
//        Ok(result)
//    }
//}
//
//impl Handler<GetTaskById> for DbExecutor {
//    type Result = Result<Task, Error>;
//    
//    fn handle(&mut self, msg: GetTaskById, _: &mut Self::Context) -> Self::Result {
//        use crate::schema::tasks::dsl::*;
//        
//        let mut vec = tasks.filter(id.eq(msg.id))
//                .limit(1)
//                .load::<Task>(&self.get().expect("Cannot get connection Pool."))
//                .expect("Cannot handle Database Execution: GetTaskById");
//        let result = vec.pop().unwrap();
//        Ok(result)
//    
//    }
//}