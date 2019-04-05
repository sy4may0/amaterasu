use crate::schema::{tasktypes};
use crate::schema::tasktypes::dsl::*;

use diesel;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager};
use r2d2::{Pool};

#[derive(Debug, Eq, PartialEq, Queryable, Serialize, Deserialize)]
pub struct TaskType {
    pub id: i32,
    pub name: String,
    
}

#[derive(Debug, Eq, PartialEq, Insertable, Serialize, Deserialize)]
#[table_name = "tasktypes"]
pub struct NewTaskType {
    pub name: String,
    
}

pub fn select_all(
        pool: &Pool<ConnectionManager<SqliteConnection>>,
    ) -> Result<Vec<TaskType>, diesel::result::Error> {
    
    let conn: &SqliteConnection = &pool.get().unwrap();
    
    let result = tasktypes.load::<TaskType>(conn)?;
    Ok(result)            
}

pub fn select_by_id(
        pool: &Pool<ConnectionManager<SqliteConnection>>,
        tasktype_id: i32,
    ) -> Result<Vec<TaskType>, diesel::result::Error> {
 
    let conn: &SqliteConnection = &pool.get().unwrap();
    
    let result = tasktypes
            .filter(id.eq(tasktype_id))
            .limit(1)
            .load::<TaskType>(conn)?;
    Ok(result)            
}