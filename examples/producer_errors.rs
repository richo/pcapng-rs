#[macro_use]
extern crate nom;
extern crate pcapng;

use std::env;
use nom::{FileProducer,MemProducer,Producer};
use nom::{ConsumerState,Consumer};
use nom::{IResult};
use pcapng::block::{RawBlock,parse_blocks,parse_block};

struct DebugConsumer<'a> {
        pub blocks: Vec<RawBlock<'a>>,
}

named!(printer,
       chain!(
           block: parse_block ,
           ||{
               println!("{:?}", block);
               &[]
           }
           ));

pusher!(print, printer);

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <foo.pcapng>", args[0]);
        return;
    }

    let mut producer = FileProducer::new(&args[1][..], 64).unwrap();
    println!("Running fileproducer");
    print(&mut producer);
}
