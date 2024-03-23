#![no_std]
#![no_main]
mod utils;
use esp32c3_hal::{clock::ClockControl, peripherals::Peripherals, prelude::*, Delay, IO, rtc_cntl::Rtc};
use esp_backtrace as _;
use esp_println::println;
use max7219::*;
use utils::*;





#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let clk = io.pins.gpio10.into_push_pull_output();
    let cs = io.pins.gpio3.into_push_pull_output();
    let din = io.pins.gpio2.into_push_pull_output();
    let rtc: Rtc<'_> = Rtc::new(peripherals.RTC_CNTL);
    let random = Random::new(&rtc);
    let clocks = ClockControl::max(system.clock_control).freeze();
    let mut delay = Delay::new(&clocks);
    let mut display = MAX7219::from_pins(1, din, cs, clk).unwrap();

    // make sure to wake the display up
    display.power_on().unwrap();
    // write given octet of ASCII characters with dots specified by 3rd param bits
    // display.write_str(0, b"00000000", 0b00000000).unwrap();
    // display.write_str(1, b"pulafoul", 0b00000000).unwrap(); // set display intensity lower
    display.set_intensity(0, 0x01).unwrap();
    // let mut data = [0x01; 8];
    let mut matrix: [[Bit; 8]; 8] = [[Bit::Low; 8]; 8];
    // matrix[3][3] = Bit::High;
    // matrix[0][7] = Bit::High;
    let mut data = to_hex(&matrix);

    loop {
        let emtpy_count  =find_emtpy_count(&matrix);
        let (i,j) = if emtpy_count <= 15 {find_emtpy_offset(&matrix)} else {(random.get_rand(8) as usize,random.get_rand(8) as usize)};
        println!("{}-{}",i,j);
        // let buffer = if data[0] == 0b10000000 {
        //     0b00000001
        // } else {
        //     data[0] << 1
        // };
        matrix[i][j] = Bit::High;
        data=to_hex(&matrix);
        display.write_raw(0b00000000, &data).unwrap();

        delay.delay_ms(50u32);
    }
}
