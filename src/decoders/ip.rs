// 0                   1                   2                   3
// 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |Version|  IHL  |Type of Service|          Total Length         |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |         Identification        |Flags|      Fragment Offset    |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |  Time to Live |    Protocol   |         Header Checksum       |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |                       Source Address                          |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |                    Destination Address                        |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |                    Options                    |    Padding    |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+

struct IpHeader<'a> {
    version: u8,
    ihl: u8,
    ty: u8,
    total_len: u16,
    ident: u16,
    flags: u8, // TODO(richo) bitflags!
    fragment_offset: u16,
    ttl: u8,
    protocol: u8,
    header_checksum: u16,
    source_addr: u32, // TODO(richo) IpAddr
    dest_addr: u32,
    options: u32, // TODO(richo) u24
    padding: u8,
    body: &'a [u8],
}

named!(ip_packet<&[u8],IpHeader>,
       chain!(
           version_ihl: le_u8 ~
           ty: le_u8 ~
           total_len: le_u16 ~
           ident: le_u16 ~
           flags_fragment_offset: le_u16 ~
           ttl: le_u8 ~
           protocol: le_u8 ~
           header_checksum: le_u16 ~
           source_addr: le_u32 ~
           dest_addr: le_u32 ~
           options_padding: le_u32,

           ||{
               let version = version_idl >> 4;
               let ihl = version_idl & 0xf;
               IpHeader {
                   ty: TY,
                   block_length: 0,
                   interface_id: interface_id,
                   timestamp_hi: timestamp_hi,
                   timestamp_lo: timestamp_lo,
                   captured_len: captured_len,
                   packet_len: packet_len,
                   data: data,
                   options: options,
                   check_length: 0,
               }
           }
           )
       );

#[test]
fn test_ip_header() {
    let input = b"\x45\x10\
                  \x00\x28\xdb\xec\x40\x00\x40\x06\
                  \xf4\x1d\xac\x13\x83\x98\xc6\xc7\
                  \x74\x42";
    match parse_block(input) {
        IResult::Done(left, block) => {
            assert_eq!(left, b"");
            if let Block::SectionHeader(blk) = block.parse() {
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
            } else {
                unreachable!();
            }
        },
        _ => {
            panic!("Hit a codepath we shouldn't have");
        },
    }
}
