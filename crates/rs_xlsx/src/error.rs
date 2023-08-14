use crate::xlsx::columns::XLSXColumn;
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use s3::creds::error::CredentialsError;
use s3::error::S3Error;
use serde::Serialize;
use thiserror::Error;

pub type XResult<T> = Result<T, XError>;

#[derive(Error, Debug)]
pub enum XError {
    #[error("invalid marketplace id: {0}")]
    InvalidMarketplaceId(i16),

    #[error("io err: {0}")]
    IO(#[from] std::io::Error),

    #[error("serde_json err: {0}")]
    SerdeJson(#[from] serde_json::Error),

    #[error("xlsx err: {0}")]
    Xlsx(#[from] xlsxwriter::XlsxError),

    #[error("unterminated formula: {0}")]
    UnterminatedFormula(String),
    #[error("invalid formula item: {0}")]
    InvalidFormulaItem(String),
    #[error("unhandled formula column: {:?}", 0)]
    UnhandledFormulaColumn(XLSXColumn),

    #[error("parse int err: {0}")]
    ParseInt(#[from] std::num::ParseIntError),

    #[error("regex err: {0}")]
    Regex(#[from] regex::Error),

    #[error("invalid coords: {0}")]
    InvalidCoords(String),

    #[error("sqlx err: {0}")]
    Sqlx(#[from] sqlx::Error),

    #[error("dotenvy err: {0}")]
    DotEnvy(#[from] dotenvy::Error),

    #[error("join err: {0}")]
    JoinError(#[from] tokio::task::JoinError),

    #[error("crate::model err: {0}")]
    Model(#[from] rs_models::XError),

    #[error("invalid operator: {0}")]
    InvalidOperator(String),
    #[error("invalid column type: {:?}", 0)]
    InvalidColumnType(XLSXColumn),

    #[error("redis err: {0}")]
    Redis(#[from] redis::RedisError),

    #[error("other: {0}")]
    Other(String),

    #[error("s3 credentials err: {0}")]
    Credentials(#[from] CredentialsError),

    #[error("s3 err: {0}")]
    S3(#[from] S3Error),

    #[error("std::str::Utf8Error: {0}")]
    StrUtf8(#[from] std::str::Utf8Error),
    #[error("std::string::FromUtf8Error: {0}")]
    StringUtf8(#[from] std::string::FromUtf8Error),

    #[error("jsonl err: {0}")]
    JsonLWrite(#[from] jsonl::WriteError),

    #[error("auth err: {0}")]
    Auth(#[from] auth::AuthError),

    #[error("no auth cookie")]
    NoAuthCookie,
    #[error("unauthorized")]
    Unauthorized,

    #[error("SP API error: {:?}", .0)]
    SPAPI(#[from] sp_api::error::SPAPIError),

    /// When there are no keys at all for the given marketplace
    #[error("no keys available for this marketplace")]
    NoKeys,
}

impl actix_web::error::ResponseError for XError {
    fn status_code(&self) -> StatusCode {
        use XError::*;

        match self {
            NoAuthCookie => StatusCode::UNAUTHORIZED,
            Unauthorized => StatusCode::UNAUTHORIZED,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
    fn error_response(&self) -> HttpResponse {
        println!("rcvd err: (status code = {} {:?}", self.status_code(), self);
        let msg = match self.status_code() {
            StatusCode::UNAUTHORIZED => ErrMessage::msg("you must be logged in to do that"),
            _ => ErrMessage::msg("something went wrong"),
        };
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(serde_json::to_string(&msg).unwrap())
    }
}

#[derive(Serialize)]
struct ErrMessage<'a> {
    message: &'a str,
}

impl<'a> ErrMessage<'a> {
    pub fn msg(msg: &'a str) -> Self {
        Self { message: msg }
    }
}
