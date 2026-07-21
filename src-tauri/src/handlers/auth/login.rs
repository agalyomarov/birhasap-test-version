#[tauri::command]
pub fn auth_login() -> Result<String, String> {
    // sqlx::query("SELECT 1")
    //     .execute(&state.db)
    //     .await
    //     .map_err(|e| e.to_string())?;
    Ok("OK".to_string())
}
