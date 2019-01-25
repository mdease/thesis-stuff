#![no_std]
#![feature(alloc)]

extern crate alloc;
extern crate tock;

use alloc::string::String;
use tock::console::Console;
use tock::led;
use tock::timer;

fn main() {
    let mut console = Console::new();

    // Timer initialization
    let mut with_callback = timer::with_callback(|_, _| {});
    let timer = with_callback.init().unwrap();

    // Blink the LEDs in a binary count pattern and scale
    // to the number of LEDs on the board.
    let led = led::get(0).unwrap();

    for _x in 0..100 {
        let start_time = timer.get_current_clock().num_ticks();

        for _y in 0..10 {
            led.on();
            // led.off();
        }

        let end_time = timer.get_current_clock().num_ticks();

        //output performance information
        let elapsed_time = (end_time - start_time) as u32;
        console.write(String::from("total num ticks: "));
        console.write(tock::fmt::u32_as_decimal(elapsed_time));
        console.write(String::from("\n"));
    }
}
