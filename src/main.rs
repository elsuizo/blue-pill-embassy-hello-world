#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{AnyPin, Level, Output, Speed};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::task]
pub async fn led_task(delays: [u64; 2], pin: AnyPin) {
    info!("Hola desde la tarea led_task");
    let mut led = Output::new(pin, Level::High, Speed::Low);
    loop {
        info!("high");
        led.set_high();
        Timer::after_millis(delays[0]).await;

        info!("low");
        led.set_low();
        Timer::after_millis(delays[1]).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Hello World!");

    spawner.must_spawn(led_task([1000, 300], p.PC13.into()));
}
