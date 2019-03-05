use nom::{IResult, le_u32};
use block::RawBlock;
use blocks::constants::*;
use options::{parse_options, Options};

pub const TY: u32 = BlockType::InterfaceStatistics as u32;

//     0                   1                   2                   3
//     0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
//    +---------------------------------------------------------------+
//  0 |                   Block Type = 0x00000005                     |
//    +---------------------------------------------------------------+
//  4 |                      Block Total Length                       |
//    +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//  8 |                         Interface ID                          |
//    +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// 12 |                        Timestamp (High)                       |
//    +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// 16 |                        Timestamp (Low)                        |
//    +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// 20 /                                                               /
//    /                      Options (variable)                       /
//    /                                                               /
//    +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//    |                      Block Total Length                       |
//    +---------------------------------------------------------------+

named!(interface_stats_body<&[u8], InterfaceStatistics>,
       do_parse!(
           interface_id: le_u32 >>
           timestamp_high: le_u32 >>
           timestamp_low: le_u32 >>
           options: opt!(complete!(parse_options)) >>
           (
               InterfaceStatistics {
                   ty: TY,
                   block_length: 0,
                   interface_id: interface_id,
                   timestamp_high: timestamp_high,
                   timestamp_low: timestamp_low,
                   options: options,
                   check_length: 0,
               }

           )
           )
       );

#[derive(Debug)]
pub struct InterfaceStatistics<'a> {
    pub ty: u32,
    pub block_length: u32,
    pub interface_id: u32,
    pub timestamp_high: u32,
    pub timestamp_low: u32,
    pub options: Option<Options<'a>>,
    pub check_length: u32,
}

pub fn parse(blk: RawBlock) -> IResult<&[u8], InterfaceStatistics> {
    match interface_stats_body(blk.body) {
        // FIXME(richo) Actually do something with the leftover bytes
        Ok((left, mut block)) => {
            block.block_length = blk.block_length;
            block.check_length = blk.check_length;
            Ok((left, block))
        },
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {

    use nom::IResult;

    use super::*;
    use block::parse_block;

    #[test]
    fn test_parse_interface_stats_header() {
        let input = b"\x05\x00\x00\x00\x6C\x00\x00\x00\x00\x00\x00\x00\x06\x3B\x05\x00\x20\xBD\x9C\
    \x64\x01\x00\x1C\x00\x43\x6F\x75\x6E\x74\x65\x72\x73\x20\x70\x72\x6F\x76\x69\x64\x65\x64\x20\
    \x62\x79\x20\x64\x75\x6D\x70\x63\x61\x70\x02\x00\x08\x00\x06\x3B\x05\x00\x6E\xD9\x8A\x63\x03\
    \x00\x08\x00\x06\x3B\x05\x00\xC8\xBC\x9C\x64\x04\x00\x08\x00\x35\x00\x00\x00\x00\x00\x00\x00\
    \x05\x00\x08\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x6C\x00\x00\x00";

        let (_, block) = parse_block(input).unwrap();
        if let Ok((left, interface_stats_header)) = parse(block) {

            assert_eq!(left, b"");
            assert_eq!(interface_stats_header.ty, TY);
        } else {
            assert!(false, "failed to parse interface_stats_header");
        }
    }
}
