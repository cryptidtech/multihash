use crate::mh::MultihashImpl;
use serde::ser;

/// Serialize instance of [`crate::mh::MultihashImpl`] into a tuple
impl ser::Serialize for MultihashImpl {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        (self.codec, self.hash.clone()).serialize(serializer)
    }
}
