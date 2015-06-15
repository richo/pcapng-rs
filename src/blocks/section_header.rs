use nom::{IResult};
use nom::{le_u64,le_u32,le_u16};
use block::{block,Block,RawBlock};

//    0                   1                   2                   3
//    0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
//    +---------------------------------------------------------------+
//  0 |                   Block Type = 0x0A0D0D0A                     |
//    +---------------------------------------------------------------+
//  4 |                      Block Total Length                       |
//    +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//  8 |                      Byte-Order Magic                         |
//    +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// 12 |          Major Version        |         Minor Version         |
//    +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// 16 |                                                               |
//    |                          Section Length                       |
//    |                                                               |
//    +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// 24 /                                                               /
//    /                      Options (variable)                       /
//    /                                                               /
//    +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//    |                      Block Total Length                       |
//    +---------------------------------------------------------------+

// FIXME(richo) work out where this should actually live
struct Options;

named!(section_header_body<&[u8],SectionHeader>,
       chain!(
           magic: le_u32 ~
           major_version: le_u16 ~
           minor_version: le_u16 ~
           _section_length: le_u64 ,
           // Can we get the blocks by virtue of knowing how much data we have left here?
           ||{
               let section_length = if _section_length == 0xFFFFFFFFFFFFFFFF {
                   SectionLength::Unspecified
               } else {
                   SectionLength::Bytes(_section_length)
               };

               assert_eq!(magic, 0x1A2B3C4D);
               SectionHeader {
                   ty: 0x0A0D0D0A,
                   block_length: 0,
                   magic: magic,
                   major_version: major_version,
                   minor_version: minor_version,
                   section_length: section_length,
                   options: None, // FIXME(richo)
                   check_length: 0,
           } }
           )
      );

#[derive(PartialEq,Debug)]
pub enum SectionLength {
    Bytes(u64),
    Unspecified,
}

// Dummy struct for now
pub struct SectionHeader {
    ty: u32,
    block_length: u32,
    magic: u32,
    major_version: u16,
    minor_version: u16,
    section_length: SectionLength,
    options: Option<Options>,
    check_length: u32,
}

pub fn parse(blk: RawBlock) -> SectionHeader {
    // TODO(richo) Actually parse out the options afterward
    // I think that we can do this by invoking an options parser, and using the fact that we're
    // dealing with slices by this point to our advantage
    match section_header_body(blk.body) {
        IResult::Done(left, mut block) => {
            block.block_length = blk.block_length;
            block.check_length = blk.check_length;
            block
        },
        _ => {
            panic!("Couldn't unpack this section_header");
        }
    }
}

#[test]
fn test_parse_section_header() {
    let input = b"\n\r\r\n\x1c\x00\x00\x00M<+\x1a\x01\x00\x00\x00\xff\xff\xff\xff\xff\xff\xff\xff\x1c\x00\x00\x00";
    match block(input) {
        IResult::Done(left, block) => {
            let section_header = parse(block);

            // Ignored because we do not currently parse the whole block
            assert_eq!(left, b"");
            assert_eq!(section_header.ty, 0x0A0D0D0A);
            assert_eq!(section_header.block_length, 28);
            assert_eq!(section_header.magic, 0x1A2B3C4D);
            assert_eq!(section_header.section_length, SectionLength::Unspecified);
            assert_eq!(section_header.check_length, 28);
        },
        _ => {
            assert_eq!(1, 2);
        },
    }
}
