# ts100

> ts100 soldering iron firmware written in Rust

:warning: You can't solder with this firmware (yet) :warning:
Take a look at [Ralim/ts100](https://github.com/Ralim/ts100) for a feature complete alternative.

![v0.1.0](/doc/v0.1.0.png)

## Hardware

- STM32F103T8U6
  - [Datasheet (pdf)][stm32f10x datasheet]
  - [Reference Manual (huge pdf)][stm32f10x refman]
- OLED M00881
  - [Datasheet (pdf)][M00881 datasheet]
  - SSD1306Z driver IC
  - I²C address: 0x3c
- Accelerometer MMA8652FC
  - [Datasheet (pdf)][MMA8652FC datasheet]
  - I²C address: 0x1d
- TMP36GRTZ
  - [Datasheet (pdf)][ti thermocouple]
  - used for cold junction compensation for the thermocouple in the tip
  - nice application note from TI on [thermocouple and cold junction (pdf)][ti thermocouple]

[stm32f10x datasheet]: http://www.st.com/content/ccc/resource/technical/document/datasheet/33/d4/6f/1d/df/0b/4c/6d/CD00161566.pdf/files/CD00161566.pdf/jcr:content/translations/en.CD00161566.pdf
[stm32f10x refman]: http://www.st.com/content/ccc/resource/technical/document/reference_manual/59/b9/ba/7f/11/af/43/d5/CD00171190.pdf/files/CD00171190.pdf/jcr:content/translations/en.CD00171190.pdf
[M00881 datasheet]: http://www.i-excellence.com/uploads/201612/585e217f4cc6e.pdf
[MMA8652FC datasheet]: http://cache.freescale.com/files/sensors/doc/data_sheet/MMA8652FC.pdf
[TMP36GRTZ datasheet]: http://www.analog.com/media/en/technical-documentation/data-sheets/TMP35_36_37.pdf
[ti thermocouple]: http://www.ti.com/lit/an/sloa204/sloa204.pdf

### Schematics

![Schematics](/doc/schematics.png)

### PCB

![Main Board Top](/doc/main_board_top.png)

![Main Board Bottom](/doc/main_board_bottom.png)

![CPU Board Top](/doc/cpu_board_top.png)

![CPU_Board_Bottom](/doc/cpu_board_bottom.png)

## Debugger Interface

The SWD interface is exposed as 4 solder pads at the bottom of the CPU board.
- VCC (NC)
- GND (NC)
- SWCLK (blue wire)
- SWDIO (red wire)

I added a little bit of kapton tape in between the CPU and main board before assembling to minimize
the damage if the solder connection breaks.

![CPU and Main board](/doc/kapton_tape.png)


## Firmware Backup

It's probably a good idea to backup the original firmware including the (horrible) bootloader. After
connecting an ST-Link programmer you can use [openocd](http://openocd.org/) to dump the flash.

Start openocd:
```
% openocd -f interface/stlink-v2.cfg -f target/stm32f1x.cfg
Open On-Chip Debugger 0.9.0 (2017-03-07-13:28)
Licensed under GNU GPL v2
For bug reports, read
        http://openocd.org/doc/doxygen/bugs.html
        Info : auto-selecting first available session transport "hla_swd". To override use 'transport select <transport>'.
        Info : The selected transport took over low-level target control. The results might differ compared to plain JTAG/SWD
        adapter speed: 1000 kHz
        adapter_nsrst_delay: 100
        none separate
        Info : Unable to match requested speed 1000 kHz, using 950 kHz
        Info : Unable to match requested speed 1000 kHz, using 950 kHz
        Info : clock speed 950 kHz
        Info : STLINK v2 JTAG v17 API v2 SWIM v4 VID 0x0483 PID 0x3748
        Info : using stlink api v2
        Info : Target voltage: 3.245003
        Info : stm32f1x.cpu: hardware has 6 breakpoints, 4 watchpoints
```

Dump the firmware:
```
% telnet localhost 4444
Trying ::1...
Trying 127.0.0.1...
Connected to localhost.
Escape character is '^]'.
Open On-Chip Debugger
> dump_image ts100_orig.bin 0x08000000 0xffff
dumped 65535 bytes in 1.386794s (46.149 KiB/s)
>
```

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
