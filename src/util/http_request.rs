use super::custom_error::AppError;
use actix_web::client::Client;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::time::Duration;

pub async fn post<T: DeserializeOwned, P: Serialize>(
    url: String,
    params: P,
) -> Result<T, AppError> {
    let response = Client::default()
        .post(url)
        .timeout(Duration::new(10, 0))
        .send_json(&params)
        .await;

    let mut response = match response {
        Ok(response) => response,
        Err(_) => return Err(AppError::Timeout),
    };

    let body = match response.body().await {
        Ok(body) => body,
        Err(_) => {
            return Err(AppError::BadClientData);
        }
    };

    let body_data = match serde_json::from_slice::<T>(&body) {
        Ok(body_data) => body_data,
        Err(_) => {
            return Err(AppError::InternalError);
        }
    };

    Ok(body_data)
}

pub async fn get<T: DeserializeOwned>(url: &str) -> Result<T, AppError> {
    let response = Client::default()
        .get(url)
        .timeout(Duration::new(10, 0))
        .send()
        .await;

    let mut response = match response {
        Ok(response) => response,
        Err(_) => return Err(AppError::Timeout),
    };

    let body = match response.body().await {
        Ok(body) => body,
        Err(_) => {
            return Err(AppError::BadClientData);
        }
    };

    let body_data = match serde_json::from_slice::<T>(&body) {
        Ok(body_data) => body_data,
        Err(_) => {
            return Err(AppError::InternalError);
        }
    };

    Ok(body_data)
}
