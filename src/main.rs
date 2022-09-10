#![no_std]
#![no_main]

use panic_halt as _;

use riscv_rt::entry;

use esp32c3_hal::{
    clock::ClockControl,
    pac::Peripherals,
    prelude::*,
    system::SystemExt,
    timer::TimerGroup,
    Rtc,
};

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    // Disable the watchdog timers. For the ESP32-C3, this includes the Super WDT and the RTC WDT.
    let mut rtc = Rtc::new(peripherals.RTC_CNTL);
    let mut timer0 = TimerGroup::new(peripherals.TIMG0, &clocks);
    let mut timer1 = TimerGroup::new(peripherals.TIMG1, &clocks);

    timer0.wdt.disable();
    timer1.wdt.disable();
    rtc.swd.disable();
    rtc.rwdt.disable();

    loop{}
}