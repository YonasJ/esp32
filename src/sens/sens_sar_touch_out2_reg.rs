#[doc = "Reader of register SENS_SAR_TOUCH_OUT2_REG"]
pub type R = crate::R<u32, super::SENS_SAR_TOUCH_OUT2_REG>;
#[doc = "Writer for register SENS_SAR_TOUCH_OUT2_REG"]
pub type W = crate::W<u32, super::SENS_SAR_TOUCH_OUT2_REG>;
#[doc = "Register SENS_SAR_TOUCH_OUT2_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::SENS_SAR_TOUCH_OUT2_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SENS_TOUCH_MEAS_OUT2`"]
pub type SENS_TOUCH_MEAS_OUT2_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SENS_TOUCH_MEAS_OUT2`"]
pub struct SENS_TOUCH_MEAS_OUT2_W<'a> {
    w: &'a mut W,
}
impl<'a> SENS_TOUCH_MEAS_OUT2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `SENS_TOUCH_MEAS_OUT3`"]
pub type SENS_TOUCH_MEAS_OUT3_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SENS_TOUCH_MEAS_OUT3`"]
pub struct SENS_TOUCH_MEAS_OUT3_W<'a> {
    w: &'a mut W,
}
impl<'a> SENS_TOUCH_MEAS_OUT3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - the counter for touch pad 2"]
    #[inline(always)]
    pub fn sens_touch_meas_out2(&self) -> SENS_TOUCH_MEAS_OUT2_R {
        SENS_TOUCH_MEAS_OUT2_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - the counter for touch pad 3"]
    #[inline(always)]
    pub fn sens_touch_meas_out3(&self) -> SENS_TOUCH_MEAS_OUT3_R {
        SENS_TOUCH_MEAS_OUT3_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - the counter for touch pad 2"]
    #[inline(always)]
    pub fn sens_touch_meas_out2(&mut self) -> SENS_TOUCH_MEAS_OUT2_W {
        SENS_TOUCH_MEAS_OUT2_W { w: self }
    }
    #[doc = "Bits 0:15 - the counter for touch pad 3"]
    #[inline(always)]
    pub fn sens_touch_meas_out3(&mut self) -> SENS_TOUCH_MEAS_OUT3_W {
        SENS_TOUCH_MEAS_OUT3_W { w: self }
    }
}
