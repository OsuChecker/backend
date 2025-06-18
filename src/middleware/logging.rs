use std::time::Instant;
use tower_http::trace::{TraceLayer, DefaultMakeSpan, DefaultOnResponse};
use axum::{
    http::Request,
    response::Response,
    middleware::{self, Next},
    body::Body,
};
use tracing::{info, Level};

pub fn logging_layer() -> TraceLayer<tower_http::classify::SharedClassifier<tower_http::classify::ServerErrorsAsFailures>> {
    TraceLayer::new_for_http()
        .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
        .on_response(DefaultOnResponse::new().level(Level::INFO))
}

pub async fn track_execution_time(req: Request<Body>, next: Next) -> Response {
    let path = req.uri().path().to_owned();
    let method = req.method().clone();
    
    let start = Instant::now();
    let response = next.run(req).await;
    let duration = start.elapsed();
    
    info!(
        "Request {} {} completed in {:.2?} with status {}",
        method,
        path,
        duration,
        response.status()
    );
    
    response
}

// Option 1: Utiliser uniquement le middleware personnalisé
pub fn setup_middleware<S>(app: axum::Router<S>) -> axum::Router<S> 
where
    S: Clone + Send + Sync + 'static,
{
    app.layer(middleware::from_fn(track_execution_time))
    // Commenté pour éviter la duplication
    // .layer(logging_layer())
} 