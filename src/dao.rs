use diesel;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager};
use r2d2::{Error,Pool};

use crate::modules::*;

#[derive(Clone)]
pub struct DAO {
    pool: Pool<ConnectionManager<SqliteConnection>>,
}

impl DAO {
    pub fn new(url: String, max_db_conn: u32)      
            -> Result<DAO, Error> {
        let manager = ConnectionManager::<SqliteConnection>::new(url); 
        let pool = Pool::builder()
                .max_size(max_db_conn)
                .build(manager)?;
                
        Ok(DAO{ pool: pool })
    }

    pub fn select_task_all(&self)
            -> Result<Vec<Task>, diesel::result::Error> {
        use crate::schema::tasks::dsl::*;
        let conn: &SqliteConnection = &self.pool.get().unwrap();
                    
        let result = tasks
                 .load::<Task>(conn)?;
        Ok(result)
    }
    
    pub fn select_task_by_id(&self, task_id: i32)
            -> Result<Vec<Task>, diesel::result::Error> {
        use crate::schema::tasks::dsl::*;
        let conn: &SqliteConnection = &self.pool.get().unwrap();
        
        let result = tasks
                .filter(id.eq(task_id))
                .limit(1)
                .load::<Task>(conn)?;
        Ok(result)
    }
    
    pub fn select_category_all(&self)
            -> Result<Vec<Category>, diesel::result::Error> {
        use crate::schema::categories::dsl::*;
        let conn: &SqliteConnection = &self.pool.get().unwrap();
        
        let result = categories.load::<Category>(conn)?;
        Ok(result)
                
    }
    
    pub fn select_category_by_id(&self, category_id: i32)
            -> Result<Vec<Category>, diesel::result::Error> {
        use crate::schema::categories::dsl::*;
        let conn: &SqliteConnection = &self.pool.get().unwrap();
        
        let result = categories
                .filter(id.eq(category_id))
                .limit(1)
                .load::<Category>(conn)?;
        Ok(result)
    }
    
    pub fn select_tasktype_all(&self)
            -> Result<Vec<TaskType>, diesel::result::Error> {
        use crate::schema::tasktypes::dsl::*;
        let conn: &SqliteConnection = &self.pool.get().unwrap();
        
        let result = tasktypes.load::<TaskType>(conn)?;
        Ok(result)            
    }
    
    pub fn select_tasktype_by_id(&self, tasktype_id: i32)
            -> Result<Vec<TaskType>, diesel::result::Error> {
        use crate::schema::tasktypes::dsl::*;
        let conn: &SqliteConnection = &self.pool.get().unwrap();
        
        let result = tasktypes
                .filter(id.eq(tasktype_id))
                .limit(1)
                .load::<TaskType>(conn)?;
        Ok(result)            
    }
 
    pub fn select_achievement_all(&self)
            -> Result<Vec<Achievement>, diesel::result::Error> {
        use crate::schema::achievements::dsl::*;
        let conn: &SqliteConnection = &self.pool.get().unwrap();
        
        let result = achievements.load::<Achievement>(conn)?;
        Ok(result)
    } 
    
    pub fn select_achievement_by_id(&self, achievement_id: i32)
            -> Result<Vec<Achievement>, diesel::result::Error> {
        use crate::schema::achievements::dsl::*;
        let conn: &SqliteConnection = &self.pool.get().unwrap();
        
        let result = achievements
                .filter(id.eq(achievement_id))
                .limit(1)
                .load::<Achievement>(conn)?;
        Ok(result)
    } 
}

