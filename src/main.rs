mod task;
mod ui;


use tauri::generate_handler;


fn main() {
tauri::Builder::default()
.invoke_handler(generate_handler![
ui::list_tasks,
ui::create_task,
ui::update_task,
ui::delete_task,
ui::toggle_status,
ui::export_tasks
])
.run(tauri::generate_context!())
.expect("error while running tauri app");
}