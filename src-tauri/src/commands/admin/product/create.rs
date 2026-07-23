use sea_orm::prelude::Decimal;
use sea_orm::ActiveModelTrait;
use sea_orm::ActiveValue::Set;
use serde::Serialize;
use tauri::State;
use uuid::Uuid;
use validator::Validate;

use crate::entities::prelude::*;
use crate::{responses::ApiErrorResponse, state::AppState};

#[tauri::command]
pub async fn admin_product_create_command(
    state: State<'_, AppState>,
    params: AdminProductCreateCommandRequest,
) -> Result<AdminProductCreateCommandResponse, ApiErrorResponse> {
    params.validate()?;
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

#[derive(serde::Deserialize, Validate)]
pub struct AdminProductCreateCommandRequest {
    #[validate(length(min = 4, max = 100, message = "Barcode dogry girizin"))]
    barcode: String,
    #[validate(length(min = 4, max = 1000, message = "Title dogry girizin"))]
    title: String,
    #[validate(range(min = 0, max = 1000_000_000, message = "Price dogry girizin"))]
    price: u32,
    #[validate(range(min = 0, max = 1000_000_000, message = "Amount dogry girizin"))]
    amount: u32,
    #[validate(length(min = 4, max = 50, message = "Dimension dogry girizin"))]
    dimension: String,
}
