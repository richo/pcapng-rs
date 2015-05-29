#[macro_use]
extern crate nom;

use nom::{Consumer,ConsumerState,MemProducer,IResult};
use nom::{le_u32};
use nom::IResult::*;

struct Block {
    ty: u32,
    block_length: u32,
    // body: &[u8],
    // padding: &[u8],
    // check_length: u32,
}


named!(block<&[u8],Block>,
       chain!(
           ty: le_u32 ~
           block_length: le_u32 ,

           ||{ Block {
               ty: ty,
               block_length: block_length,
           } }
           )
      );

#[test]
fn test_parse_block() {
    let input = b"\n\r\r\n\x1c\x00\x00\x00M<+\x1a\x01\x00\x00\x00\xff\xff\xff\xff\xff\xff\xff\xff\x1c\x00\x00\x00";
    match block(input) {
        Done(left, Block { ty, block_length }) => {
            // Ignored because we do not currently parse the whole block
            // assert_eq!(left, b"");
            assert_eq!(ty, 0x0A0D0D0A);
            assert_eq!(block_length, 28);
        },
        _ => {
            assert_eq!(1, 2);
        },
    }
}
