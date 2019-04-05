use crate::schema::{categories};
use crate::schema::categories::dsl::*;

use diesel;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager};
use r2d2::{Pool};

#[derive(Debug, Eq, PartialEq, Queryable, Serialize, Deserialize)]
pub struct Category {
    pub id: i32,
    pub name: String,
   
}

#[derive(Debug, Eq, PartialEq, Insertable, Serialize, Deserialize)]
#[table_name = "categories"]
pub struct NewCategory {
    pub name: String,
   
}

pub fn select_all(
        pool: &Pool<ConnectionManager<SqliteConnection>>,
    ) -> Result<Vec<Category>, diesel::result::Error> {
    
    let conn: &SqliteConnection = &pool.get().unwrap();
    
    let result = categories.load::<Category>(conn)?;
    Ok(result)
            
}

pub fn select_by_id(
        pool: &Pool<ConnectionManager<SqliteConnection>>,
        category_id: i32,
    ) -> Result<Vec<Category>, diesel::result::Error> {
    
    let conn: &SqliteConnection = &pool.get().unwrap();
    
    let result = categories
            .filter(id.eq(category_id))
            .limit(1)
            .load::<Category>(conn)?;
    Ok(result)
}