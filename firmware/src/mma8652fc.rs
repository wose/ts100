use blue_pill::stm32f103xx::I2C1;
use cast::u16;
use cortex_m;
use i2c;

const I2C_ADDRESS: u8 = 0x1D;

/// MMA8652FC Register Addresses
#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub enum Register {
    /// Status Register (R)
    STATUS = 0x00,

    /// [7:0] are 8 MSBs of the 14-bit X-axis sample (R)
    OUT_X_MSB = 0x01,
    /// [7:2] are 6 LSBs of the 14-bit X-axis sample (R)
    OUT_X_LSB = 0x02,
    /// [7:0] are 8 MSBs of the 14-bit Y-axis sample (R)
    OUT_Y_MSB = 0x03,
    /// [7:2] are 6 LSBs of the 14-bit Y-axis sample (R)
    OOT_Y_LSB = 0x04,
    /// [7:0] are 8 MSBs of the 14-bit Z-axis sample (R)
    OUT_Z_MSB = 0x05,
    /// [7:2] are 6 LSBs of the 14-bit Z-axis sample (R)
    OUT_Z_LSB = 0x06,

    /// FIFO Setup Register (R/W)
    F_SETUP = 0x09,
    /// Map of FIFO data capture events (R/W)
    TRIG_CFG = 0x0A,
    /// System Mode Register (R)
    SYSMOD = 0x0B,
    /// System Interrupt Status Register (R)
    INT_SOURCE = 0x0C,
    /// Device ID Register (R)
    WHO_AM_I = 0x0D,
    /// Sensor Data Configuration Register (R/W)
    XYZ_DATA_CFG = 0x0E,
    /// High Pass Filter Register (R/W)
    HP_FILTER_CUTOFF = 0x0F,

    /// Portait/tLandscape Status Register (R)
    PL_STATUS = 0x10,
    /// Portrait/Landscape Configuration Register (R/W)
    PL_CFG = 0x11,
    /// Portrait/Landscape Debounce Register (R/W)
    PL_COUNT = 0x12,
    /// Portrait/Landscape Back/Front and Z Compensation Register (R/W)
    PL_BF_ZCOMP = 0x13,
    /// Portrait/Landscape Threshold Register (R/W)
    P_L_THS_REG = 0x14,

    /// Freefall and Motion Configuration Register (R/W)
    FF_MT_CFG = 0x15,
    /// Freefall and Motion Source Register (R)
    FF_MT_SRC = 0x16,
    /// Freefall and Motion Threshold Register (R/W)
    FF_MT_THS = 0x17,
    /// Freefall Motion Count Register (R/W)
    FF_MT_COUNT = 0x18,

    /// Transient Configuration Register (R/W)
    TRANSIENT_CFG = 0x1D,
    /// Transient Source Register (R)
    TRANSIENT_SRC = 0x1E,
    /// Transient Threshold Register (R/W)
    TRANSIENT_THS = 0x1F,
    /// Transient Debounce Counter Register (R/W)
    TRANSIENT_COUNT = 0x20,

    /// Pulse Configuration Register (R/W)
    PULSE_CFG = 0x21,
    /// Pulse Source Register (R)
    PULSE_SRC = 0x22,
    /// Pulse X Threshold Register (R/W)
    PULSE_THS_X = 0x23,
    /// Pulse Y Threshold Register (R/W)
    PULSE_THS_Y = 0x24,
    /// Pulse Z Threshold Register (R/W)
    PULSE_THS_Z = 0x25,
    /// Pulse Time Window Register (R/W)
    PULSE_TLMT = 0x26,
    /// Pulse Latency Timer Register (R/W)
    PULSE_LTCY = 0x27,
    /// Second Pulse Time Window Register (R/W)
    PULSE_WIND = 0x28,

    /// Auto Sleep Inactivity Timer Register (R/W)
    ALSP_COUNT = 0x29,

    /// System Control 1 Register (R/W)
    CTRL_REG1 = 0x2A,
    /// System Control 2 Register (R/W)
    CTRL_REG2 = 0x2B,
    /// Interrupt Control Register (R/W)
    CTRL_REG3 = 0x2C,
    /// Interrupt Enable Register (R/W)
    CTRL_REG4 = 0x2D,
    /// Interrupt Configuration Register (R/W)
    CTRL_REG5 = 0x2E,

    /// X Offset Correction Register (R/W)
    OFF_X = 0x2F,
    /// Y Offset Correction Register (R/W)
    OFF_Y = 0x30,
    /// Z Offset Correction Register (R/W)
    OFF_Z = 0x31,
}

impl Register {
    pub fn addr(&self) -> u8 {
        *self as u8
    }
}

#[derive(Clone, Copy)]
pub struct Accel {
    /// X component
    pub x: i16,
    /// Y component
    pub y: i16,
    /// Z component
    pub z: i16,
}

pub struct MMA8652FC<'a>(pub &'a I2C1);

impl<'a> MMA8652FC<'a> {
    pub fn init(&self) {
        self
        // Normal Mode
            .set_register(Register::CTRL_REG2, 0);
        // Reset all registers to POR values
        self.set_register(Register::CTRL_REG2, 0x40);
        for _ in 0..10_000 {
            cortex_m::asm::nop();
        }
        // Enable motion detection for X, Y and Z axis, latch disabled
        self.set_register(Register::FF_MT_CFG, 0x78);

//        self.set_register(Register::FreefallMotionThr, 0x10);
//        self.set_register(Register::FreefallMotionCnt, 0x02);

        // Enable orientation detection
        self.set_register(Register::PL_CFG, 0x40);
        // set Debounce to 200 Counts
        self.set_register(Register::PL_COUNT, 200);
        // set Threshold to 42 degrees
        self.set_register(Register::PL_BF_ZCOMP, 0b01000111);
        // set threshold
        self.set_register(Register::P_L_THS_REG, 0b10011100);
        // enable data ready and orientation interrupt
        self.set_register(Register::CTRL_REG4, 0x01 | (1 << 4));
        // route data ready interrupt to INT1 and orientation interrupt to INT2
        self.set_register(Register::CTRL_REG5, 0x01);
        // set maximum resolution oversampling
        self.set_register(Register::CTRL_REG2, 0x12);
        // select high pass filtered data
        self.set_register(Register::XYZ_DATA_CFG, (1 << 4));
        // select high pass filtered data
        self.set_register(Register::HP_FILTER_CUTOFF, 0x03);
        // 12 Hz, active mode
        self.set_register(Register::CTRL_REG1, 0x19);
    }

    #[allow(dead_code)]
    pub fn set_sensitivity(&self, threshold: u8, filter_time: u8) -> &Self {
        let sens = 9 * 2 + 17 - 2 * threshold;
        self
        // sleep mode
            .set_register(Register::CTRL_REG1, 0);
        // set accumulation threshold
        self.set_register(Register::FF_MT_THS, (sens & 0x7F));
        // set debounce threshold
        self.set_register(Register::FF_MT_COUNT, filter_time);
        // 12 Hz, active mode
        self.set_register(Register::CTRL_REG1, 0x31)
    }

    pub fn accel(&self) -> Accel {
        let mut bytes = [0; 6];
        i2c::read(&self.0, I2C_ADDRESS, Register::OUT_X_MSB.addr(), &mut bytes);

        Accel {
            x: ((u16(bytes[0]) << 8 ) + u16(bytes[1])) as i16,
            y: ((u16(bytes[2]) << 8 ) + u16(bytes[3])) as i16,
            z: ((u16(bytes[4]) << 8 ) + u16(bytes[5])) as i16,
        }
    }

    pub fn set_register(&self, reg: Register, value: u8) -> &Self {
        i2c::write(&self.0, I2C_ADDRESS, reg.addr(), value);
        self
    }
}
