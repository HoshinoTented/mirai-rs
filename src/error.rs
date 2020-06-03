use reqwest::Error as ReqError;

use std::error::Error as StdError;

use std::fmt::Formatter;

use crate::Code;

pub type Result<T> = std::result::Result<T, Error>;

pub(crate) type BoxError = Box<dyn StdError + Send + Sync>;

#[derive(Debug)]
pub struct Error {
    inner: Box<Inner>
}

#[derive(Debug)]
struct Inner {
    kind: ErrorKind,
    source: BoxError,
}

#[derive(Debug)]
pub struct ServerError {
    code: Code,
    msg: String,
}

#[derive(Debug)]
pub struct ClientError<'m> {
    msg: &'m str
}

#[derive(Debug)]
pub enum ErrorKind {
    Server,
    Client,
}

impl Error {
    pub fn new<E>(kind: ErrorKind, source: E) -> Error
        where E: Into<BoxError> {
        Error {
            inner: Box::new(Inner {
                kind,
                source: source.into(),
            })
        }
    }
}

impl ServerError {
    pub fn new(code: Code, msg: String) -> Self {
        ServerError {
            code,
            msg,
        }
    }
}

impl<'m> ClientError<'m> {
    pub fn new(msg: &'m str) -> Self {
        ClientError {
            msg
        }
    }
}

impl<'m> std::fmt::Display for ClientError<'m> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.msg)
    }
}

impl std::fmt::Display for ServerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("[{}] {}", self.code, self.msg))
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        let msg = match self.inner.kind {
            ErrorKind::Server => {
                "Error occurs from server side:"
            },
            ErrorKind::Client => {
                "Error occurs from client side:"
            },
        };

        f.write_str(&format!("{} {:?}", msg, self.inner.source))
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        Some(self.inner.source.as_ref())
    }
}

impl StdError for ServerError {}

impl StdError for ClientError<'_> {}

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

pub(crate) fn assert(code: Code, _action: &str) -> Result<()> {
    let msg = match code {
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

    Err(Error::new(ErrorKind::Server, ServerError::new(code, msg.to_string())))
}

pub(crate) fn client_error(msg: &'static str) -> Error {
    Error::new(ErrorKind::Client, ClientError::new(msg))
}

impl From<ReqError> for Error {
    fn from(e: ReqError) -> Self {
        Error::new(ErrorKind::Client, e)
    }
}