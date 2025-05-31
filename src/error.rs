use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    LoginFail,

    // -- Model Errors
    TicketDeleteFailedIdNotFound{
        id: u64
    },
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED CLIENT ERROR").into_response()
    }
}
