use blue_pill::stm32f103xx::I2C1;

#[allow(unused)]
pub fn read(i2c1: &I2C1, slave: u8, register: u8, bytes: &mut [u8]) {
    unimplemented!();
}

pub fn write(i2c1: &I2C1, slave: u8, register: u8, value: u8) {
    while i2c1.sr2.read().busy().bit_is_set() {}

    i2c1.cr1.modify(|_, w| w.start().set_bit());
    while i2c1.sr1.read().sb().bit_is_clear() {}

    i2c1.dr.write(|w| unsafe { w.dr().bits(slave << 1) });
    while i2c1.sr1.read().addr().bit_is_clear() {}
    while i2c1.sr2.read().tra().bit_is_clear() {}

    while i2c1.sr1.read().tx_e().bit_is_clear() {}
    i2c1.dr.write(|w| unsafe { w.dr().bits(register) });
    while i2c1.sr1.read().btf().bit_is_clear() {}

    i2c1.dr.write(|w| unsafe { w.dr().bits(value) });
    while i2c1.sr1.read().btf().bit_is_clear() {}

    i2c1.cr1.modify(|_, w| w.stop().set_bit());
    while i2c1.sr1.read().sb().bit_is_set() {}
}
