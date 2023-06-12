#[tauri::command]
pub async fn to_celcius(fahrenheit: f64) -> f64 {
  let result = (fahrenheit - 32.0) * 5.0 / 9.0;
  result
}

#[tauri::command]
pub async fn to_fahrenheit(celcius: f64) -> f64 {
  celcius * 9.0 / 5.0 + 32.0
}