use crate::mh::{Multihash, SIGIL};
use core::fmt;
use multicodec::Codec;
use multiutil::{prelude::EncodedVarbytes, Varbytes};
use serde::de;

/// Deserialize instance of [`crate::mh::MultihashImpl`]
impl<'de> de::Deserialize<'de> for Multihash {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        const FIELDS: &'static [&'static str] = &["codec", "hash"];

        enum Field {
            Codec,
            Hash,
        }

        impl<'de> de::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: de::Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> de::Visitor<'de> for FieldVisitor {
                    type Value = Field;
                    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
                        write!(fmt, "`codec` or `hash`")
                    }
                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "codec" => Ok(Field::Codec),
                            "hash" => Ok(Field::Hash),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct MultihashVisitor;

        impl<'de> de::Visitor<'de> for MultihashVisitor {
            type Value = Multihash;

            fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
                write!(fmt, "struct Multihash")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Multihash, V::Error>
            where
                V: de::MapAccess<'de>,
            {
                let mut codec = None;
                let mut hash = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Codec => {
                            if codec.is_some() {
                                return Err(de::Error::duplicate_field("codec"));
                            }
                            let c: u64 = map.next_value()?;
                            codec = Some(
                                Codec::try_from(c)
                                    .map_err(|_| de::Error::custom("invalid multihash codec"))?,
                            );
                        }
                        Field::Hash => {
                            if hash.is_some() {
                                return Err(de::Error::duplicate_field("hash"));
                            }
                            let vb: EncodedVarbytes = map.next_value()?;
                            hash = Some(vb);
                        }
                    }
                }
                let codec = codec.ok_or_else(|| de::Error::missing_field("codec"))?;
                let hash = hash
                    .ok_or_else(|| de::Error::missing_field("hash"))?
                    .to_inner()
                    .to_inner();
                Ok(Multihash { codec, hash })
            }
        }

        if deserializer.is_human_readable() {
            deserializer.deserialize_struct(SIGIL.as_str(), FIELDS, MultihashVisitor)
        } else {
            let (sigil, codec, hash): (Codec, Codec, Varbytes) =
                de::Deserialize::deserialize(deserializer)?;

            if sigil != SIGIL {
                return Err(de::Error::custom(
                    "deserialized sigil is not a Multihash sigil",
                ));
            }
            Ok(Self {
                codec,
                hash: hash.to_inner(),
            })
        }
    }
}
