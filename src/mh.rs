use crate::Error;
use digest::{Digest, DynDigest};
use multibase::Base;
use multicodec::prelude::Codec;
use multitrait::prelude::{EncodeInto, TryDecodeFrom};
use multiutil::prelude::{BaseEncoded, CodecInfo, EncodingInfo, Tagged};
use std::{
    fmt,
    hash::{Hash, Hasher},
};
use typenum::consts::*;

/// the multicodec sigil for multihash
pub const SIGIL: Codec = Codec::Multihash;

/// the multihash structure
pub type Multihash = BaseEncoded<Tagged<MultihashImpl>>;

/// inner implementation of the multihash
#[derive(Clone, Default, PartialEq)]
pub struct MultihashImpl {
    /// hash codec
    pub(crate) codec: Codec,

    /// hash value
    pub(crate) hash: Vec<u8>,
}

impl CodecInfo for MultihashImpl {
    /// Return that we are a Multihash object
    fn preferred_codec() -> Codec {
        SIGIL
    }

    /// Return the hashing codec for the multihash
    fn codec(&self) -> Codec {
        self.codec
    }
}

impl EncodingInfo for MultihashImpl {
    fn preferred_encoding() -> Base {
        Base::Base16Lower
    }

    fn encoding(&self) -> Base {
        Self::preferred_encoding()
    }
}

impl Hash for MultihashImpl {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.codec.hash(state);
        self.hash.hash(state);
    }
}

impl EncodeInto for MultihashImpl {
    fn encode_into(&self) -> Vec<u8> {
        // start with the hashing codec
        let mut v = self.codec.encode_into();

        // add the hash length
        v.append(&mut self.hash.len().encode_into());

        // add the hash
        v.append(&mut self.hash.clone());

        v
    }
}

/// Exposes direct access to the hash data
impl AsRef<[u8]> for MultihashImpl {
    fn as_ref(&self) -> &[u8] {
        self.hash.as_ref()
    }
}

impl fmt::Debug for MultihashImpl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} - ('{}')", self.codec().as_str(), self.codec().code())
    }
}

impl<'a> TryDecodeFrom<'a> for MultihashImpl {
    type Error = Error;

    fn try_decode_from(bytes: &'a [u8]) -> Result<(Self, &'a [u8]), Self::Error> {
        // decode the hashing codec
        let (codec, ptr) = Codec::try_decode_from(bytes)?;

        // decode the hash size
        let (size, ptr) = usize::try_decode_from(ptr)?;

        // decode the hash bytes
        let mut hash = Vec::with_capacity(size);
        hash.extend_from_slice(&ptr[..size]);
        let ptr = &ptr[size..];

        Ok((Self { codec, hash }, ptr))
    }
}

/// Hash builder that takes the codec and the data and produces a Multihash
#[derive(Clone, Debug, Default)]
pub struct Builder {
    codec: Codec,
    encoding: Option<Base>,
}

impl Builder {
    /// create a hash with the given codec
    pub fn new(codec: Codec) -> Self {
        Builder {
            codec,
            encoding: None,
        }
    }

    /// add an encoding
    pub fn with_encoding(mut self, base: Base) -> Self {
        self.encoding = Some(base);
        self
    }

    /// build the multihash by hashing the provided data
    pub fn try_build(self, data: impl AsRef<[u8]>) -> Result<Multihash, Error> {
        let mut hasher: Box<dyn DynDigest> = match self.codec {
            Codec::Blake2B224 => Box::new(blake2::Blake2b::<U28>::new()),
            Codec::Blake2B256 => Box::new(blake2::Blake2b::<U32>::new()),
            Codec::Blake2B384 => Box::new(blake2::Blake2b::<U48>::new()),
            Codec::Blake2B512 => Box::new(blake2::Blake2b::<U64>::new()),
            Codec::Blake2S224 => Box::new(blake2::Blake2s::<U28>::new()),
            Codec::Blake2S256 => Box::new(blake2::Blake2s::<U32>::new()),
            Codec::Md5 => Box::new(md5::Md5::new()),
            Codec::Ripemd128 => Box::new(ripemd::Ripemd128::new()),
            Codec::Ripemd160 => Box::new(ripemd::Ripemd160::new()),
            Codec::Ripemd256 => Box::new(ripemd::Ripemd256::new()),
            Codec::Ripemd320 => Box::new(ripemd::Ripemd320::new()),
            Codec::Sha1 => Box::new(sha1::Sha1::new()),
            Codec::Sha2224 => Box::new(sha2::Sha224::new()),
            Codec::Sha2256 => Box::new(sha2::Sha256::new()),
            Codec::Sha2384 => Box::new(sha2::Sha384::new()),
            Codec::Sha2512 => Box::new(sha2::Sha512::new()),
            Codec::Sha2512224 => Box::new(sha2::Sha512_224::new()),
            Codec::Sha2512256 => Box::new(sha2::Sha512_256::new()),
            Codec::Sha3224 => Box::new(sha3::Sha3_224::new()),
            Codec::Sha3256 => Box::new(sha3::Sha3_256::new()),
            Codec::Sha3384 => Box::new(sha3::Sha3_384::new()),
            Codec::Sha3512 => Box::new(sha3::Sha3_512::new()),
            _ => return Err(Error::UnsupportedHash(self.codec.to_string())),
        };

        // hash the data
        hasher.update(data.as_ref());

        if let Some(encoding) = self.encoding {
            Ok(BaseEncoded::new_base(
                encoding,
                Tagged::new(MultihashImpl {
                    codec: self.codec,
                    hash: hasher.finalize().to_vec(),
                }),
            ))
        } else {
            Ok(BaseEncoded::new(Tagged::new(MultihashImpl {
                codec: self.codec,
                hash: hasher.finalize().to_vec(),
            })))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix() {
        let hashers = vec![
            Codec::Blake2B224,
            Codec::Blake2B256,
            Codec::Blake2B384,
            Codec::Blake2B512,
            Codec::Blake2S224,
            Codec::Blake2S256,
            Codec::Md5,
            Codec::Ripemd128,
            Codec::Ripemd160,
            Codec::Ripemd256,
            Codec::Ripemd320,
            Codec::Sha1,
            Codec::Sha2224,
            Codec::Sha2256,
            Codec::Sha2384,
            Codec::Sha2512,
            Codec::Sha2512224,
            Codec::Sha2512256,
            Codec::Sha3224,
            Codec::Sha3256,
            Codec::Sha3384,
            Codec::Sha3512,
        ];

        let bases = vec![
            Base::Base2,
            Base::Base8,
            Base::Base10,
            Base::Base16Lower,
            Base::Base16Upper,
            Base::Base32Lower,
            Base::Base32Upper,
            Base::Base32PadLower,
            Base::Base32PadUpper,
            Base::Base32HexLower,
            Base::Base32HexUpper,
            Base::Base32HexPadLower,
            Base::Base32HexPadUpper,
            Base::Base32Z,
            Base::Base36Lower,
            Base::Base36Upper,
            Base::Base58Flickr,
            Base::Base58Btc,
            Base::Base64,
            Base::Base64Pad,
            Base::Base64Url,
            Base::Base64UrlPad,
        ];

        for h in &hashers {
            for b in &bases {
                let mh1 = Builder::new(*h)
                    .with_encoding(*b)
                    .try_build(b"for great justice, move every zig!")
                    .unwrap();

                println!("{:?}", mh1);

                let s = mh1.to_string();

                assert_eq!(mh1, Multihash::try_from(s.as_str()).unwrap());
            }
        }
    }

    #[test]
    fn test_binary_roundtrip() {
        let mh1 = Builder::new(Codec::Sha3384)
            .try_build(b"for great justice, move every zig!")
            .unwrap();

        let v = mh1.encode_into();

        let (mh2, _) = Multihash::try_decode_from(v.as_ref()).unwrap();

        assert_eq!(mh1, mh2);
    }
}
