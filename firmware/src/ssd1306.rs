use blue_pill::stm32f103xx::I2C1;

use font5x7;
use i2c;

// Registers
const COMMAND_MODE: u8 = 0x80;
const DATA_MODE: u8 = 0x40;
// Commands
const CHARGE_PUMP_SETTING: u8 = 0x8d;
const CHARGE_PUMP_ENABLE: u8 = 0x14;

const DISPLAY_OFF: u8 = 0xAE;
const DISPLAY_ON: u8 = 0xAF;


//0x80, 0xAE,/*Display off*/
//0x80, 0xD5,/*Set display clock divide ratio / osc freq*/
//0x80, 0x52,/*Unknown*/
//0x80, 0xA8,/*Set Multiplex Ratio*/
//0x80, 0x0F, /*16 == max brightness,39==dimmest*/
//0x80, 0xC0,/*Set COM Scan direction*/
//0x80, 0xD3,/*Set Display offset*/
//0x80, 0x00,/*0 Offset*/
//0x80, 0x40,/*Set Display start line to 0*/
//0x80, 0xA0,/*Set Segment remap to normal*/
//0x80, 0x8D,/*Unknown*/
//0x80, 0x14,/*Unknown*/
//0x80, 0xDA,/*Set VCOM Pins hardware config*/
//0x80, 0x02,/*Combination 2*/
//0x80, 0x81,/*Contrast*/
//0x80, 0x33,/*51*/
//0x80, 0xD9,/*Set pre-charge period*/
//0x80, 0xF1,/**/
//0x80, 0xDB,/*Adjust VCOMH regulator ouput*/
//0x80, 0x30,/*Unknown*/
//0x80, 0xA4,/*Enable the display GDDR*/
//0x80, 0XA6,/*Normal display*/
//0x80, 0xAF /*Dispaly on*/

pub struct SSD1306<'a>(pub u8, pub &'a I2C1);

impl<'a> SSD1306<'a> {
    pub fn init(&self) {
        self.send_command(CHARGE_PUMP_SETTING)
            .send_command(CHARGE_PUMP_ENABLE)
            .send_command(DISPLAY_OFF)

//            .send_command(0xD5)
//            .send_command(0x52)
//            .send_command(0xA8)
//            .send_command(0x0F)
            .send_command(0xC0)
            .send_command(0xD3)
            .send_command(0x00)
//            .send_command(0x02)
            .send_command(0x40)
            .send_command(0xA0)
            .send_command(0x8D)
            .send_command(0x14)
            .send_command(0xDA)
            .send_command(0x02)
//            .send_command(0x81)

//            .send_command(PAGE_ADDRESSING)
//            .send_command(0xD9)
//            .send_command(0x33)
//            .send_command(0xD9)
//            .send_command(0xF1)
            .send_command(0xDB)
            .send_command(0x30)
            .send_command(0xA4)
            .send_command(0xA6)
            .send_command(DISPLAY_ON);
    }

    pub fn send_command(&self, command: u8) -> &Self {
        i2c::write(&self.1, self.0, COMMAND_MODE, command);
        self
    }

    pub fn send_data(&self, data: u8) -> &Self {
        i2c::write(&self.1, self.0, DATA_MODE, data);
        self
    }

    pub fn print(&self, x: u8, y: u8, text: &str) -> &Self {
        self.send_command(0x00 + ((6 * x + 32) & 0x0f))
            .send_command(0x10 + (((6 * x + 32) >> 4) & 0x0f))
            .send_command(0xB0 + y);

        for byte in text.as_bytes().iter().cloned() {
            // check if byte is printable
            // TODO let the font decide
            if byte >= 0x20 && byte < 0x20 + 0x60 {
                for column in 0..5 {
                    self.send_data(font5x7::FONT_5X7[(byte - 0x20) as usize * 5 + column]);
                }
                // 1 pixel space between chars
                self.send_data(0x00);
            }
        }

        self
    }
}
