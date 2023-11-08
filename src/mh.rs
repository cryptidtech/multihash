use crate::{error::Error, Result};
use digest::{Digest, DynDigest};
use multicodec::codec::Codec;
use multiutil::{EncodeInto, TryDecodeFrom};
use std::fmt;
use typenum::consts::*;

/// The main multihash structure
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Multihash {
    /// The hash codec
    pub codec: Codec,

    /// The hash data
    pub hash: Vec<u8>,
}

impl fmt::Display for Multihash {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "{} - 0x{:x} - {}",
            self.codec,
            self.hash.len(),
            hex::encode(&self.hash)
        )
    }
}

impl EncodeInto for Multihash {
    fn encode_into(&self) -> Vec<u8> {
        // start with the sigil
        let mut v = self.codec.encode_into();

        // add the key codec
        v.append(&mut self.hash.len().encode_into());

        // add the hash
        v.append(&mut self.hash.clone());

        v
    }
}

impl TryFrom<String> for Multihash {
    type Error = Error;

    fn try_from(s: String) -> std::result::Result<Self, Self::Error> {
        Self::try_from(s.as_str())
    }
}

impl TryFrom<&str> for Multihash {
    type Error = Error;

    fn try_from(s: &str) -> std::result::Result<Self, Self::Error> {
        match multibase::decode(s) {
            Ok((_, v)) => {
                let (mk, _) = Self::try_decode_from(v.as_slice())?;
                Ok(mk)
            }
            Err(e) => Err(Error::Multibase(e)),
        }
    }
}

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
        // decode the hash codec
        let (codec, ptr) = Codec::try_decode_from(bytes)?;

        // decode the hash size
        let (size, ptr) = usize::try_decode_from(ptr)?;

        // decode the hash bytes
        let mut hash = Vec::with_capacity(size);
        hash.extend_from_slice(&ptr[..size]);

        Ok((Self { codec, hash }, ptr))
    }
}

/// Hash builder that takes the codec and the data and produces a Multihash
#[derive(Clone, Debug, Default)]
pub struct Builder {
    codec: Codec,
}

impl Builder {
    /// create a hash with the given codec
    pub fn new(codec: Codec) -> Self {
        Builder { codec }
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
            .try_build(b"for great justice, move every zig!")
            .unwrap();

        assert_eq!(
            mh.hash,
            hex::decode("e28c7aeb3a876b25ed822472e47a696fe25214c1672f0972195f9b64eea41e7e")
                .unwrap()
        );

        let v = mh.encode_into();
        assert_eq!(34, v.len());
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
