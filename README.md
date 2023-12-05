# Multihash

Multiformats multihash implementation without size in the type signature. Uses
the [`multiutil::BaseEncoded`] smart pointers to wrap the Multihash into
EncodecMultihash for base encoded multihashes. This automates the
encoding/decoding to/from strings and byte slices for multihashes. It also has
serde support to/from human readable and binary formats. It also supports raw
binary encoding and decoding using [`Into<Vec<u8>>`] and [`TryFrom<&[u8]>`]
trait implementations.
