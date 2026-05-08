use crate::api;
use crate::middleware::database::db_pool;
use crate::middleware::error::ApiResult;
use crate::service::dat_bank_service;
use axum::extract::Path;
use axum::Router;
use axum::routing::{get, post};
use dat::dat_bank::DatBank;
use dat::signature_key::SignatureKeyOutOption;

pub async fn debug_router() -> Router {
    api::router().await
        .route("/dat", post(to_dat))
        .route("/dat/{dat}", get(read_dat))
}

async fn to_dat(body: String) -> ApiResult<String> {
    tracing::info!("POST /dat issue DAT (Debug)");

    let mut plain = String::new();
    let mut secret = String::new();

    let lines = body.split('\n')
        .filter(|line| !line.is_empty())
        .collect::<Vec<&str>>();

    match lines.len() {
        2 => {
            plain = lines[0].to_string();
            secret = lines[1].to_string();
        },
        1 => {
            plain = lines[0].to_string();
        },
        0 => {},
        _ => {
            return Ok("ERROR: usage:\nplain\nsecure".to_string())
        }
    }

    Ok(bank().await?.to_dat(&plain, &secret)?)
}

async fn read_dat(Path(dat): Path<String>) -> ApiResult<String> {
    tracing::info!("GET /dat Read DAT (Debug)");
    let payload = bank().await?.to_payload(dat)?.to_string_payload()?;

    Ok(format!("{}", payload))
}

async fn bank() -> ApiResult<DatBank<i64>> {
    let bank: DatBank<i64> = DatBank::new();
    let (body, _) = dat_bank_service::get_keys(SignatureKeyOutOption::FULL, db_pool()).await?;
    bank.import(&body, true)?;
    Ok(bank)
}

