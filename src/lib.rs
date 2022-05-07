//! Driver for Sensirion SHT3x-DIS digital temperature/humidity sensors

#![no_std]

extern crate byteorder;
extern crate embedded_hal;

use byteorder::{ByteOrder, BigEndian};

use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::blocking::i2c::{Read, Write, WriteRead};

pub struct SHT3x<I2C> {
    i2c: I2C,
    address: Address,
}

impl<I2C, E> SHT3x<I2C>
where
    I2C: Read<Error = E> + Write<Error = E> + WriteRead<Error = E>,
{
	/// Creates a new driver
    pub fn new(i2c: I2C, address: Address) -> Self {
        SHT3x { i2c, address }
    }

	/// Send an I2C command
    fn command(&mut self, command: Command) -> Result<(), Error<E>> {
        let mut cmd_bytes = [0; 2];
        BigEndian::write_u16(&mut cmd_bytes, command.value());
        self.i2c
            .write(self.address as u8, &cmd_bytes)
            .map_err(Error::I2c)
    }

	/// Take a temperature and humidity measurement
    pub fn measure<D: DelayMs<u8>>(&mut self, rpt: Repeatability, delay: &mut D) -> Result<Measurement, Error<E>> {
        self.command(Command::SingleShot(ClockStretch::Disabled, rpt))?;
        delay.delay_ms(rpt.max_duration());
        let mut buf = [0; 6];
        self.i2c.read(self.address as u8, &mut buf)
                .map_err(Error::I2c)?;

        // Check crc
        let temperature_crc = buf[2];
        let humidity_crc = buf[5];
        let temperature_calculated_crc = crc8(&buf[0..2]);
        let humidity_calculated_crc = crc8(&buf[3..5]);
        if temperature_crc != temperature_calculated_crc {
            return Err(Error::Crc)
        }
        if humidity_crc != humidity_calculated_crc {
            return Err(Error::Crc)
        }

        let temperature = convert_temperature(BigEndian::read_u16(&buf[0..2]));
        let humidity = convert_humidity(BigEndian::read_u16(&buf[3..5]));
        Ok(Measurement{ temperature, humidity })
    }

	/// Read the status register
    pub fn status(&mut self) -> Result<u16, Error<E>> {
        self.command(Command::Status)?;
        let mut status_bytes = [0; 2];
        self.i2c
            .read(self.address as u8, &mut status_bytes)
            .map_err(Error::I2c)?;
        Ok(BigEndian::read_u16(&status_bytes))
    }
}

fn convert_temperature(raw: u16) -> i32 {
    -4500 + (17500 * raw as i32) / 65535
}

fn convert_humidity(raw: u16) -> i32 {
    (10000 * raw as i32) / 65535
}

fn crc8(data: &[u8]) -> u8 {
    let mut crc: u8 = 0xff;
    for byte in data {
        crc ^= byte;
        for _ in 0..8 {
            if (crc & 0x80) > 0 {
                crc = (crc << 1) ^ 0x31;
            } else {
                crc <<= 1;
            }
        }
    }
    crc
}

/// Errors
#[derive(Debug)]
pub enum Error<E> {
    /// Wrong CRC
    Crc,
    /// I2C bus error
    I2c(E),
}

/// I2C address
#[derive(Copy, Clone)]
pub enum Address {
	/// Address pin held high
    High = 0x45,
	/// Address pin held low
    Low = 0x44,
}

#[allow(unused)]
enum Command {
    SingleShot(ClockStretch, Repeatability),
    Periodic(Rate, Repeatability),
    FetchData,
    PeriodicWithART,
    Break,
    SoftReset,
    HeaterEnable,
    HeaterDisable,
    Status,
    ClearStatus,
}

#[allow(unused)]
enum ClockStretch {
    Enabled,
    Disabled,
}

/// Periodic data acquisition rate
#[allow(non_camel_case_types, unused)]
enum Rate {
	/// 0.5 measurements per second
    R0_5,
	/// 1 measurement per second
    R1,
	/// 2 measurements per second
    R2,
	/// 4 measurements per second
    R4,
	/// 10 measurements per second
    R10,
}

#[derive(Copy, Clone)]
pub enum Repeatability {
    High,
    Medium,
    Low,
}

impl Repeatability {
    /// Maximum measurement duration in milliseconds
    fn max_duration(&self) -> u8 {
        match *self {
            Repeatability::Low => 4,
            Repeatability::Medium => 6,
            Repeatability::High => 15,
        }
    }
}

#[derive(Debug)]
pub struct Measurement {
    pub temperature: i32,
    pub humidity: i32,
}

impl Command {
    fn value(&self) -> u16 {
        use ClockStretch::Enabled as CSEnabled;
        use ClockStretch::Disabled as CSDisabled;
        use Rate::*;
        use Repeatability::*;
        match *self {
            // 4.3 Measurement Commands for Single Shot Data Acquisition Mode
            // Table 8
            Command::SingleShot(CSEnabled,  High)   => 0x2C06,
            Command::SingleShot(CSEnabled,  Medium) => 0x2C0D,
            Command::SingleShot(CSEnabled,  Low)    => 0x2C10,
            Command::SingleShot(CSDisabled, High)   => 0x2400,
            Command::SingleShot(CSDisabled, Medium) => 0x240B,
            Command::SingleShot(CSDisabled, Low)    => 0x2416,

            // 4.5 Measurement Commands for Periodic Data Acquisition Mode
            // Table 9
            Command::Periodic(R0_5, High)   => 0x2032,
            Command::Periodic(R0_5, Medium) => 0x2024,
            Command::Periodic(R0_5, Low)    => 0x202F,
            Command::Periodic(R1,   High)   => 0x2130,
            Command::Periodic(R1,   Medium) => 0x2126,
            Command::Periodic(R1,   Low)    => 0x212D,
            Command::Periodic(R2,   High)   => 0x2236,
            Command::Periodic(R2,   Medium) => 0x2220,
            Command::Periodic(R2,   Low)    => 0x222B,
            Command::Periodic(R4,   High)   => 0x2334,
            Command::Periodic(R4,   Medium) => 0x2322,
            Command::Periodic(R4,   Low)    => 0x2329,
            Command::Periodic(R10,  High)   => 0x2737,
            Command::Periodic(R10,  Medium) => 0x2721,
            Command::Periodic(R10,  Low)    => 0x272A,

            // 4.6 Readout of Measurement Results for Periodic Mode
            // Table 10
            Command::FetchData => 0xE000,

            // 4.7 ART command
            // Table 11
            Command::PeriodicWithART => 0x2B32,

            // 4.8 Break command
            // Table 12
            Command::Break => 0x3093,

            // 4.9 Reset
            // Table 13
            Command::SoftReset => 0x30A2,

            // 4.10 Heater
            // Table 15
            Command::HeaterEnable  => 0x306D,
            Command::HeaterDisable => 0x3066,

            // 4.11 Status register
            // Table 16
            Command::Status => 0xF32D,
            // Table 18
            Command::ClearStatus => 0x3041,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
