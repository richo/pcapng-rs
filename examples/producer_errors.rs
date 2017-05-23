#[macro_use]
extern crate nom;
extern crate pcapng;

use std::env;
use nom::{IResult,FileProducer,Producer,ConsumerState};
use pcapng::block::parse_block;

use std::fmt::Debug;
pub fn print_block<T: Debug>(input: T) -> IResult<T,()> {
  println!("{:?}", input);
  IResult::Done(input, ())
}

consumer_from_parser!(Printer<()>,
                      flat_map!(parse_block, print_block));

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <foo.pcapng>", args[0]);
        return;
    }

    let mut producer = FileProducer::new(&args[1][..], 64).unwrap();
    let mut consumer = Printer::new();
    while let &ConsumerState::Continue(_) = producer.apply(&mut consumer) {
    }
}
