#![feature(asm)]
#![feature(const_fn)]
#![feature(proc_macro)]
#![no_std]

extern crate cast;
extern crate cortex_m;
extern crate cortex_m_rtfm as rtfm;
extern crate blue_pill;
extern crate numtoa;

use blue_pill::stm32f103xx::Interrupt;
use cortex_m::peripheral::SystClkSource;
use rtfm::{app, Threshold};

mod font5x7;
mod i2c;
mod mma8652fc;
mod ssd1306;
mod state;

use mma8652fc::MMA8652FC;
use ssd1306::SSD1306;
use state::{ConfigPage, Keys, State, StateMachine};

const OLED_ADDR: u8 = 0x3c;

app! {
    device: blue_pill::stm32f103xx,

    resources: {
        static TICKS: u32 = 0;
        static STATE: StateMachine = StateMachine::new();
    },

    tasks: {
        SYS_TICK: {
            path: tick,
            resources: [TICKS],
        },
        EXTI0: {
            path: update_ui,
            resources: [I2C1, STATE],
        },
        EXTI9_5: {
            path: exti9_5,
            resources: [I2C1, STATE, GPIOA, GPIOB, EXTI],
        },
    },
}

fn init(p: init::Peripherals, _r: init::Resources) {
    // 48Mhz
    p.FLASH.acr.modify(
        |_, w| w.prftbe().enabled().latency().one(),
    );

    p.RCC.cfgr.modify(|_, w| unsafe { w.bits(0x0068840A) });
    p.RCC.cr.modify(
        |_, w| w.pllon().set_bit().hsion().set_bit(),
    );
    while p.RCC.cr.read().pllrdy().bit_is_clear() {}
    while p.RCC.cr.read().hsirdy().bit_is_clear() {}

    p.RCC.apb2enr.modify(|_, w| {
        w.iopaen().enabled().iopben().enabled().afioen().enabled()
    });

    p.AFIO.mapr.modify(|_, w| unsafe {
        w.swj_cfg().bits(2).i2c1_remap().clear_bit()
    });

    p.RCC.apb1enr.modify(|_, w| w.i2c1en().enabled());

    p.I2C1.cr1.write(|w| w.pe().clear_bit());

    p.GPIOA.crl.modify(|_, w| w.mode6().input());
    p.GPIOA.crh.modify(|_, w| {
        w.mode8().output50().cnf8().push().mode9().input()
    });
    p.GPIOA.bsrr.write(|w| {
        w.bs6().set_bit().bs8().set_bit().bs9().set_bit()
    });

    p.GPIOB.crl.modify(|_, w| {
        w.mode5()
            .input()
            .mode6()
            .output50()
            .cnf6()
            .alt_open()
            .mode7()
            .output50()
            .cnf7()
            .alt_open()
    });
    p.GPIOB.bsrr.write(|w| w.bs5().set_bit());

    p.AFIO.exticr2.modify(|_, w| unsafe {
        w.exti5().bits(1).exti6().bits(0)
    });

    p.AFIO.exticr3.modify(|_, w| unsafe { w.exti9().bits(0) });

    p.EXTI.imr.modify(|_, w| {
        w.mr5().set_bit().mr6().set_bit().mr9().set_bit()
    });
    p.EXTI.rtsr.modify(|_, w| {
        w.tr5().set_bit().tr6().set_bit().tr9().set_bit()
    });
    p.EXTI.ftsr.modify(|_, w| {
        w.tr5().set_bit().tr6().set_bit().tr9().set_bit()
    });

    p.I2C1.cr2.modify(|_, w| unsafe { w.freq().bits(24) });
    p.I2C1.cr1.modify(|_, w| w.pe().clear_bit());
    p.I2C1.trise.modify(
        |_, w| unsafe { w.trise().bits(24 + 1) },
    );
    p.I2C1.ccr.modify(|_, w| unsafe {
        w.f_s().clear_bit().duty().clear_bit().ccr().bits(120)
    });

    p.I2C1.cr1.modify(|_, w| {
        w.nostretch()
            .clear_bit()
            .ack()
            .set_bit()
            .smbus()
            .clear_bit()
    });

    p.I2C1.cr1.write(|w| w.pe().set_bit());
    p.I2C1.oar1.write(|w| unsafe {
        w.addmode()
            .clear_bit()
            .add0()
            .clear_bit()
            .add7()
            .bits(0)
            .add10()
            .bits(0)
    });

    let oled = SSD1306(OLED_ADDR, &p.I2C1);
    oled.init();
    oled.print(0, 0, "                ");
    oled.print(0, 1, "                ");

    let accel = MMA8652FC(&p.I2C1);
    accel.init();

    p.SYST.set_clock_source(SystClkSource::Core);
    p.SYST.set_reload(48_000_000);
    p.SYST.enable_interrupt();
    p.SYST.enable_counter();
}

fn idle() -> ! {
    rtfm::set_pending(Interrupt::EXTI0);

    loop {
        rtfm::wfi();
    }
}

fn tick(_t: &mut Threshold, r: SYS_TICK::Resources) {
    **r.TICKS += 1;
}

fn update_ui(_t: &mut Threshold, r: EXTI0::Resources) {
    let i2c1 = &**r.I2C1;
    let oled = SSD1306(OLED_ADDR, &i2c1);
    let am = MMA8652FC(&i2c1);
    r.STATE.update_accel(am.accel());

    r.STATE.update_state();

    oled.print(0, 0, "X ");
    oled.print(8, 0, "Y ");
    oled.print(0, 1, "Z ");

    let accel = r.STATE.get_accel();
    oled.print_number(2, 0, accel.x);
    oled.print_number(10, 0, accel.y);
    oled.print_number(2, 1, accel.z);
/*
    match r.STATE.current_state() {
        State::Idle => {
            oled.print(0, 0, "      IDLE      ");
            oled.print(0, 1, "                ");
        }
        State::Soldering => {
            oled.print(0, 0, "    SOLDERING   ");
            oled.print(0, 1, "                ");
        }
        State::Cooling => {
            oled.print(0, 0, "     COOLING    ");
            oled.print(0, 1, "                ");
        }
        State::Sleep => {
            oled.print(0, 0, "     zZzZzZ     ");
            oled.print(0, 1, "                ");
        }
        State::TemperatureControl => {
            oled.print(0, 0, " <    200 C   > ");
        }
        State::Thermometer => {
            oled.print(0, 0, "     200.1 C    ");
        }
        State::Config(page) => {
            match page {
                ConfigPage::Save => {
                    oled.print(0, 0, "Save and Reset? ");
                }
            }
        }
    }
    */
}

fn exti9_5(_t: &mut Threshold, r: EXTI9_5::Resources) {
    let exti = &**r.EXTI;
    let gpioa = &**r.GPIOA;
    let gpiob = &**r.GPIOB;
    let i2c1 = &**r.I2C1;

    // Button A
    if exti.pr.read().pr6().bit_is_set() {
        if gpioa.idr.read().idr6().bit_is_clear() {
            r.STATE.update_keys(Keys::A);
        } else {
            r.STATE.update_keys(Keys::None);
        };
        exti.pr.write(|w| w.pr6().set_bit());
    // Button B
    } else if exti.pr.read().pr9().bit_is_set() {
        if gpioa.idr.read().idr9().bit_is_clear() {
           r.STATE.update_keys(Keys::B)
        } else {
            r.STATE.update_keys(Keys::None)
        };
        exti.pr.write(|w| w.pr9().set_bit());
    // Movement
    // interrupt doesn't fire
    } else if exti.pr.read().pr5().bit_is_set() {
        if gpiob.idr.read().idr5().bit_is_clear() {
            let am = MMA8652FC(&i2c1);
            r.STATE.update_accel(am.accel());
        }
        exti.pr.write(|w| w.pr5().set_bit());
    }

    rtfm::set_pending(Interrupt::EXTI0);
}
