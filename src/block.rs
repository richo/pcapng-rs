use nom::{IResult};
use nom::{le_u32};

use blocks;

pub enum Block<'a> {
    SectionHeader(blocks::SectionHeader),
    EnhancedPacket(blocks::EnhancedPacket<'a>),
    InterfaceDescription(blocks::InterfaceDescription),
}

/// Public representation of a parsed block
#[derive(Debug)]
pub struct RawBlock<'a> {
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
    pub block_length: u32,
    pub body: &'a [u8],
    pub check_length: u32,
}

impl<'a> RawBlock<'a> {
    fn parse(self) -> Block<'a> {
        match self.ty {
            blocks::section_header::TY => Block::SectionHeader(blocks::section_header::parse(self)),
            blocks::enhanced_packet::TY => Block::EnhancedPacket(blocks::enhanced_packet::parse(self)),
            blocks::interface_description::TY => Block::InterfaceDescription(blocks::interface_description::parse(self)),
            _ => panic!("Unknown block type {:x}", self.ty),
        }
    }
}


named!(pub parse_block< &[u8],RawBlock >,
       chain!(
           ty: le_u32 ~
           block_length: le_u32 ~
           body: take!((block_length - 12) as usize) ~
           check_length: le_u32 ,

           ||{ RawBlock {
               ty: ty,
               block_length: block_length,
               body: body,
               check_length: check_length
           } }
           )
      );

named!(pub parse_blocks< &[u8],Vec<RawBlock> >,
       many1!(parse_block)
       );

#[test]
fn test_parse_block() {
    let input = b"\n\r\r\n\x1c\x00\x00\x00M<+\x1a\x01\x00\x00\x00\xff\xff\xff\xff\xff\xff\xff\xff\x1c\x00\x00\x00";
    match parse_block(input) {
        IResult::Done(left, RawBlock { ty, block_length, body, check_length }) => {
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

#[test]
fn test_parse_blocks() {
    let input = b"\n\r\r\n\x1c\x00\x00\x00M<+\x1a\x01\x00\x00\x00\xff\xff\xff\xff\xff\xff\xff\xff\x1c\x00\x00\x00\
    \n\r\r\n\x1c\x00\x00\x00M<+\x1a\x01\x00\x00\x00\xff\xff\xff\xff\xff\xff\xff\xff\x1c\x00\x00\x00";
    match parse_blocks(input) {
        IResult::Done(left, blocks) => {
            assert_eq!(blocks.len(), 2);
            for i in blocks {
                let RawBlock { ty, block_length, body, check_length } = i;
                // Ignored because we do not currently parse the whole block
                assert_eq!(left, b"");
                assert_eq!(ty, 0x0A0D0D0A);
                assert_eq!(block_length, 28);
                assert_eq!(body, b"M<+\x1a\x01\x00\x00\x00\xff\xff\xff\xff\xff\xff\xff\xff");
                assert_eq!(check_length, 28);
            }
        },
        _ => {
            assert_eq!(1, 2);
        },
    }
}
