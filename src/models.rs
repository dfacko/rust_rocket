use crate::diesel::prelude::*;
use crate::schema::*;
use anyhow::Result;
#[derive(Debug, Queryable, Serialize, Clone)]
pub struct TodoList {
    pub id: i32,
    pub title: String,
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[table_name = "todo_list"]
pub struct TodoListNew<'a> {
    pub title: &'a str,
}

#[derive(Debug, Queryable, Serialize, Clone)]
pub struct TodoItem {
    pub id: i32,
    pub list_id: i32,
    pub task: String,
    pub finished: bool,
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[table_name = "todo_item"]
pub struct TodoItemNew<'a> {
    pub list_id: i32,
    pub task: &'a str,
    pub finished: bool,
}

impl TodoList {
    pub fn get_list_by_id(
        conn: &PgConnection,
        list_id: i32,
    ) -> Result<Vec<TodoList>, diesel::result::Error> {
        use crate::schema::todo_list::dsl::*;

        match todo_list.find(list_id).load::<TodoList>(conn) {
            Ok(list) => Ok(list),
            Err(error) => Err(error),
        }
    }

    pub fn get_all_lists(conn: &PgConnection) -> Result<Vec<TodoList>, diesel::result::Error> {
        use crate::schema::todo_list::dsl::*;
        match todo_list.load::<TodoList>(conn) {
            Ok(list) => Ok(list),
            Err(error) => Err(error),
        }
    }

    pub fn create_list(
        conn: &PgConnection,
        list: TodoListNew,
    ) -> Result<TodoList, diesel::result::Error> {
        diesel::insert_into(todo_list::table)
            .values(&list)
            .get_result(conn)
    }

    pub fn delete_list(
        // returns 0 if no rows are deleted
        conn: &PgConnection,
        delete_id: i32,
    ) -> Result<usize, diesel::result::Error> {
        use crate::schema::todo_list::dsl::*;
        match diesel::delete(todo_list.filter(id.eq(delete_id))).execute(conn) {
            Ok(num_deleted_rows) => {
                println!("{:?}", num_deleted_rows);
                Ok(num_deleted_rows)
            }
            Err(error) => {
                println!("{:?}", error);
                Err(error)
            }
        }
    }
}
