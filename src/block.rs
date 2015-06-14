use nom::{Consumer,ConsumerState,MemProducer,IResult,Needed};
use nom::{le_u32};

enum BlockState {
    BlockType,
    BlockLength,
    BlockBody,
    CheckLength,
}
/// Public representation of a parsed block
struct Block<'a> {
    //  0                   1                   2                   3
    //  0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
    //  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
    //  |                          Block Type                           |
    //  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
    //  |                      Block Total Length                       |
    //  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
    //  /                          Block Body                           /
    //  /          /* variable length, aligned to 32 bits */            /
    //  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
    //  |                      Block Total Length                       |
    //  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
    ty: u32,
    block_length: u32,
    body: &'a [u8],
    check_length: u32,
}

named!(block<&[u8],Block>,
       chain!(
           ty: le_u32 ~
           block_length: le_u32 ~
           body: take!((block_length - 12) as usize) ~
           check_length: le_u32 ,

           ||{ Block {
               ty: ty,
               block_length: block_length,
               body: body,
               check_length: check_length
           } }
           )
      );

#[test]
fn test_parse_block() {
    let input = b"\n\r\r\n\x1c\x00\x00\x00M<+\x1a\x01\x00\x00\x00\xff\xff\xff\xff\xff\xff\xff\xff\x1c\x00\x00\x00";
    match block(input) {
        IResult::Done(left, Block { ty, block_length, body, check_length }) => {
            // Ignored because we do not currently parse the whole block
            assert_eq!(left, b"");
            assert_eq!(ty, 0x0A0D0D0A);
            assert_eq!(block_length, 28);
            assert_eq!(body, b"M<+\x1a\x01\x00\x00\x00\xff\xff\xff\xff\xff\xff\xff\xff");
            assert_eq!(check_length, 28);
        },
        _ => {
            assert_eq!(1, 2);
        },
    }
}
