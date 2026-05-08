use crate::env::ENV;
use crate::middleware::database::db_pool;
use crate::middleware::error::ApiResult;
use crate::service::dat_bank_service;
use axum::routing::{get, post};
use axum::{Extension, Router};
use dat::signature_key::SignatureKeyOutOption;
use std::net::IpAddr;
use dat::{VERSION_DAT_CARGO, VERSION_DAT};

pub async fn router() -> Router {
    Router::new()
        .route("/keys", post(generate_key))
        .route("/keys", get(full_key_list))
        .route("/keys/signing", get(sign_key_list))
        .route("/keys/verifying", get(verify_key_list))
        .route("/health", get(health))
        .route("/version", get(version_all))
        .route("/version/dat", get(version_dat))
        .route("/version/bank", get(version_bank))
}

async fn health() -> &'static str { "OK" }
async fn version_dat() -> &'static str { VERSION_DAT }
async fn version_bank() -> &'static str { &ENV.version }
async fn version_all() -> String {
    format!("dat {}\ndat_bank {}\ndat_cargo_library {}", VERSION_DAT, ENV.version, VERSION_DAT_CARGO)
}

pub async fn generate_key(Extension(ip_addr): Extension<IpAddr>) -> ApiResult<String> {
    let (new_kid, delete_count) = dat_bank_service::generate(db_pool()).await?;
    tracing::info!("{ip_addr} GENERATE KID-{new_kid} KEY / DELETE {delete_count} KEYS");
    Ok("OK".to_string())
}

pub async fn full_key_list(Extension(ip_addr): Extension<IpAddr>) -> ApiResult<String> {
    let (body, key_count) = dat_bank_service::get_keys(SignatureKeyOutOption::FULL, db_pool()).await?;
    tracing::info!("{ip_addr} GET {key_count} FULL KEYS");
    Ok(body)
}

pub async fn sign_key_list(Extension(ip_addr): Extension<IpAddr>) -> ApiResult<String> {
    let (body, key_count) = dat_bank_service::get_keys(SignatureKeyOutOption::SIGNING, db_pool()).await?;
    tracing::info!("{ip_addr} GET {key_count} SIGNING KEYS");
    Ok(body)
}

pub async fn verify_key_list(Extension(ip_addr): Extension<IpAddr>) -> ApiResult<String> {
    let (body, key_count) = dat_bank_service::get_keys(SignatureKeyOutOption::VERIFYING, db_pool()).await?;
    tracing::info!("{ip_addr} GET {key_count} VERIFYING KEYS");
    Ok(body)
}
