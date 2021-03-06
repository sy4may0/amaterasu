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
        pool: &Pool<ConnectionManager<MysqlConnection>>,
    ) -> Result<Vec<TaskType>, diesel::result::Error> {
    
    let conn: &MysqlConnection = &pool.get().unwrap();
    
    let result = tasktypes.load::<TaskType>(conn)?;
    Ok(result)            
}

pub fn select_by_id(
        pool: &Pool<ConnectionManager<MysqlConnection>>,
        tasktype_id: i32,
    ) -> Result<Vec<TaskType>, diesel::result::Error> {
 
    let conn: &MysqlConnection = &pool.get().unwrap();
    
    let result = tasktypes
            .filter(id.eq(tasktype_id))
            .limit(1)
            .load::<TaskType>(conn)?;
    Ok(result)            
}

pub fn insert(
        pool: &Pool<ConnectionManager<MysqlConnection>>,
        new_tasktype: NewTaskType,
    ) -> Result<(), diesel::result::Error> {
    
    let conn: &MysqlConnection = &pool.get().unwrap();
    
    diesel::insert_into(tasktypes::table)
        .values(&new_tasktype)
        .execute(conn)?;
        
    Ok(())
}

pub fn update(
        pool: &Pool<ConnectionManager<MysqlConnection>>,
        new_tasktype: NewTaskType,
        tasktype_id: i32,
    ) -> Result<(), diesel::result::Error> {
    
    let conn: &MysqlConnection = &pool.get().unwrap();
    
    diesel::update(tasktypes.filter(id.eq(tasktype_id)))
        .set((
            name.eq(new_tasktype.name),
        ))
        .execute(conn)?;
        
    Ok(())
}

pub fn delete(
        pool: &Pool<ConnectionManager<MysqlConnection>>,
        tasktype_id: i32,
    ) -> Result<(), diesel::result::Error> {
    
    let conn: &MysqlConnection = &pool.get().unwrap();
    
    diesel::delete(tasktypes.filter(id.eq(tasktype_id)))
        .execute(conn)?;
        
    Ok(())
}
