use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(Debug)]
pub struct SupercellError(pub anyhow::Error);

impl<E> From<E> for SupercellError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

impl IntoResponse for SupercellError {
    fn into_response(self) -> Response {
        {
            tracing::error!(error = ?self.0, "internal server error");
            (StatusCode::INTERNAL_SERVER_ERROR).into_response()
        }
    }
}
