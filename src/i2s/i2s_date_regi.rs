#[doc = "Reader of register I2S_DATE_REG(i)"]
pub type R = crate::R<u32, super::I2S_DATE_REGI>;
#[doc = "Writer for register I2S_DATE_REG(i)"]
pub type W = crate::W<u32, super::I2S_DATE_REGI>;
#[doc = "Register I2S_DATE_REG(i) `reset()`'s with value 0"]
impl crate::ResetValue for super::I2S_DATE_REGI {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `I2S_I2SDATE`"]
pub type I2S_I2SDATE_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `I2S_I2SDATE`"]
pub struct I2S_I2SDATE_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_I2SDATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn i2s_i2sdate(&self) -> I2S_I2SDATE_R {
        I2S_I2SDATE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn i2s_i2sdate(&mut self) -> I2S_I2SDATE_W {
        I2S_I2SDATE_W { w: self }
    }
}