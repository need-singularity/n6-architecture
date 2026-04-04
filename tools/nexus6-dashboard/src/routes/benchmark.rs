use axum::Json;
use crate::data;

pub async fn get_benchmark() -> Json<serde_json::Value> {
    match data::read_benchmark() {
        Ok(v) => Json(v),
        Err(_) => Json(serde_json::json!({"error": "no benchmark data"})),
    }
}
