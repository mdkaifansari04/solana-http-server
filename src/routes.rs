use axum::{Json, extract::{Path}, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{pubkey::Pubkey, native_token::LAMPORTS_PER_SOL};
use std::str::FromStr;

#[derive(Deserialize)]
pub struct AirdropRequest {
    pub address: String,
}

#[derive(Serialize)]
pub struct AirdropResponse {
    pub address: String,
    pub balance: u64,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

pub async fn airdrop(Json(payload): Json<AirdropRequest>) -> Result<Json<AirdropResponse>, (StatusCode, Json<ErrorResponse>)> {
    let client = RpcClient::new("https://api.devnet.solana.com".to_string());
    let pubkey = Pubkey::from_str(&payload.address)
        .map_err(|_| (StatusCode::BAD_REQUEST, Json(ErrorResponse { error: "Invalid address format".to_string() })))?;

    client.request_airdrop(&pubkey, LAMPORTS_PER_SOL)
        .map_err(|e| (StatusCode::BAD_REQUEST, Json(ErrorResponse { error: format!("Airdrop failed: {}", e) })))?;
    let balance = client.get_balance(&pubkey)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: format!("Failed to get balance: {}", e) })))?;

    Ok(Json(AirdropResponse { address: payload.address, balance }))
}

pub async fn get_balance(Path(address): Path<String>) -> Result<Json<AirdropResponse>, (StatusCode, Json<ErrorResponse>)> {
    let client = RpcClient::new("https://api.devnet.solana.com".to_string());
    let pubkey = Pubkey::from_str(&address)
        .map_err(|_| (StatusCode::BAD_REQUEST, Json(ErrorResponse { error: "Invalid address format".to_string() })))?;
    let balance = client.get_balance(&pubkey)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: format!("Failed to get balance: {}", e) })))?;
    Ok(Json(AirdropResponse { address, balance }))
}
