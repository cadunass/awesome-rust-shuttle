use actix_web::HttpResponse;

#[utoipa::path(
    get,
    path = "/health-check",
    responses(
        (status = 200, description = "Health check"),
    ),
)]
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}
