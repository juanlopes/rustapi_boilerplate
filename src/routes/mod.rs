pub mod health;

use axum::Router;
use health::routes as healthRoutes;

pub fn health_routes() -> Router {
  Router::new().nest("/health", healthRoutes())
}