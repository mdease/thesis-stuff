extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use std::time::Instant;

#[derive(Parser)]
#[allow(dead_code)]
// http://www.argenox.com/a-ble-advertising-primer/
struct BluetoothPacket {
    preamble: u8,
    access_address: u32,
    header: u16
}

fn main() {
    for _ in 0..200 {
      let start = Instant::now();

      let input = &[29, 0, 1, 0, 8, 4, 103];
      let _t = BluetoothPacket::parse_and_create(true, &mut input.to_vec());
      //println!("{:?}", t.first_field.second_field);

      let end = start.elapsed();

      println!("{} {} {}", end.as_secs(), end.subsec_millis(), end.subsec_micros());
    }
}
