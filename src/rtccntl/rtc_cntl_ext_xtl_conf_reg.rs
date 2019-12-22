#[doc = "Reader of register RTC_CNTL_EXT_XTL_CONF_REG"]
pub type R = crate::R<u32, super::RTC_CNTL_EXT_XTL_CONF_REG>;
#[doc = "Writer for register RTC_CNTL_EXT_XTL_CONF_REG"]
pub type W = crate::W<u32, super::RTC_CNTL_EXT_XTL_CONF_REG>;
#[doc = "Register RTC_CNTL_EXT_XTL_CONF_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::RTC_CNTL_EXT_XTL_CONF_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTC_CNTL_XTL_EXT_CTR_EN`"]
pub type RTC_CNTL_XTL_EXT_CTR_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_XTL_EXT_CTR_EN`"]
pub struct RTC_CNTL_XTL_EXT_CTR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_XTL_EXT_CTR_EN_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_XTL_EXT_CTR_LV`"]
pub type RTC_CNTL_XTL_EXT_CTR_LV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_XTL_EXT_CTR_LV`"]
pub struct RTC_CNTL_XTL_EXT_CTR_LV_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_XTL_EXT_CTR_LV_W<'a> {
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
impl R {
    #[doc = "Bit 31 - enable control XTAL by external pads"]
    #[inline(always)]
    pub fn rtc_cntl_xtl_ext_ctr_en(&self) -> RTC_CNTL_XTL_EXT_CTR_EN_R {
        RTC_CNTL_XTL_EXT_CTR_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - 0: power down XTAL at high level 1: power down XTAL at low level"]
    #[inline(always)]
    pub fn rtc_cntl_xtl_ext_ctr_lv(&self) -> RTC_CNTL_XTL_EXT_CTR_LV_R {
        RTC_CNTL_XTL_EXT_CTR_LV_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - enable control XTAL by external pads"]
    #[inline(always)]
    pub fn rtc_cntl_xtl_ext_ctr_en(&mut self) -> RTC_CNTL_XTL_EXT_CTR_EN_W {
        RTC_CNTL_XTL_EXT_CTR_EN_W { w: self }
    }
    #[doc = "Bit 30 - 0: power down XTAL at high level 1: power down XTAL at low level"]
    #[inline(always)]
    pub fn rtc_cntl_xtl_ext_ctr_lv(&mut self) -> RTC_CNTL_XTL_EXT_CTR_LV_W {
        RTC_CNTL_XTL_EXT_CTR_LV_W { w: self }
    }
}
