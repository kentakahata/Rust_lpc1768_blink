#![no_std]
#![no_main]


// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

/*
    para compilar para hexa utilizar o "cargo build --release" e "cargo objcopy --release -- -O ihex firmware.hex"

*/

use cortex_m::asm;
use cortex_m_rt::entry;
use lpc1769:: Peripherals;

#[entry]
fn main() -> ! {
    
    let p = Peripherals::take().unwrap();

    // Configura o pino 25 como saída
    p.GPIO.dir3.write(|w| w.pindir25().set_bit());

    loop {
        p.GPIO.set3.write(|w| w.pinset25().set_bit());
        asm::delay(200_000);
        p.GPIO.clr3.write(|w| w.pinclr25().set_bit());
        asm::delay(200_000);
    }

}
