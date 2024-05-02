use std::fmt;

#[derive(Debug)]
#[allow(dead_code)]
pub enum ErrorKind {
    BadRequest,
    Unauthorized,
    PaymentRequired,
    Forbidden,
    NotFound,
    MethodNotAllowed,
    NotAcceptable,
    RequestTimeout,
    Conflict,
    Gone,
    LengthRequired,
    PreconditionFailed,
    RequestEntityTooLarge,
    RequestUriTooLong,
    UnsupportedMediaType,
    RequestedRangeUnsatisfiable,
    ExpectationFailed,
    Expired,
    BadMapping,
    Locked,
    NoResponse,
    Canceled,
    InternalServerError,
    NotImplemented,
    ServiceUnavailable,
    SizeConstraintViolation,
    ProtocolError,
    UnknownVersion,
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
            ErrorKind::NotAcceptable => write!(f, "NotAcceptable"),
            ErrorKind::RequestTimeout => write!(f, "RequestTimeout"),
            ErrorKind::Conflict => write!(f, "Conflict"),
            ErrorKind::Gone => write!(f, "Gone"),
            ErrorKind::LengthRequired => write!(f, "LengthRequired"),
            ErrorKind::PreconditionFailed => write!(f, "PreconditionFailed"),
            ErrorKind::RequestEntityTooLarge => write!(f, "RequestEntityTooLarge"),
            ErrorKind::RequestUriTooLong => write!(f, "RequestUriTooLong"),
            ErrorKind::UnsupportedMediaType => write!(f, "UnsupportedMediaType"),
            ErrorKind::RequestedRangeUnsatisfiable => write!(f, "RequestedRangeUnsatisfiable"),
            ErrorKind::ExpectationFailed => write!(f, "ExpectationFailed"),
            ErrorKind::Expired => write!(f, "Expired"),
            ErrorKind::BadMapping => write!(f, "BadMapping"),
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

#[derive(Debug)]
pub struct Error {
    pub code: u32,
    pub message: String,
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
