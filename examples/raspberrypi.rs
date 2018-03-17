extern crate linux_embedded_hal as hal;
extern crate sht3x;

use hal::{Delay, I2cdev};
use sht3x::{SHT3x, Address};

fn main() {
    println!("Hello, SHT31!");

    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let mut sht31 = SHT3x::new(dev, Delay, Address::Low);

    println!("Status raw: {:?}", sht31.status().unwrap());
    loop {
        let m = sht31.measure().unwrap();
        println!("Temp: {:.2} Humidity: {:.2}", m.temperature, m.humidity);
    }
}
