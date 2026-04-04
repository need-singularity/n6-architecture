use axum::response::Html;

pub async fn serve_dashboard() -> Html<&'static str> {
    Html(include_str!("../../static/index.html"))
}
