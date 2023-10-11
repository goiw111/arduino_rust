#![no_std]
#![no_main]

use panic_halt as _;
use arduino_hal::port::mode::Output;
use arduino_hal::hal::port;
use arduino_hal::hal::port::Pin;
use arduino_hal::prelude::*;

struct Counter {
    counter:    u8,
    led1:       port::Pin<Output, port::PB5>,
    led2:       port::Pin<Output, port::PB4>,
    led3:       port::Pin<Output, port::PB3>,
    led4:       port::Pin<Output, port::PB2>
}

impl Counter {
    const LED1: u8 = 1;
    const LED2: u8 = Self::LED1 << 1;
    const LED3: u8 = Self::LED2 << 1;
    const LED4: u8 = Self::LED3 << 1;

    fn new(pin1: Pin<Output, port::PB5>, 
           pin2: Pin<Output, port::PB4>,
           pin3: Pin<Output, port::PB3>,
           pin4: Pin<Output, port::PB2>
           ) -> Self {
        Counter {
            counter: 0,
            led1:       pin1,
            led2:       pin2,
            led3:       pin3,
            led4:       pin4
        }
    }
}

impl Iterator for Counter {
    type Item = u8;
    fn next(&mut self) -> Option<u8> {
        if self.led1.is_set_high() ^ ((self.counter & Self::LED1) == Self::LED1) {
            self.led1.toggle();
        }
        if self.led2.is_set_high() ^ ((self.counter & Self::LED2) == Self::LED2) {
            self.led2.toggle();
        }
        if self.led3.is_set_high() ^ ((self.counter & Self::LED3) == Self::LED3) {
            self.led3.toggle();
        }
        if self.led4.is_set_high() ^ ((self.counter & Self::LED4) == Self::LED4) {
            self.led4.toggle();
        }
        let tmp = self.counter;
        if tmp < 0b0000_1111 {
            self.counter += 1;
        } else {    
            self.counter = 0;
        }
        Some(tmp)
    }
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    for n in Counter::new(
        pins.d13.into_output(),
        pins.d12.into_output(),
        pins.d11.into_output(),
        pins.d10.into_output()
    ) {
        ufmt::uwriteln!(&mut serial, "number is {}\r", n).void_unwrap();
        arduino_hal::delay_ms(2000);
    }
    loop {}
}
