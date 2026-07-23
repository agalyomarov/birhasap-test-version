use sea_orm::prelude::Decimal;
use sea_orm::ActiveModelTrait;
use sea_orm::ActiveValue::Set;
use serde::Serialize;
use tauri::State;
use uuid::Uuid;

use crate::entities::prelude::*;
use crate::{responses::ApiErrorResponse, state::AppState};

#[tauri::command]
pub async fn admin_product_create_command(
    state: State<'_, AppState>,
    params: AdminProductCreateCommandRequest,
) -> Result<AdminProductCreateCommandResponse, ApiErrorResponse> {
    ProductSchema {
        uuid: Set(Uuid::now_v7().to_string()),
        barcode: Set(params.barcode),
        title: Set(params.title),
        price: Set(Decimal::from(params.price)),
        amount: Set(Decimal::from(params.amount)),
        dimension: Set(params.dimension),
        ..Default::default()
    }
    .insert(&state.db)
    .await
    .map_err(|_e| ApiErrorResponse::new("Haryt gosmakda yalnyshlyk yuze chykdy", "DB_ERROR"))?;
    Ok(AdminProductCreateCommandResponse)
}

#[derive(Serialize)]
pub struct AdminProductCreateCommandResponse;

#[derive(serde::Deserialize)]
pub struct AdminProductCreateCommandRequest {
    barcode: String,
    title: String,
    price: u32,
    amount: u32,
    dimension: String,
}
