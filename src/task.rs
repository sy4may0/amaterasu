use super::schema::tasks;

#[derive(Debug, Eq, PartialEq, Queryable)]
pub struct Task {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub category: u32,
    pub tasktype: u32,
    pub status: bool,
    pub unplanned: bool,
    
}

#[derive(Debug, Eq, PartialEq, Insertable)]
#[table_name = "tasks"]
pub struct NewTask {
    pub name: String, 
    pub description: String,
    pub category: i32,
    pub tasktype: i32,
    pub status: i32, 
    pub unplanned: i32,
    
}