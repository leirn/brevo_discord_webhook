use actix_web::ResponseError;
use derive_more::Display;

#[derive(Debug, Display)]
pub enum AuthorizationError {
    #[display(fmt = "Access denied, invalid ip address")]
    InvalidIPAddress,
}

impl ResponseError for AuthorizationError {}
