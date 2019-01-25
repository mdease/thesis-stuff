extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use std::time::Instant;

#[derive(Parser)]
#[allow(dead_code)]
struct AppHeader {
    version: u32,
    flags: u32,
    protected_size: usize,
    mem_offset: usize,
    ram_offset: usize,
    ram_size: usize
}

fn main() {
    for _ in 0..200 {
      let start = Instant::now();

      let input = &[1, 0, 0, 0, 2, 1, 0, 35, 0, 0, 8, 0, 0, 0, 0, 4, 0, 0, 0, 6, 0, 0, 2, 0];
      let _t = AppHeader::parse_and_create(true, &mut input.to_vec());
      //println!("{:?}", t.first_field.second_field);

      let end = start.elapsed();

      println!("{} {} {}", end.as_secs(), end.subsec_millis(), end.subsec_micros());
    }
}
