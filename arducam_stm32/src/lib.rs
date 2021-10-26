#![no_std]
#![deny(unused_imports)]
#![deny(unsafe_code)]
#![deny(warnings)]
#![allow(non_upper_case_globals)]
// #![allow(non_camel_case_types)]
// #![allow(non_snake_case)]

mod ov5642_regs;

use defmt::Format;
use embedded_hal::{
    blocking::{
        delay::DelayMs,
        i2c::{self, SevenBitAddress},
        spi::Transfer,
    },
    digital::v2::OutputPin,
};
use ov5642_regs::*;

pub enum CameraType {
    OV2640,
    OV5642,
}

// const MAX_FIFO_SIZE: i32 = 0x7FFFFF;
// const ARDUCHIP_FRAMES: i32 = 0x01;
// const ARDUCHIP_TIM: i32 = 0x03;
// const VSYNCnLEVEL_MASK: i32 = 0x02;
// const ARDUCHIP_TRIG: i32 = 0x41;
// const CAP_DONE_MASK: i32 = 0x08;

enum OV5642ChipID {
    High = 0x300a,
    Low = 0x300b,
}

// enum OV2640Size {
//     S160x120 = 0,
//     S176x144 = 1,
//     S320x240 = 2,
//     S352x288 = 3,
//     S640x480 = 4,
//     S800x600 = 5,
//     S1024x768 = 6,
//     S1280x1024 = 7,
//     S1600x1200 = 8,
// }

pub enum OV5642Size {
    S320x240,
    S640x480,
    S1024x768,
    S1280x960,
    S1600x1200,
    S2048x1536,
    S2592x1944,
    S1920x1080,
}

pub enum LightMode {
    AdvancedAWB,
    SimpleAWB,
    ManualDay,
    ManualA,
    Manualcwf,
    ManualCloudy,
}

pub enum CameraHue {
    Dn180,
    Dn150,
    Dn120,
    Dn90,
    Dn60,
    Dn30,
    Dn0,
    D30,
    D60,
    D90,
    D120,
    D150,
}

pub enum CameraMode {
    Auto,
    Sunny,
    Cloudy,
    Office,
    Home,
}

pub enum CameraEffect {
    Antique,
    Greenish,
    Reddish,
    BW,
    Negative,
    BWnegative,
    Normal,
    Sepia,
    Overexposure,
    Solarize,
    Blueish,
    Yellowish,
}

pub enum CameraExposure {
    En17 = 0,
    En13 = 1,
    En10 = 2,
    En07 = 3,
    En03 = 4,
    EDefault = 5,
    E03 = 6,
    E07 = 7,
    E10 = 8,
    E13 = 9,
    E17 = 10,
}

pub enum CameraSharpness {
    AutoDefault,
    Auto1,
    Auto2,
    ManualOff,
    Manual1,
    Manual2,
    Manual3,
    Manual4,
    Manual5,
}

pub enum MirrorFlip {
    Mirror,
    Flip,
    MirrorFlip,
    Normal,
}

pub enum CameraSaturation {
    S4,
    S3,
    S2,
    S1,
    S0,
    Sn1,
    Sn2,
    Sn3,
    Sn4,
}

pub enum CameraBrightness {
    B4,
    B3,
    B2,
    B1,
    B0,
    Bn1,
    Bn2,
    Bn3,
    Bn4,
}

pub enum CameraContrast {
    C4,
    C3,
    C2,
    C1,
    C0,
    Cn1,
    Cn2,
    Cn3,
    Cn4,
}

pub enum CameraQuality {
    High,
    Default,
    Low,
}

pub enum CameraPattern {
    ColorBar,
    ColorSquare,
    BWSquare,
    DLI,
}

pub enum CameraFormat {
    BMP,
    JPEG,
    RAW,
}

#[derive(Format, Debug)]
pub enum ArducamError {
    SpiSend,
    SpiRead,
    I2cRead,
    I2cWrite,
    CSLow,
    CSHigh,
    CameraNotFound,
}

#[repr(u8)]
enum Instruction {
    Capture = 0x84,
    CheckDone = 0x41,
    SingleFifoRead = 0x3D,
}

pub struct Arducam<CS, SPI, I2C> {
    format: CameraFormat,
    camera_type: CameraType,
    cs: CS,
    i2c_address: u8,
    pub spi: SPI,
    i2c: I2C,
}
impl<CS, SPI, I2C> Arducam<CS, SPI, I2C>
where
    CS: OutputPin,
    SPI: Transfer<u8>,
    I2C: i2c::Read<SevenBitAddress> + i2c::Write<SevenBitAddress>,
{
    pub fn new(camera_type: CameraType, cs: CS, spi: SPI, i2c: I2C) -> Self {
        return Self {
            format: CameraFormat::JPEG,
            camera_type,
            cs,
            i2c_address: 0x30,
            spi,
            i2c,
        };
    }

    pub fn init(
        &mut self,
        delay: &mut impl DelayMs<u32>,
        format: CameraFormat,
    ) -> Result<(), ArducamError> {
        self.format = format;

        // __init__
        // self.i2c = bitbangio.I2C(scl=board.GP9, sda=board.GP8,frequency=1000000)
        self.spi_send(0x07, 0x80)?;
        delay.delay_ms(100);
        self.spi_send(0x07, 0x00)?;
        delay.delay_ms(100);

        // CameraDetection
        loop {
            match self.camera_type {
                CameraType::OV2640 => {
                    // self.i2c_address = 0x30;
                    // self.wr_sensor_reg_8_8(0xff, 0x01)?;
                    // let id_h = self.rd_sensor_reg_8_8(0x0a)?;
                    // let id_l = self.rd_sensor_reg_8_8(0x0b)?;
                    // if id_h == 0x26 && ((id_l == 0x40) || (id_l == 0x42)) {
                    //     // print('CameraType is OV2640')
                    //     break;
                    // } else {
                    //     // print('Can\'t find OV2640 module');
                    // }
                }
                CameraType::OV5642 => {
                    self.i2c_address = 0x3c;
                    self.wr_sensor_reg_16_8(0xff, 0x01)?;
                    let id_h = self.rd_sensor_reg_16_8(OV5642ChipID::High as u16)?;
                    let id_l = self.rd_sensor_reg_16_8(OV5642ChipID::Low as u16)?;
                    if (id_h == 0x56) && (id_l == 0x42) {
                        // print('CameraType is OV5642')
                        break;
                    } else {
                        return Err(ArducamError::CameraNotFound);
                        // print('Can\'t find OV5642 module')
                    }
                }
            }
            delay.delay_ms(1_000);
        }

        // SPI_Test
        loop {
            self.spi_send(0x00, 0x56)?;
            let value = self.spi_read(0x00)?;
            defmt::debug!("0x00 {:x}", value);
            if value == 0x56 {
                defmt::debug!("SPI interface OK");
                break;
            } else {
                defmt::error!("SPI interface Error");
            }

            delay.delay_ms(1_000);
        }

        // CameraInit
        match self.camera_type {
            CameraType::OV2640 => {
                //   self.wr_sensor_reg_8_8(0xff,0x01);
                //   self.wr_sensor_reg_8_8(0x12,0x80);
                //   delay.delay_ms(100);
                //   self.wr_sensor_regs_8_8(delay,OV2640_JPEG_INIT);
                //   self.wrSensorRegs8_8(OV2640_YUV422);
                //   self.wrSensorRegs8_8(OV2640_JPEG);
                //   self.wr_sensor_reg_8_8(0xff,0x01);
                //   self.wr_sensor_reg_8_8(0x15,0x00);
                //   self.wrSensorRegs8_8(OV2640_320x240_JPEG);
            }
            CameraType::OV5642 => {
                self.wr_sensor_reg_16_8(0x3008, 0x80)?;

                match self.format {
                    CameraFormat::RAW => {
                        self.wr_sensor_regs_16_8(delay, &OV5642_1280x960_RAW)?;
                        self.wr_sensor_regs_16_8(delay, &OV5642_640x480_RAW)?;
                    }
                    CameraFormat::JPEG => {
                        self.wr_sensor_regs_16_8(delay, &OV5642_QVGA_Preview1)?;
                        self.wr_sensor_regs_16_8(delay, &OV5642_QVGA_Preview2)?;
                        delay.delay_ms(100);

                        delay.delay_ms(100);
                        self.wr_sensor_regs_16_8(delay, &OV5642_JPEG_Capture_QSXGA)?;
                        self.wr_sensor_regs_16_8(delay, &ov5642_320x240)?;

                        delay.delay_ms(100);
                        self.wr_sensor_reg_16_8(0x3818, 0xa8)?;
                        self.wr_sensor_reg_16_8(0x3621, 0x10)?;
                        self.wr_sensor_reg_16_8(0x3801, 0xb0)?;
                        self.wr_sensor_reg_16_8(0x4407, 0x04)?;
                    }
                    CameraFormat::BMP => {
                        self.wr_sensor_regs_16_8(delay, &OV5642_QVGA_Preview1)?;
                        self.wr_sensor_regs_16_8(delay, &OV5642_QVGA_Preview2)?;
                        delay.delay_ms(100);

                        self.wr_sensor_reg_16_8(0x4740, 0x21)?;
                        self.wr_sensor_reg_16_8(0x501e, 0x2a)?;
                        self.wr_sensor_reg_16_8(0x5002, 0xf8)?;
                        self.wr_sensor_reg_16_8(0x501f, 0x01)?;
                        self.wr_sensor_reg_16_8(0x4300, 0x61)?;
                        let reg_val = self.rd_sensor_reg_16_8(0x3818)?;
                        self.wr_sensor_reg_16_8(0x3818, (reg_val | 0x60) & 0xff)?;
                        let other_reg_val = self.rd_sensor_reg_16_8(0x3621)?;
                        self.wr_sensor_reg_16_8(0x3621, other_reg_val & 0xdf)?;
                    }
                }
            }
        };

        return Ok(());
    }

    pub fn spi_send(&mut self, address: u8, value: u8) -> Result<(), ArducamError> {
        let maskbits: u8 = 0x80;
        // let buffer: [u8; 2] = [address | maskbits, value];
        // buffer[0] = address | maskbits;
        // buffer[1] = value;
        self.set_cs_low()?;
        if let Some(_err) = self.spi.transfer(&mut [address | maskbits, value]).err() {
            return Err(ArducamError::SpiSend);
        }
        self.set_cs_high()?;
        return Ok(());
    }

    fn spi_read(&mut self, address: u8) -> Result<u8, ArducamError> {
        let maskbits = 0x7f;
        let word = address & maskbits;
        let mut buffer = [word];
        self.set_cs_low()?;
        let received = self
            .spi
            .transfer(&mut buffer)
            .or(Err(ArducamError::SpiRead))?;

        // self.spi.send(word).or(Err(ArducamError::SpiSend))?;
        // let read_word = self.spi.read().or(Err(ArducamError::SpiRead))?;
        self.set_cs_high()?;
        return Ok(received[0]);
    }

    pub fn set_camera_format(&mut self, format: CameraFormat) {
        self.format = format;
    }

    pub fn set_fifo_burst(&mut self) -> Result<(), ArducamError> {
        self.spi
            .transfer(&mut [0x3c])
            .or(Err(ArducamError::SpiSend))?;
        // self.spi.send(0x3c).or(Err(ArducamError::SpiSend))?;
        return Ok(());
    }

    pub fn clear_fifo_flag(&mut self) -> Result<(), ArducamError> {
        self.spi_send(0x04, 0x01)?;
        return Ok(());
    }

    pub fn flush_fifo(&mut self) -> Result<(), ArducamError> {
        self.spi_send(0x04, 0x01)?;
        return Ok(());
    }

    pub fn start_capture(&mut self) -> Result<(), ArducamError> {
        self.spi_send(0x04, 0x02)?;
        return Ok(());
    }

    pub fn read_fifo_length(&mut self) -> Result<u32, ArducamError> {
        let len1 = self.spi_read(0x42)? as u32;
        let len2 = self.spi_read(0x43)? as u32;
        let len3 = self.spi_read(0x44)? as u32;
        let masked_len3 = len3 & 0x7f;
        return Ok(((masked_len3 << 16) | (len2 << 8) | (len1)) & 0x07fffff);
    }

    pub fn get_bit(&mut self, addr: u8, bit: u8) -> Result<u8, ArducamError> {
        let value = self.spi_read(addr)?;
        return Ok(value & bit);
    }

    // fn set_bit(&mut self, addr: u8, bit: u8) -> Result<(), ()> {
    //     let temp = self.spi_read(addr)?;
    //     self.spi_send(addr, temp & (!bit))?;
    //     return Ok(());
    // }

    pub fn get_frame(
        &mut self,
        delay: &mut impl DelayMs<u32>,
        data: &mut [u8; 128],
    ) -> Result<(), ArducamError> {
        self.set_cs_low()?;
        self.spi
            .transfer(&mut [Instruction::Capture as u8 | 0x80, 0x02])
            .or(Err(ArducamError::SpiSend))?;
        self.set_cs_high()?;

        delay.delay_ms(1);

        // check done
        loop {
            self.set_cs_low()?;
            let mut words = [Instruction::CheckDone as u8 & 0x7f, 0x00];
            let answer = self
                .spi
                .transfer(&mut words)
                .or(Err(ArducamError::SpiSend))?;

            self.set_cs_high()?;
            delay.delay_ms(100);

            let result = answer[0] & (1 << 3);
            defmt::debug!(
                "check done result {:#010b} {:#010b} {}",
                answer[0],
                result,
                answer
            );
            // return Ok(());
            if result != 0 {
                // done

                self.set_cs_low()?;
                data[0] = Instruction::SingleFifoRead as u8;
                data[1] = 0x00;
                self.spi.transfer(data).or(Err(ArducamError::SpiSend))?;

                self.set_cs_high()?;

                return Ok(());
            }
        }
    }

    pub fn set_cs_low(&mut self) -> Result<(), ArducamError> {
        self.cs.set_low().or(Err(ArducamError::CSLow))?;
        return Ok(());
    }

    pub fn set_cs_high(&mut self) -> Result<(), ArducamError> {
        self.cs.set_high().or(Err(ArducamError::CSHigh))?;
        return Ok(());
    }

    fn iic_write(&mut self, buf: &[u8]) -> Result<(), ArducamError> {
        self.i2c
            .write(self.i2c_address, buf)
            .or(Err(ArducamError::I2cWrite))?;
        return Ok(());
    }

    fn iic_read(&mut self, buf: &mut [u8]) -> Result<(), ArducamError> {
        self.i2c
            .read(self.i2c_address, buf)
            .or(Err(ArducamError::I2cRead))?;
        return Ok(());
    }

    fn rd_sensor_reg_16_8(&mut self, addr: u16) -> Result<u8, ArducamError> {
        let mut buffer: [u8; 2] = [(addr >> 8) as u8 & 0xff, addr as u8 & 0xff];
        let mut rt: [u8; 1] = [0; 1];
        self.iic_write(&mut buffer)?;
        self.iic_read(&mut rt)?;
        return Ok(rt[0]);
    }

    fn wr_sensor_reg_16_8(&mut self, addr: u16, val: u8) -> Result<(), ArducamError> {
        let mut buffer: [u8; 3] = [(addr >> 8) as u8 & 0xff, addr as u8 & 0xff, val];
        self.iic_write(&mut buffer)?;
        return Ok(());
    }

    fn wr_sensor_regs_16_8(
        &mut self,
        delay: &mut impl DelayMs<u32>,
        reg_value: &[(u16, u8)],
    ) -> Result<(), ArducamError> {
        for (addr, val) in reg_value {
            if *addr == 0xffff && *val == 0xff {
                return Ok(());
            }
            self.wr_sensor_reg_16_8(*addr, *val)?;
            delay.delay_ms(3);
        }
        return Ok(());
    }

    // fn rd_sensor_reg_8_8(&mut self, addr: u8) -> Result<u8, ()> {
    //     let mut buffer: [u8; 1] = [addr];
    //     // self.iic_write(&mut buffer)?;
    //     self.iic_read(&mut buffer).or(Err(()))?;
    //     return Ok(buffer[0]);
    // }

    // fn wr_sensor_reg_8_8(&mut self, addr: u8, val: u8) -> Result<(), ()> {
    //     let mut buffer: [u8; 2] = [addr, val];
    //     self.iic_write(&mut buffer)?;
    //     return Ok(());
    // }

    // fn wr_sensor_regs_8_8(
    //     &mut self,
    //     delay: &mut impl DelayUs<u32>,
    //     reg_value: Iter<(u8, u8)>,
    // ) -> Result<(), ()> {
    //     for (addr, val) in reg_value {
    //         if *addr == 0xff && *val == 0xff {
    //             return Ok(());
    //         }
    //         self.wr_sensor_reg_8_8(*addr, *val);
    //         delay.delay_us(1_000);
    //     }
    //     return Ok(());
    // }

    pub fn set_jpeg_size(
        &mut self,
        delay: &mut impl DelayMs<u32>,
        size: OV5642Size,
    ) -> Result<(), ArducamError> {
        match size {
            OV5642Size::S320x240 => self.wr_sensor_regs_16_8(delay, &ov5642_320x240)?,
            OV5642Size::S640x480 => self.wr_sensor_regs_16_8(delay, &ov5642_640x480)?,
            OV5642Size::S1024x768 => self.wr_sensor_regs_16_8(delay, &ov5642_1024x768)?,
            OV5642Size::S1280x960 => self.wr_sensor_regs_16_8(delay, &ov5642_1280x960)?,
            OV5642Size::S1600x1200 => self.wr_sensor_regs_16_8(delay, &ov5642_1600x1200)?,
            OV5642Size::S2048x1536 => self.wr_sensor_regs_16_8(delay, &ov5642_2048x1536)?,
            OV5642Size::S2592x1944 => self.wr_sensor_regs_16_8(delay, &ov5642_2592x1944)?,
            _ => self.wr_sensor_regs_16_8(delay, &ov5642_320x240)?,
        };
        return Ok(());
    }

    pub fn set_light_mode(&mut self, mode: LightMode) -> Result<(), ArducamError> {
        match mode {
            LightMode::AdvancedAWB => {
                self.wr_sensor_reg_16_8(0x3406, 0x0)?;
                self.wr_sensor_reg_16_8(0x5192, 0x04)?;
                self.wr_sensor_reg_16_8(0x5191, 0xf8)?;
                self.wr_sensor_reg_16_8(0x518d, 0x26)?;
                self.wr_sensor_reg_16_8(0x518f, 0x42)?;
                self.wr_sensor_reg_16_8(0x518e, 0x2b)?;
                self.wr_sensor_reg_16_8(0x5190, 0x42)?;
                self.wr_sensor_reg_16_8(0x518b, 0xd0)?;
                self.wr_sensor_reg_16_8(0x518c, 0xbd)?;
                self.wr_sensor_reg_16_8(0x5187, 0x18)?;
                self.wr_sensor_reg_16_8(0x5188, 0x18)?;
                self.wr_sensor_reg_16_8(0x5189, 0x56)?;
                self.wr_sensor_reg_16_8(0x518a, 0x5c)?;
                self.wr_sensor_reg_16_8(0x5186, 0x1c)?;
                self.wr_sensor_reg_16_8(0x5181, 0x50)?;
                self.wr_sensor_reg_16_8(0x5184, 0x20)?;
                self.wr_sensor_reg_16_8(0x5182, 0x11)?;
                self.wr_sensor_reg_16_8(0x5183, 0x0)?;
            }
            LightMode::SimpleAWB => {
                self.wr_sensor_reg_16_8(0x3406, 0x00)?;
                self.wr_sensor_reg_16_8(0x5183, 0x80)?;
                self.wr_sensor_reg_16_8(0x5191, 0xff)?;
                self.wr_sensor_reg_16_8(0x5192, 0x00)?;
            }
            LightMode::ManualDay => {
                self.wr_sensor_reg_16_8(0x3406, 0x1)?;
                self.wr_sensor_reg_16_8(0x3400, 0x7)?;
                self.wr_sensor_reg_16_8(0x3401, 0x32)?;
                self.wr_sensor_reg_16_8(0x3402, 0x4)?;
                self.wr_sensor_reg_16_8(0x3403, 0x0)?;
                self.wr_sensor_reg_16_8(0x3404, 0x5)?;
                self.wr_sensor_reg_16_8(0x3405, 0x36)?;
            }
            LightMode::ManualA => {
                self.wr_sensor_reg_16_8(0x3406, 0x1)?;
                self.wr_sensor_reg_16_8(0x3400, 0x4)?;
                self.wr_sensor_reg_16_8(0x3401, 0x88)?;
                self.wr_sensor_reg_16_8(0x3402, 0x4)?;
                self.wr_sensor_reg_16_8(0x3403, 0x0)?;
                self.wr_sensor_reg_16_8(0x3404, 0x8)?;
                self.wr_sensor_reg_16_8(0x3405, 0xb6)?;
            }

            LightMode::Manualcwf => {
                self.wr_sensor_reg_16_8(0x3406, 0x1)?;
                self.wr_sensor_reg_16_8(0x3400, 0x6)?;
                self.wr_sensor_reg_16_8(0x3401, 0x13)?;
                self.wr_sensor_reg_16_8(0x3402, 0x4)?;
                self.wr_sensor_reg_16_8(0x3403, 0x0)?;
                self.wr_sensor_reg_16_8(0x3404, 0x7)?;
                self.wr_sensor_reg_16_8(0x3405, 0xe2)?;
            }
            LightMode::ManualCloudy => {
                self.wr_sensor_reg_16_8(0x3406, 0x1)?;
                self.wr_sensor_reg_16_8(0x3400, 0x7)?;
                self.wr_sensor_reg_16_8(0x3401, 0x88)?;
                self.wr_sensor_reg_16_8(0x3402, 0x4)?;
                self.wr_sensor_reg_16_8(0x3403, 0x0)?;
                self.wr_sensor_reg_16_8(0x3404, 0x5)?;
                self.wr_sensor_reg_16_8(0x3405, 0x0)?;
            }
        }

        return Ok(());
    }

    pub fn set_color_saturation(
        &mut self,
        saturation: CameraSaturation,
    ) -> Result<(), ArducamError> {
        match saturation {
            CameraSaturation::S4 => {
                self.wr_sensor_reg_16_8(0x5001, 0xff)?;
                self.wr_sensor_reg_16_8(0x5583, 0x80)?;
                self.wr_sensor_reg_16_8(0x5584, 0x80)?;
                self.wr_sensor_reg_16_8(0x5580, 0x02)?;
            }
            CameraSaturation::S3 => {
                self.wr_sensor_reg_16_8(0x5001, 0xff)?;
                self.wr_sensor_reg_16_8(0x5583, 0x70)?;
                self.wr_sensor_reg_16_8(0x5584, 0x70)?;
                self.wr_sensor_reg_16_8(0x5580, 0x02)?;
            }
            CameraSaturation::S2 => {
                self.wr_sensor_reg_16_8(0x5001, 0xff)?;
                self.wr_sensor_reg_16_8(0x5583, 0x60)?;
                self.wr_sensor_reg_16_8(0x5584, 0x60)?;
                self.wr_sensor_reg_16_8(0x5580, 0x02)?;
            }
            CameraSaturation::S1 => {
                self.wr_sensor_reg_16_8(0x5001, 0xff)?;
                self.wr_sensor_reg_16_8(0x5583, 0x50)?;
                self.wr_sensor_reg_16_8(0x5584, 0x50)?;
                self.wr_sensor_reg_16_8(0x5580, 0x02)?;
            }
            CameraSaturation::S0 => {
                self.wr_sensor_reg_16_8(0x5001, 0xff)?;
                self.wr_sensor_reg_16_8(0x5583, 0x40)?;
                self.wr_sensor_reg_16_8(0x5584, 0x40)?;
                self.wr_sensor_reg_16_8(0x5580, 0x02)?;
            }
            CameraSaturation::Sn1 => {
                self.wr_sensor_reg_16_8(0x5001, 0xff)?;
                self.wr_sensor_reg_16_8(0x5583, 0x30)?;
                self.wr_sensor_reg_16_8(0x5584, 0x30)?;
                self.wr_sensor_reg_16_8(0x5580, 0x02)?;
            }
            CameraSaturation::Sn2 => {
                self.wr_sensor_reg_16_8(0x5001, 0xff)?;
                self.wr_sensor_reg_16_8(0x5583, 0x20)?;
                self.wr_sensor_reg_16_8(0x5584, 0x20)?;
                self.wr_sensor_reg_16_8(0x5580, 0x02)?;
            }
            CameraSaturation::Sn3 => {
                self.wr_sensor_reg_16_8(0x5001, 0xff)?;
                self.wr_sensor_reg_16_8(0x5583, 0x10)?;
                self.wr_sensor_reg_16_8(0x5584, 0x10)?;
                self.wr_sensor_reg_16_8(0x5580, 0x02)?;
            }
            CameraSaturation::Sn4 => {
                self.wr_sensor_reg_16_8(0x5001, 0xff)?;
                self.wr_sensor_reg_16_8(0x5583, 0x00)?;
                self.wr_sensor_reg_16_8(0x5584, 0x00)?;
                self.wr_sensor_reg_16_8(0x5580, 0x02)?;
            }
        }

        return Ok(());
    }

    pub fn set_brightness(&mut self, brightness: CameraBrightness) -> Result<(), ArducamError> {
        match brightness {
            CameraBrightness::B4 => {
                self.wr_sensor_reg_16_8(0x5001, 0xff)?;
                self.wr_sensor_reg_16_8(0x5589, 0x40)?;
                self.wr_sensor_reg_16_8(0x5580, 0x04)?;
                self.wr_sensor_reg_16_8(0x558a, 0x00)?;
            }
            CameraBrightness::B3 => {
                self.wr_sensor_reg_16_8(0x5001, 0xff)?;
                self.wr_sensor_reg_16_8(0x5589, 0x30)?;
                self.wr_sensor_reg_16_8(0x5580, 0x04)?;
                self.wr_sensor_reg_16_8(0x558a, 0x00)?;
            }
            CameraBrightness::B2 => {
                self.wr_sensor_reg_16_8(0x5001, 0xff)?;
                self.wr_sensor_reg_16_8(0x5589, 0x20)?;
                self.wr_sensor_reg_16_8(0x5580, 0x04)?;
                self.wr_sensor_reg_16_8(0x558a, 0x00)?;
            }
            CameraBrightness::B1 => {
                self.wr_sensor_reg_16_8(0x5001, 0xff)?;
                self.wr_sensor_reg_16_8(0x5589, 0x10)?;
                self.wr_sensor_reg_16_8(0x5580, 0x04)?;
                self.wr_sensor_reg_16_8(0x558a, 0x00)?;
            }
            CameraBrightness::B0 => {
                self.wr_sensor_reg_16_8(0x5001, 0xff)?;
                self.wr_sensor_reg_16_8(0x5589, 0x00)?;
                self.wr_sensor_reg_16_8(0x5580, 0x04)?;
                self.wr_sensor_reg_16_8(0x558a, 0x00)?;
            }
            CameraBrightness::Bn1 => {
                self.wr_sensor_reg_16_8(0x5001, 0xff)?;
                self.wr_sensor_reg_16_8(0x5589, 0x10)?;
                self.wr_sensor_reg_16_8(0x5580, 0x04)?;
                self.wr_sensor_reg_16_8(0x558a, 0x08)?;
            }
            CameraBrightness::Bn2 => {
                self.wr_sensor_reg_16_8(0x5001, 0xff)?;
                self.wr_sensor_reg_16_8(0x5589, 0x20)?;
                self.wr_sensor_reg_16_8(0x5580, 0x04)?;
                self.wr_sensor_reg_16_8(0x558a, 0x08)?;
            }
            CameraBrightness::Bn3 => {
                self.wr_sensor_reg_16_8(0x5001, 0xff)?;
                self.wr_sensor_reg_16_8(0x5589, 0x30)?;
                self.wr_sensor_reg_16_8(0x5580, 0x04)?;
                self.wr_sensor_reg_16_8(0x558a, 0x08)?;
            }
            CameraBrightness::Bn4 => {
                self.wr_sensor_reg_16_8(0x5001, 0xff)?;
                self.wr_sensor_reg_16_8(0x5589, 0x40)?;
                self.wr_sensor_reg_16_8(0x5580, 0x04)?;
                self.wr_sensor_reg_16_8(0x558a, 0x08)?;
            }
        }
        return Ok(());
    }

    pub fn set_contrast(&mut self, contrast: CameraContrast) -> Result<(), ArducamError> {
        match contrast {
            CameraContrast::C4 => {
                self.wr_sensor_reg_16_8(0x5001, 0xff)?;
                self.wr_sensor_reg_16_8(0x5580, 0x04)?;
                self.wr_sensor_reg_16_8(0x5587, 0x30)?;
                self.wr_sensor_reg_16_8(0x5588, 0x30)?;
                self.wr_sensor_reg_16_8(0x558a, 0x00)?;
            }
            CameraContrast::C3 => {
                self.wr_sensor_reg_16_8(0x5001, 0xff)?;
                self.wr_sensor_reg_16_8(0x5580, 0x04)?;
                self.wr_sensor_reg_16_8(0x5587, 0x2c)?;
                self.wr_sensor_reg_16_8(0x5588, 0x2c)?;
                self.wr_sensor_reg_16_8(0x558a, 0x00)?;
            }
            CameraContrast::C2 => {
                self.wr_sensor_reg_16_8(0x5001, 0xff)?;
                self.wr_sensor_reg_16_8(0x5580, 0x04)?;
                self.wr_sensor_reg_16_8(0x5587, 0x28)?;
                self.wr_sensor_reg_16_8(0x5588, 0x28)?;
                self.wr_sensor_reg_16_8(0x558a, 0x00)?;
            }
            CameraContrast::C1 => {
                self.wr_sensor_reg_16_8(0x5001, 0xff)?;
                self.wr_sensor_reg_16_8(0x5580, 0x04)?;
                self.wr_sensor_reg_16_8(0x5587, 0x24)?;
                self.wr_sensor_reg_16_8(0x5588, 0x24)?;
                self.wr_sensor_reg_16_8(0x558a, 0x00)?;
            }
            CameraContrast::C0 => {
                self.wr_sensor_reg_16_8(0x5001, 0xff)?;
                self.wr_sensor_reg_16_8(0x5580, 0x04)?;
                self.wr_sensor_reg_16_8(0x5587, 0x20)?;
                self.wr_sensor_reg_16_8(0x5588, 0x20)?;
                self.wr_sensor_reg_16_8(0x558a, 0x00)?;
            }
            CameraContrast::Cn1 => {
                self.wr_sensor_reg_16_8(0x5001, 0xff)?;
                self.wr_sensor_reg_16_8(0x5580, 0x04)?;
                self.wr_sensor_reg_16_8(0x5587, 0x1C)?;
                self.wr_sensor_reg_16_8(0x5588, 0x1C)?;
                self.wr_sensor_reg_16_8(0x558a, 0x1C)?;
            }
            CameraContrast::Cn2 => {
                self.wr_sensor_reg_16_8(0x5001, 0xff)?;
                self.wr_sensor_reg_16_8(0x5580, 0x04)?;
                self.wr_sensor_reg_16_8(0x5587, 0x18)?;
                self.wr_sensor_reg_16_8(0x5588, 0x18)?;
                self.wr_sensor_reg_16_8(0x558a, 0x00)?;
            }
            CameraContrast::Cn3 => {
                self.wr_sensor_reg_16_8(0x5001, 0xff)?;
                self.wr_sensor_reg_16_8(0x5580, 0x04)?;
                self.wr_sensor_reg_16_8(0x5587, 0x14)?;
                self.wr_sensor_reg_16_8(0x5588, 0x14)?;
                self.wr_sensor_reg_16_8(0x558a, 0x00)?;
            }
            CameraContrast::Cn4 => {
                self.wr_sensor_reg_16_8(0x5001, 0xff)?;
                self.wr_sensor_reg_16_8(0x5580, 0x04)?;
                self.wr_sensor_reg_16_8(0x5587, 0x10)?;
                self.wr_sensor_reg_16_8(0x5588, 0x10)?;
                self.wr_sensor_reg_16_8(0x558a, 0x00)?;
            }
        }
        return Ok(());
    }

    pub fn set_hue(&mut self, degree: CameraHue) -> Result<(), ArducamError> {
        match degree {
            CameraHue::Dn180 => {
                self.wr_sensor_reg_16_8(0x5001, 0xff)?;
                self.wr_sensor_reg_16_8(0x5580, 0x01)?;
                self.wr_sensor_reg_16_8(0x5581, 0x80)?;
                self.wr_sensor_reg_16_8(0x5582, 0x00)?;
                self.wr_sensor_reg_16_8(0x558a, 0x32)?;
            }
            CameraHue::Dn150 => {
                self.wr_sensor_reg_16_8(0x5001, 0xff)?;
                self.wr_sensor_reg_16_8(0x5580, 0x01)?;
                self.wr_sensor_reg_16_8(0x5581, 0x6f)?;
                self.wr_sensor_reg_16_8(0x5582, 0x40)?;
                self.wr_sensor_reg_16_8(0x558a, 0x32)?;
            }
            CameraHue::Dn120 => {
                self.wr_sensor_reg_16_8(0x5001, 0xff)?;
                self.wr_sensor_reg_16_8(0x5580, 0x01)?;
                self.wr_sensor_reg_16_8(0x5581, 0x40)?;
                self.wr_sensor_reg_16_8(0x5582, 0x6f)?;
                self.wr_sensor_reg_16_8(0x558a, 0x32)?;
            }
            CameraHue::Dn90 => {
                self.wr_sensor_reg_16_8(0x5001, 0xff)?;
                self.wr_sensor_reg_16_8(0x5580, 0x01)?;
                self.wr_sensor_reg_16_8(0x5581, 0x00)?;
                self.wr_sensor_reg_16_8(0x5582, 0x80)?;
                self.wr_sensor_reg_16_8(0x558a, 0x02)?;
            }
            CameraHue::Dn60 => {
                self.wr_sensor_reg_16_8(0x5001, 0xff)?;
                self.wr_sensor_reg_16_8(0x5580, 0x01)?;
                self.wr_sensor_reg_16_8(0x5581, 0x40)?;
                self.wr_sensor_reg_16_8(0x5582, 0x6f)?;
                self.wr_sensor_reg_16_8(0x558a, 0x02)?;
            }
            CameraHue::Dn30 => {
                self.wr_sensor_reg_16_8(0x5001, 0xff)?;
                self.wr_sensor_reg_16_8(0x5580, 0x01)?;
                self.wr_sensor_reg_16_8(0x5581, 0x6f)?;
                self.wr_sensor_reg_16_8(0x5582, 0x40)?;
                self.wr_sensor_reg_16_8(0x558a, 0x02)?;
            }
            CameraHue::Dn0 => {
                self.wr_sensor_reg_16_8(0x5001, 0xff)?;
                self.wr_sensor_reg_16_8(0x5580, 0x01)?;
                self.wr_sensor_reg_16_8(0x5581, 0x80)?;
                self.wr_sensor_reg_16_8(0x5582, 0x00)?;
                self.wr_sensor_reg_16_8(0x558a, 0x01)?;
            }
            CameraHue::D30 => {
                self.wr_sensor_reg_16_8(0x5001, 0xff)?;
                self.wr_sensor_reg_16_8(0x5580, 0x01)?;
                self.wr_sensor_reg_16_8(0x5581, 0x6f)?;
                self.wr_sensor_reg_16_8(0x5582, 0x40)?;
                self.wr_sensor_reg_16_8(0x558a, 0x01)?;
            }
            CameraHue::D60 => {
                self.wr_sensor_reg_16_8(0x5001, 0xff)?;
                self.wr_sensor_reg_16_8(0x5580, 0x01)?;
                self.wr_sensor_reg_16_8(0x5581, 0x40)?;
                self.wr_sensor_reg_16_8(0x5582, 0x6f)?;
                self.wr_sensor_reg_16_8(0x558a, 0x01)?;
            }
            CameraHue::D90 => {
                self.wr_sensor_reg_16_8(0x5001, 0xff)?;
                self.wr_sensor_reg_16_8(0x5580, 0x01)?;
                self.wr_sensor_reg_16_8(0x5581, 0x00)?;
                self.wr_sensor_reg_16_8(0x5582, 0x80)?;
                self.wr_sensor_reg_16_8(0x558a, 0x31)?;
            }
            CameraHue::D120 => {
                self.wr_sensor_reg_16_8(0x5001, 0xff)?;
                self.wr_sensor_reg_16_8(0x5580, 0x01)?;
                self.wr_sensor_reg_16_8(0x5581, 0x40)?;
                self.wr_sensor_reg_16_8(0x5582, 0x6f)?;
                self.wr_sensor_reg_16_8(0x558a, 0x31)?;
            }
            CameraHue::D150 => {
                self.wr_sensor_reg_16_8(0x5001, 0xff)?;
                self.wr_sensor_reg_16_8(0x5580, 0x01)?;
                self.wr_sensor_reg_16_8(0x5581, 0x6f)?;
                self.wr_sensor_reg_16_8(0x5582, 0x40)?;
                self.wr_sensor_reg_16_8(0x558a, 0x31)?;
            }
        }
        return Ok(());
    }

    pub fn set_special_effects(&mut self, effect: CameraEffect) -> Result<(), ArducamError> {
        match effect {
            CameraEffect::Blueish => {
                self.wr_sensor_reg_16_8(0x5001, 0xff)?;
                self.wr_sensor_reg_16_8(0x5580, 0x18)?;
                self.wr_sensor_reg_16_8(0x5585, 0xa0)?;
                self.wr_sensor_reg_16_8(0x5586, 0x40)?;
            }
            CameraEffect::Greenish => {
                self.wr_sensor_reg_16_8(0x5001, 0xff)?;
                self.wr_sensor_reg_16_8(0x5580, 0x18)?;
                self.wr_sensor_reg_16_8(0x5585, 0x60)?;
                self.wr_sensor_reg_16_8(0x5586, 0x60)?;
            }
            CameraEffect::Reddish => {
                self.wr_sensor_reg_16_8(0x5001, 0xff)?;
                self.wr_sensor_reg_16_8(0x5580, 0x18)?;
                self.wr_sensor_reg_16_8(0x5585, 0x80)?;
                self.wr_sensor_reg_16_8(0x5586, 0xc0)?;
            }
            CameraEffect::BW => {
                self.wr_sensor_reg_16_8(0x5001, 0xff)?;
                self.wr_sensor_reg_16_8(0x5580, 0x18)?;
                self.wr_sensor_reg_16_8(0x5585, 0x80)?;
                self.wr_sensor_reg_16_8(0x5586, 0x80)?;
            }
            CameraEffect::Negative => {
                self.wr_sensor_reg_16_8(0x5001, 0xff)?;
                self.wr_sensor_reg_16_8(0x5580, 0x40)?;
            }
            CameraEffect::Sepia => {
                self.wr_sensor_reg_16_8(0x5001, 0xff)?;
                self.wr_sensor_reg_16_8(0x5580, 0x18)?;
                self.wr_sensor_reg_16_8(0x5585, 0x40)?;
                self.wr_sensor_reg_16_8(0x5586, 0xa0)?;
            }
            CameraEffect::Normal => {
                self.wr_sensor_reg_16_8(0x5001, 0x7f)?;
                self.wr_sensor_reg_16_8(0x5580, 0x00)?;
            }
            _ => {}
        }
        return Ok(());
    }

    pub fn set_exposure_level(&mut self, level: CameraExposure) -> Result<(), ArducamError> {
        match level {
            CameraExposure::En17 => {
                self.wr_sensor_reg_16_8(0x3a0f, 0x10)?;
                self.wr_sensor_reg_16_8(0x3a10, 0x08)?;
                self.wr_sensor_reg_16_8(0x3a1b, 0x10)?;
                self.wr_sensor_reg_16_8(0x3a1e, 0x08)?;
                self.wr_sensor_reg_16_8(0x3a11, 0x20)?;
                self.wr_sensor_reg_16_8(0x3a1f, 0x10)?;
            }
            CameraExposure::En13 => {
                self.wr_sensor_reg_16_8(0x3a0f, 0x18)?;
                self.wr_sensor_reg_16_8(0x3a10, 0x10)?;
                self.wr_sensor_reg_16_8(0x3a1b, 0x18)?;
                self.wr_sensor_reg_16_8(0x3a1e, 0x10)?;
                self.wr_sensor_reg_16_8(0x3a11, 0x30)?;
                self.wr_sensor_reg_16_8(0x3a1f, 0x10)?;
            }
            CameraExposure::En10 => {
                self.wr_sensor_reg_16_8(0x3a0f, 0x20)?;
                self.wr_sensor_reg_16_8(0x3a10, 0x18)?;
                self.wr_sensor_reg_16_8(0x3a11, 0x41)?;
                self.wr_sensor_reg_16_8(0x3a1b, 0x20)?;
                self.wr_sensor_reg_16_8(0x3a1e, 0x18)?;
                self.wr_sensor_reg_16_8(0x3a1f, 0x10)?;
            }
            CameraExposure::En07 => {
                self.wr_sensor_reg_16_8(0x3a0f, 0x28)?;
                self.wr_sensor_reg_16_8(0x3a10, 0x20)?;
                self.wr_sensor_reg_16_8(0x3a11, 0x51)?;
                self.wr_sensor_reg_16_8(0x3a1b, 0x28)?;
                self.wr_sensor_reg_16_8(0x3a1e, 0x20)?;
                self.wr_sensor_reg_16_8(0x3a1f, 0x10)?;
            }
            CameraExposure::En03 => {
                self.wr_sensor_reg_16_8(0x3a0f, 0x30)?;
                self.wr_sensor_reg_16_8(0x3a10, 0x28)?;
                self.wr_sensor_reg_16_8(0x3a11, 0x61)?;
                self.wr_sensor_reg_16_8(0x3a1b, 0x30)?;
                self.wr_sensor_reg_16_8(0x3a1e, 0x28)?;
                self.wr_sensor_reg_16_8(0x3a1f, 0x10)?;
            }
            CameraExposure::EDefault => {
                self.wr_sensor_reg_16_8(0x3a0f, 0x38)?;
                self.wr_sensor_reg_16_8(0x3a10, 0x30)?;
                self.wr_sensor_reg_16_8(0x3a11, 0x61)?;
                self.wr_sensor_reg_16_8(0x3a1b, 0x38)?;
                self.wr_sensor_reg_16_8(0x3a1e, 0x30)?;
                self.wr_sensor_reg_16_8(0x3a1f, 0x10)?;
            }
            CameraExposure::E03 => {
                self.wr_sensor_reg_16_8(0x3a0f, 0x40)?;
                self.wr_sensor_reg_16_8(0x3a10, 0x38)?;
                self.wr_sensor_reg_16_8(0x3a11, 0x71)?;
                self.wr_sensor_reg_16_8(0x3a1b, 0x40)?;
                self.wr_sensor_reg_16_8(0x3a1e, 0x38)?;
                self.wr_sensor_reg_16_8(0x3a1f, 0x10)?;
            }
            CameraExposure::E07 => {
                self.wr_sensor_reg_16_8(0x3a0f, 0x48)?;
                self.wr_sensor_reg_16_8(0x3a10, 0x40)?;
                self.wr_sensor_reg_16_8(0x3a11, 0x80)?;
                self.wr_sensor_reg_16_8(0x3a1b, 0x48)?;
                self.wr_sensor_reg_16_8(0x3a1e, 0x40)?;
                self.wr_sensor_reg_16_8(0x3a1f, 0x20)?;
            }
            CameraExposure::E10 => {
                self.wr_sensor_reg_16_8(0x3a0f, 0x50)?;
                self.wr_sensor_reg_16_8(0x3a10, 0x48)?;
                self.wr_sensor_reg_16_8(0x3a11, 0x90)?;
                self.wr_sensor_reg_16_8(0x3a1b, 0x50)?;
                self.wr_sensor_reg_16_8(0x3a1e, 0x48)?;
                self.wr_sensor_reg_16_8(0x3a1f, 0x20)?;
            }
            CameraExposure::E13 => {
                self.wr_sensor_reg_16_8(0x3a0f, 0x58)?;
                self.wr_sensor_reg_16_8(0x3a10, 0x50)?;
                self.wr_sensor_reg_16_8(0x3a11, 0x91)?;
                self.wr_sensor_reg_16_8(0x3a1b, 0x58)?;
                self.wr_sensor_reg_16_8(0x3a1e, 0x50)?;
                self.wr_sensor_reg_16_8(0x3a1f, 0x20)?;
            }
            CameraExposure::E17 => {
                self.wr_sensor_reg_16_8(0x3a0f, 0x60)?;
                self.wr_sensor_reg_16_8(0x3a10, 0x58)?;
                self.wr_sensor_reg_16_8(0x3a11, 0xa0)?;
                self.wr_sensor_reg_16_8(0x3a1b, 0x60)?;
                self.wr_sensor_reg_16_8(0x3a1e, 0x58)?;
                self.wr_sensor_reg_16_8(0x3a1f, 0x20)?;
            }
        }
        return Ok(());
    }

    pub fn set_sharpness(&mut self, sharpness: CameraSharpness) -> Result<(), ArducamError> {
        match sharpness {
            CameraSharpness::AutoDefault => {
                self.wr_sensor_reg_16_8(0x530A, 0x00)?;
                self.wr_sensor_reg_16_8(0x530c, 0x0)?;
                self.wr_sensor_reg_16_8(0x530d, 0xc)?;
                self.wr_sensor_reg_16_8(0x5312, 0x40)?;
            }
            CameraSharpness::Auto1 => {
                self.wr_sensor_reg_16_8(0x530A, 0x00)?;
                self.wr_sensor_reg_16_8(0x530c, 0x4)?;
                self.wr_sensor_reg_16_8(0x530d, 0x18)?;
                self.wr_sensor_reg_16_8(0x5312, 0x20)?;
            }
            CameraSharpness::Auto2 => {
                self.wr_sensor_reg_16_8(0x530A, 0x00)?;
                self.wr_sensor_reg_16_8(0x530c, 0x8)?;
                self.wr_sensor_reg_16_8(0x530d, 0x30)?;
                self.wr_sensor_reg_16_8(0x5312, 0x10)?;
            }
            CameraSharpness::ManualOff => {
                self.wr_sensor_reg_16_8(0x530A, 0x08)?;
                self.wr_sensor_reg_16_8(0x531e, 0x00)?;
                self.wr_sensor_reg_16_8(0x531f, 0x00)?;
            }
            CameraSharpness::Manual1 => {
                self.wr_sensor_reg_16_8(0x530A, 0x08)?;
                self.wr_sensor_reg_16_8(0x531e, 0x04)?;
                self.wr_sensor_reg_16_8(0x531f, 0x04)?;
            }
            CameraSharpness::Manual2 => {
                self.wr_sensor_reg_16_8(0x530A, 0x08)?;
                self.wr_sensor_reg_16_8(0x531e, 0x08)?;
                self.wr_sensor_reg_16_8(0x531f, 0x08)?;
            }
            CameraSharpness::Manual3 => {
                self.wr_sensor_reg_16_8(0x530A, 0x08)?;
                self.wr_sensor_reg_16_8(0x531e, 0x0c)?;
                self.wr_sensor_reg_16_8(0x531f, 0x0c)?;
            }
            CameraSharpness::Manual4 => {
                self.wr_sensor_reg_16_8(0x530A, 0x08)?;
                self.wr_sensor_reg_16_8(0x531e, 0x0f)?;
                self.wr_sensor_reg_16_8(0x531f, 0x0f)?;
            }
            CameraSharpness::Manual5 => {
                self.wr_sensor_reg_16_8(0x530A, 0x08)?;
                self.wr_sensor_reg_16_8(0x531e, 0x1f)?;
                self.wr_sensor_reg_16_8(0x531f, 0x1f)?;
            }
        }
        return Ok(());
    }

    pub fn set_mirror_flip(&mut self, mirror_flip: MirrorFlip) -> Result<(), ArducamError> {
        match mirror_flip {
            MirrorFlip::Mirror => {
                {
                    let mut reg_val = self.rd_sensor_reg_16_8(0x3818)?;
                    reg_val = reg_val | 0x00;
                    reg_val = reg_val & 0x9F;
                    self.wr_sensor_reg_16_8(0x3818, reg_val)?;
                }
                {
                    let mut reg_val = self.rd_sensor_reg_16_8(0x3621)?;
                    reg_val = reg_val | 0x20;
                    self.wr_sensor_reg_16_8(0x3621, reg_val)?;
                }
            }
            MirrorFlip::Flip => {
                {
                    let mut reg_val = self.rd_sensor_reg_16_8(0x3818)?;
                    reg_val = reg_val | 0x20;
                    reg_val = reg_val & 0xbF;
                    self.wr_sensor_reg_16_8(0x3818, reg_val)?;
                }
                {
                    let mut reg_val = self.rd_sensor_reg_16_8(0x3621)?;
                    reg_val = reg_val | 0x20;
                    self.wr_sensor_reg_16_8(0x3621, reg_val)?;
                }
            }
            MirrorFlip::MirrorFlip => {
                {
                    let mut reg_val = self.rd_sensor_reg_16_8(0x3818)?;
                    reg_val = reg_val | 0x60;
                    reg_val = reg_val & 0xFF;
                    self.wr_sensor_reg_16_8(0x3818, reg_val)?;
                }
                {
                    let mut reg_val = self.rd_sensor_reg_16_8(0x3621)?;
                    reg_val = reg_val & 0xdf;
                    self.wr_sensor_reg_16_8(0x3621, reg_val)?;
                }
            }
            MirrorFlip::Normal => {
                {
                    let mut reg_val = self.rd_sensor_reg_16_8(0x3818)?;
                    reg_val = reg_val | 0x40;
                    reg_val = reg_val & 0xdF;
                    self.wr_sensor_reg_16_8(0x3818, reg_val)?;
                }
                {
                    let mut reg_val = self.rd_sensor_reg_16_8(0x3621)?;
                    reg_val = reg_val & 0xdf;
                    self.wr_sensor_reg_16_8(0x3621, reg_val)?;
                }
            }
        }
        return Ok(());
    }

    pub fn set_compress_quality(&mut self, quality: CameraQuality) -> Result<(), ArducamError> {
        match quality {
            CameraQuality::High => self.wr_sensor_reg_16_8(0x4407, 0x02)?,
            CameraQuality::Default => self.wr_sensor_reg_16_8(0x4407, 0x04)?,
            CameraQuality::Low => self.wr_sensor_reg_16_8(0x4407, 0x08)?,
        }
        return Ok(());
    }

    pub fn test_pattern(&mut self, pattern: CameraPattern) -> Result<(), ArducamError> {
        match pattern {
            CameraPattern::ColorBar => {
                self.wr_sensor_reg_16_8(0x503d, 0x80)?;
                self.wr_sensor_reg_16_8(0x503e, 0x00)?;
            }
            CameraPattern::ColorSquare => {
                self.wr_sensor_reg_16_8(0x503d, 0x85)?;
                self.wr_sensor_reg_16_8(0x503e, 0x12)?;
            }
            CameraPattern::BWSquare => {
                self.wr_sensor_reg_16_8(0x503d, 0x85)?;
                self.wr_sensor_reg_16_8(0x503e, 0x1a)?;
            }
            CameraPattern::DLI => {
                self.wr_sensor_reg_16_8(0x4741, 0x4)?;
            }
        }
        return Ok(());
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
