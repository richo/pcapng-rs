use nom::{IResult, le_u32, le_u16};
use block::RawBlock;
use options::{parse_options, Options};

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
       do_parse!(
              link_type: le_u16
           >> reserved:  le_u16
           >> snap_len:  le_u32
           >> options:   opt!(complete!(parse_options))
           >> (
               InterfaceDescription {
                   ty: TY,
                   block_length: 0,
                   link_type: link_type,
                   reserved: reserved,
                   snap_len: snap_len,
                   options: options,
                   check_length: 0,
               }

           )
       )
);

#[derive(Debug)]
pub struct InterfaceDescription<'a> {
    pub ty: u32,
    pub block_length: u32,
    pub link_type: u16,
    pub reserved: u16,
    pub snap_len: u32,
    // sduquette: Make options a Vec<Opt> instead?
    pub options: Option<Options<'a>>,
    pub check_length: u32,
}

pub fn parse(blk: RawBlock) -> IResult<&[u8], InterfaceDescription> {
    match interface_description_body(blk.body) {
        // FIXME(richo) Actually do something with the leftover bytes
        Ok((left, mut block)) => {
            block.block_length = blk.block_length;
            block.check_length = blk.check_length;
            Ok((left, block))
        },
        Err(e) => Err(e)
    }
}

#[cfg(test)]
mod tests {

    use nom::IResult;

    use super::*;
    use block::parse_block;
    use blocks::constants::{BlockType, LinkType, LinkTypeOptions};

    #[test]
    fn test_parse_interface_description_header() {
        let input = b"\x01\x00\x00\x00\x88\x00\x00\x00\x01\x00\x00\x00\x00\x00\x04\x00\x02\x00\x32\
\x00\x5C\x44\x65\x76\x69\x63\x65\x5C\x4E\x50\x46\x5F\x7B\x45\x34\x43\x31\x34\x31\x32\x38\
\x2D\x34\x31\x46\x35\x2D\x34\x32\x43\x35\x2D\x39\x41\x35\x35\x2D\x44\x36\x32\x32\x33\x42\
\x30\x32\x43\x32\x42\x31\x7D\x00\x00\x09\x00\x01\x00\x06\x00\x00\x00\x0C\x00\x2B\x00\x33\
\x32\x2D\x62\x69\x74\x20\x57\x69\x6E\x64\x6F\x77\x73\x20\x37\x20\x53\x65\x72\x76\x69\x63\
\x65\x20\x50\x61\x63\x6B\x20\x31\x2C\x20\x62\x75\x69\x6C\x64\x20\x37\x36\x30\x31\x00\x00\
\x00\x00\x00\x88\x00\x00\x00";

        let (_, block) = parse_block(input).unwrap();
        let (left, interface_description_header) = parse(block).unwrap();
        assert_eq!(left, b"");
        assert_eq!(interface_description_header.ty, BlockType::InterfaceDescription as u32);
        assert_eq!(interface_description_header.block_length, 136);
        assert_eq!(interface_description_header.link_type, LinkType::ETHERNET as u16);
        assert_eq!(interface_description_header.snap_len, 0x40000);
        assert_eq!(interface_description_header.check_length, 136);

        if let Some(opts) = interface_description_header.options {
            assert_eq!(opts.options.len(), 4);

            let o = &opts.options[0];
            assert_eq!(o.code, LinkTypeOptions::Name as u16);
            assert_eq!(o.length, 0x32);
            assert_eq!(o.value[..], b"\\Device\\NPF_{E4C14128-41F5-42C5-9A55-D6223B02C2B1}"[..]);

            let o = &opts.options[1];
            assert_eq!(o.code, LinkTypeOptions::TsResol as u16);
            assert_eq!(o.length, 1);
            assert_eq!(o.value[..], b"\x06"[..]);

            let o = &opts.options[2];
            assert_eq!(o.code, LinkTypeOptions::OS as u16);
            assert_eq!(o.value[..], b"32-bit Windows 7 Service Pack 1, build 7601"[..]);
        } else {
            panic!("Oh shiii");
        }
    }
}
