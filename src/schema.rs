table! {
    achievements (id) {
        id -> Integer,
        task -> Integer,
        date -> Date,
        planned_time -> Nullable<Time>,
        actual_time -> Nullable<Time>,
        progress -> Integer,
        is_close_on -> Integer,
    }
}

table! {
    categories (id) {
        id -> Integer,
        name -> Varchar,
    }
}

table! {
    tasks (id) {
        id -> Integer,
        name -> Varchar,
        description -> Nullable<Text>,
        category -> Integer,
        tasktype -> Integer,
        status -> Integer,
        unplanned -> Integer,
    }
}

table! {
    tasktypes (id) {
        id -> Integer,
        name -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    achievements,
    categories,
    tasks,
    tasktypes,
);
