//! A platform agnostic driver to interface with the FT5336 touch controller
//!
//! This driver was build using ['embedded-hal'] traits.
//!
//! [`embedded-hal`]: https://docs.rs/embedded-hal

#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]

#[allow(unused_imports)]
use panic_semihosting;

const FT5336_DEV_MODE_REG: u8 = 0x00;
const FT5336_DEV_MODE_BIT_MASK: u8 = 0x70;
const FT5336_DEV_MODE_BIT_POSITION: u8 = 4;

const FT5336_TD_STAT_REG: u8 = 0x02;
const FT5336_P1_XH_TP_BIT_MASK: u8 = 0x0F;
const FT5336_P1_YH_TP_BIT_MASK: u8 = 0x0F;

const FT5336_OK: u8 = 0x00;

// /* Max detectable simultaneous touches */
const FT5336_MAX_NB_TOUCH: u8 = 5;

// /* Touch FT5336 IDs */
// const FT5336_ID: u8 = 0x51;

// /* Possible values of FT5336_DEV_MODE_REG */
const FT5336_DEV_MODE_WORKING: u8 = 0x00;
const FT5336_DEV_MODE_FACTORY: u8 = 0x04;
// First touch register:
const FT5336_P1_XH_REG: u8 = 0x03;

// /* Possible values of FT5336_GEST_ID_REG */
const FT5336_GEST_ID_NO_GESTURE: u8 = 0x00;
const FT5336_GEST_ID_MOVE_UP: u8 = 0x10;
const FT5336_GEST_ID_MOVE_RIGHT: u8 = 0x14;
const FT5336_GEST_ID_MOVE_DOWN: u8 = 0x18;
const FT5336_GEST_ID_MOVE_LEFT: u8 = 0x1C;
const FT5336_GEST_ID_ZOOM_IN: u8 = 0x48;
const FT5336_GEST_ID_ZOOM_OUT: u8 = 0x49;

const FT5336_CHIP_ID_REG: u8 = 0xA8;
const FT5336_FIRMID_REG: u8 = 0xA6;
const FT5336_ID: u8 = 0x51;

// /* Values Pn_XH and Pn_YH related */
// const FT5336_TOUCH_EVT_FLAG_PRESS_DOWN: u8 = 0x00;
// const FT5336_TOUCH_EVT_FLAG_LIFT_UP: u8 = 0x01;
// const FT5336_TOUCH_EVT_FLAG_CONTACT: u8 = 0x02;
// const FT5336_TOUCH_EVT_FLAG_NO_EVENT: u8 = 0x03;

// /* Possible values of FT5336_GMODE_REG */
// const FT5336_G_MODE_INTERRUPT_POLLING: u8 = 0x00;
// const FT5336_G_MODE_INTERRUPT_TRIGGER: u8 = 0x01;

const FT5336_GEST_ID_REG: u8 = 0x01;
const FT5336_RADIAN_VALUE_REG: u8 = 0x91;

const FT5336_AUTO_CALIBRATION_ENABLED: bool = false;
const FT5336_MAX_X_LENGTH: u16 = 800_u16;
const FT5336_MAX_Y_LENGTH: u16 = 480_u16;

/// The standard device I2C address is here as a define, but
/// it is left as a parameter in the code so that it can be used
/// behind a device address translator if required.
#[allow(dead_code)]
const FT5336_TOUCHPAD_ADDR: u8 = 0x38;

use embedded_hal as hal;
use hal::blocking::{delay::DelayMs, i2c};

use core::marker::PhantomData;

/// A simple collection of the capabilities of the chip
/// later realised in a const.
#[derive(Copy, Clone, Debug)]
pub struct Ft5336Capabilities {
    #[allow(dead_code)]
    multi_touch: bool,
    #[allow(dead_code)]
    gesture: bool,
    #[allow(dead_code)]
    max_touch: u8,
    #[allow(dead_code)]
    max_x_length: u16,
    #[allow(dead_code)]
    may_y_length: u16,
}

const TRUE: bool = true;

const FT5336_CAPABILITIES: Ft5336Capabilities = Ft5336Capabilities {
    multi_touch: TRUE,
    gesture: TRUE,
    max_touch: FT5336_MAX_NB_TOUCH,
    max_x_length: FT5336_MAX_X_LENGTH,
    may_y_length: FT5336_MAX_Y_LENGTH,
};

/// Touch structure - derived from the available I2C registers
/// There are ten available touch registers on the chip, but also
/// a defined maximum of 5 in FT5336_MAX_NB_TOUCH above.
/// The touch registers occur in banks of 6, for each of the ten
/// potential touches, defined as follows, and the registers are
/// contiguous. That means that a single read can get all of the
/// data for one touch, or all of the data for all the touches.
/// In the absence of documentation on the MISC register, it is being
/// ignored.
// #define FT5336_P1_XH_REG            0x03U
// #define FT5336_P1_XL_REG            0x04U
// #define FT5336_P1_YH_REG            0x05U
// #define FT5336_P1_YL_REG            0x06U
// #define FT5336_P1_WEIGHT_REG        0x07U
// #define FT5336_P1_MISC_REG          0x08U
//   followed by:
// #define FT5336_P2_XH_REG            0x09U
// etc
#[derive(Copy, Clone, Debug)]
pub struct TouchState {
    /// Was a touch detected:
    pub detected: bool,
    /// X postion
    pub x: u16,
    /// Y position
    pub y: u16,
    /// Weight of touch
    pub weight: u8,
    /// Misc (contents not known)
    pub misc: u8,
}

/// When a gesture is polled it could be one of these:
pub enum GestureKind {
    /// No gesture detected
    None,
    /// Up gesture
    Up,
    /// Right gesture
    Right,
    /// Down gesture
    Down,
    /// Left gesture
    Left,
    /// ZoomIn gesture
    ZoomIn,
    /// ZoomOut gesture
    ZoomOut,
    /// Fault gesture
    Fault,
}

/// Structure that holds the values for a gesture
/// The name is what's in the c code.
/// The register definitions are:
/// pub const FT5336_RADIAN_VALUE_REG: u8 = 0x91;
/// pub const FT5336_OFFSET_LR_REG: u8 = 0x92;
/// pub const FT5336_OFFSET_UD_REG: u8 = 0x93;
/// pub const FT5336_DISTANCE_LR_REG: u8 = 0x94;
/// pub const FT5336_DISTANCE_UD_REG: u8 = 0x95;
/// pub const FT5336_DISTANCE_ZOOM_REG: u8 = 0x96;
pub struct GestureInit<I2C> {
    addr: u8,
    i2c: PhantomData<I2C>,

    /// radians required to sense a circle (probably not used)
    pub radian: u8,
    /// Offset distance left right
    pub offset_left_right: u8,
    /// Offset distance up down
    pub offset_up_down: u8,
    /// Distance for swipes left right
    pub distance_left_right: u8,
    /// Distance for swipes up down
    pub distance_up_down: u8,
    /// Distance for zoom
    pub distance_zoom: u8,
}

/// I wasn't able to get gestures to work. I suspect something is required in
/// the control register, but I don't know what. Also, this STM page (for nominally the same device):
/// <https://github.com/ryankurte/stm32-base/blob/master/drivers/BSP/Components/ft5336/ft5336.c>
/// has a different set of gestures available to the list above.
impl<'b, I2C, E> GestureInit<I2C>
where
    I2C: i2c::WriteRead<Error = E> + i2c::Write<Error = E> + i2c::Read<Error = E>,
{
    /// Initialise. Takes the I2C address just to avoid transferring it all the time.
    /// It turns out the gesture init registers are contiguous, see comment above
    /// or definitions of FT5336_RADIAN_VALUE_REG and what follow, so they're also
    /// in the initialiser.
    ///
    /// This code didn't work in the STM32F7 Discovery - It wouldn't read parameters set.
    pub fn new(addr: u8) -> GestureInit<I2C> {
        GestureInit {
            i2c: PhantomData,
            addr,
            radian: 0,
            offset_left_right: 0,
            offset_up_down: 0,
            distance_left_right: 0,
            distance_up_down: 0,
            distance_zoom: 0,
        }
    }

    /// Fill the gesture struct with the values held for it on the
    /// touchscreen
    pub fn read(&mut self, i2c: &mut I2C) -> Result<&str, &str> {
        let mut buf: [u8; 6] = [4; 6];
        let result = i2c.write_read(self.addr, &[FT5336_RADIAN_VALUE_REG], &mut buf);

        match result {
            Err(_e) => Err("Error reading gesture init registers"),
            Ok(_d) => {
                self.radian = buf[0];
                self.offset_left_right = buf[1];
                self.offset_up_down = buf[2];
                self.distance_left_right = buf[3];
                self.distance_up_down = buf[4];
                self.distance_zoom = buf[5];
                Ok("Success reading gesture init")
            }
        }
    }

    /// Write the six parameters of the gesture_init type into the FT5663
    pub fn write(
        &mut self,
        i2c: &mut I2C,
        radian: u8,
        offset_lr: u8,
        offset_ud: u8,
        dist_lr: u8,
        dist_up: u8,
        zoom: u8,
    ) -> Result<&str, &str> {
        let mut entries: [u8; 6] = [radian, offset_lr, offset_ud, dist_lr, dist_up, zoom];

        let result = i2c.write_read(self.addr, &mut [FT5336_RADIAN_VALUE_REG], &mut entries);
        if let Err(_g) = result {
            Err("Error setting address in GestureInit")
        } else {
            // let result = i2c.write(self.addr, &mut entries);
            // match result {
            // Err(_e) => Err("Error writing GestureInit"),
            Ok("Okay writing GestureInit")
            // }
        }
    }
}

/// FT5883 driver
pub struct Ft5336<'a, I2C> {
    i2c: PhantomData<I2C>,
    addr: u8,
    delay: &'a mut dyn DelayMs<u32>,
}

impl<'a, I2C, E> Ft5336<'a, I2C>
where
    I2C: i2c::WriteRead<Error = E> + i2c::Write<Error = E>,
{
    /// Creates a new sensor associated with an I2C peripheral.
    ///
    /// Phantom I2C ensures that whatever I2C bus the device was created on is the one that is used for all future interations.
    pub fn new(_i2c: &I2C, addr: u8, delay_source: &'a mut impl DelayMs<u32>) -> Result<Self, E> {
        let ft5336 = Ft5336 {
            i2c: PhantomData,
            addr: addr,
            delay: delay_source,
        };
        Ok(ft5336)
    }

    /// Initialise device and disable interupt mode.
    /// FT5336 should be calibrated once after each power up.
    pub fn init(&mut self, i2c: &mut I2C) {
        // -> Result<Self, E> {
        if FT5336_AUTO_CALIBRATION_ENABLED {
            self.ts_calibration(i2c).unwrap();
        }
        // FT5336_DisableIT(i2c)?;
        // Ok(*self)
    }

    //pub fn DisableIT(&self, i2c: &mut I2C) -> Result<u8, E> {}
    /// Future test code
    pub fn test(&self, _i2c: &mut I2C) {}

    ///As the ft5336 library owns the delay, the simplest way to
    /// deliver it to the callign code seems to be to return a function call.
    pub fn delay_ms(&mut self, delay: u32) {
        self.delay.delay_ms(delay);
    }

    /// Returns the structure that contains all the preset capabilities
    /// of the FT5336
    pub fn get_capabilities(&self) -> Ft5336Capabilities {
        FT5336_CAPABILITIES
    }

    /// Read whether the FT5663 is in dev mode or not
    pub fn dev_mode_r(&self, i2c: &mut I2C) -> Result<u8, E> {
        let mut buf: [u8; 1] = [0];

        i2c.write_read(self.addr, &[FT5336_DEV_MODE_REG], &mut buf)?;

        let mut value = buf[0];
        value &= FT5336_DEV_MODE_BIT_MASK;
        value &= FT5336_DEV_MODE_BIT_POSITION;

        Ok(value)
    }

    /// Put the FT5663 into dev mode
    pub fn dev_mode_w(&self, i2c: &mut I2C, value: u8) -> Result<bool, E> {
        let mut buf: [u8; 1] = [0];

        i2c.write_read(self.addr, &[FT5336_DEV_MODE_REG], &mut buf)?;

        let mut tmp = buf[0];

        tmp &= !FT5336_DEV_MODE_BIT_MASK;
        tmp |= value << FT5336_DEV_MODE_BIT_POSITION;

        i2c.write(self.addr, &[tmp])?;

        Ok(value == 0)
    }

    /// Run an internal calibration on the FT5336
    pub fn ts_calibration(&mut self, i2c: &mut I2C) -> Result<bool, &str> {
        //} -> Result<Self, E> {
        let mut _ret = FT5336_OK;
        let mut _nbr_attempt: u32;
        let mut _read_data: u8;
        let mut _end_calibration: u8;

        let _result = self.dev_mode_w(i2c, FT5336_DEV_MODE_FACTORY);

        self.delay.delay_ms(300);

        for _attempt in 0..100 {
            match self.dev_mode_r(i2c) {
                Err(_e) => return Err("Bad comms in ts_calibration"),
                Ok(n) => {
                    if n == FT5336_DEV_MODE_WORKING {
                        return Ok(true);
                    }
                }
            }
            self.delay.delay_ms(200);
        }
        Err("Calibration does not return")
    }

    /// Read the touch device status
    pub fn td_status(&self, i2c: &mut I2C) -> Result<u8, E> {
        let mut buf: [u8; 1] = [0];
        i2c.write_read(self.addr, &[FT5336_TD_STAT_REG], &mut buf)?;
        Ok(buf[0])
    }

    /// Read the touch device chip ID. It should be 0x51 if it is the FT5336 on the
    /// stm32f746 Discovery board
    pub fn chip_id(&self, i2c: &mut I2C) -> Result<u8, &str> {
        let mut buf: [u8; 1] = [0];
        match i2c.write_read(self.addr, &[FT5336_CHIP_ID_REG], &mut buf) {
            Err(_e) => Err("Chip ID call failed"),
            Ok(_a) => {
                if buf[0] != FT5336_ID {
                    Err("error in chip ID")
                } else {
                    Ok(buf[0])
                }
            }
        }
    }

    /// Is the device being touched? If so, how many fingers?
    pub fn detect_touch(&mut self, i2c: &mut I2C) -> Result<u8, &str> {
        match self.td_status(i2c) {
            Err(_e) => Err("Error getting touch data"),
            Ok(n) => {
                if n < FT5336_MAX_NB_TOUCH {
                    Ok(n)
                } else {
                    Ok(0)
                }
            }
        }
    }

    /// Retrieve the FT5336 firmware id
    pub fn firmware_id(&mut self, i2c: &mut I2C) -> Result<u8, &str> {
        let mut buf: [u8; 1] = [0];
        match i2c.write_read(self.addr, &[FT5336_FIRMID_REG], &mut buf) {
            Err(_e) => Err("Error getting firmware ID"),
            Ok(_d) => Ok(buf[0]),
        }
    }

    /// Retrieve the Gesture Init variable
    pub fn gesture_radian_read(&mut self, i2c: &mut I2C) -> Result<u8, &str> {
        let mut buf: [u8; 1] = [0];
        match i2c.write_read(self.addr, &[FT5336_RADIAN_VALUE_REG], &mut buf) {
            Err(_e) => Err("Error getting Gesture Init: RADIAN VALUE REG"),
            Ok(_d) => Ok(buf[0]),
        }
    }

    /// Write the Gesture Init variable
    pub fn gesture_radian_write(&self, i2c: &mut I2C, value: u8) -> Result<bool, E> {
        let mut buf: [u8; 1] = [value];

        i2c.write_read(self.addr, &[FT5336_RADIAN_VALUE_REG], &mut buf)?;

        Ok(value == 0)
    }

    /// Fetch the touch data specified by touch_i
    /// touch_i should go from 1 to FT5336_MAX_NB_TOUCH
    pub fn get_touch(&mut self, i2c: &mut I2C, touch_i: u8) -> Result<TouchState, E> {
        let mut buf: [u8; 6] = [0; 6];
        i2c.write_read(self.addr, &[FT5336_P1_XH_REG + 6 * (touch_i - 1)], &mut buf)?;

        // Tried copying the c code literally here. It makes no difference though
        let x: u16 = (FT5336_P1_XH_TP_BIT_MASK & buf[0]) as u16 * 256 + buf[1] as u16;
        let y: u16 = (FT5336_P1_YH_TP_BIT_MASK & buf[2]) as u16 * 256 + buf[3] as u16;

        Ok(TouchState {
            detected: true,
            x,
            y,
            weight: buf[4],
            misc: buf[5],
        })
    }

    /// Get gestures interpreted by touchscreen
    pub fn get_gesture(&mut self, i2c: &mut I2C) -> Result<GestureKind, E> {
        let mut buf: [u8; 1] = [0];
        i2c.write_read(self.addr, &[FT5336_GEST_ID_REG], &mut buf)?;

        let g: GestureKind = match buf[0] {
            FT5336_GEST_ID_NO_GESTURE => GestureKind::None,
            FT5336_GEST_ID_MOVE_UP => GestureKind::Up,
            FT5336_GEST_ID_MOVE_RIGHT => GestureKind::Right,
            FT5336_GEST_ID_MOVE_DOWN => GestureKind::Down,
            FT5336_GEST_ID_MOVE_LEFT => GestureKind::Left,
            FT5336_GEST_ID_ZOOM_IN => GestureKind::ZoomIn,
            FT5336_GEST_ID_ZOOM_OUT => GestureKind::ZoomOut,
            _ => GestureKind::Fault,
        };
        Ok(g)
    }
}
