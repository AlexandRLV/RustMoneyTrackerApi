use axum::{http::StatusCode, response::IntoResponse};
use serde::Serialize;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum Error {
    LoginFail,
    AuthFailNoAuthTokenCookie,
    AuthFailTokenWrongFormat,
    AuthFailContextNotInRequestExt,
    TicketDeleteFailIdNotFound { id: u64 },
}

impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> core::result::Result<(), std::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        println!("--> {:<12} - Error - {self:?}", "INTO_RESPONSE");
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();
        response.extensions_mut().insert(self);
        response
    }
}

impl Error {
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        #[allow(unreachable_patterns)]
        match self {
            Self::LoginFail => (
                StatusCode::FORBIDDEN,
                ClientError::LOGIN_FAIL
            ),
            Self::AuthFailContextNotInRequestExt | Self::AuthFailNoAuthTokenCookie | Self::AuthFailTokenWrongFormat => (
                StatusCode::FORBIDDEN,
                ClientError::NO_AUTH
            ),
            Self::TicketDeleteFailIdNotFound { .. } => (
                StatusCode::BAD_REQUEST,
                ClientError::INVALID_PARAMS
            ),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::SERVICE_ERROR
            ),
        }
    }
}

#[derive(Debug, strum_macros::AsRefStr)]
#[allow(non_camel_case_types)]
pub enum ClientError {
    LOGIN_FAIL,
    NO_AUTH,
    INVALID_PARAMS,
    SERVICE_ERROR,
}