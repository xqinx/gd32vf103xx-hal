#![deny(unsafe_code)]
#![no_main]
#![no_std]

extern crate panic_halt;

use riscv_rt::entry;
use gd32vf103xx_hal::{pac, prelude::*};

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    let mut rcu = dp.RCU.configure().ext_hf_clock(8_u32.mhz()).
        sysclk(108_u32.mhz()).freeze();

    // Acquire the GPIOA peripheral. This also enables the clock for GPIOA in
    // the RCC register.
    let gpioa = dp.GPIOA.split(&mut rcu);

    // Configure PA1 as output.
    let mut led = gpioa.pa1.into_push_pull_output();

    loop {
        // Set the LED high one million times in a row.
        for _ in 0..1_000_000 {
            led.set_high().unwrap();
        }

        // Set the LED low one million times in a row.
        for _ in 0..1_000_000 {
            led.set_low().unwrap();
        }
    }
}
