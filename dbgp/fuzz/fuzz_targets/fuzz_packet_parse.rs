#![no_main]
#[macro_use] extern crate libfuzzer_sys;
extern crate dbgp;

use dbgp::packets::{Packet,Init};

fuzz_target!(|data: &[u8]| {
    let _: Result<Packet<Init>, _> = dbgp::packets::Packet::deserialize(data);
    // fuzzed code goes here
});
