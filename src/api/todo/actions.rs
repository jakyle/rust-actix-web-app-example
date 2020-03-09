use super::todo::{NewTodo, Todo, UpdateTodo};
use crate::schema::todos::dsl::*;
use diesel::prelude::*;
use uuid::Uuid;

pub fn find_todo_by_id(
    conn: &PgConnection,
    uid: &str,
) -> Result<Option<Todo>, diesel::result::Error> {
    let uuid: Uuid = Uuid::parse_str(&uid).unwrap();

    let todo_item = todos.filter(id.eq(uuid)).first::<Todo>(conn).optional()?;

    Ok(todo_item)
}

pub fn select_all_todos(conn: &PgConnection) -> Result<Vec<Todo>, diesel::result::Error> {
    let all_todos = todos.get_results(conn)?;

    Ok(all_todos)
}

pub fn insert_new_todo(
    new_todo: NewTodo,
    conn: &PgConnection,
) -> Result<Todo, diesel::result::Error> {
    let todo = diesel::insert_into(todos)
        .values(&new_todo)
        .get_result(conn)?;

    Ok(todo)
}

pub fn update_todo(
    conn: &PgConnection,
    updated_todo: UpdateTodo,
    uid: &str,
) -> Result<Todo, diesel::result::Error> {
    let uuid: Uuid = Uuid::parse_str(&uid).unwrap();

    let todo = diesel::update(todos.filter(id.eq(uuid)))
        .set(updated_todo)
        .get_result(conn)?;

    Ok(todo)
}

pub fn delete_todo(conn: &PgConnection, uid: &str) -> Result<(), diesel::result::Error> {
    let uuid: Uuid = Uuid::parse_str(&uid).unwrap();

    diesel::delete(todos.filter(id.eq(uuid))).execute(conn)?;

    Ok(())
}
