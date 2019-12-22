#[doc = "Reader of register SENS_SAR_SLAVE_ADDR4_REG"]
pub type R = crate::R<u32, super::SENS_SAR_SLAVE_ADDR4_REG>;
#[doc = "Writer for register SENS_SAR_SLAVE_ADDR4_REG"]
pub type W = crate::W<u32, super::SENS_SAR_SLAVE_ADDR4_REG>;
#[doc = "Register SENS_SAR_SLAVE_ADDR4_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::SENS_SAR_SLAVE_ADDR4_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SENS_I2C_DONE`"]
pub type SENS_I2C_DONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SENS_I2C_DONE`"]
pub struct SENS_I2C_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> SENS_I2C_DONE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `SENS_I2C_RDATA`"]
pub type SENS_I2C_RDATA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENS_I2C_RDATA`"]
pub struct SENS_I2C_RDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> SENS_I2C_RDATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 22)) | (((value as u32) & 0xff) << 22);
        self.w
    }
}
#[doc = "Reader of field `SENS_I2C_SLAVE_ADDR6`"]
pub type SENS_I2C_SLAVE_ADDR6_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SENS_I2C_SLAVE_ADDR6`"]
pub struct SENS_I2C_SLAVE_ADDR6_W<'a> {
    w: &'a mut W,
}
impl<'a> SENS_I2C_SLAVE_ADDR6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 11)) | (((value as u32) & 0x07ff) << 11);
        self.w
    }
}
#[doc = "Reader of field `SENS_I2C_SLAVE_ADDR7`"]
pub type SENS_I2C_SLAVE_ADDR7_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SENS_I2C_SLAVE_ADDR7`"]
pub struct SENS_I2C_SLAVE_ADDR7_W<'a> {
    w: &'a mut W,
}
impl<'a> SENS_I2C_SLAVE_ADDR7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | ((value as u32) & 0x07ff);
        self.w
    }
}
impl R {
    #[doc = "Bit 30 - indicate I2C done"]
    #[inline(always)]
    pub fn sens_i2c_done(&self) -> SENS_I2C_DONE_R {
        SENS_I2C_DONE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bits 22:29 - I2C read data"]
    #[inline(always)]
    pub fn sens_i2c_rdata(&self) -> SENS_I2C_RDATA_R {
        SENS_I2C_RDATA_R::new(((self.bits >> 22) & 0xff) as u8)
    }
    #[doc = "Bits 11:21"]
    #[inline(always)]
    pub fn sens_i2c_slave_addr6(&self) -> SENS_I2C_SLAVE_ADDR6_R {
        SENS_I2C_SLAVE_ADDR6_R::new(((self.bits >> 11) & 0x07ff) as u16)
    }
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn sens_i2c_slave_addr7(&self) -> SENS_I2C_SLAVE_ADDR7_R {
        SENS_I2C_SLAVE_ADDR7_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bit 30 - indicate I2C done"]
    #[inline(always)]
    pub fn sens_i2c_done(&mut self) -> SENS_I2C_DONE_W {
        SENS_I2C_DONE_W { w: self }
    }
    #[doc = "Bits 22:29 - I2C read data"]
    #[inline(always)]
    pub fn sens_i2c_rdata(&mut self) -> SENS_I2C_RDATA_W {
        SENS_I2C_RDATA_W { w: self }
    }
    #[doc = "Bits 11:21"]
    #[inline(always)]
    pub fn sens_i2c_slave_addr6(&mut self) -> SENS_I2C_SLAVE_ADDR6_W {
        SENS_I2C_SLAVE_ADDR6_W { w: self }
    }
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn sens_i2c_slave_addr7(&mut self) -> SENS_I2C_SLAVE_ADDR7_W {
        SENS_I2C_SLAVE_ADDR7_W { w: self }
    }
}
