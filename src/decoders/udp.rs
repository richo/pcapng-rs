// named!(udp_packet_body<&[u8],UdpPacket>,
//        chain!(
//            interface_id: le_u32 ~
//            timestamp_hi: le_u32 ~
//            timestamp_lo: le_u32 ~
//            captured_len: le_u32 ~
//            packet_len: le_u32 ~
//            )
//        )
