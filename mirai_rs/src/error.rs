use std::error::{Error};

use reqwest::Error as ReqError;

use std::fmt::Formatter;

use crate::Code;

pub type HttpResult<T> = std::result::Result<T, HttpError>;

#[derive(Debug)]
pub enum HttpError {
    Reqwest(ReqError),
    StatusCode(StatusCodeError),
}

impl From<ReqError> for HttpError {
    fn from(e: ReqError) -> Self {
        HttpError::Reqwest(e)
    }
}

impl From<StatusCodeError> for HttpError {
    fn from(e: StatusCodeError) -> Self {
        HttpError::StatusCode(e)
    }
}

impl std::fmt::Display for HttpError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            HttpError::Reqwest(e) => { e.fmt(f) },
            HttpError::StatusCode(e) => { e.fmt(f) },
        }
    }
}

impl Error for HttpError {}

#[derive(Debug)]
pub struct StatusCodeError {
    code: Code,
    action: String,
}

impl std::fmt::Display for StatusCodeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let msg = match self.code {
            SUCCESS => return Ok(()),
            WRONG_AUTH_KEY => "Wrong auth key",
            NO_SUCH_BOT => "No such bot",
            WRONG_SESSION => "Wrong session",
            UNAUTHORIZED => "Session wasn't authorized",
            NO_SUCH_TARGET => "No such target",
            NO_SUCH_FILE => "No such file",
            PERMISSION_DENIED => "Bot permission denied",
            MUTED => "Bot was muted",
            MESSAGE_TOO_LONG => "Message is too long",
            BAD_REQUEST => "Bad request",

            _ => "Unknown code"
        };

        f.write_str(&format!("[{}] {}", self.action, msg))
    }
}

impl Error for StatusCodeError {}

const SUCCESS: Code = 0;
const WRONG_AUTH_KEY: Code = 1;
const NO_SUCH_BOT: Code = 2;
const WRONG_SESSION: Code = 3;
const UNAUTHORIZED: Code = 4;
const NO_SUCH_TARGET: Code = 5;
const NO_SUCH_FILE: Code = 6;
const PERMISSION_DENIED: Code = 10;
const MUTED: Code = 20;
const MESSAGE_TOO_LONG: Code = 30;
const BAD_REQUEST: Code = 400;

pub(crate) fn assert(code: Code, action: &str) -> HttpResult<()> {
    Err(StatusCodeError {
        code,
        action: action.to_string(),
    }.into())
}