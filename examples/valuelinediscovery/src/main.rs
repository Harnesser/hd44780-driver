//! Run LCD from VALUELINEDISCOVERY board (STM32F100)
//!
//! Uses:
//!  * https://github.com/JohnDoneth/hd44780-driver
//!  * https://github.com/stm32-rs/stm32f1xx-hal
//!
//! Connections:
//!
//! LCD       Pin     Direction   Function
//! ------------------------------------------------------------------------
//! 1  Vss    
//! 2  Vcc    
//! 3  Vee
//! 4  RS     PC5     Output      H: Data Register L: Instruction Register
//! 5  R/Wb   PC4     Output      H: Read L: Write (mostly write)
//! 6  E      PC13    Output      Enable signal (falling edge)
//! 7  DB0    -                   Not connected
//! 8  DB1    -                   Not connected
//! 9  DB2    -                   Not connected
//! 10 DB3    -                   Not connected
//! 11 DB4    PC0     Output      Data line
//! 12 DB5    PC1     Output      Data line
//! 13 DB6    PC2     Output      Data line
//! 14 DB7    PC3     Output      Data line (MSB)
//! 15 LED+
//! 16 LED-
//!
//! This code also lights the green LED on PC9 for the duration of the
//! LCD setup routine

#![no_std]
#![no_main]

extern crate panic_halt;
extern crate stm32f1xx_hal as hal;

use core::fmt::Write;

use hal::prelude::*;
use hal::stm32;
//use hal::gpio::GpioExt;
//use hal::flash::FlashExt;
//use hal::rcc::RccExt;

use cortex_m_rt::entry;
use hd44780_driver::{Cursor, CursorBlink, Display, DisplayMode, HD44780};

#[entry]
fn main() -> ! {

    // Grab ... Cortex M
    let cp = cortex_m::Peripherals::take().unwrap();

    // Grab device peripherals
    let dp = stm32::Peripherals::take().unwrap();

    let mut rcc = dp.RCC.constrain();
    let mut flash = dp.FLASH.constrain();
    let mut gpioc = dp.GPIOC.split(&mut rcc.apb2);

    // set up delay thingy
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let delay = hal::delay::Delay::new(cp.SYST, clocks);

    // setup indication
    let mut green_led = gpioc.pc9.into_push_pull_output(&mut gpioc.crh);
    green_led.set_high();

    // Configure pins
    let rs = gpioc.pc5.into_push_pull_output(&mut gpioc.crl);
    let en = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);
    let b4 = gpioc.pc0.into_push_pull_output(&mut gpioc.crl);
    let b5 = gpioc.pc1.into_push_pull_output(&mut gpioc.crl);
    let b6 = gpioc.pc2.into_push_pull_output(&mut gpioc.crl);
    let b7 = gpioc.pc3.into_push_pull_output(&mut gpioc.crl);

    // Write. Always.
    let rw = gpioc.pc4.into_push_pull_output(&mut gpioc.crl).set_low();

    let mut lcd = HD44780::new_4bit(rs, en, b4, b5, b6, b7, delay);
    lcd.reset();
    lcd.clear();
    lcd.set_display_mode(
        DisplayMode {
            display: Display::On,
            cursor_visibility: Cursor::Visible,
            cursor_blink: CursorBlink::On,
        }
    );
    green_led.set_low();
    lcd.write_str("Hello, world!").unwrap();

    loop {}
}
