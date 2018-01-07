use blue_pill::stm32f103xx::I2C1;
use cast::u16;
use i2c;

const I2C_ADDRESS: u8 = 0x1D;

/// MMA8652FC Register Addresses
#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum Register {
    /// Status Register
    Status = 0x00,

    /// [7:0] are 8 MSBs of the 14-bit X-axis sample
    OutXMsb = 0x01,
    /// [7:2] are 6 LSBs of the 14-bit X-axis sample
    OutXLsb = 0x02,
    /// [7:0] are 8 MSBs of the 14-bit Y-axis sample
    OutYMsb = 0x03,
    /// [7:2] are 6 LSBs of the 14-bit Y-axis sample
    OutYLsb = 0x04,
    /// [7:0] are 8 MSBs of the 14-bit Z-axis sample
    OutZMsb = 0x05,
    /// [7:2] are 6 LSBs of the 14-bit Z-axis sample
    OutZLsb = 0x06,

    /// FIFO Setup Register
    FifoSetup = 0x09,
    /// Map of FIFO data capture events
    TrigCfg = 0x0A,
    /// System Mode Register
    SysMod = 0x0B,
    /// System Interrupt Status Register
    IntSource = 0x0C,
    /// Device ID Register
    WhoAmI = 0x0D,
    /// Sensor Data Configuration Register
    XyzDataCfg = 0x0E,
    /// High Pass Filter Register
    HpFilterCutOff = 0x0F,

    /// Portait/tLandscape Status Register
    PortLandStatus = 0x10,
    /// Portrait/Landscape Configuration Register
    PortLandCfg = 0x11,
    /// Portrait/Landscape Debounce Register
    PortLandDeb = 0x12,
    /// Portrait/Landscape Back/Front and Z Compensation Register
    PortLandBFZComp = 0x13,
    /// Portrait/Landscape Threshold Register
    PortLandThr = 0x14,

    /// Freefall and Motion Configuration Register
    FreefallMotionCfg = 0x15,
    /// Freefall and Motion Source Register
    FreefallMotionSrc = 0x16,
    /// Freefall and Motion Threshold Register
    FreefallMotionThr = 0x17,
    /// Freefall Motion Count Register
    FreefallMotionCnt = 0x18,

    /// Transient Configuration Register
    TransCfg = 0x1D,
    /// Transient Source Register
    TransSrc = 0x1E,
    /// Transient Threshold Register
    TransThr = 0x1F,
    /// Transient Debounce Counter Register
    TransCnt = 0x20,

    /// Pulse Configuration Register
    PulseCfg = 0x21,
    /// Pulse Source Register
    PulseSrc = 0x22,
    /// Pulse X Threshold Register
    PulseXThr = 0x23,
    /// Pulse Y Threshold Register
    PulseYThr = 0x24,
    /// Pulse Z Threshold Register
    PulseZThr = 0x25,
    /// Pulse Time Window Register
    PulseTimeWin = 0x26,
    /// Pulse Latency Timer Register
    PulseLatTimer = 0x27,
    /// Second Pulse Time Window Register
    PulseTime2Win = 0x28,

    /// Auto Sleep Inactivity Timer Register
    AutoSleepTimer = 0x29,

    /// System Control 1 Register
    Ctrl1 = 0x2A,
    /// System Control 2 Register
    Ctrl2 = 0x2B,
    /// Interrupt Control Register
    Ctrl3 = 0x2C,
    /// Interrupt Enable Register
    Ctrl4 = 0x2D,
    /// Interrupt Configuration Register
    Ctrl5 = 0x2E,

    /// X Offset Correction Register
    OffX = 0x2F,
    /// Y Offset Correction Register
    OffY = 0x30,
    /// Z Offset Correction Register
    OffZ = 0x31,

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
            .set_register(Register::Ctrl2, 0);
            // Reset all registers to POR values
//            self.set_register(Register::Ctrl2, 0x40);
            // Enable motion detection for X, Y and Z axis, latch disabled
        self.set_register(Register::FreefallMotionCfg, 0x78);
            // Enable orientation detection
        self.set_register(Register::PortLandCfg, 0x40);
            // set Debounce to 200 Counts
        self.set_register(Register::PortLandDeb, 200);
            // set Threshold to 42 degrees
        self.set_register(Register::PortLandBFZComp, 0b01000111);
            // set threshold
        self.set_register(Register::PortLandThr, 0b10011100);
            // enable data ready and orientation interrupt
        self.set_register(Register::Ctrl4, 0x01 | (1 << 4));
            // route data ready interrupt to INT1 and orientation interrupt to INT2
        self.set_register(Register::Ctrl5, 0x01);
            // set maximum resolution oversampling
        self.set_register(Register::Ctrl2, 0x12);
            // select high pass filtered data
        self.set_register(Register::XyzDataCfg, (1 << 4));
            // select high pass filtered data
        self.set_register(Register::HpFilterCutOff, 0x03);
            // 12 Hz, active mode
        self.set_register(Register::Ctrl1, 0x19);
    }

    #[allow(dead_code)]
    pub fn set_sensitivity(&self, threshold: u8, filter_time: u8) -> &Self {
        let sens = 9 * 2 + 17 - 2 * threshold;
        self
            // sleep mode
            .set_register(Register::Ctrl1, 0);
            // set accumulation threshold
            self.set_register(Register::FreefallMotionThr, (sens & 0x7F));
            // set debounce threshold
        self.set_register(Register::FreefallMotionCnt, filter_time);
            // 12 Hz, active mode
        self.set_register(Register::Ctrl1, 0x31)
    }

    pub fn accel(&self) -> Accel {
        let mut bytes = [0; 6];
        i2c::read(&self.0, I2C_ADDRESS, Register::OutXMsb.addr(), &mut bytes);

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
