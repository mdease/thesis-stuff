// #![no_std]
// #![feature(lang_items, start)]
extern crate core;
extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use std::time::Instant;

#[allow(dead_code)]
#[derive(Parser)]
struct Command {
    driver_number: usize,
    subdriver_number: usize,
    arg0: usize,
    arg1: usize,
}

// #[derive(Parser)]
// #[allow(dead_code)]
// struct Child {
//   first_field: u16,
//   second_field: u16,
// }

// #[derive(Parser)]
// #[allow(dead_code)]
// struct TypesParser {
//   first_field: Child,
//   second_field: u16,
// }

//use core::panic::PanicInfo;

// #[panic_handler]
// fn panic(_info: &PanicInfo) -> ! {
//     loop {}
// }

// #[lang = "eh_personality"]
// pub extern "C" fn eh_personality() {}

// #[start]
// fn start(_argc: isize, _argv: *const *const u8) -> isize {
//   let input = [9, 0, 1, 0, 8, 0, 0, 0];
//   let t = TypesParser::parse_and_create(true, &input);
//   0
// }

fn main() {
  for _ in 0..200 {
    let start = Instant::now();

    // let mut offset: usize = 0;

    // let c = Child { first_field: 0, second_field: 0, third_field: 0, fourth_field: 0 };
    // Child::validate(&c as *const Child as *const u8);

    let input = [9, 0, 1, 0, 8, 0, 0, 0, 2, 2, 2, 2, 0, 9, 0, 9];
    let t = Command::parse(&input);
    // println!("{:?}", t.first_field.first_field);
    // println!("{:?}", t.first_field.second_field);
    // println!("{:?}", t.first_field.third_field);
    // println!("{:?}", t.second_field);

    // Child::validate(&t as *const Child as *const u8);

    let end = start.elapsed();

    println!("{}", end.subsec_nanos());
    // println!("{} {} {} {}", end.as_secs(), end.subsec_millis(), end.subsec_micros(), end.subsec_nanos());
  }
}
