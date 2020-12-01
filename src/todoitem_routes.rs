use super::models::*;
use crate::db::Conn;
use rocket_contrib::json::Json;
use serde_json::Value;

#[post("/addlist", format = "application/json", data = "<newlist>")]
pub fn addlist(conn: Conn, newlist: Json<TodoListNew>) -> Json<Value> {
    match TodoList::create_list(&conn, newlist.into_inner()) {
        Ok(list) => Json(json!({
            "status":200,
            "result": list
        })),
        Err(error) => {
            println!("error: {}", error);
            Json(json!({
                "status":400,
                "result": "something went wrong"
            }))
        }
    }
}
