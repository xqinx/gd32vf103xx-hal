#![no_main]
#![no_std]

extern crate panic_halt;

use riscv_rt::entry;
use gd32vf103xx_hal::{pac::{self, TIMER0, TIMER6},
                        prelude::*,
                        delay::{Delay, McycleDelay},
                        timer::Timer,
                     };

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
    //let mut delay = McycleDelay::new(&rcu.clocks);
    //FIXME: improve ergonomics
    //FIXME: I would like to create sub 1 Hz delays
    let mut delay = Delay::<TIMER0>::new(Timer::timer0(dp.TIMER0,1_u32.hz(),&mut rcu));
    let mut delay6 = Delay::<TIMER6>::new(Timer::timer6(dp.TIMER6,1_u32.hz(),&mut rcu));

    led.set_low().unwrap();
    delay.delay_ms(900_u32);

    led.set_high().unwrap();
    delay6.delay_ms(1000_u32);

    led.set_low().unwrap();
    delay.delay_ms(1000_u32);

    led.set_high().unwrap();
    delay.delay_ms(1000_u32);

    loop {
    }
}
