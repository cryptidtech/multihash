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
            .with_encoding(Base::Base58Btc)
            .try_build(b"for great justice, move every zig!")
            .unwrap();

        //println!("{}", hex::encode(mh.as_ref()));

        assert_tokens(
            &mh.to_inner(), // convert to Tagged<MultihashImpl>
            &[
                Token::Tuple { len: 2 },
                Token::U64(Codec::Multihash.code()),
                Token::Tuple { len: 2 },
                Token::U64(Codec::Blake2S256.code()),
                Token::Seq { len: Some(32) },
                Token::U8(0x64),
                Token::U8(0x22),
                Token::U8(0x03),
                Token::U8(0x12),
                Token::U8(0x5d),
                Token::U8(0x59),
                Token::U8(0xe8),
                Token::U8(0xb9),
                Token::U8(0x3e),
                Token::U8(0xdb),
                Token::U8(0x67),
                Token::U8(0x6f),
                Token::U8(0xc7),
                Token::U8(0x8d),
                Token::U8(0xe9),
                Token::U8(0xc5),
                Token::U8(0x87),
                Token::U8(0xcf),
                Token::U8(0x52),
                Token::U8(0xcc),
                Token::U8(0xc6),
                Token::U8(0xf2),
                Token::U8(0x19),
                Token::U8(0x03),
                Token::U8(0x2d),
                Token::U8(0xa1),
                Token::U8(0xf3),
                Token::U8(0x77),
                Token::U8(0x08),
                Token::U8(0x23),
                Token::U8(0x32),
                Token::U8(0xb0),
                Token::SeqEnd,
                Token::TupleEnd,
                Token::TupleEnd,
            ],
        );
    }

    #[test]
    fn test_serde_string() {
        let mh = Builder::new(Codec::Blake2S256)
            .with_encoding(Base::Base58Btc)
            .try_build(b"for great justice, move every zig!")
            .unwrap();

        assert_tokens(
            &mh.readable(),
            &[Token::String(
                "z2fxXB6wkDUNsk96BNo1nB84GoEhtwEU3YoFLPVbzpg6ystG88i3",
            )],
        );
    }
}
