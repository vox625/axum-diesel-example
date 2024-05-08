use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::todos;

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = todos)]
pub struct NewTodo {
    pub description: String,
}

#[derive(Queryable, AsChangeset, Selectable, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Todo {
    pub id: i32,
    pub description: String,
    pub completed: bool,
}
