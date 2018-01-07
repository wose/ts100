use blue_pill::stm32f103xx::I2C1;

pub fn read(i2c1: &I2C1, slave: u8, register: u8, bytes: &mut [u8]) {
    // wait for idle i2c interface
    while i2c1.sr2.read().busy().bit_is_set() {}

    // enable ack
    i2c1.cr1.modify(|_, w| w
                    .ack().set_bit()
                    .pos().clear_bit());

    // generate start
    i2c1.cr1.modify(|_, w| w.start().set_bit());
    while i2c1.sr1.read().sb().bit_is_clear() {}

    i2c1.dr.write(|w| unsafe { w.dr().bits(slave << 1) });
    while i2c1.sr1.read().addr().bit_is_clear() {}
    while i2c1.sr2.read().tra().bit_is_clear() {}

    while i2c1.sr1.read().tx_e().bit_is_clear() {}
    i2c1.dr.write(|w| unsafe { w.dr().bits(register) });

    // generate start
    i2c1.cr1.modify(|_, w| w.start().set_bit());
    while i2c1.sr1.read().sb().bit_is_clear() {}

    // send address
    i2c1.dr.write(|w| unsafe { w.dr().bits((slave << 1) | 0x0001) });

    // EV6
    while i2c1.sr1.read().addr().bit_is_clear() {}

    match bytes.len() {
        1 => {
            // clear ack bit
            i2c1.cr1.modify(|_, w| w.ack().clear_bit());
            // EV6_1 clear ADDR generate STOP
            let _ = i2c1.sr2.read().bits();
            i2c1.cr1.modify(|_, w| w.stop().set_bit());
            // EV7
            while i2c1.sr1.read().rx_ne().bit_is_clear() {}
            bytes[0] = i2c1.dr.read().dr().bits();
        }
        2 => {
            // set position
            i2c1.cr1.modify(|_, w| w.pos().set_bit());
            // EV6_1 clear ADDR, clear ACK
            let _ = i2c1.sr2.read().bits();
            i2c1.cr1.modify(|_, w| w.ack().clear_bit());
            // EV7_3 wait for BTF
            while i2c1.sr1.read().btf().bit_is_clear() {}
            // generate STOP
            i2c1.cr1.modify(|_, w| w.stop().set_bit());
            // read DR twice
            bytes[0] = i2c1.dr.read().dr().bits();
            bytes[1] = i2c1.dr.read().dr().bits();
        }
        length => {
            // clear ADDR
            let _ = i2c1.sr2.read().bits();
            for byte in 0..length - 3 {
                // EV7 wait for BTF
                while i2c1.sr1.read().btf().bit_is_clear() {}
                bytes[byte] = i2c1.dr.read().dr().bits();
            }

            while i2c1.sr1.read().btf().bit_is_clear() {}
            // EV7_2 clear ACK
            i2c1.cr1.modify(|_, w| w.ack().clear_bit());
            bytes[length - 3] = i2c1.dr.read().dr().bits();
            // generate STOP
            i2c1.cr1.modify(|_, w| w.stop().set_bit());
            bytes[length - 2] = i2c1.dr.read().dr().bits();

            while i2c1.sr2.read().busy().bit_is_set() {}
//            while i2c1.sr2.read().msl().bit_is_set() {}
            while i2c1.sr1.read().rx_ne().bit_is_clear() {}

            bytes[length - 1] = i2c1.dr.read().dr().bits();
        }
    }

//    while i2c1.sr1.read().stopf().bit_is_clear() {}
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
