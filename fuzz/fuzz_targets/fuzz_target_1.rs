#![no_main]
#[macro_use]
extern crate libfuzzer_sys;
use libfuzzer_sys::fuzz_target;
use pingpong_word;

fuzz_target!(|data: &[u8]| {
    // fuzzed code goes here
    if let Ok(s) = std::str::from_utf8(data) {
        let _ = pingpong_word::is_pingpong(&s.to_owned());
    }
});
