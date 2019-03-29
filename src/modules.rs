use super::schema::*;
use chrono::{NaiveTime, NaiveDate};

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