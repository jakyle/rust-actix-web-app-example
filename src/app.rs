use crate::api::{dog::dog_routes::*, todo::todo_routes::*};
use actix_web::{middleware, web, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

pub async fn start() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info,diesel=debug");
    env_logger::init();

    let bind = "127.0.0.1:8080";
    println!("Now hosting site on {}", bind);

    let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<PgConnection>::new(connspec);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .data(web::JsonConfig::default().limit(4096))
            .service(index)
            .service(web::scope("/dog").service(message).service(get_all_dogs))
            .service(
                web::scope("/todo")
                    .service(get_todo)
                    .service(get_all_todo)
                    .service(post_todo)
                    .service(put_todo)
                    .service(delete_todo),
            )
    })
    .bind(&bind)?
    .run()
    .await
}
