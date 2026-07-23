use std::str::FromStr;

use migration::prelude::rust_decimal::prelude::ToPrimitive;
use sea_orm::{EntityTrait, PaginatorTrait, QueryOrder};
use serde::Serialize;
use tauri::State;

use crate::entities::prelude::*;
use crate::enums::SortDirection;
use crate::{responses::ApiErrorResponse, state::AppState};

#[tauri::command]
pub async fn admin_product_list_command(
    state: State<'_, AppState>,
    params: AdminProductListCommandRequest,
) -> Result<AdminProductListCommandResponse, ApiErrorResponse> {
    // 1. Определяем направление сортировки
    let order = match params.sort_direction {
        SortDirection::Asc => sea_orm::Order::Asc,
        SortDirection::Desc => sea_orm::Order::Desc,
    };

    let sort_column = ProductColumn::from_str(&params.sort_column).map_err(|_| {
        ApiErrorResponse::new("Nädogry sortirlama sütün name", "INVALID_SORT_COLUMN")
    })?;

    let paginator = ProductEntity::find()
        .order_by(sort_column, order)
        .paginate(&state.db, params.limit);

    let total_pages = paginator
        .num_pages()
        .await
        .map_err(|_| ApiErrorResponse::new("Maglumatlary alyp bolmady", "DB_ERROR"))?;

    let db_products = paginator
        .fetch_page(params.page - 1) // В SeaORM страницы начинаются с 0
        .await
        .map_err(|_| ApiErrorResponse::new("Maglumatlary alyp bolmady", "DB_ERROR"))?;

    // 3. Преобразуем модели БД в объекты ответа
    let products = db_products
        .into_iter()
        .map(|p| AdminProductListCommandResponseProductDTO {
            uuid: p.uuid,
            barcode: p.barcode,
            title: p.title,
            price: p.price.to_f32().unwrap_or(0.0),
            amount: p.amount.to_f32().unwrap_or(0.0),
            dimension: p.dimension,
            created_at: p.created_at,
            updated_at: p.updated_at,
        })
        .collect();

    Ok(AdminProductListCommandResponse {
        products,
        total_pages,
        current_page: params.page,
    })
}

#[derive(Serialize)]
pub struct AdminProductListCommandResponse {
    pub products: Vec<AdminProductListCommandResponseProductDTO>,
    pub total_pages: u64,
    pub current_page: u64,
}

#[derive(serde::Deserialize)]
pub struct AdminProductListCommandRequest {
    page: u64,
    limit: u64,
    sort_direction: SortDirection,
    sort_column: String,
}

#[derive(Serialize)]
pub struct AdminProductListCommandResponseProductDTO {
    pub uuid: String,
    pub barcode: String,
    pub title: String,
    pub price: f32,
    pub amount: f32,
    pub dimension: String,
    pub created_at: String,
    pub updated_at: String,
}
