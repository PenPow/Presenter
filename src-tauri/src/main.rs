#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use comrak::{markdown_to_html, ComrakOptions};

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![parse])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn parse(md: &str) -> String {
	return markdown_to_html(md, &ComrakOptions::default());
}
