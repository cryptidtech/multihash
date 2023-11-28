use crate::mh::{Multihash, SIGIL};
use multiutil::Varbytes;
use serde::ser::{self, SerializeStruct};

/// Serialize instance of [`crate::Multihash`]
impl ser::Serialize for Multihash {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        if serializer.is_human_readable() {
            let mut ss = serializer.serialize_struct(SIGIL.as_str(), 2)?;
            let _ = ss.serialize_field("codec", &self.codec.code());
            let _ = ss.serialize_field("hash", &Varbytes::encoded_new(self.hash.clone()))?;
            ss.end()
        } else {
            (SIGIL, self.codec, Varbytes(self.hash.clone())).serialize(serializer)
        }
    }
}
