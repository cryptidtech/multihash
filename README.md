# Multihash

Multiformats multihash implementation without size in the type signature. Uses
the [`multiutil::BaseEncoded`] and [`multiutil::Tagged`] smart pointers to 
wrap the MultihashImpl. This automates the encoding/decoding to/from strings 
and byte slices for multihashes. It also has serde support to/from strings for
`BaseEncoded<T>` and to/from tuples for `Tagged<T>`.
