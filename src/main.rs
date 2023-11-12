#![allow(unused, dead_code)]
use kdam::Bar;
use std::env;
const USAGE: &str = "(BITS) (ALGORITHM)\
where BITS is in the range [4..12]\
where ALGORITHM is one of [b, mb, both]";
fn main() {
    let mut args = env::args();
    let program = args.next().expect("o_O wut?");
    let Some(bits) = args.next() else {
        eprintln!("USAGE: {program} {USAGE}");
        std::process::exit(1);
    };
    let Some(algo) = args.next() else {
        eprintln!("Expected 2 arguments, got 1.");
        eprintln!("USAGE: {program} {USAGE}");
        std::process::exit(1);
    };
    let None = args.next() else {
        eprintln!("Expected 2 arguments, got >2.");
        eprintln!("USAGE: {program} {USAGE}");
        std::process::exit(1);
    };
    let Ok(bits) = bits.parse::<u8>() else {
        eprint!("Expected an integer, got instead \"{bits}\".");
        std::process::exit(1);
    };
    println!("Executing {algo} on two {bits} bit numbers.");
    let count = bits * bits;
    //todo put bar here
    for x in 0..(1<<bits) {
        for y in 0..(1<<bits) {
            let expected = x * y;
            
        }
    }
}

fn booth(x: u16, y: u16) -> u32 {
    
    0
}

fn modbooth(x: u16, y: u16) -> u32 {
    todo!()

    0
}
