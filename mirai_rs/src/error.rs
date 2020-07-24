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

/// Server Error is an error from server, such as login failed or parsing server response failed.
#[derive(Debug)]
pub struct ServerError {
    code: Code,
    msg: String,
}

/// Client Error is an error from client, will be threw when unwrapping a friend channel as a group channel.
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

    pub fn is_server(&self) -> bool {
        match self.inner.kind {
            ErrorKind::Server => true,
            _ => false
        }
    }

    pub fn is_client(&self) -> bool {
        match self.inner.kind {
            ErrorKind::Client => true,
            _ => false
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

impl std::fmt::Display for ClientError<'_> {
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
                "Error occurred from server side:"
            },
            ErrorKind::Client => {
                "Error occurred from client side:"
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

pub(crate) fn assert(code: Code, action: &str) -> Result<()> {
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

    Err(Error::new(ErrorKind::Server, ServerError::new(code, format!("[{}] {}", action, msg))))
}

pub(crate) fn client_error(msg: &'static str) -> Error {
    Error::new(ErrorKind::Client, ClientError::new(msg))
}

impl From<ReqError> for Error {
    fn from(e: ReqError) -> Self {
        Error::new(ErrorKind::Server, e)
    }
}