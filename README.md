# ts100

> ts100 soldering iron firmware written in Rust

## Hardware

- STM32F103T8U6
  - [Datasheet](http://www.st.com/content/ccc/resource/technical/document/datasheet/33/d4/6f/1d/df/0b/4c/6d/CD00161566.pdf/files/CD00161566.pdf/jcr:content/translations/en.CD00161566.pdf)
- OLED M00881
  - SSD1306 controller
  - I²C address: 0x3c
- Accelerometer MMA8652FC
  - [Datasheet](http://cache.freescale.com/files/sensors/doc/data_sheet/MMA8652FC.pdf)
  - I²C address: 0x1d


![Schematics](/doc/schematics.png)


## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
