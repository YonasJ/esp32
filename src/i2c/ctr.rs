#[doc = "Reader of register CTR"]
pub type R = crate::R<u32, super::CTR>;
#[doc = "Writer for register CTR"]
pub type W = crate::W<u32, super::CTR>;
#[doc = "Register CTR `reset()`'s with value 0"]
impl crate::ResetValue for super::CTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `I2C_CLK_EN`"]
pub type I2C_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_CLK_EN`"]
pub struct I2C_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_CLK_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `I2C_RX_LSB_FIRST`"]
pub type I2C_RX_LSB_FIRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_RX_LSB_FIRST`"]
pub struct I2C_RX_LSB_FIRST_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_RX_LSB_FIRST_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `I2C_TX_LSB_FIRST`"]
pub type I2C_TX_LSB_FIRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_TX_LSB_FIRST`"]
pub struct I2C_TX_LSB_FIRST_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_TX_LSB_FIRST_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `I2C_TRANS_START`"]
pub type I2C_TRANS_START_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_TRANS_START`"]
pub struct I2C_TRANS_START_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_TRANS_START_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `I2C_MS_MODE`"]
pub type I2C_MS_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_MS_MODE`"]
pub struct I2C_MS_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_MS_MODE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `I2C_SAMPLE_SCL_LEVEL`"]
pub type I2C_SAMPLE_SCL_LEVEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_SAMPLE_SCL_LEVEL`"]
pub struct I2C_SAMPLE_SCL_LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_SAMPLE_SCL_LEVEL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `I2C_SCL_FORCE_OUT`"]
pub type I2C_SCL_FORCE_OUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_SCL_FORCE_OUT`"]
pub struct I2C_SCL_FORCE_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_SCL_FORCE_OUT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `I2C_SDA_FORCE_OUT`"]
pub type I2C_SDA_FORCE_OUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_SDA_FORCE_OUT`"]
pub struct I2C_SDA_FORCE_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_SDA_FORCE_OUT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 8 - This is the clock gating control bit for reading or writing registers."]
    #[inline(always)]
    pub fn i2c_clk_en(&self) -> I2C_CLK_EN_R {
        I2C_CLK_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - This bit is used to control the storage mode for received datas. 1: receive data from most significant bit 0: receive data from least significant bit"]
    #[inline(always)]
    pub fn i2c_rx_lsb_first(&self) -> I2C_RX_LSB_FIRST_R {
        I2C_RX_LSB_FIRST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - This bit is used to control the sending mode for data need to be send. 1: receive data from most significant bit 0: receive data from least significant bit"]
    #[inline(always)]
    pub fn i2c_tx_lsb_first(&self) -> I2C_TX_LSB_FIRST_R {
        I2C_TX_LSB_FIRST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Set this bit to start sending data in txfifo."]
    #[inline(always)]
    pub fn i2c_trans_start(&self) -> I2C_TRANS_START_R {
        I2C_TRANS_START_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Set this bit to configure the module as i2c master clear this bit to configure the module as i2c slave."]
    #[inline(always)]
    pub fn i2c_ms_mode(&self) -> I2C_MS_MODE_R {
        I2C_MS_MODE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Set this bit to sample data in SCL low level. clear this bit to sample data in SCL high level."]
    #[inline(always)]
    pub fn i2c_sample_scl_level(&self) -> I2C_SAMPLE_SCL_LEVEL_R {
        I2C_SAMPLE_SCL_LEVEL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1: normally ouput scl clock 0: exchange the function of scl_o and scl_oe (scl_o is the original internal output scl signal scl_oe is the enable bit for the internal output scl signal)"]
    #[inline(always)]
    pub fn i2c_scl_force_out(&self) -> I2C_SCL_FORCE_OUT_R {
        I2C_SCL_FORCE_OUT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 1: normally ouput sda data 0: exchange the function of sda_o and sda_oe (sda_o is the original internal output sda signal sda_oe is the enable bit for the internal output sda signal)"]
    #[inline(always)]
    pub fn i2c_sda_force_out(&self) -> I2C_SDA_FORCE_OUT_R {
        I2C_SDA_FORCE_OUT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - This is the clock gating control bit for reading or writing registers."]
    #[inline(always)]
    pub fn i2c_clk_en(&mut self) -> I2C_CLK_EN_W {
        I2C_CLK_EN_W { w: self }
    }
    #[doc = "Bit 7 - This bit is used to control the storage mode for received datas. 1: receive data from most significant bit 0: receive data from least significant bit"]
    #[inline(always)]
    pub fn i2c_rx_lsb_first(&mut self) -> I2C_RX_LSB_FIRST_W {
        I2C_RX_LSB_FIRST_W { w: self }
    }
    #[doc = "Bit 6 - This bit is used to control the sending mode for data need to be send. 1: receive data from most significant bit 0: receive data from least significant bit"]
    #[inline(always)]
    pub fn i2c_tx_lsb_first(&mut self) -> I2C_TX_LSB_FIRST_W {
        I2C_TX_LSB_FIRST_W { w: self }
    }
    #[doc = "Bit 5 - Set this bit to start sending data in txfifo."]
    #[inline(always)]
    pub fn i2c_trans_start(&mut self) -> I2C_TRANS_START_W {
        I2C_TRANS_START_W { w: self }
    }
    #[doc = "Bit 4 - Set this bit to configure the module as i2c master clear this bit to configure the module as i2c slave."]
    #[inline(always)]
    pub fn i2c_ms_mode(&mut self) -> I2C_MS_MODE_W {
        I2C_MS_MODE_W { w: self }
    }
    #[doc = "Bit 2 - Set this bit to sample data in SCL low level. clear this bit to sample data in SCL high level."]
    #[inline(always)]
    pub fn i2c_sample_scl_level(&mut self) -> I2C_SAMPLE_SCL_LEVEL_W {
        I2C_SAMPLE_SCL_LEVEL_W { w: self }
    }
    #[doc = "Bit 1 - 1: normally ouput scl clock 0: exchange the function of scl_o and scl_oe (scl_o is the original internal output scl signal scl_oe is the enable bit for the internal output scl signal)"]
    #[inline(always)]
    pub fn i2c_scl_force_out(&mut self) -> I2C_SCL_FORCE_OUT_W {
        I2C_SCL_FORCE_OUT_W { w: self }
    }
    #[doc = "Bit 0 - 1: normally ouput sda data 0: exchange the function of sda_o and sda_oe (sda_o is the original internal output sda signal sda_oe is the enable bit for the internal output sda signal)"]
    #[inline(always)]
    pub fn i2c_sda_force_out(&mut self) -> I2C_SDA_FORCE_OUT_W {
        I2C_SDA_FORCE_OUT_W { w: self }
    }
}