#![no_main]
use libfuzzer_sys::fuzz_target;

use libfuzzer_sys::arbitrary::{self, Arbitrary};

#[derive(Debug, Arbitrary)]
struct FuzzingInput<'a> {
    options: u32,
    markdown: &'a str,
}

fuzz_target!(|data: FuzzingInput<'_>| {
    let opts = markdown_weaver::Options::from_bits_truncate(data.options);

    for _ in markdown_weaver::Parser::new_ext(data.markdown, opts) {}
});
