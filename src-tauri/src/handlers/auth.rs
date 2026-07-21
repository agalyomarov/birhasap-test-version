use sea_orm::{EntityTrait, PaginatorTrait};
use tauri::State;

use crate::{entities::prelude::*, state::AppState};

#[tauri::command]
pub async fn auth_login(state: State<'_, AppState>) -> Result<String, String> {
    let count = UserEntity::find()
        .count(&state.db)
        .await
        .map_err(|e| e.to_string())?;

    Ok(format!("Users: {}", count))
}
