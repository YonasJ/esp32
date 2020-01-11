#[doc = "Reader of register INT_CLR"]
pub type R = crate::R<u32, super::INT_CLR>;
#[doc = "Writer for register INT_CLR"]
pub type W = crate::W<u32, super::INT_CLR>;
#[doc = "Register INT_CLR `reset()`'s with value 0"]
impl crate::ResetValue for super::INT_CLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTC_I2C_TIME_OUT_INT_CLR`"]
pub type RTC_I2C_TIME_OUT_INT_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_I2C_TIME_OUT_INT_CLR`"]
pub struct RTC_I2C_TIME_OUT_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_I2C_TIME_OUT_INT_CLR_W<'a> {
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
#[doc = "Reader of field `RTC_I2C_TRANS_COMPLETE_INT_CLR`"]
pub type RTC_I2C_TRANS_COMPLETE_INT_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_I2C_TRANS_COMPLETE_INT_CLR`"]
pub struct RTC_I2C_TRANS_COMPLETE_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_I2C_TRANS_COMPLETE_INT_CLR_W<'a> {
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
#[doc = "Reader of field `RTC_I2C_MASTER_TRANS_COMPLETE_INT_CLR`"]
pub type RTC_I2C_MASTER_TRANS_COMPLETE_INT_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_I2C_MASTER_TRANS_COMPLETE_INT_CLR`"]
pub struct RTC_I2C_MASTER_TRANS_COMPLETE_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_I2C_MASTER_TRANS_COMPLETE_INT_CLR_W<'a> {
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
#[doc = "Reader of field `RTC_I2C_ARBITRATION_LOST_INT_CLR`"]
pub type RTC_I2C_ARBITRATION_LOST_INT_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_I2C_ARBITRATION_LOST_INT_CLR`"]
pub struct RTC_I2C_ARBITRATION_LOST_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_I2C_ARBITRATION_LOST_INT_CLR_W<'a> {
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
#[doc = "Reader of field `RTC_I2C_SLAVE_TRANS_COMPLETE_INT_CLR`"]
pub type RTC_I2C_SLAVE_TRANS_COMPLETE_INT_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_I2C_SLAVE_TRANS_COMPLETE_INT_CLR`"]
pub struct RTC_I2C_SLAVE_TRANS_COMPLETE_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_I2C_SLAVE_TRANS_COMPLETE_INT_CLR_W<'a> {
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
impl R {
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rtc_i2c_time_out_int_clr(&self) -> RTC_I2C_TIME_OUT_INT_CLR_R {
        RTC_I2C_TIME_OUT_INT_CLR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn rtc_i2c_trans_complete_int_clr(&self) -> RTC_I2C_TRANS_COMPLETE_INT_CLR_R {
        RTC_I2C_TRANS_COMPLETE_INT_CLR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rtc_i2c_master_trans_complete_int_clr(&self) -> RTC_I2C_MASTER_TRANS_COMPLETE_INT_CLR_R {
        RTC_I2C_MASTER_TRANS_COMPLETE_INT_CLR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rtc_i2c_arbitration_lost_int_clr(&self) -> RTC_I2C_ARBITRATION_LOST_INT_CLR_R {
        RTC_I2C_ARBITRATION_LOST_INT_CLR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rtc_i2c_slave_trans_complete_int_clr(&self) -> RTC_I2C_SLAVE_TRANS_COMPLETE_INT_CLR_R {
        RTC_I2C_SLAVE_TRANS_COMPLETE_INT_CLR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rtc_i2c_time_out_int_clr(&mut self) -> RTC_I2C_TIME_OUT_INT_CLR_W {
        RTC_I2C_TIME_OUT_INT_CLR_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn rtc_i2c_trans_complete_int_clr(&mut self) -> RTC_I2C_TRANS_COMPLETE_INT_CLR_W {
        RTC_I2C_TRANS_COMPLETE_INT_CLR_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rtc_i2c_master_trans_complete_int_clr(
        &mut self,
    ) -> RTC_I2C_MASTER_TRANS_COMPLETE_INT_CLR_W {
        RTC_I2C_MASTER_TRANS_COMPLETE_INT_CLR_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rtc_i2c_arbitration_lost_int_clr(&mut self) -> RTC_I2C_ARBITRATION_LOST_INT_CLR_W {
        RTC_I2C_ARBITRATION_LOST_INT_CLR_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rtc_i2c_slave_trans_complete_int_clr(
        &mut self,
    ) -> RTC_I2C_SLAVE_TRANS_COMPLETE_INT_CLR_W {
        RTC_I2C_SLAVE_TRANS_COMPLETE_INT_CLR_W { w: self }
    }
}