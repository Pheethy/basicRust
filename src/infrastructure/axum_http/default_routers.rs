use axum::http::StatusCode;
use axum::response::IntoResponse;

pub async fn not_found() -> impl IntoResponse {
    return (StatusCode::OK, "ODOR TECH!").into_response();
}

pub async fn health_check() -> impl IntoResponse {
    return (StatusCode::OK, "Pheet!").into_response();
}
