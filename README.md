# [SHT3x](https://crates.io/crates/sht3x)

A platform agnostic driver to interface with the Sensirion SHT3x-DIS temperature/humidity sensor via I2C.

This driver was built using [`embedded-hal`] traits.

[`embedded-hal`]: https://docs.rs/embedded-hal/

## Documentation
 Read the detailed documentation [here](https://docs.rs/sht3x/)

## What works

- Take a temperature & humidity measurement
- Read the status register
- Reset command

## TODO

- [ ] Implement ALERT functionality
- [ ] Implement periodic measurements
- [ ] Heater command
- [ ] Add an option to use the hardware reset pin

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Resources

The following resources were consulted when making this driver:
- https://sensirion.com/media/documents/213E6A3B/61641DC3/Sensirion_Humidity_Sensors_SHT3x_Datasheet_digital.pdf
