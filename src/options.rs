use nom::{IResult};
use nom::{le_u64,le_u32,le_u16};
use util;

// FIXME(richo) Flesh this out properly with it's own discrete parser.
#[derive(Debug)]
pub struct Options<'a> {
    pub options: Vec<Opt<'a>>,
}

#[derive(Debug)]
pub struct Opt<'a> {
    code: u16,
    length: u16,
    value: &'a [u8],
}

//  0                   1                   2                   3
//  0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |      Option Code              |         Option Length         |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// /                       Option Value                            /
// /          /* variable length, aligned to 32 bits */            /
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// /                                                               /
// /                 . . . other options . . .                     /
// /                                                               /
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |   Option Code == opt_endofopt  |  Option Length == 0          |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+

named!(option<&[u8],Opt>,
       chain!(
           code: le_u16 ~
           length: le_u16 ~
           value: take!(length as usize) ~
           take!(util::pad_to_32bits(length as usize)),
           ||{
               Opt {
                   code: code,
                   length: length,
                   value: value,
               }
           }
           )
      );

// It's not abundantly clear to me that this is actually safe.
// My belief is that because we're operating on a &[u8] that was carved out of the high level
// buffer, and that *it* is a fat pointer with a length, the runtime will stop us from running off
// the end, but it needs to be actually proven.
named!(pub parse_options< &[u8],Options >,
       chain!(
           opts: many1!(option),
           ||{
               // It's also not super clear to me that we actually want to include the final option
               // in the vector
               if let Some(last) = opts.last() {
                   assert_eq!(last.code, 0x0);
                   assert_eq!(last.length, 0x0);
               }
               Options {
                   options: opts
               }
           }
           )
      );


#[test]
fn test_parse_options() {
    let input = b"\x12\x42\x08\x00asdfasdf\x00\x00\x00\x00";
    match parse_options(input) {
        IResult::Done(left, opts) => {
            assert_eq!(opts.options.len(), 2);
            let o = &opts.options[0];
            assert_eq!(o.code, 0x4212);
            assert_eq!(o.length, 0x08);
            assert_eq!(o.value, b"asdfasdf");

        },
        _ => {
            panic!("Hit a codepath we shouldn't have");
        },
    }
}

