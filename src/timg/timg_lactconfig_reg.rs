#[doc = "Reader of register TIMG_LACTCONFIG_REG"]
pub type R = crate::R<u32, super::TIMG_LACTCONFIG_REG>;
#[doc = "Writer for register TIMG_LACTCONFIG_REG"]
pub type W = crate::W<u32, super::TIMG_LACTCONFIG_REG>;
#[doc = "Register TIMG_LACTCONFIG_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::TIMG_LACTCONFIG_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TIMG_LACT_EN`"]
pub type TIMG_LACT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMG_LACT_EN`"]
pub struct TIMG_LACT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMG_LACT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `TIMG_LACT_INCREASE`"]
pub type TIMG_LACT_INCREASE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMG_LACT_INCREASE`"]
pub struct TIMG_LACT_INCREASE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMG_LACT_INCREASE_W<'a> {
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
#[doc = "Reader of field `TIMG_LACT_AUTORELOAD`"]
pub type TIMG_LACT_AUTORELOAD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMG_LACT_AUTORELOAD`"]
pub struct TIMG_LACT_AUTORELOAD_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMG_LACT_AUTORELOAD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `TIMG_LACT_DIVIDER`"]
pub type TIMG_LACT_DIVIDER_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TIMG_LACT_DIVIDER`"]
pub struct TIMG_LACT_DIVIDER_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMG_LACT_DIVIDER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 13)) | (((value as u32) & 0xffff) << 13);
        self.w
    }
}
#[doc = "Reader of field `TIMG_LACT_EDGE_INT_EN`"]
pub type TIMG_LACT_EDGE_INT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMG_LACT_EDGE_INT_EN`"]
pub struct TIMG_LACT_EDGE_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMG_LACT_EDGE_INT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `TIMG_LACT_LEVEL_INT_EN`"]
pub type TIMG_LACT_LEVEL_INT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMG_LACT_LEVEL_INT_EN`"]
pub struct TIMG_LACT_LEVEL_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMG_LACT_LEVEL_INT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `TIMG_LACT_ALARM_EN`"]
pub type TIMG_LACT_ALARM_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMG_LACT_ALARM_EN`"]
pub struct TIMG_LACT_ALARM_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMG_LACT_ALARM_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `TIMG_LACT_LAC_EN`"]
pub type TIMG_LACT_LAC_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMG_LACT_LAC_EN`"]
pub struct TIMG_LACT_LAC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMG_LACT_LAC_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `TIMG_LACT_CPST_EN`"]
pub type TIMG_LACT_CPST_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMG_LACT_CPST_EN`"]
pub struct TIMG_LACT_CPST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMG_LACT_CPST_EN_W<'a> {
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
#[doc = "Reader of field `TIMG_LACT_RTC_ONLY`"]
pub type TIMG_LACT_RTC_ONLY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMG_LACT_RTC_ONLY`"]
pub struct TIMG_LACT_RTC_ONLY_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMG_LACT_RTC_ONLY_W<'a> {
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
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn timg_lact_en(&self) -> TIMG_LACT_EN_R {
        TIMG_LACT_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn timg_lact_increase(&self) -> TIMG_LACT_INCREASE_R {
        TIMG_LACT_INCREASE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn timg_lact_autoreload(&self) -> TIMG_LACT_AUTORELOAD_R {
        TIMG_LACT_AUTORELOAD_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bits 13:28"]
    #[inline(always)]
    pub fn timg_lact_divider(&self) -> TIMG_LACT_DIVIDER_R {
        TIMG_LACT_DIVIDER_R::new(((self.bits >> 13) & 0xffff) as u16)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn timg_lact_edge_int_en(&self) -> TIMG_LACT_EDGE_INT_EN_R {
        TIMG_LACT_EDGE_INT_EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn timg_lact_level_int_en(&self) -> TIMG_LACT_LEVEL_INT_EN_R {
        TIMG_LACT_LEVEL_INT_EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn timg_lact_alarm_en(&self) -> TIMG_LACT_ALARM_EN_R {
        TIMG_LACT_ALARM_EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn timg_lact_lac_en(&self) -> TIMG_LACT_LAC_EN_R {
        TIMG_LACT_LAC_EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn timg_lact_cpst_en(&self) -> TIMG_LACT_CPST_EN_R {
        TIMG_LACT_CPST_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn timg_lact_rtc_only(&self) -> TIMG_LACT_RTC_ONLY_R {
        TIMG_LACT_RTC_ONLY_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn timg_lact_en(&mut self) -> TIMG_LACT_EN_W {
        TIMG_LACT_EN_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn timg_lact_increase(&mut self) -> TIMG_LACT_INCREASE_W {
        TIMG_LACT_INCREASE_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn timg_lact_autoreload(&mut self) -> TIMG_LACT_AUTORELOAD_W {
        TIMG_LACT_AUTORELOAD_W { w: self }
    }
    #[doc = "Bits 13:28"]
    #[inline(always)]
    pub fn timg_lact_divider(&mut self) -> TIMG_LACT_DIVIDER_W {
        TIMG_LACT_DIVIDER_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn timg_lact_edge_int_en(&mut self) -> TIMG_LACT_EDGE_INT_EN_W {
        TIMG_LACT_EDGE_INT_EN_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn timg_lact_level_int_en(&mut self) -> TIMG_LACT_LEVEL_INT_EN_W {
        TIMG_LACT_LEVEL_INT_EN_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn timg_lact_alarm_en(&mut self) -> TIMG_LACT_ALARM_EN_W {
        TIMG_LACT_ALARM_EN_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn timg_lact_lac_en(&mut self) -> TIMG_LACT_LAC_EN_W {
        TIMG_LACT_LAC_EN_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn timg_lact_cpst_en(&mut self) -> TIMG_LACT_CPST_EN_W {
        TIMG_LACT_CPST_EN_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn timg_lact_rtc_only(&mut self) -> TIMG_LACT_RTC_ONLY_W {
        TIMG_LACT_RTC_ONLY_W { w: self }
    }
}
