#![no_std]
#![no_main]
use core::ptr::write_volatile;

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m::asm::nop;
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {

    const GPIO3_OUT: *mut u32 = 0x2009_C063 as *mut u32;
    const GPIO3_PIN25_ADRS: u32 = 1<<1;

    unsafe{
        write_volatile(GPIO3_OUT, GPIO3_PIN25_ADRS);
    }

    const GPIO3_SET: *mut u32 = 0x2009_C07B as *mut u32;
    const GPIO3_CLR: *mut u32 = 0x2009_C07F as *mut u32;
    let mut is_on = false;
    loop{
        unsafe{
            if  is_on{
                write_volatile(GPIO3_CLR, GPIO3_PIN25_ADRS); 
            }
            else{
                write_volatile(GPIO3_SET, GPIO3_PIN25_ADRS); 
            }
            for _ in 0..400_000{
                nop();
            }
            is_on = !is_on;

        }

    }

}
