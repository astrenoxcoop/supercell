use super::{
    context::WebContext, handle_describe_feed_generator::handle_describe_feed_generator,
    handle_get_feed_skeleton::handle_get_feed_skeleton, handle_index::handle_index,
    handle_well_known::handle_well_known,
};
use axum::{http::HeaderValue, routing::get, Router};
use http::{
    header::{ACCEPT, ACCEPT_LANGUAGE},
    Method,
};
use std::time::Duration;
use tower_http::cors::CorsLayer;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;

pub fn build_router(web_context: WebContext) -> Router {
    Router::new()
        .route("/", get(handle_index))
        .route("/.well-known/did.json", get(handle_well_known))
        .route(
            "/xrpc/app.bsky.feed.getFeedSkeleton",
            get(handle_get_feed_skeleton),
        )
        .route(
            "/xrpc/app.bsky.feed.describeFeedGenerator",
            get(handle_describe_feed_generator),
        )
        .layer((
            TraceLayer::new_for_http(),
            TimeoutLayer::new(Duration::from_secs(10)),
        ))
        .layer(
            CorsLayer::new()
                .allow_origin(web_context.external_base.parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET])
                .allow_headers([ACCEPT_LANGUAGE, ACCEPT]),
        )
        .with_state(web_context.clone())
}
