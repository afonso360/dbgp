#![no_main]
#[macro_use] extern crate libfuzzer_sys;
extern crate dbgp;

use dbgp::escape::escape;

fuzz_target!(|data: &[u8]| {
    let string = match String::from_utf8(data.to_vec()) {
        Ok(x) => x,
        Err(_) => return
    };

    let _ = escape(string);
});

