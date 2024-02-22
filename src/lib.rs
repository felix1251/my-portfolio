// mod database;
// mod routes;

// use axum;
// use tokio;
use tracing;
use tracing_subscriber;

pub async fn run(_db_uri: String) {
  tracing_subscriber::fmt()
    .with_max_level(tracing::Level::DEBUG)
    .init();

  // tracing::debug!("Connecting to DB");
  // let db = database::init(db_uri).await;
  // tracing::debug!("DB Connected");

  // routes
  // let services = routes::create_routes(db).await;

  // let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
  // let local_address = listener.local_addr().unwrap();

  // tracing::debug!("listening on http://{}", local_address);

  // // run our app with hyper, listening globally on port 3000
  // axum::serve(listener, services).await.unwrap();
}
