use anyhow::Result;
use axum::{extract::State, response::IntoResponse, Json};
use serde_json::json;

use crate::errors::SupercellError;

use super::context::WebContext;

pub async fn handle_describe_feed_generator(
    State(web_context): State<WebContext>,
) -> Result<impl IntoResponse, SupercellError> {
    Ok(Json(json!({
        "feeds": web_context.feeds.keys().map(|k| json!({"uri": k})).collect::<Vec<_>>(),
    })))
}
