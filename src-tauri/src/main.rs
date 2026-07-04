// Предотвращает дополнительное окно консоли в выпуске Windows, НЕ УДАЛЯЙТЕ!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    app_lib::run();
}
