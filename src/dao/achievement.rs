use crate::schema::{achievements};
use crate::schema::achievements::dsl::*;

use diesel;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager};
use r2d2::{Pool};
use chrono::{NaiveTime, NaiveDate};

#[derive(Debug, Eq, PartialEq, Queryable, Serialize, Deserialize)]
pub struct Achievement {
    pub id: i32,
    pub task: i32,
    pub date: NaiveDate,
    pub planned_time: Option<NaiveTime>,
    pub actual_time: Option<NaiveTime>,
    pub progress: i32,
    pub is_close_on: i32,
}

#[derive(Debug, Eq, PartialEq, Insertable, Serialize, Deserialize)]
#[table_name = "achievements"]
pub struct NewAchievement {
    pub task: i32,
    pub date: NaiveDate,
    pub planned_time: Option<NaiveTime>,
    pub actual_time: Option<NaiveTime>,
    pub progress: i32,
    pub is_close_on: i32,
}

pub fn select_all(
        pool: &Pool<ConnectionManager<SqliteConnection>>,
    ) -> Result<Vec<Achievement>, diesel::result::Error> {
    
    let conn: &SqliteConnection = &pool.get().unwrap();
    
    let result = achievements.load::<Achievement>(conn)?;
    Ok(result)
} 

pub fn select_by_id(
        pool: &Pool<ConnectionManager<SqliteConnection>>,
        achievement_id: i32,
    ) -> Result<Vec<Achievement>, diesel::result::Error> {
 
    let conn: &SqliteConnection = &pool.get().unwrap();
    
    let result = achievements
            .filter(id.eq(achievement_id))
            .limit(1)
            .load::<Achievement>(conn)?;
    Ok(result)
}

pub fn select_by_date(
        pool: &Pool<ConnectionManager<SqliteConnection>>,
        begin: NaiveDate,
        end: NaiveDate,
    ) -> Result<Vec<Achievement>, diesel::result::Error> {
    
    let conn: &SqliteConnection = &pool.get().unwrap();
    
    let result = achievements
            .filter(date.between(begin, end))
            .load::<Achievement>(conn)?;
    Ok(result)
    
}

pub fn insert(
        pool: &Pool<ConnectionManager<SqliteConnection>>,
        new_achievement: NewAchievement,
    ) -> Result<(), diesel::result::Error> {
    
    let conn: &SqliteConnection = &pool.get().unwrap();
    
    diesel::insert_into(achievements::table)
        .values(&new_achievement)
        .execute(conn)?;
        
    Ok(())
}

pub fn update(
        pool: &Pool<ConnectionManager<SqliteConnection>>,
        new_achievement: NewAchievement,
        achievement_id: i32
    ) -> Result<(), diesel::result::Error> {
    
    let conn: &SqliteConnection = &pool.get().unwrap();
    
    diesel::update(achievements.filter(id.eq(achievement_id)))
        .set((
            task.eq(new_achievement.task),
            date.eq(new_achievement.date),
            planned_time.eq(new_achievement.planned_time),
            actual_time.eq(new_achievement.actual_time),
            progress.eq(new_achievement.progress),
            is_close_on.eq(new_achievement.is_close_on),
        ))
        .execute(conn)?;
        
    Ok(())
}

pub fn delete(
        pool: &Pool<ConnectionManager<SqliteConnection>>,
        achievement_id: i32,
    ) -> Result<(), diesel::result::Error> {
    
    let conn: &SqliteConnection = &pool.get().unwrap();
    
    diesel::delete(achievements.filter(id.eq(achievement_id)))
        .execute(conn)?;
        
    Ok(())
}
