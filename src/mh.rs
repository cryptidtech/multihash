use crate::{error::Error, Result};
use digest::{Digest, DynDigest};
use multibase::Base;
use multicodec::codec::Codec;
use multiutil::{EncodeInto, TryDecodeFrom};
use std::{
    fmt,
    hash::{Hash, Hasher},
};
use typenum::consts::*;

/// the multicodec sigil for multihash
pub const SIGIL: Codec = Codec::Multihash;

/// The main multihash structure
#[derive(Clone)]
pub struct Multihash {
    /// The hash codec
    codec: Codec,

    /// The multibase encoding
    encoding: Base,

    /// The hash data
    hash: Vec<u8>,
}

impl Multihash {
    /// get the hash codec
    pub fn codec(&self) -> Codec {
        self.codec
    }

    /// get the encoding format
    pub fn encoding(&self) -> Base {
        self.encoding
    }
}

impl PartialEq for Multihash {
    fn eq(&self, other: &Self) -> bool {
        self.codec == other.codec && self.hash == other.hash
    }
}

impl Eq for Multihash {}

impl Hash for Multihash {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.codec.hash(state);
        self.hash.hash(state);
    }
}

impl Default for Multihash {
    fn default() -> Self {
        Multihash {
            codec: Codec::default(),
            encoding: Base::Base16Lower,
            hash: Vec::default(),
        }
    }
}

impl fmt::Debug for Multihash {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Base::*;
        let base = match self.encoding {
            Identity => "Raw Binary",
            Base2 => "Base2",
            Base8 => "Base8",
            Base10 => "Base10",
            Base16Lower => "Base16 Lower",
            Base16Upper => "Base16 Upper",
            Base32Lower => "Base32 Lower",
            Base32Upper => "Base32 Upper",
            Base32PadLower => "Base32 Lower w/Padding",
            Base32PadUpper => "Base32 Upper w/Padding",
            Base32HexLower => "Base32 Hex Lower",
            Base32HexUpper => "Base32 Hex Upper",
            Base32HexPadLower => "Base32 Hex Lower w/Padding",
            Base32HexPadUpper => "Base32 Hex Upper w/Padding",
            Base32Z => "Z-Base32",
            Base36Lower => "Base36 Lower",
            Base36Upper => "Base36 Upper",
            Base58Flickr => "Base58 Flickr",
            Base58Btc => "Base58 Bitcoin",
            Base64 => "Base64",
            Base64Pad => "Base64 w/Padding",
            Base64Url => "Base64 URL Safe",
            Base64UrlPad => "Base64 URL Safe w/Padding",
        };

        writeln!(
            f,
            "Multihash (0x31) - {} - {} ({}) - {}",
            self.codec,
            base,
            self.encoding.code(),
            multibase::encode(self.encoding, &self.hash)
        )
    }
}

impl EncodeInto for Multihash {
    fn encode_into(&self) -> Vec<u8> {
        // start with the sigil
        let mut v = SIGIL.encode_into();

        // add the multihash codec
        v.append(&mut self.codec.encode_into());

        // add the hash length
        v.append(&mut self.hash.len().encode_into());

        // add the hash
        v.append(&mut self.hash.clone());

        v
    }
}

/// Exposes direct access to the hash data
impl AsRef<[u8]> for Multihash {
    fn as_ref(&self) -> &[u8] {
        self.hash.as_ref()
    }
}

/// Convert the multihash to a String using the specified encoding
impl ToString for Multihash {
    fn to_string(&self) -> String {
        let v = self.encode_into();
        multibase::encode(self.encoding, &v)
    }
}

/// Try decoding the multihash from a multibase encoded string
impl TryFrom<String> for Multihash {
    type Error = Error;

    fn try_from(s: String) -> std::result::Result<Self, Self::Error> {
        Self::try_from(s.as_str())
    }
}

/// Try decoding the multihash from a multibase encoded &str
impl TryFrom<&str> for Multihash {
    type Error = Error;

    fn try_from(s: &str) -> std::result::Result<Self, Self::Error> {
        match multibase::decode(s) {
            Ok((base, v)) => {
                let (mut mk, _) = Self::try_decode_from(v.as_slice())?;
                mk.encoding = base;
                Ok(mk)
            }
            Err(e) => Err(Error::Multibase(e)),
        }
    }
}

/// Try decoding the multihash from a multiformat encoded byte array
impl TryFrom<Vec<u8>> for Multihash {
    type Error = Error;

    fn try_from(v: Vec<u8>) -> std::result::Result<Self, Self::Error> {
        let (mk, _) = Self::try_decode_from(v.as_slice())?;
        Ok(mk)
    }
}

impl<'a> TryDecodeFrom<'a> for Multihash {
    type Error = Error;

    fn try_decode_from(bytes: &'a [u8]) -> std::result::Result<(Self, &'a [u8]), Self::Error> {
        // decode the sigil first
        let (sigil, ptr) = Codec::try_decode_from(bytes)?;
        if sigil != SIGIL {
            return Err(Error::MissingSigil);
        }

        // decode the hash codec
        let (codec, ptr) = Codec::try_decode_from(ptr)?;

        // decode the hash size
        let (size, ptr) = usize::try_decode_from(ptr)?;

        // decode the hash bytes
        let mut hash = Vec::with_capacity(size);
        hash.extend_from_slice(&ptr[..size]);

        Ok((
            Self {
                codec,
                encoding: Base::Base16Lower,
                hash,
            },
            ptr,
        ))
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
    pub fn try_build(self, data: impl AsRef<[u8]>) -> Result<Multihash> {
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
            _ => anyhow::bail!(Error::UnsupportedHash(self.codec)),
        };

        // hash the data
        hasher.update(data.as_ref());

        Ok(Multihash {
            codec: self.codec,
            encoding: self.encoding.unwrap_or(Base::Base16Lower),
            hash: hasher.finalize().to_vec(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        let mh = Builder::new(Codec::Sha2256)
            .with_encoding(Base::Base58Btc)
            .try_build(b"for great justice, move every zig!")
            .unwrap();

        assert_eq!(
            mh.hash,
            hex::decode("e28c7aeb3a876b25ed822472e47a696fe25214c1672f0972195f9b64eea41e7e")
                .unwrap()
        );

        println!("{:?}", mh);
        println!("{}", mh.to_string());

        let v = mh.encode_into();
        assert_eq!(35, v.len());
    }

    #[test]
    fn test_roundtrip() {
        let mh1 = Builder::new(Codec::Sha3384)
            .try_build(b"for great justice, move every zig!")
            .unwrap();

        let v = mh1.encode_into();

        let (mh2, _) = Multihash::try_decode_from(v.as_ref()).unwrap();

        assert_eq!(mh1, mh2);
    }
}
