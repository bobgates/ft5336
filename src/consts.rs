/* Current mode register of the FT5336 (R/W) */
pub const FT5336_DEV_MODE_REG: u8 = 0x00;
pub const FT5336_DEV_MODE_BIT_MASK: u8 = 0x70;
pub const FT5336_DEV_MODE_BIT_POSITION: u8 = 4;

// /* Gesture ID register */
// pub const FT5336_GEST_ID_REG: u8 = 0x01;

// /* Touch Data Status register : gives number of active touch points (0..2) */
pub const FT5336_TD_STAT_REG: u8 = 0x02;
pub const FT5336_TD_STATUS_BIT_MASK: u8 = 0x0F;
pub const FT5336_TD_STATUS_BIT_POSITION: u8 = 0;

// /* P1 X, Y coordinates, weight and misc registers */
// pub const FT5336_P1_XH_REG: u8 = 0x03;
// pub const FT5336_P1_XL_REG: u8 = 0x04;
// pub const FT5336_P1_YH_REG: u8 = 0x05;
// pub const FT5336_P1_YL_REG: u8 = 0x06;
// pub const FT5336_P1_WEIGHT_REG: u8 = 0x07;
// pub const FT5336_P1_MISC_REG: u8 = 0x08;

pub const FT5336_P1_XH_TP_BIT_MASK: u8 = 0x0F;
pub const FT5336_P1_XH_TP_BIT_POSITION: u8 = 0;
pub const FT5336_P1_XL_TP_BIT_MASK: u8 = 0xFF;
pub const FT5336_P1_XL_TP_BIT_POSITION: u8 = 0;
pub const FT5336_P1_YH_TID_BIT_MASK: u8 = 0xF0;
pub const FT5336_P1_YH_TID_BIT_POSITION: u8 = 4;
pub const FT5336_P1_YH_TP_BIT_MASK: u8 = 0x0F;
pub const FT5336_P1_YH_TP_BIT_POSITION: u8 = 0;
pub const FT5336_P1_YL_TP_BIT_MASK: u8 = 0xFF;
pub const FT5336_P1_YL_TP_BIT_POSITION: u8 = 0;
pub const FT5336_P1_WEIGHT_BIT_MASK: u8 = 0xFF;
pub const FT5336_P1_WEIGHT_BIT_POSITION: u8 = 0;
pub const FT5336_P1_MISC_BIT_MASK: u8 = 0xF0;
pub const FT5336_P1_MISC_BIT_POSITION: u8 = 4;

// /* P2 X, Y coordinates, weight and misc registers */
// pub const FT5336_P2_XH_REG: u8 = 0x09;
// pub const FT5336_P2_XL_REG: u8 = 0x0A;
// pub const FT5336_P2_YH_REG: u8 = 0x0B;
// pub const FT5336_P2_YL_REG: u8 = 0x0C;
// pub const FT5336_P2_WEIGHT_REG: u8 = 0x0D;
// pub const FT5336_P2_MISC_REG: u8 = 0x0E;

// /* P3 X, Y coordinates, weight and misc registers */
// pub const FT5336_P3_XH_REG: u8 = 0x0F;
// pub const FT5336_P3_XL_REG: u8 = 0x10;
// pub const FT5336_P3_YH_REG: u8 = 0x11;
// pub const FT5336_P3_YL_REG: u8 = 0x12;
// pub const FT5336_P3_WEIGHT_REG: u8 = 0x13;
// pub const FT5336_P3_MISC_REG: u8 = 0x14;

// /* P4 X, Y coordinates, weight and misc registers */
// pub const FT5336_P4_XH_REG: u8 = 0x15;
// pub const FT5336_P4_XL_REG: u8 = 0x16;
// pub const FT5336_P4_YH_REG: u8 = 0x17;
// pub const FT5336_P4_YL_REG: u8 = 0x18;
// pub const FT5336_P4_WEIGHT_REG: u8 = 0x19;
// pub const FT5336_P4_MISC_REG: u8 = 0x1A;

// /* P5 X, Y coordinates, weight and misc registers */
// pub const FT5336_P5_XH_REG: u8 = 0x1B;
// pub const FT5336_P5_XL_REG: u8 = 0x1C;
// pub const FT5336_P5_YH_REG: u8 = 0x1D;
// pub const FT5336_P5_YL_REG: u8 = 0x1E;
// pub const FT5336_P5_WEIGHT_REG: u8 = 0x1F;
// pub const FT5336_P5_MISC_REG: u8 = 0x20;

// /* P6 X, Y coordinates, weight and misc registers */
// pub const FT5336_P6_XH_REG: u8 = 0x21;
// pub const FT5336_P6_XL_REG: u8 = 0x22;
// pub const FT5336_P6_YH_REG: u8 = 0x23;
// pub const FT5336_P6_YL_REG: u8 = 0x24;
// pub const FT5336_P6_WEIGHT_REG: u8 = 0x25;
// pub const FT5336_P6_MISC_REG: u8 = 0x26;

// /* P7 X, Y coordinates, weight and misc registers */
// pub const FT5336_P7_XH_REG: u8 = 0x27;
// pub const FT5336_P7_XL_REG: u8 = 0x28;
// pub const FT5336_P7_YH_REG: u8 = 0x29;
// pub const FT5336_P7_YL_REG: u8 = 0x2A;
// pub const FT5336_P7_WEIGHT_REG: u8 = 0x2B;
// pub const FT5336_P7_MISC_REG: u8 = 0x2C;

// /* P8 X, Y coordinates, weight and misc registers */
// pub const FT5336_P8_XH_REG: u8 = 0x2D;
// pub const FT5336_P8_XL_REG: u8 = 0x2E;
// pub const FT5336_P8_YH_REG: u8 = 0x2F;
// pub const FT5336_P8_YL_REG: u8 = 0x30;
// pub const FT5336_P8_WEIGHT_REG: u8 = 0x31;
// pub const FT5336_P8_MISC_REG: u8 = 0x32;

// /* P9 X, Y coordinates, weight and misc registers */
// pub const FT5336_P9_XH_REG: u8 = 0x33;
// pub const FT5336_P9_XL_REG: u8 = 0x34;
// pub const FT5336_P9_YH_REG: u8 = 0x35;
// pub const FT5336_P9_YL_REG: u8 = 0x36;
// pub const FT5336_P9_WEIGHT_REG: u8 = 0x37;
// pub const FT5336_P9_MISC_REG: u8 = 0x38;

// /* P10 X, Y coordinates, weight and misc registers */
// pub const FT5336_P10_XH_REG: u8 = 0x39;
// pub const FT5336_P10_XL_REG: u8 = 0x3A;
// pub const FT5336_P10_YH_REG: u8 = 0x3B;
// pub const FT5336_P10_YL_REG: u8 = 0x3C;
// pub const FT5336_P10_WEIGHT_REG: u8 = 0x3D;
// pub const FT5336_P10_MISC_REG: u8 = 0x3E;

// /* Threshold for touch detection */
// pub const FT5336_TH_GROUP_REG: u8 = 0x80;

// /* Filter function coefficients */
// pub const FT5336_TH_DIFF_REG: u8 = 0x85;

// /* Control register */
// pub const FT5336_CTRL_REG: u8 = 0x86;

// /* The time period of switching from Active mode to Monitor mode when there is no touching */
// pub const FT5336_TIMEENTERMONITOR_REG: u8 = 0x87;

// /* Report rate in Active mode */
// pub const FT5336_PERIODACTIVE_REG: u8 = 0x88;

// /* Report rate in Monitor mode */
// pub const FT5336_PERIODMONITOR_REG: u8 = 0x89;

// /* The value of the minimum allowed angle while Rotating gesture mode */
pub const FT5336_RADIAN_VALUE_REG: u8 = 0x91;

// /* Maximum offset while Moving Left and Moving Right gesture */
pub const FT5336_OFFSET_LR_REG: u8 = 0x92;

// /* Maximum offset while Moving Up and Moving Down gesture */
pub const FT5336_OFFSET_UD_REG: u8 = 0x93;

// /* Minimum distance while Moving Left and Moving Right gesture */
pub const FT5336_DISTANCE_LR_REG: u8 = 0x94;

// /* Minimum distance while Moving Up and Moving Down gesture */
pub const FT5336_DISTANCE_UD_REG: u8 = 0x95;

// /* Maximum distance while Zoom In and Zoom Out gesture */
pub const FT5336_DISTANCE_ZOOM_REG: u8 = 0x96;

// /* High 8-bit of LIB Version info */
// pub const FT5336_LIB_VER_H_REG: u8 = 0xA1;

// /* Low 8-bit of LIB Version info */
// pub const FT5336_LIB_VER_L_REG: u8 = 0xA2;

// /* Chip Selecting */
// pub const FT5336_CIPHER_REG: u8 = 0xA3;

// /* Interrupt mode register (used when in interrupt mode) */
// pub const FT5336_GMODE_REG: u8 = 0xA4;

// /* Current power mode the FT5336 system is in (R) */
// pub const FT5336_PWR_MODE_REG: u8 = 0xA5;

// /* FT5336 firmware version */
// pub const FT5336_FIRMID_REG: u8 = 0xA6;

// /* FT5336 Chip identification register */
// pub const FT5336_CHIP_ID_REG: u8 = 0xA8;

// /* Release code version */
// pub const FT5336_RELEASE_CODE_ID_REG: u8 = 0xAF;

// /* Current operating mode the FT5336 system is in (R) */
// pub const FT5336_STATE_REG: u8 = 0xBC;
