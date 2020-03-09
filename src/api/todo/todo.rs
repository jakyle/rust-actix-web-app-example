use crate::schema::todos;
use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use futures::future::{ready, Ready};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Queryable)]
pub struct Todo {
    pub id: Uuid,
    pub finished: Option<bool>,
    pub item: String,
    pub description: Option<String>,
}

#[derive(Debug, Insertable, Clone, Deserialize)]
#[table_name = "todos"]
pub struct NewTodo {
    pub item: String,
    pub description: Option<String>,
}

#[derive(Deserialize, AsChangeset)]
#[table_name = "todos"]
pub struct UpdateTodo {
    pub item: Option<String>,
    pub description: Option<String>,
    pub finished: Option<bool>,
}

#[derive(Serialize)]
pub struct TodoResponse {
    pub id: String,
    pub item: String,
    pub finished: bool,
    pub description: Option<String>,
}

impl TodoResponse {
    pub fn new_from_todo(todo: &Todo) -> TodoResponse {
        TodoResponse {
            id: Uuid::to_string(&todo.id),
            item: todo.item.clone(),
            description: todo.description.clone(),
            finished: todo.finished.unwrap_or(false),
        }
    }
}

impl Responder for Todo {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let todo_response = TodoResponse::new_from_todo(&self);

        let body = serde_json::to_string(&todo_response).unwrap();

        // Create response and set content type
        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body)))
    }
}
