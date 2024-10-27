use crate::error::MikuError;

/// Convert a T value to U if possible. If not return MikuError.
/// Currently only used for converting byte slices into byte arrays conveniently.
pub fn convert_bytes<T, U>(bytes: T) -> Result<U, MikuError> 
where 
    T: TryInto<U>
{
    bytes.try_into().map_err(|_| MikuError::BytesConversionError)
}
