#![no_std]
#![feature(alloc)]

extern crate alloc;
extern crate corepack;
extern crate tock;

use alloc::string::String;
use tock::console::Console;
use tock::ble_composer;
use tock::simple_ble;
use tock::timer;

#[allow(unused_variables)]
fn main() {
    let mut console = Console::new();

    //timer initialization
    let mut with_callback = timer::with_callback(|_, _| {});
    let timer = with_callback.init().unwrap();

    // Test code
    for _x in 0..100 {
        let start_time = timer.get_current_clock().num_ticks();

        for _y in 0..100 {
            //data that is to be transferred
            let name = String::from("Tock!");
            let uuid: [u8; 2] = [0x00, 0x18]; //esp. what this is
            let str_payload = "tA16GBYuu0fXYoZBdbxgS9oHM4zTCwf7m64nBTYoi5iuglOTozzNUhZDhlNXQLQxvon8JmYjuqiSLNLNyPGqhWpAU4Hfj1bLXjcntA16GBYuu0fXYoZBdbxgS9oHM4zTCwf7m64nBTYoi5iuglOTozzNUhZDhlNXQLQxvon8JmYjuqiSLNLNyPGqhWpAU4Hfj1bLXjcn";
            let payload = corepack::to_bytes(str_payload).unwrap();

            let mut buffer = simple_ble::BleAdvertisingDriver::create_advertising_buffer();
            let mut gap_payload = ble_composer::BlePayload::new();

            //transfer data via bluetooth
            gap_payload.add_flag(ble_composer::flags::LE_GENERAL_DISCOVERABLE);
            gap_payload.add(ble_composer::gap_types::UUID, &uuid);
            gap_payload.add(
                ble_composer::gap_types::COMPLETE_LOCAL_NAME,
                name.as_bytes(),
            );
            gap_payload.add_service_payload([91, 79], &payload);

            let handle =
                simple_ble::BleAdvertisingDriver::initialize(100, &gap_payload, &mut buffer).unwrap();
        }

        let end_time = timer.get_current_clock().num_ticks();

        //output performance information
        let elapsed_time = (end_time - start_time) as u32;
        console.write(String::from("total num ticks: "));
        console.write(tock::fmt::u32_as_decimal(elapsed_time));
        console.write(String::from("\n"));
    }

    loop {}
}
