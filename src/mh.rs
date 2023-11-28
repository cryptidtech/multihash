use crate::Error;
use digest::{Digest, DynDigest};
use multibase::Base;
use multicodec::Codec;
use multitrait::TryDecodeFrom;
use multiutil::{BaseEncoded, CodecInfo, EncodingInfo, Varbytes};
use std::{
    fmt,
    hash::{Hash, Hasher},
};
use typenum::consts::*;

/// the multicodec sigil for multihash
pub const SIGIL: Codec = Codec::Multihash;

/// a base encoded multihash
pub type EncodedMultihash = BaseEncoded<Multihash>;

/// inner implementation of the multihash
#[derive(Clone, Default, PartialEq)]
pub struct Multihash {
    /// hash codec
    pub(crate) codec: Codec,

    /// hash value
    pub(crate) hash: Vec<u8>,
}

impl CodecInfo for Multihash {
    /// Return that we are a Multihash object
    fn preferred_codec() -> Codec {
        SIGIL
    }

    /// Return the hashing codec for the multihash
    fn codec(&self) -> Codec {
        self.codec
    }
}

impl EncodingInfo for Multihash {
    fn preferred_encoding() -> Base {
        Base::Base16Lower
    }

    fn encoding(&self) -> Base {
        Self::preferred_encoding()
    }
}

impl Hash for Multihash {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // hash in the multihash sigil
        SIGIL.hash(state);
        // hash in the hash codec
        self.codec.hash(state);
        // hash in the digest bytes
        self.hash.hash(state);
    }
}

impl Into<Vec<u8>> for Multihash {
    fn into(self) -> Vec<u8> {
        let mut v = Vec::default();
        // add in the multihash sigil
        v.append(&mut SIGIL.into());
        // add in the hash codec
        v.append(&mut self.codec.clone().into());
        // add in the hash data
        v.append(&mut Varbytes(self.hash.clone()).into());
        v
    }
}

impl<'a> TryFrom<&'a [u8]> for Multihash {
    type Error = Error;

    fn try_from(s: &'a [u8]) -> Result<Self, Self::Error> {
        let (mh, _) = Self::try_decode_from(s)?;
        Ok(mh)
    }
}

impl<'a> TryDecodeFrom<'a> for Multihash {
    type Error = Error;

    fn try_decode_from(bytes: &'a [u8]) -> Result<(Self, &'a [u8]), Self::Error> {
        // decode the sigil
        let (sigil, ptr) = Codec::try_decode_from(bytes)?;
        if sigil != SIGIL {
            return Err(Error::MissingSigil);
        }

        // decode the hashing codec
        let (codec, ptr) = Codec::try_decode_from(ptr)?;

        // decode the hash bytes
        let (hash, ptr) = Varbytes::try_decode_from(ptr)?;

        // pull the inner Vec<u8> out of Varbytes
        let hash = hash.to_inner();

        Ok((Self { codec, hash }, ptr))
    }
}

/// Exposes direct access to the hash data
impl AsRef<[u8]> for Multihash {
    fn as_ref(&self) -> &[u8] {
        self.hash.as_ref()
    }
}

impl fmt::Debug for Multihash {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} - (0x{:x}) - {} - (0x{:x})",
            SIGIL.as_str(),
            SIGIL.code(),
            self.codec().as_str(),
            self.codec().code()
        )
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

    /// build a base encoded multihash
    pub fn try_build_encoded(self, data: impl AsRef<[u8]>) -> Result<EncodedMultihash, Error> {
        let mh = self.clone().try_build(data)?;
        if let Some(encoding) = self.encoding {
            Ok(BaseEncoded::new_base(encoding, mh))
        } else {
            Ok(mh.into())
        }
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
        Ok(Multihash {
            codec: self.codec,
            hash: hasher.finalize().to_vec(),
        })
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
                    .try_build_encoded(b"for great justice, move every zig!")
                    .unwrap();
                //println!("{:?}", mh1);
                let s = mh1.to_string();
                assert_eq!(mh1, EncodedMultihash::try_from(s.as_str()).unwrap());
            }
        }
    }

    #[test]
    fn test_binary_roundtrip() {
        let mh1 = Builder::new(Codec::Sha3384)
            .try_build(b"for great justice, move every zig!")
            .unwrap();

        let v: Vec<u8> = mh1.clone().into();

        let mh2 = Multihash::try_from(v.as_ref()).unwrap();

        assert_eq!(mh1, mh2);
    }

    #[test]
    fn test_encoded() {
        let mh = Builder::new(Codec::Sha3256)
            .try_build_encoded(b"for great justice, move every zig!")
            .unwrap();
        println!("{:?}", mh);
    }
}
