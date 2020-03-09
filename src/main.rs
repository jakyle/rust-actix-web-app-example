pub mod api;
mod app;
mod schema;
pub mod util;

#[macro_use]
extern crate diesel;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    app::start().await
}
