//! Serde (de)serialization for [`crate::prelude::MultihashImpl`]
mod de;
mod ser;

#[cfg(test)]
mod tests {
    use crate::prelude::{Base, Builder, Codec};
    use serde_test::{assert_tokens, Configure, Token};

    #[test]
    fn test_serde_binary() {
        let mh = Builder::new(Codec::Blake2S256)
            .try_build(b"for great justice, move every zig!")
            .unwrap();

        assert_tokens(
            &mh.compact(), // convert to Tagged<MultihashImpl>
            &[
                Token::Tuple { len: 3 },
                Token::Bytes(&[0x31]), // Codec::Multihash as varuint
                Token::Bytes(&[0xE0, 0xE4, 0x02]), // Codec::Blake2S256 as varuint
                Token::Bytes(&[
                    // Varbytes, which is a varuint encoded len followed by bytes
                    0x20, 0x64, 0x22, 0x03, 0x12, 0x5d, 0x59, 0xe8, 0xb9, 0x3e, 0xdb, 0x67, 0x6f,
                    0xc7, 0x8d, 0xe9, 0xc5, 0x87, 0xcf, 0x52, 0xcc, 0xc6, 0xf2, 0x19, 0x03, 0x2d,
                    0xa1, 0xf3, 0x77, 0x08, 0x23, 0x32, 0xb0,
                ]),
                Token::TupleEnd,
            ],
        );
    }

    #[test]
    fn test_serde_encoded_string() {
        let mh = Builder::new(Codec::Blake2S256)
            .with_encoding(Base::Base58Btc)
            .try_build_encoded(b"for great justice, move every zig!")
            .unwrap();

        assert_tokens(
            &mh.readable(),
            &[Token::String(
                "z2fxXB6wkDUNsk96BNo1nB84GoEhtwEU3YoFLPVbzpg6ystG88i3",
            )],
        );
    }

    #[test]
    fn test_serde_string() {
        let mh = Builder::new(Codec::Blake2S256)
            .try_build(b"for great justice, move every zig!")
            .unwrap();
        assert_tokens(
            &mh.readable(),
            &[
                Token::Struct {
                    name: "Multihash",
                    len: 2,
                },
                Token::Str("codec"),
                Token::U64(0xb260_u64),
                Token::Str("hash"),
                Token::Str("f20642203125d59e8b93edb676fc78de9c587cf52ccc6f219032da1f377082332b0"),
                Token::StructEnd,
            ],
        );
    }
}
