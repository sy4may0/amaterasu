use crate::schema::{tasks};
use crate::schema::tasks::dsl::*;

use diesel;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager};
use r2d2::{Pool};

#[derive(Debug, Eq, PartialEq, Queryable, Serialize, Deserialize)]
pub struct Task {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub category: i32,
    pub tasktype: i32,
    pub status: i32,
    pub unplanned: i32,
    
}

#[derive(Debug, Eq, PartialEq, Insertable, Serialize, Deserialize)]
#[table_name = "tasks"]
pub struct NewTask {
    pub name: String, 
    pub description: Option<String>,
    pub category: i32,
    pub tasktype: i32,
    pub status: i32, 
    pub unplanned: i32,
    
}

pub fn select_all(
        pool: &Pool<ConnectionManager<SqliteConnection>>,
    ) -> Result<Vec<Task>, diesel::result::Error> {
    
    let conn: &SqliteConnection = &pool.get().unwrap();
                
    let result = tasks
             .load::<Task>(conn)?;
    Ok(result)
}

pub fn select_by_id(
        pool: &Pool<ConnectionManager<SqliteConnection>>,
        task_id: i32,
    ) -> Result<Vec<Task>, diesel::result::Error> {
 
    let conn: &SqliteConnection = &pool.get().unwrap();
    
    let result = tasks
            .filter(id.eq(task_id))
            .limit(1)
            .load::<Task>(conn)?;
    Ok(result)
}

pub fn insert(
        pool: &Pool<ConnectionManager<SqliteConnection>>,
        new_task: NewTask,
    ) -> Result<(), diesel::result::Error> {
    
    let conn: &SqliteConnection = &pool.get().unwrap();
    
    diesel::insert_into(tasks::table)
        .values(&new_task)
        .execute(conn)?;
        
    Ok(())
}

pub fn update(
        pool: &Pool<ConnectionManager<SqliteConnection>>,
        new_task: NewTask,
        task_id: i32,
    ) -> Result<(), diesel::result::Error> {
    
    let conn: &SqliteConnection = &pool.get().unwrap();
    
    diesel::update(tasks.filter(id.eq(task_id)))
        .set((
            name.eq(new_task.name),
            description.eq(new_task.description),
            category.eq(new_task.category),
            tasktype.eq(new_task.tasktype),
            status.eq(new_task.status),
            unplanned.eq(new_task.unplanned),
        ))
        .execute(conn)?;
    
    Ok(())
}