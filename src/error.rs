use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    LoginFail,

    //-- Auth Errors
    
    AuthTokenNotFound,

    // -- Model Errors
    TicketDeleteFailedIdNotFound { id: u64 },
    TicketCannotBeCreated
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}",self)).into_response()
    }
}
