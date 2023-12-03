use thiserror::Error;

/// Errors created by this library
#[derive(Clone, Debug, Eq, Error, PartialEq)]
#[non_exhaustive]
pub enum Error {
    /// A multibase conversion error
    #[error(transparent)]
    Multibase(#[from] multibase::Error),

    /// A multicodec decoding error
    #[error(transparent)]
    Multicodec(#[from] multicodec::Error),

    /// Multiutil error
    #[error(transparent)]
    Multiutil(#[from] multiutil::Error),

    /// Multitrait error
    #[error(transparent)]
    Multitrait(#[from] multitrait::Error),

    /// Missing sigil 0x31
    #[error("Missing Multihash sigil")]
    MissingSigil,

    /// Missing hash data
    #[error("Missing hash data")]
    MissingHash,

    /// Error with the hash scheme
    #[error("Unsupported hash algorithm: {0}")]
    UnsupportedHash(multicodec::Codec),
}
