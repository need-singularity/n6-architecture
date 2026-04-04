use axum::Json;
use crate::data;

pub async fn get_history() -> Json<Vec<serde_json::Value>> {
    Json(data::read_growth_log())
}
