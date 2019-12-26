#![deny(unsafe_code)]
#![feature(crate_visibility_modifier)]
#![no_std]
#![no_main]

extern crate panic_semihosting;

use stm32f1xx_hal::prelude::*;
use embedded_hal::digital::v2::OutputPin;

type Pins = [&'static mut dyn OutputPin<Error = core::convert::Infallible>; 2];

#[rtfm::app(device = stm32f1xx_hal::device, peripherals = true)]
const APP: () = {
    struct Resources {
        pins: Pins,
    }

    #[init]
    fn init(c: init::Context) -> init::LateResources {
        // Get access to the device specific peripherals from the peripheral access crate
        let mut rcc = c.device.RCC.constrain();

        // Acquire the GPIO peripherals
        let mut gpioa = c.device.GPIOA.split(&mut rcc.apb2);

        let pins : Pins = [
            &mut gpioa.pa8.into_push_pull_output(&mut gpioa.crh),
            &mut gpioa.pa9.into_push_pull_output(&mut gpioa.crh),
        ];

        init::LateResources { pins: pins }
    }

    #[idle(resources = [pins])]
    fn idle(c: idle::Context) -> ! {
        // Initialise pins high
        for pin in c.resources.pins.iter_mut() {
            pin.set_high().unwrap();
        }

        loop {
        }
    }
};
