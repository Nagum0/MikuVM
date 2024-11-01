use thiserror::Error;

use crate::types::MikuType;

#[derive(Debug, Error)]
pub enum MikuError {
    /// Type errors
    #[error("UNKNOWN TYPE ERROR: {}", _0)]
    UnknownTypeError(u8),
    #[error("BYTE CONVERSION ERROR")]
    BytesConversionError,

    /// Operation errors
    #[error("UNDEFINED OPERATION BETWEEN TYPES: {}", ._0)]
    UndefinedOperationBetweenTypesError(String),
    #[error("DIVISION BY ZERO")]
    DivisionByZeroError,

    /// Stack errors
    #[error("STACK OVERFLOW")]
    StackOverflow,
    #[error("STACK UNDERFLOW")]
    StackUnderflow,

    /// RAM errors
    #[error("USED DATA SPACE")]
    UsedDataSpace,
    #[error("SEGMENTATION FAULT")]
    SegmentationFault,
    #[error("INVALID POINTER TYPE: {:?}", ._0)]
    InvalidPointerType(MikuType),
}
