//!
#![warn(missing_docs)]
#![deny(
    trivial_casts,
    trivial_numeric_casts,
    unused_import_braces,
    unused_qualifications
)]

/// Errors produced by this library
pub mod error;
pub use error::Error;

/// Multihash type and functions
pub mod mh;
pub use mh::{Builder, EncodedMultihash, Multihash};

/// Serde serialization for Multihash
#[cfg(feature = "serde")]
pub mod serde;

/// ...and in the darkness bind them
pub mod prelude {
    pub use super::mh::{Builder, Multihash};
    /// re-exports
    pub use multibase::Base;
    pub use multicodec::prelude::Codec;
    pub use multiutil::prelude::BaseEncoded;
}
