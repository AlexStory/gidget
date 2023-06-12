#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod conversions;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      conversions::temperature::to_celcius,
      conversions::temperature::to_fahrenheit,
      conversions::image_conversion::convert_image
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
