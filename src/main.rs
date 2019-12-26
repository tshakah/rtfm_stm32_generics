#![deny(unsafe_code)]
#![feature(crate_visibility_modifier)]
#![no_std]
#![no_main]

extern crate panic_semihosting;

use stm32f1xx_hal::{prelude::*, gpio};
use embedded_hal::digital::v2::OutputPin;

type Pins = [gpio::Pxx<gpio::Output<gpio::PushPull>>; 2];

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

        // Construct the array
        let pins : Pins = [
            gpioa.pa8.into_push_pull_output(&mut gpioa.crh).downgrade(),
            gpioa.pa9.into_push_pull_output(&mut gpioa.crh).downgrade(),
        ];

        // Initialise the resources and return
        init::LateResources { pins: pins }
    }

    #[idle(resources = [pins])]
    fn idle(c: idle::Context) -> ! {
        // Set pins high
        for pin in c.resources.pins.iter_mut() {
            pin.set_high().unwrap();
        }

        loop {
            // Do other things
        }
    }
};
