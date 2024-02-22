mod routes;

pub async fn run() {
  tracing_subscriber::fmt()
    .with_max_level(tracing::Level::DEBUG)
    .init();

  // routes
  let services = routes::create_routes();

  let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
  let local_address = listener.local_addr().unwrap();

  tracing::debug!("listening on http://{}", local_address);

  // run our app with hyper, listening globally on port 3000
  axum::serve(listener, services).await.unwrap();
}
