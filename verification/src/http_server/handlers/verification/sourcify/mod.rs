mod api;
mod client;
mod metadata;
mod types;

use self::api::SourcifyApiClient;
use self::types::ApiRequest;
use actix_web::web;
use actix_web::{error::Error, web::Json};

use super::VerificationResponse;

pub use client::SourcifyClient;

pub async fn verify(
    sourcify_client: web::Data<SourcifyApiClient>,
    params: Json<ApiRequest>,
) -> Result<Json<VerificationResponse>, Error> {
    let response =
        api::verify_using_sourcify_client(sourcify_client.into_inner(), params.into_inner())
            .await?;
    Ok(Json(response))
}
