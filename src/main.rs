#![allow(unused, dead_code)]
use kdam::Bar;
use std::env;
fn main() {
    let mut args = env::args();
    let program = args.next().expect("o_O wut?");

    println!("Hello, world!");
}
