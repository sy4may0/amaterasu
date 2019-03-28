table! {
    categories (id) {
        id -> Nullable<Integer>,
        name -> Text,
    }
}

table! {
    tasks (id) {
        id -> Nullable<Integer>,
        name -> Text,
        description -> Nullable<Text>,
        category -> Integer,
        tasktype -> Integer,
        status -> Integer,
        unplanned -> Integer,
    }
}

table! {
    tasktypes (id) {
        id -> Nullable<Integer>,
        name -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    categories,
    tasks,
    tasktypes,
);
