use actix_cors::Cors;
use actix_web::http;
use actix_web::middleware::Logger;
use actix_web::web::{self, ServiceConfig};
use awesome_rust_shuttle::routes::health_check;
use awesome_rust_shuttle::utils::AppState;
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::CustomError;
use sqlx::PgPool;

#[shuttle_runtime::main]
async fn main(
    #[shuttle_runtime::Secrets] secrets: shuttle_runtime::SecretStore,
    #[shuttle_shared_db::Postgres(
        local_uri = "postgres://postgres:password@postgres:5432/devcontainer"
    )]
    pool: PgPool,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    tracing::info!("Tracing subscriber initialized.");

    // let config_content =
    //     fs::read_to_string("configuration/base.yml").expect("Failed to read configuration");
    // let config: Config = serde_yaml::from_str(&config_content).expect("Failed to parse YAML");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .map_err(CustomError::new)?;

    let state = web::Data::new(AppState { pool, secrets });

    // let default_rate_limit_config = create_default_rate_limit_config();
    // let auth_strict_rate_limit_config = create_strict_rate_limit_config();
    // let boat_creation_strict_rate_limit_config = create_strict_rate_limit_config();

    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(
            web::scope("")
                .app_data(state)
                .wrap(Logger::default())
                .wrap(
                    Cors::default()
                        .allow_any_origin()
                        .allowed_methods(vec!["GET", "PATCH", "POST", "PUT", "DELETE"])
                        .allowed_headers(vec![
                            http::header::AUTHORIZATION,
                            http::header::CONTENT_TYPE,
                        ])
                        .supports_credentials(),
                )
                .route("/health-check", web::get().to(health_check)),
        );
    };

    Ok(config.into())
}
