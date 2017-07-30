#![feature(asm)]
#![feature(const_fn)]
#![feature(proc_macro)]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rtfm as rtfm;
extern crate blue_pill;

use cortex_m::peripheral::SystClkSource;
use rtfm::{app, Threshold};

mod font5x7;
mod i2c;
mod ssd1306;

use ssd1306::SSD1306;

const OLED_ADDR: u8 = 0x3c;

app! {
    device: blue_pill::stm32f103xx,

    resources: {
        static TICKS: u32 = 0;
    },

    tasks: {
        SYS_TICK: {
            path: tick,
            resources: [I2C1, TICKS],
        },
    },
}

fn init(p: init::Peripherals, _r: init::Resources) {
    // 48Mhz
    p.FLASH.acr.modify(|_, w| w.prftbe().enabled().latency().one());

    p.RCC.cfgr.modify(|_, w| unsafe { w.bits(0x0068840A) });
    p.RCC.cr.modify(|_, w| w
                    .pllon().set_bit()
                    .hsion().set_bit());
    while p.RCC.cr.read().pllrdy().bit_is_clear() {}
    while p.RCC.cr.read().hsirdy().bit_is_clear() {}

    p.RCC.apb2enr.modify(|_, w| w
                         .iopaen().enabled()
                         .iopben().enabled()
                         .afioen().enabled()
    );
    p.AFIO.mapr.modify(|_, w| unsafe { w
                                       .swj_cfg().bits(2)
                                       .i2c1_remap().clear_bit() });

    p.RCC.apb1enr.modify(|_, w| w.i2c1en().enabled());

    p.I2C1.cr1.write(|w| w.pe().clear_bit());

    p.GPIOA.crh.modify(|_, w| w.mode8().output50().cnf8().push());
    p.GPIOA.bsrr.write(|w| w.bs8().set_bit());

    p.GPIOB.crl.modify(|_, w| w
                       .mode6().output50().cnf6().alt_open()
                       .mode7().output50().cnf7().alt_open());

    p.I2C1.cr2.modify(|_, w| unsafe { w.freq().bits(24) } );
    p.I2C1.cr1.modify(|_, w| w.pe().clear_bit());
    p.I2C1.trise.modify(|_, w| unsafe { w.trise().bits(24 + 1) } );
    p.I2C1.ccr.modify(|_, w| unsafe { w.
                                      f_s().clear_bit()
                                      .duty().clear_bit()
                                      .ccr().bits(120) } );

    p.I2C1.cr1.modify(|_, w| w
                      .nostretch().clear_bit()
                      .ack().set_bit()
                      .smbus().clear_bit()
    );

   p.I2C1.cr1.write(|w| w.pe().set_bit());
        p.I2C1.oar1.write(|w| unsafe {
        w.addmode().clear_bit().add0().clear_bit().add7().bits(0).add10().bits(0) });

    let oled = SSD1306(OLED_ADDR, &p.I2C1);
    oled.init();

    p.SYST.set_clock_source(SystClkSource::Core);
    p.SYST.set_reload(48_000_000);
    p.SYST.enable_interrupt();
    p.SYST.enable_counter();

}

fn idle() -> ! {
    loop {
        rtfm::wfi();
    }
}

fn tick(_t: &mut Threshold,  r: SYS_TICK::Resources) {
    **r.TICKS += 1;

    let i2c1 = &**r.I2C1;

    let oled = SSD1306(OLED_ADDR, &i2c1);
    oled.print(0, 0, "   Hello from   ");
    oled.print(0, 1, "      Rust      ");
}
