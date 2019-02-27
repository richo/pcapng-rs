extern crate nom;
extern crate pcapng;

use std::env;
use std::fs;
use std::io::Read;
use pcapng::block::parse_blocks;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <foo.pcapng>", args[0]);
        return;
    }

    let mut fh = fs::File::open(&args[1]).unwrap();
    let mut buf: Vec<u8> = Vec::new();
    let _ = fh.read_to_end(&mut buf);

    match parse_blocks(buf.as_slice()) {
        Ok((_, blocks)) => {
            for i in blocks {
                if let Ok((_, blk)) = i.parse() {
                    println!("{:?}", blk);
                }
            }
        }
        Err(nom::Err::Error(e))      => panic!("Error: {:?}", e),
        Err(nom::Err::Incomplete(i)) => panic!("Incomplete: {:?}", i),
        Err(nom::Err::Failure(f))     => panic!("Failure: {:?}", f),
    }
}
