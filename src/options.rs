use nom::*;
use util;

// FIXME(richo) Flesh this out properly with it's own discrete parser.
#[derive(Debug)]
pub struct Options<'a> {
    pub options: Vec<Opt<'a>>,
}

#[derive(Debug)]
pub struct Opt<'a> {
    pub code: u16,
    pub length: u16,
    pub value: &'a [u8],
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

named!(option<Opt>,
       do_parse!(
              code:   le_u16
           >> length: le_u16
           >> value:  take!(length as usize )
           >>         take!(util::pad_to_32bits(length as usize))
           >> (
               Opt {
                   code: code,
                   length: length,
                   value: value,
               }
           )
       )
);

// It's not abundantly clear to me that this is actually safe.
// My belief is that because we're operating on a &[u8] that was carved out of the high level
// buffer, and that *it* is a fat pointer with a length, the runtime will stop us from running off
// the end, but it needs to be actually proven.
named!(pub parse_options<Options>,
       do_parse!(
              opts: many1!(complete!(option))
           >> ( {
               // It's also not super clear to me that we actually want to include the final option
               // in the vector.
               if let Some(last) = opts.last() {
                   assert_eq!(last.code, 0x0);
                   assert_eq!(last.length, 0x0);
               }
               Options {
                   options: opts
               }
           })
       )
);

#[cfg(test)]


#[test]
fn test_parse_options() {
    let input = b"\x12\x42\x08\x00asdfasdf\x00\x00\x00\x00";
    match parse_options(input) {
        Ok((left, opts)) => {
            assert_eq!(left, b"");
            assert_eq!(opts.options.len(), 2);
            let o = &opts.options[0];
            assert_eq!(o.code, 0x4212);
            assert_eq!(o.length, 0x08);
            assert_eq!(o.value, b"asdfasdf");

        }
        err => {
            panic!("Hit a codepath we shouldn't have: {:?}", err);
        }
    }
}

#[test]
fn test_multiple_options() {
    let input = b"\x03\x00\x0b\x00\x57\x69\x6e\x64\
                \x6f\x77\x73\x20\x58\x50\x00\x00\x04\x00\x0c\x00\x54\x65\x73\x74\
                \x30\x30\x34\x2e\x65\x78\x65\x00\x00\x00\x00\x00";
    match parse_options(input) {
        Ok((left, opts)) => {
            assert_eq!(left, []);
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
        }
        err => {
            panic!("Hit a codepath we shouldn't have: {:?}", err);
        }
    }
}
