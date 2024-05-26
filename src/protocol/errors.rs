use std::fmt;

///
/// All the error types that can be returned.
///
/// # Example
/// ```rust
/// use shdp::prelude::common::error::ErrorKind;
///
/// fn main() {
///     let error = ErrorKind::NotFound;
///     println!("{}", error);
/// }
/// ```
#[derive(Debug)]
#[allow(dead_code)]
pub enum ErrorKind {
    /// The request could not be understood by the server due to malformed syntax.
    BadRequest,
    /// The request requires user authentication.
    Unauthorized,
    /// The payment is required.
    PaymentRequired,
    /// The server understood the request, but it refuses to authorize it.
    Forbidden,
    /// The server has not found anything matching the request.
    NotFound,
    /// The method specified in the request is not allowed for the resource identified by the request.
    MethodNotAllowed,
    /// The server cannot generate a response that the client will accept.
    RequestTimeout,
    /// The request could not be completed due to a conflict with the current state of the resource.
    Conflict,
    /// The requested resource is no longer available at the server and no forwarding address is known.
    Gone,
    /// The request entity is larger than the server is willing or able to process.
    RequestEntityTooLarge,
    /// The request range is not satisfiable.
    RequestedRangeUnsatisfiable,
    /// The server cannot or will not process the request due to an apparent client error.
    ExpectationFailed,
    /// The request has expired.
    Expired,
    /// The resource is locked.
    Locked,
    /// The server has not found anything matching the request.
    NoResponse,
    /// The request has been canceled.
    Canceled,
    /// The server encountered an unexpected condition that prevented it from fulfilling the request.
    InternalServerError,
    /// The server does not support the functionality required to fulfill the request.
    NotImplemented,
    /// The server is currently unable to handle the request due to a temporary overloading or maintenance of the server.
    ServiceUnavailable,
    /// The request is larger than the server is willing or able to process.
    SizeConstraintViolation,
    /// The server encountered an unexpected condition that prevented it from fulfilling the request.
    ProtocolError,
    /// The server does not support the HTTP protocol version used in the request.
    UnknownVersion,
    ///
    /// User defined error. It can be used to wrap any error type.
    ///
    /// # Example
    /// ```rust
    /// use shdp::prelude::common::error::ErrorKind;
    ///
    /// fn main() {
    ///     let error = ErrorKind::UserDefined(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "User defined error")));
    ///     println!("{}", error);
    /// }
    /// ```
    UserDefined(Box<dyn std::error::Error>),
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorKind::BadRequest => write!(f, "BadRequest"),
            ErrorKind::Unauthorized => write!(f, "Unauthorized"),
            ErrorKind::PaymentRequired => write!(f, "PaymentRequired"),
            ErrorKind::Forbidden => write!(f, "Forbidden"),
            ErrorKind::NotFound => write!(f, "NotFound"),
            ErrorKind::MethodNotAllowed => write!(f, "MethodNotAllowed"),
            ErrorKind::RequestTimeout => write!(f, "RequestTimeout"),
            ErrorKind::Conflict => write!(f, "Conflict"),
            ErrorKind::Gone => write!(f, "Gone"),
            ErrorKind::RequestEntityTooLarge => write!(f, "RequestEntityTooLarge"),
            ErrorKind::RequestedRangeUnsatisfiable => write!(f, "RequestedRangeUnsatisfiable"),
            ErrorKind::ExpectationFailed => write!(f, "ExpectationFailed"),
            ErrorKind::Expired => write!(f, "Expired"),
            ErrorKind::Locked => write!(f, "Locked"),
            ErrorKind::NoResponse => write!(f, "NoResponse"),
            ErrorKind::Canceled => write!(f, "Canceled"),
            ErrorKind::InternalServerError => write!(f, "InternalServerError"),
            ErrorKind::NotImplemented => write!(f, "NotImplemented"),
            ErrorKind::ServiceUnavailable => write!(f, "ServiceUnavailable"),
            ErrorKind::SizeConstraintViolation => write!(f, "SizeConstraintViolation"),
            ErrorKind::ProtocolError => write!(f, "ProtocolError"),
            ErrorKind::UnknownVersion => write!(f, "UnknownVersion"),
            ErrorKind::UserDefined(e) => write!(f, "{}", e),
        }
    }
}

///
/// A basic error structure.
/// # Example
/// ```rust
/// use shdp::prelude::common::error::{Error, ErrorKind};
///
/// fn main() {
///     let error = Error {
///         code: 404,
///         message: "Not Found".to_string(),
///         kind: ErrorKind::NotFound,
///     };
///
///     println!("{}", error);
/// }
/// ```
#[derive(Debug)]
pub struct Error {
    /// The error code.
    pub code: u32,
    /// The error message.
    pub message: String,
    /// The error kind.
    pub kind: ErrorKind,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Error: [{}]:{} -> {}",
            self.kind, self.code, self.message
        )
    }
}
