use crate::mh::{Multihash, SIGIL};
use core::fmt;
use multicodec::Codec;
use multiutil::{EncodedVarbytes, Varbytes};
use serde::{
    de::{Error, MapAccess, Visitor},
    Deserialize, Deserializer,
};

/// Deserialize instance of [`crate::Multihash`]
impl<'de> Deserialize<'de> for Multihash {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        const FIELDS: &'static [&'static str] = &["codec", "hash"];

        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            Codec,
            Hash,
        }

        struct MultihashVisitor;

        impl<'de> Visitor<'de> for MultihashVisitor {
            type Value = Multihash;

            fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
                write!(fmt, "struct Multihash")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Multihash, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut codec = None;
                let mut hash = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Codec => {
                            if codec.is_some() {
                                return Err(Error::duplicate_field("codec"));
                            }
                            let s: &str = map.next_value()?;
                            codec = Some(
                                Codec::try_from(s)
                                    .map_err(|_| Error::custom("invalid multihash codec"))?,
                            );
                        }
                        Field::Hash => {
                            if hash.is_some() {
                                return Err(Error::duplicate_field("hash"));
                            }
                            let vb: EncodedVarbytes = map.next_value()?;
                            hash = Some(vb.to_inner().to_inner());
                        }
                    }
                }
                let codec = codec.ok_or_else(|| Error::missing_field("codec"))?;
                let hash = hash.ok_or_else(|| Error::missing_field("hash"))?;
                Ok(Multihash { codec, hash })
            }
        }

        if deserializer.is_human_readable() {
            deserializer.deserialize_struct(SIGIL.as_str(), FIELDS, MultihashVisitor)
        } else {
            let (sigil, codec, hash): (Codec, Codec, Varbytes) =
                Deserialize::deserialize(deserializer)?;

            if sigil != SIGIL {
                return Err(Error::custom("deserialized sigil is not a Multihash sigil"));
            }
            Ok(Self {
                codec,
                hash: hash.to_inner(),
            })
        }
    }
}
