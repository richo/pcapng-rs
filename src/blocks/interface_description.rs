use nom::{IResult};
use nom::{le_u64,le_u32,le_u16};
use block::{parse_block,Block,RawBlock};
use options::Options;

pub const TY: u32 = 0x00000001;

//     0                   1                   2                   3
//     0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
//    +---------------------------------------------------------------+
//  0 |                    Block Type = 0x00000001                    |
//    +---------------------------------------------------------------+
//  4 |                      Block Total Length                       |
//    +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//  8 |           LinkType            |           Reserved            |
//    +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// 12 |                            SnapLen                            |
//    +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// 16 /                                                               /
//    /                      Options (variable)                       /
//    /                                                               /
//    +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//    |                      Block Total Length                       |
//    +---------------------------------------------------------------+

named!(interface_description_body<&[u8],InterfaceDescription>,
       chain!(
           link_type: le_u16 ~
           reserved: le_u16 ~
           snap_len: le_u32 ,
           ||{
               InterfaceDescription {
                   ty: TY,
                   block_length: 0,
                   link_type: link_type,
                   reserved: reserved,
                   snap_len: snap_len,
                   options: None, // FIXME(richo)
                   check_length: 0,
               }

           }
           )
       );

pub struct InterfaceDescription {
    ty: u32,
    block_length: u32,
    link_type: u16,
    reserved: u16,
    snap_len: u32,
    options: Option<Options>,
    check_length: u32,
}

pub fn parse(blk: RawBlock) -> InterfaceDescription {
    match interface_description_body(blk.body) {
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
