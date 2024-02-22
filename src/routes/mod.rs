use axum::Router;
use tower_http::trace::TraceLayer;

pub fn create_routes() -> Router {
  Router::new()
    // home page
    // .route("/", get(home::home))
    // Not found page
    // .fallback(not_found::not_found)
    // Tracing via logs
    .layer(TraceLayer::new_for_http())
  // assets
  // .nest_service("/global-assets", ServeDir::new("global-assets"))
  // .nest_service("/assets", ServeDir::new("fjc-client/assets"))
}
