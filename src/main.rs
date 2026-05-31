//! Basic "hello world" blink demo for the MSP-EXP430FR5994 LaunchPad in Rust!

#![no_main]
#![no_std]

use panic_halt as _;
use msp430::asm;
use msp430_rt::entry;

#[entry]
fn main() -> ! {
    // 1. Initialize the board's peripherals
    let peripherals = msp430fr5994::Peripherals::take().unwrap();

    // 2. Disable the Watchdog Timer (WDT) immediately
    peripherals.wdt_a.wdtctl().write(|w| unsafe { w.bits(0x5A80) });

    let p1 = peripherals.p1;

    // 3. 0x01 configures Pin 0 (P1.0) as an output pin while keeping others as inputs
    p1.p1dir().write(|w| unsafe { w.bits(0x01) });

    loop {
        // 4. Set bit 0 to high (Turn LED on)
        p1.p1out().write(|w| unsafe { w.bits(0x01) });
        
        for _ in 0..10000 {
            asm::nop();
        }

        // 5. Clear bit 0 to low (Turn LED off)
        p1.p1out().write(|w| unsafe { w.bits(0x00) });
        
        for _ in 0..10000 {
            asm::nop();
        }
    }
}

