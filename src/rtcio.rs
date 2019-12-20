#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RTC_GPIO_OUT_REG"]
    pub rtc_gpio_out_reg: RTC_GPIO_OUT_REG,
    #[doc = "0x04 - RTC_GPIO_OUT_W1TS_REG"]
    pub rtc_gpio_out_w1ts_reg: RTC_GPIO_OUT_W1TS_REG,
    #[doc = "0x08 - RTC_GPIO_OUT_W1TC_REG"]
    pub rtc_gpio_out_w1tc_reg: RTC_GPIO_OUT_W1TC_REG,
    #[doc = "0x0c - RTC_GPIO_ENABLE_REG"]
    pub rtc_gpio_enable_reg: RTC_GPIO_ENABLE_REG,
    #[doc = "0x10 - RTC_GPIO_ENABLE_W1TS_REG"]
    pub rtc_gpio_enable_w1ts_reg: RTC_GPIO_ENABLE_W1TS_REG,
    #[doc = "0x14 - RTC_GPIO_ENABLE_W1TC_REG"]
    pub rtc_gpio_enable_w1tc_reg: RTC_GPIO_ENABLE_W1TC_REG,
    #[doc = "0x18 - RTC_GPIO_STATUS_REG"]
    pub rtc_gpio_status_reg: RTC_GPIO_STATUS_REG,
    #[doc = "0x1c - RTC_GPIO_STATUS_W1TS_REG"]
    pub rtc_gpio_status_w1ts_reg: RTC_GPIO_STATUS_W1TS_REG,
    #[doc = "0x20 - RTC_GPIO_STATUS_W1TC_REG"]
    pub rtc_gpio_status_w1tc_reg: RTC_GPIO_STATUS_W1TC_REG,
    #[doc = "0x24 - RTC_GPIO_IN_REG"]
    pub rtc_gpio_in_reg: RTC_GPIO_IN_REG,
    #[doc = "0x28 - RTC_GPIO_PIN0_REG"]
    pub rtc_gpio_pin0_reg: RTC_GPIO_PIN0_REG,
    #[doc = "0x2c - RTC_GPIO_PIN1_REG"]
    pub rtc_gpio_pin1_reg: RTC_GPIO_PIN1_REG,
    #[doc = "0x30 - RTC_GPIO_PIN2_REG"]
    pub rtc_gpio_pin2_reg: RTC_GPIO_PIN2_REG,
    #[doc = "0x34 - RTC_GPIO_PIN3_REG"]
    pub rtc_gpio_pin3_reg: RTC_GPIO_PIN3_REG,
    #[doc = "0x38 - RTC_GPIO_PIN4_REG"]
    pub rtc_gpio_pin4_reg: RTC_GPIO_PIN4_REG,
    #[doc = "0x3c - RTC_GPIO_PIN5_REG"]
    pub rtc_gpio_pin5_reg: RTC_GPIO_PIN5_REG,
    #[doc = "0x40 - RTC_GPIO_PIN6_REG"]
    pub rtc_gpio_pin6_reg: RTC_GPIO_PIN6_REG,
    #[doc = "0x44 - RTC_GPIO_PIN7_REG"]
    pub rtc_gpio_pin7_reg: RTC_GPIO_PIN7_REG,
    #[doc = "0x48 - RTC_GPIO_PIN8_REG"]
    pub rtc_gpio_pin8_reg: RTC_GPIO_PIN8_REG,
    #[doc = "0x4c - RTC_GPIO_PIN9_REG"]
    pub rtc_gpio_pin9_reg: RTC_GPIO_PIN9_REG,
    #[doc = "0x50 - RTC_GPIO_PIN10_REG"]
    pub rtc_gpio_pin10_reg: RTC_GPIO_PIN10_REG,
    #[doc = "0x54 - RTC_GPIO_PIN11_REG"]
    pub rtc_gpio_pin11_reg: RTC_GPIO_PIN11_REG,
    #[doc = "0x58 - RTC_GPIO_PIN12_REG"]
    pub rtc_gpio_pin12_reg: RTC_GPIO_PIN12_REG,
    #[doc = "0x5c - RTC_GPIO_PIN13_REG"]
    pub rtc_gpio_pin13_reg: RTC_GPIO_PIN13_REG,
    #[doc = "0x60 - RTC_GPIO_PIN14_REG"]
    pub rtc_gpio_pin14_reg: RTC_GPIO_PIN14_REG,
    #[doc = "0x64 - RTC_GPIO_PIN15_REG"]
    pub rtc_gpio_pin15_reg: RTC_GPIO_PIN15_REG,
    #[doc = "0x68 - RTC_GPIO_PIN16_REG"]
    pub rtc_gpio_pin16_reg: RTC_GPIO_PIN16_REG,
    #[doc = "0x6c - RTC_GPIO_PIN17_REG"]
    pub rtc_gpio_pin17_reg: RTC_GPIO_PIN17_REG,
    #[doc = "0x70 - RTC_IO_RTC_DEBUG_SEL_REG"]
    pub rtc_io_rtc_debug_sel_reg: RTC_IO_RTC_DEBUG_SEL_REG,
    #[doc = "0x74 - RTC_IO_DIG_PAD_HOLD_REG"]
    pub rtc_io_dig_pad_hold_reg: RTC_IO_DIG_PAD_HOLD_REG,
    #[doc = "0x78 - RTC_IO_HALL_SENS_REG"]
    pub rtc_io_hall_sens_reg: RTC_IO_HALL_SENS_REG,
    #[doc = "0x7c - RTC_IO_SENSOR_PADS_REG"]
    pub rtc_io_sensor_pads_reg: RTC_IO_SENSOR_PADS_REG,
    #[doc = "0x80 - RTC_IO_ADC_PAD_REG"]
    pub rtc_io_adc_pad_reg: RTC_IO_ADC_PAD_REG,
    #[doc = "0x84 - RTC_IO_PAD_DAC1_REG"]
    pub rtc_io_pad_dac1_reg: RTC_IO_PAD_DAC1_REG,
    #[doc = "0x88 - RTC_IO_PAD_DAC2_REG"]
    pub rtc_io_pad_dac2_reg: RTC_IO_PAD_DAC2_REG,
    #[doc = "0x8c - RTC_IO_XTAL_32K_PAD_REG"]
    pub rtc_io_xtal_32k_pad_reg: RTC_IO_XTAL_32K_PAD_REG,
    #[doc = "0x90 - RTC_IO_TOUCH_CFG_REG"]
    pub rtc_io_touch_cfg_reg: RTC_IO_TOUCH_CFG_REG,
    #[doc = "0x94 - RTC_IO_TOUCH_PAD0_REG"]
    pub rtc_io_touch_pad0_reg: RTC_IO_TOUCH_PAD0_REG,
    #[doc = "0x98 - RTC_IO_TOUCH_PAD1_REG"]
    pub rtc_io_touch_pad1_reg: RTC_IO_TOUCH_PAD1_REG,
    #[doc = "0x9c - RTC_IO_TOUCH_PAD2_REG"]
    pub rtc_io_touch_pad2_reg: RTC_IO_TOUCH_PAD2_REG,
    #[doc = "0xa0 - RTC_IO_TOUCH_PAD3_REG"]
    pub rtc_io_touch_pad3_reg: RTC_IO_TOUCH_PAD3_REG,
    #[doc = "0xa4 - RTC_IO_TOUCH_PAD4_REG"]
    pub rtc_io_touch_pad4_reg: RTC_IO_TOUCH_PAD4_REG,
    #[doc = "0xa8 - RTC_IO_TOUCH_PAD5_REG"]
    pub rtc_io_touch_pad5_reg: RTC_IO_TOUCH_PAD5_REG,
    #[doc = "0xac - RTC_IO_TOUCH_PAD6_REG"]
    pub rtc_io_touch_pad6_reg: RTC_IO_TOUCH_PAD6_REG,
    #[doc = "0xb0 - RTC_IO_TOUCH_PAD7_REG"]
    pub rtc_io_touch_pad7_reg: RTC_IO_TOUCH_PAD7_REG,
    #[doc = "0xb4 - RTC_IO_TOUCH_PAD8_REG"]
    pub rtc_io_touch_pad8_reg: RTC_IO_TOUCH_PAD8_REG,
    #[doc = "0xb8 - RTC_IO_TOUCH_PAD9_REG"]
    pub rtc_io_touch_pad9_reg: RTC_IO_TOUCH_PAD9_REG,
    #[doc = "0xbc - RTC_IO_EXT_WAKEUP0_REG"]
    pub rtc_io_ext_wakeup0_reg: RTC_IO_EXT_WAKEUP0_REG,
    #[doc = "0xc0 - RTC_IO_XTL_EXT_CTR_REG"]
    pub rtc_io_xtl_ext_ctr_reg: RTC_IO_XTL_EXT_CTR_REG,
    #[doc = "0xc4 - RTC_IO_SAR_I2C_IO_REG"]
    pub rtc_io_sar_i2c_io_reg: RTC_IO_SAR_I2C_IO_REG,
    #[doc = "0xc8 - RTC_IO_DATE_REG"]
    pub rtc_io_date_reg: RTC_IO_DATE_REG,
}
#[doc = "RTC_GPIO_OUT_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_gpio_out_reg](rtc_gpio_out_reg) module"]
pub type RTC_GPIO_OUT_REG = crate::Reg<u32, _RTC_GPIO_OUT_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_GPIO_OUT_REG;
#[doc = "`read()` method returns [rtc_gpio_out_reg::R](rtc_gpio_out_reg::R) reader structure"]
impl crate::Readable for RTC_GPIO_OUT_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_gpio_out_reg::W](rtc_gpio_out_reg::W) writer structure"]
impl crate::Writable for RTC_GPIO_OUT_REG {}
#[doc = "RTC_GPIO_OUT_REG"]
pub mod rtc_gpio_out_reg;
#[doc = "RTC_GPIO_OUT_W1TS_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_gpio_out_w1ts_reg](rtc_gpio_out_w1ts_reg) module"]
pub type RTC_GPIO_OUT_W1TS_REG = crate::Reg<u32, _RTC_GPIO_OUT_W1TS_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_GPIO_OUT_W1TS_REG;
#[doc = "`read()` method returns [rtc_gpio_out_w1ts_reg::R](rtc_gpio_out_w1ts_reg::R) reader structure"]
impl crate::Readable for RTC_GPIO_OUT_W1TS_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_gpio_out_w1ts_reg::W](rtc_gpio_out_w1ts_reg::W) writer structure"]
impl crate::Writable for RTC_GPIO_OUT_W1TS_REG {}
#[doc = "RTC_GPIO_OUT_W1TS_REG"]
pub mod rtc_gpio_out_w1ts_reg;
#[doc = "RTC_GPIO_OUT_W1TC_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_gpio_out_w1tc_reg](rtc_gpio_out_w1tc_reg) module"]
pub type RTC_GPIO_OUT_W1TC_REG = crate::Reg<u32, _RTC_GPIO_OUT_W1TC_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_GPIO_OUT_W1TC_REG;
#[doc = "`read()` method returns [rtc_gpio_out_w1tc_reg::R](rtc_gpio_out_w1tc_reg::R) reader structure"]
impl crate::Readable for RTC_GPIO_OUT_W1TC_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_gpio_out_w1tc_reg::W](rtc_gpio_out_w1tc_reg::W) writer structure"]
impl crate::Writable for RTC_GPIO_OUT_W1TC_REG {}
#[doc = "RTC_GPIO_OUT_W1TC_REG"]
pub mod rtc_gpio_out_w1tc_reg;
#[doc = "RTC_GPIO_ENABLE_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_gpio_enable_reg](rtc_gpio_enable_reg) module"]
pub type RTC_GPIO_ENABLE_REG = crate::Reg<u32, _RTC_GPIO_ENABLE_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_GPIO_ENABLE_REG;
#[doc = "`read()` method returns [rtc_gpio_enable_reg::R](rtc_gpio_enable_reg::R) reader structure"]
impl crate::Readable for RTC_GPIO_ENABLE_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_gpio_enable_reg::W](rtc_gpio_enable_reg::W) writer structure"]
impl crate::Writable for RTC_GPIO_ENABLE_REG {}
#[doc = "RTC_GPIO_ENABLE_REG"]
pub mod rtc_gpio_enable_reg;
#[doc = "RTC_GPIO_ENABLE_W1TS_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_gpio_enable_w1ts_reg](rtc_gpio_enable_w1ts_reg) module"]
pub type RTC_GPIO_ENABLE_W1TS_REG = crate::Reg<u32, _RTC_GPIO_ENABLE_W1TS_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_GPIO_ENABLE_W1TS_REG;
#[doc = "`read()` method returns [rtc_gpio_enable_w1ts_reg::R](rtc_gpio_enable_w1ts_reg::R) reader structure"]
impl crate::Readable for RTC_GPIO_ENABLE_W1TS_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_gpio_enable_w1ts_reg::W](rtc_gpio_enable_w1ts_reg::W) writer structure"]
impl crate::Writable for RTC_GPIO_ENABLE_W1TS_REG {}
#[doc = "RTC_GPIO_ENABLE_W1TS_REG"]
pub mod rtc_gpio_enable_w1ts_reg;
#[doc = "RTC_GPIO_ENABLE_W1TC_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_gpio_enable_w1tc_reg](rtc_gpio_enable_w1tc_reg) module"]
pub type RTC_GPIO_ENABLE_W1TC_REG = crate::Reg<u32, _RTC_GPIO_ENABLE_W1TC_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_GPIO_ENABLE_W1TC_REG;
#[doc = "`read()` method returns [rtc_gpio_enable_w1tc_reg::R](rtc_gpio_enable_w1tc_reg::R) reader structure"]
impl crate::Readable for RTC_GPIO_ENABLE_W1TC_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_gpio_enable_w1tc_reg::W](rtc_gpio_enable_w1tc_reg::W) writer structure"]
impl crate::Writable for RTC_GPIO_ENABLE_W1TC_REG {}
#[doc = "RTC_GPIO_ENABLE_W1TC_REG"]
pub mod rtc_gpio_enable_w1tc_reg;
#[doc = "RTC_GPIO_STATUS_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_gpio_status_reg](rtc_gpio_status_reg) module"]
pub type RTC_GPIO_STATUS_REG = crate::Reg<u32, _RTC_GPIO_STATUS_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_GPIO_STATUS_REG;
#[doc = "`read()` method returns [rtc_gpio_status_reg::R](rtc_gpio_status_reg::R) reader structure"]
impl crate::Readable for RTC_GPIO_STATUS_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_gpio_status_reg::W](rtc_gpio_status_reg::W) writer structure"]
impl crate::Writable for RTC_GPIO_STATUS_REG {}
#[doc = "RTC_GPIO_STATUS_REG"]
pub mod rtc_gpio_status_reg;
#[doc = "RTC_GPIO_STATUS_W1TS_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_gpio_status_w1ts_reg](rtc_gpio_status_w1ts_reg) module"]
pub type RTC_GPIO_STATUS_W1TS_REG = crate::Reg<u32, _RTC_GPIO_STATUS_W1TS_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_GPIO_STATUS_W1TS_REG;
#[doc = "`read()` method returns [rtc_gpio_status_w1ts_reg::R](rtc_gpio_status_w1ts_reg::R) reader structure"]
impl crate::Readable for RTC_GPIO_STATUS_W1TS_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_gpio_status_w1ts_reg::W](rtc_gpio_status_w1ts_reg::W) writer structure"]
impl crate::Writable for RTC_GPIO_STATUS_W1TS_REG {}
#[doc = "RTC_GPIO_STATUS_W1TS_REG"]
pub mod rtc_gpio_status_w1ts_reg;
#[doc = "RTC_GPIO_STATUS_W1TC_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_gpio_status_w1tc_reg](rtc_gpio_status_w1tc_reg) module"]
pub type RTC_GPIO_STATUS_W1TC_REG = crate::Reg<u32, _RTC_GPIO_STATUS_W1TC_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_GPIO_STATUS_W1TC_REG;
#[doc = "`read()` method returns [rtc_gpio_status_w1tc_reg::R](rtc_gpio_status_w1tc_reg::R) reader structure"]
impl crate::Readable for RTC_GPIO_STATUS_W1TC_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_gpio_status_w1tc_reg::W](rtc_gpio_status_w1tc_reg::W) writer structure"]
impl crate::Writable for RTC_GPIO_STATUS_W1TC_REG {}
#[doc = "RTC_GPIO_STATUS_W1TC_REG"]
pub mod rtc_gpio_status_w1tc_reg;
#[doc = "RTC_GPIO_IN_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_gpio_in_reg](rtc_gpio_in_reg) module"]
pub type RTC_GPIO_IN_REG = crate::Reg<u32, _RTC_GPIO_IN_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_GPIO_IN_REG;
#[doc = "`read()` method returns [rtc_gpio_in_reg::R](rtc_gpio_in_reg::R) reader structure"]
impl crate::Readable for RTC_GPIO_IN_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_gpio_in_reg::W](rtc_gpio_in_reg::W) writer structure"]
impl crate::Writable for RTC_GPIO_IN_REG {}
#[doc = "RTC_GPIO_IN_REG"]
pub mod rtc_gpio_in_reg;
#[doc = "RTC_GPIO_PIN0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_gpio_pin0_reg](rtc_gpio_pin0_reg) module"]
pub type RTC_GPIO_PIN0_REG = crate::Reg<u32, _RTC_GPIO_PIN0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_GPIO_PIN0_REG;
#[doc = "`read()` method returns [rtc_gpio_pin0_reg::R](rtc_gpio_pin0_reg::R) reader structure"]
impl crate::Readable for RTC_GPIO_PIN0_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_gpio_pin0_reg::W](rtc_gpio_pin0_reg::W) writer structure"]
impl crate::Writable for RTC_GPIO_PIN0_REG {}
#[doc = "RTC_GPIO_PIN0_REG"]
pub mod rtc_gpio_pin0_reg;
#[doc = "RTC_GPIO_PIN1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_gpio_pin1_reg](rtc_gpio_pin1_reg) module"]
pub type RTC_GPIO_PIN1_REG = crate::Reg<u32, _RTC_GPIO_PIN1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_GPIO_PIN1_REG;
#[doc = "`read()` method returns [rtc_gpio_pin1_reg::R](rtc_gpio_pin1_reg::R) reader structure"]
impl crate::Readable for RTC_GPIO_PIN1_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_gpio_pin1_reg::W](rtc_gpio_pin1_reg::W) writer structure"]
impl crate::Writable for RTC_GPIO_PIN1_REG {}
#[doc = "RTC_GPIO_PIN1_REG"]
pub mod rtc_gpio_pin1_reg;
#[doc = "RTC_GPIO_PIN2_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_gpio_pin2_reg](rtc_gpio_pin2_reg) module"]
pub type RTC_GPIO_PIN2_REG = crate::Reg<u32, _RTC_GPIO_PIN2_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_GPIO_PIN2_REG;
#[doc = "`read()` method returns [rtc_gpio_pin2_reg::R](rtc_gpio_pin2_reg::R) reader structure"]
impl crate::Readable for RTC_GPIO_PIN2_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_gpio_pin2_reg::W](rtc_gpio_pin2_reg::W) writer structure"]
impl crate::Writable for RTC_GPIO_PIN2_REG {}
#[doc = "RTC_GPIO_PIN2_REG"]
pub mod rtc_gpio_pin2_reg;
#[doc = "RTC_GPIO_PIN3_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_gpio_pin3_reg](rtc_gpio_pin3_reg) module"]
pub type RTC_GPIO_PIN3_REG = crate::Reg<u32, _RTC_GPIO_PIN3_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_GPIO_PIN3_REG;
#[doc = "`read()` method returns [rtc_gpio_pin3_reg::R](rtc_gpio_pin3_reg::R) reader structure"]
impl crate::Readable for RTC_GPIO_PIN3_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_gpio_pin3_reg::W](rtc_gpio_pin3_reg::W) writer structure"]
impl crate::Writable for RTC_GPIO_PIN3_REG {}
#[doc = "RTC_GPIO_PIN3_REG"]
pub mod rtc_gpio_pin3_reg;
#[doc = "RTC_GPIO_PIN4_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_gpio_pin4_reg](rtc_gpio_pin4_reg) module"]
pub type RTC_GPIO_PIN4_REG = crate::Reg<u32, _RTC_GPIO_PIN4_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_GPIO_PIN4_REG;
#[doc = "`read()` method returns [rtc_gpio_pin4_reg::R](rtc_gpio_pin4_reg::R) reader structure"]
impl crate::Readable for RTC_GPIO_PIN4_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_gpio_pin4_reg::W](rtc_gpio_pin4_reg::W) writer structure"]
impl crate::Writable for RTC_GPIO_PIN4_REG {}
#[doc = "RTC_GPIO_PIN4_REG"]
pub mod rtc_gpio_pin4_reg;
#[doc = "RTC_GPIO_PIN5_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_gpio_pin5_reg](rtc_gpio_pin5_reg) module"]
pub type RTC_GPIO_PIN5_REG = crate::Reg<u32, _RTC_GPIO_PIN5_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_GPIO_PIN5_REG;
#[doc = "`read()` method returns [rtc_gpio_pin5_reg::R](rtc_gpio_pin5_reg::R) reader structure"]
impl crate::Readable for RTC_GPIO_PIN5_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_gpio_pin5_reg::W](rtc_gpio_pin5_reg::W) writer structure"]
impl crate::Writable for RTC_GPIO_PIN5_REG {}
#[doc = "RTC_GPIO_PIN5_REG"]
pub mod rtc_gpio_pin5_reg;
#[doc = "RTC_GPIO_PIN6_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_gpio_pin6_reg](rtc_gpio_pin6_reg) module"]
pub type RTC_GPIO_PIN6_REG = crate::Reg<u32, _RTC_GPIO_PIN6_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_GPIO_PIN6_REG;
#[doc = "`read()` method returns [rtc_gpio_pin6_reg::R](rtc_gpio_pin6_reg::R) reader structure"]
impl crate::Readable for RTC_GPIO_PIN6_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_gpio_pin6_reg::W](rtc_gpio_pin6_reg::W) writer structure"]
impl crate::Writable for RTC_GPIO_PIN6_REG {}
#[doc = "RTC_GPIO_PIN6_REG"]
pub mod rtc_gpio_pin6_reg;
#[doc = "RTC_GPIO_PIN7_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_gpio_pin7_reg](rtc_gpio_pin7_reg) module"]
pub type RTC_GPIO_PIN7_REG = crate::Reg<u32, _RTC_GPIO_PIN7_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_GPIO_PIN7_REG;
#[doc = "`read()` method returns [rtc_gpio_pin7_reg::R](rtc_gpio_pin7_reg::R) reader structure"]
impl crate::Readable for RTC_GPIO_PIN7_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_gpio_pin7_reg::W](rtc_gpio_pin7_reg::W) writer structure"]
impl crate::Writable for RTC_GPIO_PIN7_REG {}
#[doc = "RTC_GPIO_PIN7_REG"]
pub mod rtc_gpio_pin7_reg;
#[doc = "RTC_GPIO_PIN8_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_gpio_pin8_reg](rtc_gpio_pin8_reg) module"]
pub type RTC_GPIO_PIN8_REG = crate::Reg<u32, _RTC_GPIO_PIN8_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_GPIO_PIN8_REG;
#[doc = "`read()` method returns [rtc_gpio_pin8_reg::R](rtc_gpio_pin8_reg::R) reader structure"]
impl crate::Readable for RTC_GPIO_PIN8_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_gpio_pin8_reg::W](rtc_gpio_pin8_reg::W) writer structure"]
impl crate::Writable for RTC_GPIO_PIN8_REG {}
#[doc = "RTC_GPIO_PIN8_REG"]
pub mod rtc_gpio_pin8_reg;
#[doc = "RTC_GPIO_PIN9_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_gpio_pin9_reg](rtc_gpio_pin9_reg) module"]
pub type RTC_GPIO_PIN9_REG = crate::Reg<u32, _RTC_GPIO_PIN9_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_GPIO_PIN9_REG;
#[doc = "`read()` method returns [rtc_gpio_pin9_reg::R](rtc_gpio_pin9_reg::R) reader structure"]
impl crate::Readable for RTC_GPIO_PIN9_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_gpio_pin9_reg::W](rtc_gpio_pin9_reg::W) writer structure"]
impl crate::Writable for RTC_GPIO_PIN9_REG {}
#[doc = "RTC_GPIO_PIN9_REG"]
pub mod rtc_gpio_pin9_reg;
#[doc = "RTC_GPIO_PIN10_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_gpio_pin10_reg](rtc_gpio_pin10_reg) module"]
pub type RTC_GPIO_PIN10_REG = crate::Reg<u32, _RTC_GPIO_PIN10_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_GPIO_PIN10_REG;
#[doc = "`read()` method returns [rtc_gpio_pin10_reg::R](rtc_gpio_pin10_reg::R) reader structure"]
impl crate::Readable for RTC_GPIO_PIN10_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_gpio_pin10_reg::W](rtc_gpio_pin10_reg::W) writer structure"]
impl crate::Writable for RTC_GPIO_PIN10_REG {}
#[doc = "RTC_GPIO_PIN10_REG"]
pub mod rtc_gpio_pin10_reg;
#[doc = "RTC_GPIO_PIN11_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_gpio_pin11_reg](rtc_gpio_pin11_reg) module"]
pub type RTC_GPIO_PIN11_REG = crate::Reg<u32, _RTC_GPIO_PIN11_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_GPIO_PIN11_REG;
#[doc = "`read()` method returns [rtc_gpio_pin11_reg::R](rtc_gpio_pin11_reg::R) reader structure"]
impl crate::Readable for RTC_GPIO_PIN11_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_gpio_pin11_reg::W](rtc_gpio_pin11_reg::W) writer structure"]
impl crate::Writable for RTC_GPIO_PIN11_REG {}
#[doc = "RTC_GPIO_PIN11_REG"]
pub mod rtc_gpio_pin11_reg;
#[doc = "RTC_GPIO_PIN12_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_gpio_pin12_reg](rtc_gpio_pin12_reg) module"]
pub type RTC_GPIO_PIN12_REG = crate::Reg<u32, _RTC_GPIO_PIN12_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_GPIO_PIN12_REG;
#[doc = "`read()` method returns [rtc_gpio_pin12_reg::R](rtc_gpio_pin12_reg::R) reader structure"]
impl crate::Readable for RTC_GPIO_PIN12_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_gpio_pin12_reg::W](rtc_gpio_pin12_reg::W) writer structure"]
impl crate::Writable for RTC_GPIO_PIN12_REG {}
#[doc = "RTC_GPIO_PIN12_REG"]
pub mod rtc_gpio_pin12_reg;
#[doc = "RTC_GPIO_PIN13_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_gpio_pin13_reg](rtc_gpio_pin13_reg) module"]
pub type RTC_GPIO_PIN13_REG = crate::Reg<u32, _RTC_GPIO_PIN13_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_GPIO_PIN13_REG;
#[doc = "`read()` method returns [rtc_gpio_pin13_reg::R](rtc_gpio_pin13_reg::R) reader structure"]
impl crate::Readable for RTC_GPIO_PIN13_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_gpio_pin13_reg::W](rtc_gpio_pin13_reg::W) writer structure"]
impl crate::Writable for RTC_GPIO_PIN13_REG {}
#[doc = "RTC_GPIO_PIN13_REG"]
pub mod rtc_gpio_pin13_reg;
#[doc = "RTC_GPIO_PIN14_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_gpio_pin14_reg](rtc_gpio_pin14_reg) module"]
pub type RTC_GPIO_PIN14_REG = crate::Reg<u32, _RTC_GPIO_PIN14_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_GPIO_PIN14_REG;
#[doc = "`read()` method returns [rtc_gpio_pin14_reg::R](rtc_gpio_pin14_reg::R) reader structure"]
impl crate::Readable for RTC_GPIO_PIN14_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_gpio_pin14_reg::W](rtc_gpio_pin14_reg::W) writer structure"]
impl crate::Writable for RTC_GPIO_PIN14_REG {}
#[doc = "RTC_GPIO_PIN14_REG"]
pub mod rtc_gpio_pin14_reg;
#[doc = "RTC_GPIO_PIN15_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_gpio_pin15_reg](rtc_gpio_pin15_reg) module"]
pub type RTC_GPIO_PIN15_REG = crate::Reg<u32, _RTC_GPIO_PIN15_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_GPIO_PIN15_REG;
#[doc = "`read()` method returns [rtc_gpio_pin15_reg::R](rtc_gpio_pin15_reg::R) reader structure"]
impl crate::Readable for RTC_GPIO_PIN15_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_gpio_pin15_reg::W](rtc_gpio_pin15_reg::W) writer structure"]
impl crate::Writable for RTC_GPIO_PIN15_REG {}
#[doc = "RTC_GPIO_PIN15_REG"]
pub mod rtc_gpio_pin15_reg;
#[doc = "RTC_GPIO_PIN16_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_gpio_pin16_reg](rtc_gpio_pin16_reg) module"]
pub type RTC_GPIO_PIN16_REG = crate::Reg<u32, _RTC_GPIO_PIN16_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_GPIO_PIN16_REG;
#[doc = "`read()` method returns [rtc_gpio_pin16_reg::R](rtc_gpio_pin16_reg::R) reader structure"]
impl crate::Readable for RTC_GPIO_PIN16_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_gpio_pin16_reg::W](rtc_gpio_pin16_reg::W) writer structure"]
impl crate::Writable for RTC_GPIO_PIN16_REG {}
#[doc = "RTC_GPIO_PIN16_REG"]
pub mod rtc_gpio_pin16_reg;
#[doc = "RTC_GPIO_PIN17_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_gpio_pin17_reg](rtc_gpio_pin17_reg) module"]
pub type RTC_GPIO_PIN17_REG = crate::Reg<u32, _RTC_GPIO_PIN17_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_GPIO_PIN17_REG;
#[doc = "`read()` method returns [rtc_gpio_pin17_reg::R](rtc_gpio_pin17_reg::R) reader structure"]
impl crate::Readable for RTC_GPIO_PIN17_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_gpio_pin17_reg::W](rtc_gpio_pin17_reg::W) writer structure"]
impl crate::Writable for RTC_GPIO_PIN17_REG {}
#[doc = "RTC_GPIO_PIN17_REG"]
pub mod rtc_gpio_pin17_reg;
#[doc = "RTC_IO_RTC_DEBUG_SEL_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_io_rtc_debug_sel_reg](rtc_io_rtc_debug_sel_reg) module"]
pub type RTC_IO_RTC_DEBUG_SEL_REG = crate::Reg<u32, _RTC_IO_RTC_DEBUG_SEL_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_IO_RTC_DEBUG_SEL_REG;
#[doc = "`read()` method returns [rtc_io_rtc_debug_sel_reg::R](rtc_io_rtc_debug_sel_reg::R) reader structure"]
impl crate::Readable for RTC_IO_RTC_DEBUG_SEL_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_io_rtc_debug_sel_reg::W](rtc_io_rtc_debug_sel_reg::W) writer structure"]
impl crate::Writable for RTC_IO_RTC_DEBUG_SEL_REG {}
#[doc = "RTC_IO_RTC_DEBUG_SEL_REG"]
pub mod rtc_io_rtc_debug_sel_reg;
#[doc = "RTC_IO_DIG_PAD_HOLD_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_io_dig_pad_hold_reg](rtc_io_dig_pad_hold_reg) module"]
pub type RTC_IO_DIG_PAD_HOLD_REG = crate::Reg<u32, _RTC_IO_DIG_PAD_HOLD_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_IO_DIG_PAD_HOLD_REG;
#[doc = "`read()` method returns [rtc_io_dig_pad_hold_reg::R](rtc_io_dig_pad_hold_reg::R) reader structure"]
impl crate::Readable for RTC_IO_DIG_PAD_HOLD_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_io_dig_pad_hold_reg::W](rtc_io_dig_pad_hold_reg::W) writer structure"]
impl crate::Writable for RTC_IO_DIG_PAD_HOLD_REG {}
#[doc = "RTC_IO_DIG_PAD_HOLD_REG"]
pub mod rtc_io_dig_pad_hold_reg;
#[doc = "RTC_IO_HALL_SENS_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_io_hall_sens_reg](rtc_io_hall_sens_reg) module"]
pub type RTC_IO_HALL_SENS_REG = crate::Reg<u32, _RTC_IO_HALL_SENS_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_IO_HALL_SENS_REG;
#[doc = "`read()` method returns [rtc_io_hall_sens_reg::R](rtc_io_hall_sens_reg::R) reader structure"]
impl crate::Readable for RTC_IO_HALL_SENS_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_io_hall_sens_reg::W](rtc_io_hall_sens_reg::W) writer structure"]
impl crate::Writable for RTC_IO_HALL_SENS_REG {}
#[doc = "RTC_IO_HALL_SENS_REG"]
pub mod rtc_io_hall_sens_reg;
#[doc = "RTC_IO_SENSOR_PADS_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_io_sensor_pads_reg](rtc_io_sensor_pads_reg) module"]
pub type RTC_IO_SENSOR_PADS_REG = crate::Reg<u32, _RTC_IO_SENSOR_PADS_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_IO_SENSOR_PADS_REG;
#[doc = "`read()` method returns [rtc_io_sensor_pads_reg::R](rtc_io_sensor_pads_reg::R) reader structure"]
impl crate::Readable for RTC_IO_SENSOR_PADS_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_io_sensor_pads_reg::W](rtc_io_sensor_pads_reg::W) writer structure"]
impl crate::Writable for RTC_IO_SENSOR_PADS_REG {}
#[doc = "RTC_IO_SENSOR_PADS_REG"]
pub mod rtc_io_sensor_pads_reg;
#[doc = "RTC_IO_ADC_PAD_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_io_adc_pad_reg](rtc_io_adc_pad_reg) module"]
pub type RTC_IO_ADC_PAD_REG = crate::Reg<u32, _RTC_IO_ADC_PAD_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_IO_ADC_PAD_REG;
#[doc = "`read()` method returns [rtc_io_adc_pad_reg::R](rtc_io_adc_pad_reg::R) reader structure"]
impl crate::Readable for RTC_IO_ADC_PAD_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_io_adc_pad_reg::W](rtc_io_adc_pad_reg::W) writer structure"]
impl crate::Writable for RTC_IO_ADC_PAD_REG {}
#[doc = "RTC_IO_ADC_PAD_REG"]
pub mod rtc_io_adc_pad_reg;
#[doc = "RTC_IO_PAD_DAC1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_io_pad_dac1_reg](rtc_io_pad_dac1_reg) module"]
pub type RTC_IO_PAD_DAC1_REG = crate::Reg<u32, _RTC_IO_PAD_DAC1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_IO_PAD_DAC1_REG;
#[doc = "`read()` method returns [rtc_io_pad_dac1_reg::R](rtc_io_pad_dac1_reg::R) reader structure"]
impl crate::Readable for RTC_IO_PAD_DAC1_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_io_pad_dac1_reg::W](rtc_io_pad_dac1_reg::W) writer structure"]
impl crate::Writable for RTC_IO_PAD_DAC1_REG {}
#[doc = "RTC_IO_PAD_DAC1_REG"]
pub mod rtc_io_pad_dac1_reg;
#[doc = "RTC_IO_PAD_DAC2_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_io_pad_dac2_reg](rtc_io_pad_dac2_reg) module"]
pub type RTC_IO_PAD_DAC2_REG = crate::Reg<u32, _RTC_IO_PAD_DAC2_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_IO_PAD_DAC2_REG;
#[doc = "`read()` method returns [rtc_io_pad_dac2_reg::R](rtc_io_pad_dac2_reg::R) reader structure"]
impl crate::Readable for RTC_IO_PAD_DAC2_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_io_pad_dac2_reg::W](rtc_io_pad_dac2_reg::W) writer structure"]
impl crate::Writable for RTC_IO_PAD_DAC2_REG {}
#[doc = "RTC_IO_PAD_DAC2_REG"]
pub mod rtc_io_pad_dac2_reg;
#[doc = "RTC_IO_XTAL_32K_PAD_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_io_xtal_32k_pad_reg](rtc_io_xtal_32k_pad_reg) module"]
pub type RTC_IO_XTAL_32K_PAD_REG = crate::Reg<u32, _RTC_IO_XTAL_32K_PAD_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_IO_XTAL_32K_PAD_REG;
#[doc = "`read()` method returns [rtc_io_xtal_32k_pad_reg::R](rtc_io_xtal_32k_pad_reg::R) reader structure"]
impl crate::Readable for RTC_IO_XTAL_32K_PAD_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_io_xtal_32k_pad_reg::W](rtc_io_xtal_32k_pad_reg::W) writer structure"]
impl crate::Writable for RTC_IO_XTAL_32K_PAD_REG {}
#[doc = "RTC_IO_XTAL_32K_PAD_REG"]
pub mod rtc_io_xtal_32k_pad_reg;
#[doc = "RTC_IO_TOUCH_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_io_touch_cfg_reg](rtc_io_touch_cfg_reg) module"]
pub type RTC_IO_TOUCH_CFG_REG = crate::Reg<u32, _RTC_IO_TOUCH_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_IO_TOUCH_CFG_REG;
#[doc = "`read()` method returns [rtc_io_touch_cfg_reg::R](rtc_io_touch_cfg_reg::R) reader structure"]
impl crate::Readable for RTC_IO_TOUCH_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_io_touch_cfg_reg::W](rtc_io_touch_cfg_reg::W) writer structure"]
impl crate::Writable for RTC_IO_TOUCH_CFG_REG {}
#[doc = "RTC_IO_TOUCH_CFG_REG"]
pub mod rtc_io_touch_cfg_reg;
#[doc = "RTC_IO_TOUCH_PAD0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_io_touch_pad0_reg](rtc_io_touch_pad0_reg) module"]
pub type RTC_IO_TOUCH_PAD0_REG = crate::Reg<u32, _RTC_IO_TOUCH_PAD0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_IO_TOUCH_PAD0_REG;
#[doc = "`read()` method returns [rtc_io_touch_pad0_reg::R](rtc_io_touch_pad0_reg::R) reader structure"]
impl crate::Readable for RTC_IO_TOUCH_PAD0_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_io_touch_pad0_reg::W](rtc_io_touch_pad0_reg::W) writer structure"]
impl crate::Writable for RTC_IO_TOUCH_PAD0_REG {}
#[doc = "RTC_IO_TOUCH_PAD0_REG"]
pub mod rtc_io_touch_pad0_reg;
#[doc = "RTC_IO_TOUCH_PAD1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_io_touch_pad1_reg](rtc_io_touch_pad1_reg) module"]
pub type RTC_IO_TOUCH_PAD1_REG = crate::Reg<u32, _RTC_IO_TOUCH_PAD1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_IO_TOUCH_PAD1_REG;
#[doc = "`read()` method returns [rtc_io_touch_pad1_reg::R](rtc_io_touch_pad1_reg::R) reader structure"]
impl crate::Readable for RTC_IO_TOUCH_PAD1_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_io_touch_pad1_reg::W](rtc_io_touch_pad1_reg::W) writer structure"]
impl crate::Writable for RTC_IO_TOUCH_PAD1_REG {}
#[doc = "RTC_IO_TOUCH_PAD1_REG"]
pub mod rtc_io_touch_pad1_reg;
#[doc = "RTC_IO_TOUCH_PAD2_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_io_touch_pad2_reg](rtc_io_touch_pad2_reg) module"]
pub type RTC_IO_TOUCH_PAD2_REG = crate::Reg<u32, _RTC_IO_TOUCH_PAD2_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_IO_TOUCH_PAD2_REG;
#[doc = "`read()` method returns [rtc_io_touch_pad2_reg::R](rtc_io_touch_pad2_reg::R) reader structure"]
impl crate::Readable for RTC_IO_TOUCH_PAD2_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_io_touch_pad2_reg::W](rtc_io_touch_pad2_reg::W) writer structure"]
impl crate::Writable for RTC_IO_TOUCH_PAD2_REG {}
#[doc = "RTC_IO_TOUCH_PAD2_REG"]
pub mod rtc_io_touch_pad2_reg;
#[doc = "RTC_IO_TOUCH_PAD3_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_io_touch_pad3_reg](rtc_io_touch_pad3_reg) module"]
pub type RTC_IO_TOUCH_PAD3_REG = crate::Reg<u32, _RTC_IO_TOUCH_PAD3_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_IO_TOUCH_PAD3_REG;
#[doc = "`read()` method returns [rtc_io_touch_pad3_reg::R](rtc_io_touch_pad3_reg::R) reader structure"]
impl crate::Readable for RTC_IO_TOUCH_PAD3_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_io_touch_pad3_reg::W](rtc_io_touch_pad3_reg::W) writer structure"]
impl crate::Writable for RTC_IO_TOUCH_PAD3_REG {}
#[doc = "RTC_IO_TOUCH_PAD3_REG"]
pub mod rtc_io_touch_pad3_reg;
#[doc = "RTC_IO_TOUCH_PAD4_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_io_touch_pad4_reg](rtc_io_touch_pad4_reg) module"]
pub type RTC_IO_TOUCH_PAD4_REG = crate::Reg<u32, _RTC_IO_TOUCH_PAD4_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_IO_TOUCH_PAD4_REG;
#[doc = "`read()` method returns [rtc_io_touch_pad4_reg::R](rtc_io_touch_pad4_reg::R) reader structure"]
impl crate::Readable for RTC_IO_TOUCH_PAD4_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_io_touch_pad4_reg::W](rtc_io_touch_pad4_reg::W) writer structure"]
impl crate::Writable for RTC_IO_TOUCH_PAD4_REG {}
#[doc = "RTC_IO_TOUCH_PAD4_REG"]
pub mod rtc_io_touch_pad4_reg;
#[doc = "RTC_IO_TOUCH_PAD5_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_io_touch_pad5_reg](rtc_io_touch_pad5_reg) module"]
pub type RTC_IO_TOUCH_PAD5_REG = crate::Reg<u32, _RTC_IO_TOUCH_PAD5_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_IO_TOUCH_PAD5_REG;
#[doc = "`read()` method returns [rtc_io_touch_pad5_reg::R](rtc_io_touch_pad5_reg::R) reader structure"]
impl crate::Readable for RTC_IO_TOUCH_PAD5_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_io_touch_pad5_reg::W](rtc_io_touch_pad5_reg::W) writer structure"]
impl crate::Writable for RTC_IO_TOUCH_PAD5_REG {}
#[doc = "RTC_IO_TOUCH_PAD5_REG"]
pub mod rtc_io_touch_pad5_reg;
#[doc = "RTC_IO_TOUCH_PAD6_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_io_touch_pad6_reg](rtc_io_touch_pad6_reg) module"]
pub type RTC_IO_TOUCH_PAD6_REG = crate::Reg<u32, _RTC_IO_TOUCH_PAD6_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_IO_TOUCH_PAD6_REG;
#[doc = "`read()` method returns [rtc_io_touch_pad6_reg::R](rtc_io_touch_pad6_reg::R) reader structure"]
impl crate::Readable for RTC_IO_TOUCH_PAD6_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_io_touch_pad6_reg::W](rtc_io_touch_pad6_reg::W) writer structure"]
impl crate::Writable for RTC_IO_TOUCH_PAD6_REG {}
#[doc = "RTC_IO_TOUCH_PAD6_REG"]
pub mod rtc_io_touch_pad6_reg;
#[doc = "RTC_IO_TOUCH_PAD7_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_io_touch_pad7_reg](rtc_io_touch_pad7_reg) module"]
pub type RTC_IO_TOUCH_PAD7_REG = crate::Reg<u32, _RTC_IO_TOUCH_PAD7_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_IO_TOUCH_PAD7_REG;
#[doc = "`read()` method returns [rtc_io_touch_pad7_reg::R](rtc_io_touch_pad7_reg::R) reader structure"]
impl crate::Readable for RTC_IO_TOUCH_PAD7_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_io_touch_pad7_reg::W](rtc_io_touch_pad7_reg::W) writer structure"]
impl crate::Writable for RTC_IO_TOUCH_PAD7_REG {}
#[doc = "RTC_IO_TOUCH_PAD7_REG"]
pub mod rtc_io_touch_pad7_reg;
#[doc = "RTC_IO_TOUCH_PAD8_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_io_touch_pad8_reg](rtc_io_touch_pad8_reg) module"]
pub type RTC_IO_TOUCH_PAD8_REG = crate::Reg<u32, _RTC_IO_TOUCH_PAD8_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_IO_TOUCH_PAD8_REG;
#[doc = "`read()` method returns [rtc_io_touch_pad8_reg::R](rtc_io_touch_pad8_reg::R) reader structure"]
impl crate::Readable for RTC_IO_TOUCH_PAD8_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_io_touch_pad8_reg::W](rtc_io_touch_pad8_reg::W) writer structure"]
impl crate::Writable for RTC_IO_TOUCH_PAD8_REG {}
#[doc = "RTC_IO_TOUCH_PAD8_REG"]
pub mod rtc_io_touch_pad8_reg;
#[doc = "RTC_IO_TOUCH_PAD9_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_io_touch_pad9_reg](rtc_io_touch_pad9_reg) module"]
pub type RTC_IO_TOUCH_PAD9_REG = crate::Reg<u32, _RTC_IO_TOUCH_PAD9_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_IO_TOUCH_PAD9_REG;
#[doc = "`read()` method returns [rtc_io_touch_pad9_reg::R](rtc_io_touch_pad9_reg::R) reader structure"]
impl crate::Readable for RTC_IO_TOUCH_PAD9_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_io_touch_pad9_reg::W](rtc_io_touch_pad9_reg::W) writer structure"]
impl crate::Writable for RTC_IO_TOUCH_PAD9_REG {}
#[doc = "RTC_IO_TOUCH_PAD9_REG"]
pub mod rtc_io_touch_pad9_reg;
#[doc = "RTC_IO_EXT_WAKEUP0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_io_ext_wakeup0_reg](rtc_io_ext_wakeup0_reg) module"]
pub type RTC_IO_EXT_WAKEUP0_REG = crate::Reg<u32, _RTC_IO_EXT_WAKEUP0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_IO_EXT_WAKEUP0_REG;
#[doc = "`read()` method returns [rtc_io_ext_wakeup0_reg::R](rtc_io_ext_wakeup0_reg::R) reader structure"]
impl crate::Readable for RTC_IO_EXT_WAKEUP0_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_io_ext_wakeup0_reg::W](rtc_io_ext_wakeup0_reg::W) writer structure"]
impl crate::Writable for RTC_IO_EXT_WAKEUP0_REG {}
#[doc = "RTC_IO_EXT_WAKEUP0_REG"]
pub mod rtc_io_ext_wakeup0_reg;
#[doc = "RTC_IO_XTL_EXT_CTR_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_io_xtl_ext_ctr_reg](rtc_io_xtl_ext_ctr_reg) module"]
pub type RTC_IO_XTL_EXT_CTR_REG = crate::Reg<u32, _RTC_IO_XTL_EXT_CTR_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_IO_XTL_EXT_CTR_REG;
#[doc = "`read()` method returns [rtc_io_xtl_ext_ctr_reg::R](rtc_io_xtl_ext_ctr_reg::R) reader structure"]
impl crate::Readable for RTC_IO_XTL_EXT_CTR_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_io_xtl_ext_ctr_reg::W](rtc_io_xtl_ext_ctr_reg::W) writer structure"]
impl crate::Writable for RTC_IO_XTL_EXT_CTR_REG {}
#[doc = "RTC_IO_XTL_EXT_CTR_REG"]
pub mod rtc_io_xtl_ext_ctr_reg;
#[doc = "RTC_IO_SAR_I2C_IO_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_io_sar_i2c_io_reg](rtc_io_sar_i2c_io_reg) module"]
pub type RTC_IO_SAR_I2C_IO_REG = crate::Reg<u32, _RTC_IO_SAR_I2C_IO_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_IO_SAR_I2C_IO_REG;
#[doc = "`read()` method returns [rtc_io_sar_i2c_io_reg::R](rtc_io_sar_i2c_io_reg::R) reader structure"]
impl crate::Readable for RTC_IO_SAR_I2C_IO_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_io_sar_i2c_io_reg::W](rtc_io_sar_i2c_io_reg::W) writer structure"]
impl crate::Writable for RTC_IO_SAR_I2C_IO_REG {}
#[doc = "RTC_IO_SAR_I2C_IO_REG"]
pub mod rtc_io_sar_i2c_io_reg;
#[doc = "RTC_IO_DATE_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtc_io_date_reg](rtc_io_date_reg) module"]
pub type RTC_IO_DATE_REG = crate::Reg<u32, _RTC_IO_DATE_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_IO_DATE_REG;
#[doc = "`read()` method returns [rtc_io_date_reg::R](rtc_io_date_reg::R) reader structure"]
impl crate::Readable for RTC_IO_DATE_REG {}
#[doc = "`write(|w| ..)` method takes [rtc_io_date_reg::W](rtc_io_date_reg::W) writer structure"]
impl crate::Writable for RTC_IO_DATE_REG {}
#[doc = "RTC_IO_DATE_REG"]
pub mod rtc_io_date_reg;