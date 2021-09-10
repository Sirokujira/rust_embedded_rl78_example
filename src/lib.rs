#![no_std]
#![feature(asm)]
#![feature(core_intrinsics)]
use core::intrinsics::volatile_store;

// not available Atomic
use core::panic::PanicInfo;
#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}


extern crate rl78hal;
use rl78hal::*;



// pub extern "C" fn wait_() {
// asm("nop");
// }

#[no_mangle]
// pub extern "C" fn rust_main(_argc: isize, _argv: *const *const u8) -> isize {
pub extern "C" fn rust_main() -> isize {
    // utils::port::pullup_all();

    // rl78hal::INTFL;
    // rl78hal::device_PM4.B3 = [0u8; 0];  // output

    // Address is iodefine.h
    // usize : 16bit
    // const led_pin_pt1: *mut u32 = (0xFFF27) as *mut u32;
    const led_pin_pt1: *mut u32 = (0xFF27) as *mut u32;
    const led_pt: *mut u32 = (0xFF07) as *mut u32;
    const sw_pt: *mut u32 = (0xFF0D) as *mut u32;
    unsafe {
      // un_pm7.un_pm7__bindgen_ty_1.bit6 = 0;
      // un_pm7.un_pm7__bindgen_ty_1.bit7 = 0;

      // use core?
      // ::core::ptr::write_volatile(led_pin_pt1, ::core::ptr::read_volatile(led_pin_pt1) | 0x00);
      volatile_store(led_pin_pt1, *led_pin_pt1 | 0<<6);
      volatile_store(led_pin_pt1, *led_pin_pt1 | 0<<7);
      loop {
        // inline assembly is unsupported on this target
        // for _ in 1..400000 {
        //     unsafe { asm!("nop"); }
        // }
        // let sw = P13.BIT.bit7;
        let sw = ::core::ptr::read_volatile(sw_pt);
        if (sw == 1)
        {
            // P7.BIT.bit6 = 1;
            // P7.BIT.bit7 = 0;
            // ::core::ptr::write_volatile(led_pt, ::core::ptr::read_volatile(led_pt) | 0x40);
            volatile_store(led_pt, *led_pt | 1<<6);
            volatile_store(led_pt, *led_pt | 0<<7);
        }
        else
        {
            // P7.BIT.bit6 = 0;
            // P7.BIT.bit7 = 1;
            // ::core::ptr::write_volatile(led_pt, ::core::ptr::read_volatile(led_pt) | 0x80);
            volatile_store(led_pt, *led_pt | 0<<6);
            volatile_store(led_pt, *led_pt | 1<<7);
        }
      }
    }
    0
}
// pub extern "C" fn rust_main() { /* ... */ }
// pub extern "C" fn rust_main() -> () {
//     loop {
//       unsafe {
//         HAL_GPIO_TogglePin(GPIOB, GPIO_PIN_7);
//         HAL_Delay(200);
//         HAL_GPIO_TogglePin(GPIOB, GPIO_PIN_6);
//         HAL_Delay(200);
//       }
//     }
// }

// available Atomic Instruction
// use panic_halt as _;

// use wio_terminal as wio;
// use wio::hal::clock::GenericClockController;
// use wio::hal::delay::Delay;
// use wio::pac::{CorePeripherals, Peripherals};
// use wio::prelude::*;
// use wio::{entry, Pins, Sets};
// cortex-m-rt
// #[entry]
// fn main() -> ! {
//     // let mut peripherals = Peripherals::take().unwrap();
//     // let core = CorePeripherals::take().unwrap();
// 
//     // let mut clocks = GenericClockController::with_external_32kosc(
//     //     peripherals.GCLK,
//     //     &mut peripherals.MCLK,
//     //     &mut peripherals.OSC32KCTRL,
//     //     &mut peripherals.OSCCTRL,
//     //     &mut peripherals.NVMCTRL,
//     // );
//     // let mut delay = Delay::new(core.SYST, &mut clocks);
// 
//     // let mut sets: Sets = Pins::new(peripherals.PORT).split();
//     // let mut user_led = sets.user_led.into_push_pull_output(&mut sets.port);
// 
//     loop {
//         //user_led.toggle();
//         //delay.delay_ms(500u16);
//     }
// }

// #[no_mangle]
// pub extern "C" fn rust_init() {
// }

