use std::sync::Arc;
use std::time::Duration;

use axum::extract::MatchedPath;
use axum::http::Request;
use axum::routing::{get};
use axum::Router;

use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;

use clap::Args;
use thiserror::__private::AsDisplay;
use tracing::{info_span, Span};

use crate::error::AppResult;
use crate::server::routes::default;

struct ServerState {}

impl ServerState {
    fn new() -> Self {
        Self { }
    }
}

#[derive(Debug, Args)]
pub struct RouterArgs {
}

impl RouterArgs {
    pub fn build_router(&self) -> AppResult<Router> {
        init_router(self)
    }
}

fn init_router(_args: &RouterArgs) -> AppResult<Router> {
    let state = Arc::new(ServerState::new());

    let router = Router::new()
        .route("/health", get(default::health_check))
        .with_state(state)
        .layer((
            TraceLayer::new_for_http()
                .make_span_with(|request: &Request<_>| {
                    // Log the matched route's path (with placeholders not filled in).
                    let matched_path = request
                        .extensions()
                        .get::<MatchedPath>()
                        .map(MatchedPath::as_str);
                    tracing::trace!("{}", request.uri().as_display());

                    info_span!(
                        "http_request",
                        method = ?request.method(),
                        matched_path,
                        some_other_field = tracing::field::Empty,
                    )
                })
                .on_request(|_request: &Request<_>, _span: &Span| {
                    // You can use `_span.record("some_other_field", value)` in one of these
                    // closures to attach a value to the initially empty field in the info_span
                    // created above.
                }),
            TimeoutLayer::new(Duration::from_secs(15)),
        ));
    Ok(router)
}
