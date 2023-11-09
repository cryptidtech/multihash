use thiserror::Error;

/// Errors created by this library
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    /// A multibase conversion error
    #[error(transparent)]
    Multibase(#[from] multibase::Error),

    /// A multicodec decoding error
    #[error(transparent)]
    Multicodec(#[from] multicodec::error::Error),

    /// Multiutil error
    #[error(transparent)]
    Multiutil(#[from] multiutil::Error),

    /// Missing sigil 0x31
    #[error("Missing Multihash sigil")]
    MissingSigil,

    /// Error with the hash scheme
    #[error("Unsupported hash algorithm: {0}")]
    UnsupportedHash(multicodec::codec::Codec),
}
