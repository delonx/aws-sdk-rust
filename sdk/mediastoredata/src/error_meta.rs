// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    ContainerNotFoundException(crate::error::ContainerNotFoundException),
    InternalServerError(crate::error::InternalServerError),
    ObjectNotFoundException(crate::error::ObjectNotFoundException),
    RequestedRangeNotSatisfiableException(crate::error::RequestedRangeNotSatisfiableException),
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ContainerNotFoundException(inner) => inner.fmt(f),
            Error::InternalServerError(inner) => inner.fmt(f),
            Error::ObjectNotFoundException(inner) => inner.fmt(f),
            Error::RequestedRangeNotSatisfiableException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::DeleteObjectError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::DeleteObjectError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DeleteObjectErrorKind::ContainerNotFoundException(inner) => {
                    Error::ContainerNotFoundException(inner)
                }
                crate::error::DeleteObjectErrorKind::InternalServerError(inner) => {
                    Error::InternalServerError(inner)
                }
                crate::error::DeleteObjectErrorKind::ObjectNotFoundException(inner) => {
                    Error::ObjectNotFoundException(inner)
                }
                crate::error::DeleteObjectErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::DescribeObjectError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::DescribeObjectError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeObjectErrorKind::ContainerNotFoundException(inner) => {
                    Error::ContainerNotFoundException(inner)
                }
                crate::error::DescribeObjectErrorKind::InternalServerError(inner) => {
                    Error::InternalServerError(inner)
                }
                crate::error::DescribeObjectErrorKind::ObjectNotFoundException(inner) => {
                    Error::ObjectNotFoundException(inner)
                }
                crate::error::DescribeObjectErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::GetObjectError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::GetObjectError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::GetObjectErrorKind::ContainerNotFoundException(inner) => {
                    Error::ContainerNotFoundException(inner)
                }
                crate::error::GetObjectErrorKind::InternalServerError(inner) => {
                    Error::InternalServerError(inner)
                }
                crate::error::GetObjectErrorKind::ObjectNotFoundException(inner) => {
                    Error::ObjectNotFoundException(inner)
                }
                crate::error::GetObjectErrorKind::RequestedRangeNotSatisfiableException(inner) => {
                    Error::RequestedRangeNotSatisfiableException(inner)
                }
                crate::error::GetObjectErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::ListItemsError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::ListItemsError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListItemsErrorKind::ContainerNotFoundException(inner) => {
                    Error::ContainerNotFoundException(inner)
                }
                crate::error::ListItemsErrorKind::InternalServerError(inner) => {
                    Error::InternalServerError(inner)
                }
                crate::error::ListItemsErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::PutObjectError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::PutObjectError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::PutObjectErrorKind::ContainerNotFoundException(inner) => {
                    Error::ContainerNotFoundException(inner)
                }
                crate::error::PutObjectErrorKind::InternalServerError(inner) => {
                    Error::InternalServerError(inner)
                }
                crate::error::PutObjectErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl std::error::Error for Error {}