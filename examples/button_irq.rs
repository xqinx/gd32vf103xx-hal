#![no_main]
#![no_std]

extern crate panic_halt;

use core::cell::RefCell;
use core::ops::DerefMut;

//use riscv::interrupt::Mutex;
use riscv_rt::entry;
use gd32vf103xx_hal::{pac::{self, interrupt::*},
                      prelude::*,
                      eclic::{EclicExt, TriggerType, LevelPriorityBits, Level},
                      exti::{TriggerEdge, Exti, ExtiLine, InternalLine, ExtiEvent},
                      gpio::*};

// crate riscv is not using bare-metal 0.2.5 nor does it specify const_fn feature
// the following Mutex init for static var won't work. Use a workaround
// below (Wrapper)
//static LED: Mutex<RefCell<Option<gpioa::PA1<Output<PushPull>>>>> = Mutex::new(RefCell::new(None));

struct Wrapper(RefCell<Option<gpioa::PA1<Output<PushPull>>>>);
unsafe impl Sync for Wrapper {}
impl Wrapper {
    pub const fn new() -> Self {
        Wrapper(RefCell::new(None))
    }
}

static LED:Wrapper = Wrapper::new();

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    let mut rcu = dp.RCU.configure().sysclk(8_u32.mhz()).freeze();

    // Acquire the GPIOA peripheral. This also enables the clock for GPIOA in
    // the RCU register.
    let gpioa = dp.GPIOA.split(&mut rcu);

    // Configure PA1 as output.
    let led = gpioa.pa1.into_push_pull_output();

    // Configure PB9 pin as input
    let button = dp.GPIOB.split(&mut rcu).pb9.into_pull_up_input();

    // configure AFIO to set the EXTI source
    let mut afio = dp.AFIO.constrain(&mut rcu);
    afio.extiss(button.port(), button.pin_number());

    // configure exti for exti line
    let mut exti = Exti::new(dp.EXTI);
    let extiline = ExtiLine::from_gpio_line(button.pin_number()).unwrap();
    exti.listen(extiline, TriggerEdge::Falling);

    // test event enable
    exti.gen_event(extiline, ExtiEvent::Enable);

    // test lvd interrupt
    //exti.listen(ExtiLine::from_internal_line(InternalLine::Lvd), TriggerEdge::Falling);

    // store LED in the wrapper
    riscv::interrupt::free(|_cs| {
        *LED.0.borrow_mut() = Some(led);
    });

    // configure eclic to enable interrupt
    // we need to setup nlbits, its default value is 0
    pac::ECLIC::set_level_priority_bits(LevelPriorityBits::L3P1);
    pac::ECLIC::set_level(Interrupt::EXTI_LINE9_5,Level::L1);
    pac::ECLIC::set_trigger_type(Interrupt::EXTI_LINE9_5,TriggerType::FallingEdge);
    // Enable interupt
    unsafe { pac::ECLIC::unmask(Interrupt::EXTI_LINE9_5); }

    // global interrupt enable
    unsafe { riscv::register::mstatus::set_mie(); }

    loop {
    }
}

//TODO: use `rt` feature from pac, which provides macro interrupt!
#[export_name = "EXTI_LINE9_5"]
#[allow(non_snake_case)]
pub unsafe extern "C" fn EXTI_LINE9_5() {
    static mut STATE: bool = false;

    riscv::interrupt::free(|_cs| {
        if Exti::is_pending(ExtiLine::from_gpio_line(9).unwrap()) {
            Exti::clear(ExtiLine::from_gpio_line(9).unwrap());
        }

        if let Some(ref mut led) = LED.0.borrow_mut().deref_mut() {
            if STATE {
                led.set_low().unwrap();
                STATE = false;
            } else {
                led.set_high().unwrap();
                STATE = true;
            }
        }
    });
}
