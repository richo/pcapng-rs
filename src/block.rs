use nom::{le_u32, IResult};

use blocks;
use util;

#[derive(Debug)]
pub enum Block<'a> {
    SectionHeader(blocks::SectionHeader<'a>),
    EnhancedPacket(blocks::EnhancedPacket<'a>),
    InterfaceDescription(blocks::InterfaceDescription<'a>),
    InterfaceStatistics(blocks::InterfaceStatistics<'a>),
    UnknownBlock(RawBlock<'a>),
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
    pub ty: u32,
    pub block_length: u32,
    pub body: &'a [u8],
    pub check_length: u32,
}

impl<'a> RawBlock<'a> {
    pub fn parse(self) -> IResult<&'a [u8], Block<'a> > {
        match self.ty {
            blocks::section_header::TY => {
                match blocks::section_header::parse(self) {
                    Ok((left, blk)) => Ok((left, Block::SectionHeader(blk))),
                    Err(e) => Err(e),
                }
            }

            blocks::enhanced_packet::TY => {
                match blocks::enhanced_packet::parse(self) {
                    Ok((left, blk)) => Ok((left, Block::EnhancedPacket(blk))),
                    Err(e) => Err(e),
                }
            }

            blocks::interface_stats::TY => {
                match blocks::interface_stats::parse(self) {
                    Ok((left, blk)) => {
                        Ok((left, Block::InterfaceStatistics(blk)))
                    },
                    Err(e) => Err(e),
                }
            }

            blocks::interface_description::TY => {
                match blocks::interface_description::parse(self) {
                    Ok((left, blk)) => {
                        Ok((left, Block::InterfaceDescription(blk)))
                    },
                    Err(e) => Err(e),
                }
            }
            _ => Ok((&[], Block::UnknownBlock(self))),
        }
    }
}


named!(pub parse_block<RawBlock >,
       do_parse!(
              ty: le_u32
           >> block_length: le_u32
           >> body: take!((block_length - 12) as usize)
           >> take!(util::pad_to_32bits((block_length - 12) as usize))
           >> check_length: le_u32

           >> ( RawBlock {
               ty: ty,
               block_length: block_length,
               body: body,
               check_length: check_length
           } )
           )
      );

named!(pub parse_blocks<Vec<RawBlock> >,
       many1!(complete!(parse_block))
       );


#[cfg(test)]
mod tests {
use super::*;

#[test]
fn test_parse_block() {
    let input = b"\n\r\r\n\x1c\x00\x00\x00M<+\x1a\x01\x00\x00\x00\xff\xff\xff\xff\xff\xff\xff\xff\x1c\x00\x00\x00";
    match parse_block(input) {
        Ok((left, RawBlock { ty, block_length, body, check_length })) => {
            // Ignored because we do not currently parse the whole block
            assert_eq!(left, b"");
            assert_eq!(ty, 0x0A0D0D0A);
            assert_eq!(block_length, 28);
            assert_eq!(body, b"M<+\x1a\x01\x00\x00\x00\xff\xff\xff\xff\xff\xff\xff\xff");
            assert_eq!(check_length, 28);
        }
        _ => {
            assert_eq!(1, 2);
        }
    }
}

#[test]
fn test_parse_blocks() {
    let input = b"\n\r\r\n\x1c\x00\x00\x00M<+\x1a\x01\x00\x00\x00\xff\xff\xff\xff\xff\xff\xff\xff\x1c\x00\x00\x00\
    \n\r\r\n\x1c\x00\x00\x00M<+\x1a\x01\x00\x00\x00\xff\xff\xff\xff\xff\xff\xff\xff\x1c\x00\x00\x00";
    match parse_blocks(input) {
        Ok((left, blocks)) => {
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
        }
        err => {
            println!("error: {:?}", err);
            assert_eq!(1, 2);
        }
    }
}

#[test]
fn test_parse_weird_length_block() {
    let input = b"\n\r\r\n\x1b\x00\x00\x00<+\x1a\x01\x00\x00\x00\xff\xff\xff\xff\xff\xff\xff\xff\x00\x1b\x00\x00\x00";
    match parse_block(input) {
        Ok((left, RawBlock { ty, block_length, body, check_length })) => {
            // Ignored because we do not currently parse the whole block
            assert_eq!(left, b"");
            assert_eq!(ty, 0x0A0D0D0A);
            assert_eq!(27, block_length);
            assert_eq!(body.len() + 12, block_length as usize);
            assert_eq!(body, b"<+\x1a\x01\x00\x00\x00\xff\xff\xff\xff\xff\xff\xff\xff");
            assert_eq!(body.len() + 12, check_length as usize);
        }
        _ => {
            unreachable!("Couldn't parse the block");
        }
    }
}

#[test]
fn test_multiple_options() {
    let input = b"\x0a\x0d\x0d\x0a\x40\x00\x00\x00\x4d\x3c\x2b\x1a\x01\x00\x00\x00\
                \xff\xff\xff\xff\xff\xff\xff\xff\x03\x00\x0b\x00\x57\x69\x6e\x64\
                \x6f\x77\x73\x20\x58\x50\x00\x00\x04\x00\x0c\x00\x54\x65\x73\x74\
                \x30\x30\x34\x2e\x65\x78\x65\x00\x00\x00\x00\x00\x40\x00\x00\x00";
    match parse_block(input) {
        Ok((left, block)) => {
            assert_eq!(left, b"");
            match block.parse() {
                Ok((_, Block::SectionHeader(blk))) => {
                    if let Some(opts) = blk.options {
                        assert_eq!(opts.options.len(), 3);

                        let o = &opts.options[0];
                        assert_eq!(o.code, 0x03);
                        assert_eq!(o.length, 0x0b);
                        assert_eq!(&o.value[..], b"Windows XP\x00");

                        let o = &opts.options[1];
                        assert_eq!(o.code, 0x04);
                        assert_eq!(o.length, 0x0c);
                        assert_eq!(&o.value[..], b"Test004.exe\x00");

                        let o = &opts.options[2];
                        assert_eq!(o.code, 0x00);
                        assert_eq!(o.length, 0x00);
                        assert_eq!(&o.value[..], b"");
                    } else {
                        unreachable!();
                    }
                } ,
                err =>{
                    panic!("error: {:?}", err);
                }
            }
        }
        _ => {
            panic!("Hit a codepath we shouldn't have");
        }
    }
}
}
