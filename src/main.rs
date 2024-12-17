#![no_std]
#![no_main]
use panic_halt as _;
use cortex_m_rt as _;

use rp_pico::entry;
use rp_pico::hal::pac;
use rp_pico::hal;

use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin;

const XTAL_FREQ_HZ : u32 = 12_000_000u32;
const DELAY : u32 = 1000;

#[entry]
fn rusty_main() -> ! {

  // grab the peripheral access object
  let mut pac = pac::Peripherals::take().unwrap();

  // set up watchdog driver
  let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

  // configure the clocks
  let clocks = hal::clocks::init_clocks_and_plls(
    XTAL_FREQ_HZ,
    pac.XOSC,
    pac.CLOCKS,
    pac.PLL_SYS,
    pac.PLL_USB,
    &mut pac.RESETS,
    &mut watchdog
  ).unwrap();

  // set up the timer
  let mut timer = hal::Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);
  
  // gpio pin control
  let sio = hal::Sio::new(pac.SIO);

  // set pin bank to default
  let pins = hal::gpio::Pins::new(
    pac.IO_BANK0, pac.PADS_BANK0, sio.gpio_bank0, &mut pac.RESETS
  );

  let mut led = pins.gpio14 . into_push_pull_output();
  loop {
    led.set_high() 
      . unwrap();
    timer.delay_ms(DELAY);
    led.set_low() 
      . unwrap();
    timer.delay_ms(DELAY);
  }
}

