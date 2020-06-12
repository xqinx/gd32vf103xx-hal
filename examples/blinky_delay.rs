#![no_main]
#![no_std]

extern crate panic_halt;

use riscv_rt::entry;
use gd32vf103xx_hal::{pac, prelude::*, delay::McycleDelay};

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    let mut rcu = dp.RCU.configure().sysclk(8_u32.mhz()).freeze();

    // Acquire the GPIOA peripheral. This also enables the clock for GPIOA in
    // the RCC register.
    let gpioa = dp.GPIOA.split(&mut rcu);

    // Configure PA1 as output.
    let mut led = gpioa.pa1.into_push_pull_output();

    // Get the delay provider.
    let mut delay = McycleDelay::new(&rcu.clocks);

    loop {
        led.set_high().unwrap();
        delay.delay_ms(1000_u16);

        led.set_low().unwrap();
        delay.delay_ms(1000_u16);
    }
}
