use crate::{
    models::{Todo, TodoStatus},
    AppState,
};

#[tauri::command]
pub async fn add_todo(state: tauri::State<'_, AppState>, description: &str) -> Result<(), String> {
    let db = &state.db;

    sqlx::query("INSERT INTO todos (description, status) VALUES (?1, ?2)")
        .bind(description)
        .bind(TodoStatus::Incomplete)
        .execute(db)
        .await
        .map_err(|e| format!("Error saving todo: {}", e))?;

    Ok(())
}

#[tauri::command]
pub async fn get_todos(state: tauri::State<'_, AppState>) -> Result<Vec<Todo>, String> {
    let db = &state.db;
    let todos = sqlx::query_as::<_, Todo>("SELECT * FROM todos")
        .fetch_all(db)
        .await
        .map_err(|e| format!("Failed to get todos: {}", e))?;

    Ok(todos)
}

#[tauri::command]
pub async fn update_todo(state: tauri::State<'_, AppState>, todo: Todo) -> Result<(), String> {
    let db = &state.db;

    sqlx::query("UPDATE todos SET description = ?1, status = ?2 WHERE id = ?3")
        .bind(todo.description)
        .bind(todo.status)
        .bind(todo.id)
        .execute(db)
        .await
        .map_err(|e| format!("could not update todo {}", e))?;

    Ok(())
}

#[tauri::command]
pub async fn delete_todo(state: tauri::State<'_, AppState>, id: u16) -> Result<(), String> {
    let db = &state.db;

    sqlx::query("DELETE FROM todos WHERE id = ?1")
        .bind(id)
        .execute(db)
        .await
        .map_err(|e| format!("could not delete todo {}", e))?;

    Ok(())
}
