//! From thejpster's example
//! https://github.com/rust-embedded-community/menu/blob/master/examples/simple.rs

#![deny(unsafe_code)]
#![no_main]
#![no_std]

use menu::*;
use core::fmt::Write;
use nb::block;
extern crate panic_halt;

use riscv_rt::entry;
use gd32vf103xx_hal::{pac, prelude::*, serial::*};
use crate::pac::USART0;

const ROOT_MENU: Menu<Output> = Menu {
    label: "root",
    items: &[
        &Item {
            item_type: ItemType::Callback {
                function: select_foo,
                parameters: &[
                    Parameter::Mandatory {
                        parameter_name: "a",
                        help: Some("This is the help text for 'a'"),
                    },
                    Parameter::Optional {
                        parameter_name: "b",
                        help: None,
                    },
                    Parameter::Named {
                        parameter_name: "verbose",
                        help: None,
                    },
                    Parameter::NamedValue {
                        parameter_name: "level",
                        argument_name: "INT",
                        help: Some("Set the level of the dangle"),
                    },
                ],
            },
            command: "foo",
            help: Some(
                "Makes a foo appear.

This is some extensive help text.

It contains multiple paragraphs and should be preceeded by the parameter list.
",
            ),
        },
        &Item {
            item_type: ItemType::Callback {
                function: select_bar,
                parameters: &[],
            },
            command: "bar",
            help: Some("fandoggles a bar"),
        },
        &Item {
            item_type: ItemType::Menu(&Menu {
                label: "sub",
                items: &[
                    &Item {
                        item_type: ItemType::Callback {
                            function: select_baz,
                            parameters: &[],
                        },
                        command: "baz",
                        help: Some("thingamobob a baz"),
                    },
                    &Item {
                        item_type: ItemType::Callback {
                            function: select_quux,
                            parameters: &[],
                        },
                        command: "quux",
                        help: Some("maximum quux"),
                    },
                ],
                entry: Some(enter_sub),
                exit: Some(exit_sub),
            }),
            command: "sub",
            help: Some("enter sub-menu"),
        },
    ],
    entry: Some(enter_root),
    exit: Some(exit_root),
};

struct Output(Tx<USART0>);

impl core::fmt::Write for Output
{
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.0.write_str(s)
    }
}

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    let mut rcu = dp.RCU.configure().ext_hf_clock(8_u32.mhz()).
        sysclk(108_u32.mhz()).freeze();

    // Acquire the GPIOA peripheral. This also enables the clock for GPIOA in
    // the RCC register.
    let gpioa = dp.GPIOA.split(&mut rcu);

    let pin_tx = gpioa.pa9;
    let pin_rx = gpioa.pa10;

    let mut afio = dp.AFIO.constrain(&mut rcu);

    let serial = Serial::new(
        dp.USART0,
        (pin_tx, pin_rx),
        Config::default().baudrate(9_600.bps()),
        &mut afio,
        &mut rcu,
    );

    let (tx, mut rx) = serial.split();

    let mut buffer = [0u8; 64];
    let mut r = Runner::new(&ROOT_MENU, &mut buffer, Output(tx));

    loop {
        match block!(rx.read()) {
            Ok(b) => {
                r.input_byte(b);
            }
            Err(_e) => (),
        }
    }
}

fn enter_root(_menu: &Menu<Output>, context: &mut Output) {
    writeln!(context, "In enter_root").unwrap();
}

fn exit_root(_menu: &Menu<Output>, context: &mut Output) {
    writeln!(context, "In exit_root").unwrap();
}

fn select_foo<'a>(_menu: &Menu<Output>, item: &Item<Output>, args: &[&str], context: &mut Output) {
    writeln!(context, "In select_foo. Args = {:?}", args).unwrap();
    writeln!(
        context,
        "a = {:?}",
        ::menu::argument_finder(item, args, "a")
    )
    .unwrap();
    writeln!(
        context,
        "b = {:?}",
        ::menu::argument_finder(item, args, "b")
    )
    .unwrap();
    writeln!(
        context,
        "verbose = {:?}",
        ::menu::argument_finder(item, args, "verbose")
    )
    .unwrap();
    writeln!(
        context,
        "level = {:?}",
        ::menu::argument_finder(item, args, "level")
    )
    .unwrap();
    writeln!(
        context,
        "no_such_arg = {:?}",
        ::menu::argument_finder(item, args, "no_such_arg")
    )
    .unwrap();
}

fn select_bar<'a>(_menu: &Menu<Output>, _item: &Item<Output>, args: &[&str], context: &mut Output) {
    writeln!(context, "In select_bar. Args = {:?}", args).unwrap();
}

fn enter_sub(_menu: &Menu<Output>, context: &mut Output) {
    writeln!(context, "In enter_sub").unwrap();
}

fn exit_sub(_menu: &Menu<Output>, context: &mut Output) {
    writeln!(context, "In exit_sub").unwrap();
}

fn select_baz<'a>(_menu: &Menu<Output>, _item: &Item<Output>, args: &[&str], context: &mut Output) {
    writeln!(context, "In select_baz: Args = {:?}", args).unwrap();
}

fn select_quux<'a>(
    _menu: &Menu<Output>,
    _item: &Item<Output>,
    args: &[&str],
    context: &mut Output,
) {
    writeln!(context, "In select_quux: Args = {:?}", args).unwrap();
}
