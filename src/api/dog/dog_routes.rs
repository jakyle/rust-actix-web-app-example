use super::models::{all_dogs, api_dog};
use crate::util::http_request;
use actix_web::{get, web, Responder};

const DOG_CEO_API: &str = "https://dog.ceo/api/";

#[get("/{id}/{name}/index.html")]
pub async fn index(info: web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", info.1, info.0)
}

#[get("/random")]
pub async fn message() -> impl Responder {
    http_request::get::<api_dog::DogMessage>(&format!("{}breeds/image/random", DOG_CEO_API)).await
}

#[get("/all")]
pub async fn get_all_dogs() -> impl Responder {
    http_request::get::<all_dogs::AllDogs>(&format!("{}breeds/list/all", DOG_CEO_API)).await
}
