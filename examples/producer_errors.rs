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

impl<'a> Consumer for DebugConsumer<'a> {
    fn consume(&mut self, input: &[u8]) -> ConsumerState {
        match pcapng::block::parse_blocks(input) {
            IResult::Done(_, blocks) => {
                for i in blocks {
                    self.blocks.push(i);
                }
            }
            IResult::Error(e)      => panic!("Error: {:?}", e),
            IResult::Incomplete(_) => panic!("Incomplete"),
        }
        ConsumerState::ConsumerDone
    }

    fn end(&mut self) {
        println!("Done!");
    }
}

named!(printer,
       chain!(
           block: parse_block ,
           ||{
               println!("Got a blocks");
               // for i in blocks {
               //     println!("{:?}", i);
               // }
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

    // This works, and prints my packet
    let input = b"\n\r\r\n\x1c\x00\x00\x00M<+\x1a\x01\x00\x00\x00\xff\xff\xff\xff\xff\xff\xff\xff\x1c\x00\x00\x00";
    match pcapng::block::parse_blocks(input) {
        IResult::Done(_, blocks) => {
            for i in blocks {
                println!("consuming: {:?}", i);
                // self.blocks.push(i);
            }
        }
        IResult::Error(e)      => panic!("Error: {:?}", e),
        IResult::Incomplete(_) => panic!("Incomplete"),
    }

    // This explodes, hitting the Error case in debug consumer
    let mut producer = MemProducer::new(b"\n\r\r\n\x1c\x00\x00\x00M<+\x1a\x01\x00\x00\x00\xff\xff\xff\xff\xff\xff\xff\xff\x1c\x00\x00\x00", 8);
    let mut c = DebugConsumer { blocks: vec![] };
    println!("running memproducer");
    c.run(&mut producer);
    for i in c.blocks {
        // println!("{:?}", i.parse());
        println!("{:?}", i);
    }

    // As does this
    let mut producer = FileProducer::new(&args[1][..], 64).unwrap();
    let mut c = DebugConsumer { blocks: vec![] };
    println!("Running fileproducer");
    c.run(&mut producer);
    for i in c.blocks {
        // println!("{:?}", i.parse());
        println!("{:?}", i);
    }
}
