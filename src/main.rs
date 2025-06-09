mod models;
mod auth;
mod handlers;
mod templates;

use handlers::{auth_handlers, url_handlers, page_handlers};
use auth::jwt::JwtService;
use auth::middleware::optional_auth_middleware;

use axum::{
    routing::{get, post, delete},
    Router, serve, middleware,
};
use dotenvy::dotenv;
use sqlx::PgPool;
use std::env;
use std::net::SocketAddr;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub jwt_service: JwtService,
    pub base_url: String,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    
    let jwt_secret = env::var("JWT_SECRET").unwrap_or_else(|_| {
        eprintln!("WARNING: JWT_SECRET not set in .env file. Using default key. This is NOT secure for production!");
        "your-fallback-secret-key-change-this-in-production".to_string()
    });

    let base_url = env::var("BASE_URL").unwrap_or_else(|_| {
        println!("BASE_URL not set in .env, defaulting to http://localhost:3000");
        "http://localhost:3000".to_string()
    });

    let base_url = base_url.trim_end_matches('/').to_string();

    let pool = PgPool::connect(&database_url).await?;
    let jwt_service = JwtService::new(&jwt_secret);

    let app_state = AppState {
        pool,
        jwt_service,
        base_url,
    };

    let app = Router::new()
        // Page routes
        .route("/", get(page_handlers::home_handler))
        .route("/signin", get(page_handlers::signin_page_handler))
        .route("/signup", get(page_handlers::signup_page_handler))
        .route("/profile", get(page_handlers::profile_page_handler))

        // API routes
        .route("/register", post(auth_handlers::register_handler))
        .route("/login", post(auth_handlers::login_handler))
        .route("/shorten", post(url_handlers::shorten_handler))
        .route("/my-urls", get(url_handlers::get_my_urls_handler))
        .route("/delete-url/{id}", delete(url_handlers::delete_url_handler))
        .route("/{code}", get(url_handlers::redirect_handler))
        .layer(middleware::from_fn_with_state(
            app_state.jwt_service.clone(),
            optional_auth_middleware,
        ))
        .with_state(app_state);

    let port = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .expect("PORT must be a valid number");

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    
    println!("Server starting...");
    println!("Listening on: {}", addr);
    println!("Base URL configured as: {}", 
        env::var("BASE_URL").unwrap_or_else(|_| "http://localhost:3000 (default)".to_string()));

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    serve(listener, app).await.unwrap();

    Ok(())
}