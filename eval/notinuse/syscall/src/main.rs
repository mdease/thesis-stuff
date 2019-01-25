extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use std::time::Instant;

#[derive(Parser)]
#[allow(dead_code)]
struct SyscallRequest {
    pid: u32,
    syscall_num: u32,
    arg1: usize,
    arg2: usize,
    arg3: usize
}

fn main() {
    for _ in 0..200 {
      let start = Instant::now();

      let input = &[9, 0, 1, 0, 8, 0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0];
      let _t = SyscallRequest::parse_and_create(true, &mut input.to_vec());
      //println!("{:?}", t.first_field.second_field);

      let end = start.elapsed();

      println!("{} {} {}", end.as_secs(), end.subsec_millis(), end.subsec_micros());
    }
}
