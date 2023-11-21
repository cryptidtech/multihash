use crate::mh::MultihashImpl;
use serde::de;

/// Deserialize instance of [`crate::mh::MultihashImpl`]
impl<'de> de::Deserialize<'de> for MultihashImpl {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let (codec, hash) = de::Deserialize::deserialize(deserializer)?;
        Ok(Self { codec, hash })
    }
}
