use super::{
    actions,
    todo::{NewTodo, TodoResponse, UpdateTodo},
};
use actix_web::web::Json;
use actix_web::{body::Body, delete, get, post, put, web, Error, HttpResponse, Responder};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[get("")]
pub async fn get_all_todo(pool: web::Data<DbPool>) -> Result<Json<Vec<TodoResponse>>, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let all_todos = web::block(move || actions::select_all_todos(&conn))
        .await
        .map_err(|e| {
            println!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    let todo_responses = all_todos
        .iter()
        .map(|todo| TodoResponse::new_from_todo(todo))
        .collect();

    Ok(Json(todo_responses))
}

#[get("/{id}")]
pub async fn get_todo(
    pool: web::Data<DbPool>,
    path: web::Path<String>,
) -> Result<impl Responder, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let todo = web::block(move || actions::find_todo_by_id(&conn, &path.into_inner()))
        .await
        .map_err(|e| {
            println!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(todo)
}

#[post("")]
pub async fn post_todo(
    pool: web::Data<DbPool>,
    body: web::Json<NewTodo>,
) -> Result<impl Responder, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let todo = web::block(move || actions::insert_new_todo(body.into_inner(), &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(todo)
}

#[put("/{id}")]
pub async fn put_todo(
    pool: web::Data<DbPool>,
    path: web::Path<String>,
    body: web::Json<UpdateTodo>,
) -> Result<impl Responder, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let todo =
        web::block(move || actions::update_todo(&conn, body.into_inner(), &path.into_inner()))
            .await
            .map_err(|e| {
                eprintln!("{}", e);
                HttpResponse::InternalServerError().finish()
            })?;

    Ok(todo)
}

#[delete("/{id}")]
pub async fn delete_todo(
    pool: web::Data<DbPool>,
    path: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    web::block(move || actions::delete_todo(&conn, &path.into_inner()))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().body(Body::from("record removed")))
}
