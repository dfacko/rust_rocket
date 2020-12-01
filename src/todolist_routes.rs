use super::models::*;
use crate::db::Conn;
use rocket_contrib::json::Json;
use serde_json::Value;

#[get("/lists", format = "application/json")]
pub fn lists(conn: Conn) -> Json<Value> {
    let lists = TodoList::get_all_lists(&conn);

    match lists {
        Ok(lists) => Json(json!({
            "status":200,
            "result":lists
        })),
        Err(error) => {
            println!("error: {}", error);
            Json(json!({
                "status":400,
                "result":"oops"
            }))
        }
    }
}

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

#[get("/listbyid/<id>", format = "application/json")]
pub fn list_by_id(conn: Conn, id: i32) -> Json<Value> {
    match TodoList::get_list_by_id(&conn, id) {
        Ok(list) => Json(json!({
            "status":200,
            "result": list
        })),
        Err(error) => {
            println!("error: {}", error);
            Json(json!({
                "status":400,
                "result": "something went wron"
            }))
        }
    }
}

#[delete("/listbyid/<id>", format = "application/json")]
pub fn delete_list(conn: Conn, id: i32) -> Json<Value> {
    match TodoList::delete_list(&conn, id) {
        Ok(list) => Json(json!({
            "status":200,
            "result": format!("{} list has been deleted",list)
        })),
        Err(error) => {
            println!("error: {}", error);
            Json(json!({
                "status":400,
                "result": "something went wron"
            }))
        }
    }
}
