pcapng-rs
=========

pcapng offers you a a pcapng parser in native rust code. A couple of variations
on how to read pcapng files in from a file are in `examples/`.

Under the hood, it usees [nom][nom] to implement it's parsing, which lets this
library stay small and compact. It's currently in a very unstable state, and
I'll probably shuffle a lot of interfaces around (Especially surrounding where
the actual Block classes live), but if you build something atop it, please let
me know and I'll attempt to accomodate.

At the highest level, the easiest way to get packets is to read the whole
pcapng file into memory, and then run the parser over it:

```rust
let mut fh = fs::File::open("filename.pcapng").unwrap();
let mut buf: Vec<u8> = Vec::new();
let read = fh.read_to_end(&mut buf);

match pcapng::block::parse_blocks(&buf[..]) {
    IResult::Done(_, blocks) => {
        for i in blocks {
            println!("{:?}", i.parse());
        }
    }
    IResult::Error(e)      => panic!("Error: {:?}", e),
    IResult::Incomplete(i) => panic!("Incomplete: {:?}", i),

}
```

Other approaches using the actual Consumer infra are preferable if you want to
stream, but involve writing much more code.

## Contact

If you're using this, I would love to know. I'm reachable as `richo` on
freenode or mozilla's irc.

## License

Released under the terms of the MIT license.

[nom]: https://github.com/Geal/nom
