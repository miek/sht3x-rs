//! Driver for Sensirion SHT3x-DIS digital temperature/humidity sensors

#![no_std]

extern crate embedded_hal;

use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::blocking::i2c::{Read, Write, WriteRead};

pub struct SHT3x<I2C, D> {
    i2c: I2C,
    delay: D,
}

impl<I2C, D, E> SHT3x<I2C, D>
where
    I2C: Read<Error = E> + Write<Error = E> + WriteRead<Error = E>,
    D: DelayMs<u8>,
{

}

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

enum ClockStretch {
    Enabled,
    Disabled,
}

#[allow(non_camel_case_types)]
enum Rate {
    R0_5,
    R1,
    R2,
    R4,
    R10,
}

enum Repeatability {
    High,
    Medium,
    Low,
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
