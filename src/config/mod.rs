use std::sync::Arc;

use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

use crate::{router::router::router, ApiContext};

pub mod app;

pub async fn http_serve(listener: TcpListener, api_context: ApiContext) {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);
    let app = router().layer(cors).with_state(Arc::new(api_context));
    //let router = router().with_state(Arc::new(api_context));

    axum::serve(listener, app).await.unwrap();
}
