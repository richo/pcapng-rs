#[macro_use]
extern crate nom;
extern crate pcapng;

use std::env;
use std::fs;
use std::io::{Read,Write};
use nom::{FileProducer,MemProducer,Producer};
use nom::{ConsumerState,Consumer};
use nom::{IResult};
use pcapng::block::{RawBlock,parse_blocks,parse_block};

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <foo.pcapng>", args[0]);
        return;
    }

    // I can't currently get the Producer machinery to work for me, so instead we read into a
    // gigantor byte array and dump that in

    // This explodes, claiming to be incomplete. I *think* there's potentially an underflow in my
    // pcap

    let mut fh = fs::File::open(&args[1]).unwrap();
    let mut buf: Vec<u8> = Vec::new();
    let read = fh.read_to_end(&mut buf);

    println!("Seeded the buffer with {}", buf.len());

    match pcapng::block::parse_blocks(&buf[..]) {
        IResult::Done(_, blocks) => {
            for i in blocks {
                println!("block: {:?}", i.parse());
            }
        }
        IResult::Error(e)      => panic!("Error: {:?}", e),
        IResult::Incomplete(i) => panic!("Incomplete: {:?}", i),

    }
}
