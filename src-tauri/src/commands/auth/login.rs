use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::Serialize;
use tauri::State;

use crate::{
    entities::prelude::*,
    requests::CommandRequest,
    responses::{ApiErrorResponse, CommandResponse},
    state::AppState,
};

#[tauri::command]
pub async fn auth_login_command(
    state: State<'_, AppState>,
    params: CommandRequest<AuthLoginCommandRequest>,
) -> Result<CommandResponse<AuthLoginCommandResponse>, ApiErrorResponse> {
    let user: Option<UserSchema::Model> = UserEntity::find()
        .filter(UserSchema::Column::Login.eq(params.data.login))
        .filter(UserSchema::Column::Password.eq(params.data.password))
        .one(&state.db)
        .await
        .map_err(|_e| ApiErrorResponse::new("Ulanyjy ya-da pinkod nadogry", "AUTH_FAILED"))?;

    match user {
        Some(user) => {
            let response = AuthLoginCommandResponse::new(user.uuid, "admin");
            return Ok(CommandResponse { data: response });
        }

        None => Err(ApiErrorResponse::new(
            "Ulanyjy ya-da pinkod nadogry",
            "NOT FOUND",
        )),
    }
}

#[derive(Serialize, Debug)]
pub struct AuthLoginCommandResponse {
    token: String,
    role: String,
}

impl AuthLoginCommandResponse {
    pub fn new(token: impl Into<String>, role: impl Into<String>) -> Self {
        Self {
            token: token.into(),
            role: role.into(),
        }
    }
}

#[derive(serde::Deserialize)]
pub struct AuthLoginCommandRequest {
    login: String,
    password: String,
}
