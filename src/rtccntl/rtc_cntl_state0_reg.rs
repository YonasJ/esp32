#[doc = "Reader of register RTC_CNTL_STATE0_REG"]
pub type R = crate::R<u32, super::RTC_CNTL_STATE0_REG>;
#[doc = "Writer for register RTC_CNTL_STATE0_REG"]
pub type W = crate::W<u32, super::RTC_CNTL_STATE0_REG>;
#[doc = "Register RTC_CNTL_STATE0_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::RTC_CNTL_STATE0_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTC_CNTL_SLEEP_EN`"]
pub type RTC_CNTL_SLEEP_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_SLEEP_EN`"]
pub struct RTC_CNTL_SLEEP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_SLEEP_EN_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_SLP_REJECT`"]
pub type RTC_CNTL_SLP_REJECT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_SLP_REJECT`"]
pub struct RTC_CNTL_SLP_REJECT_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_SLP_REJECT_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_SLP_WAKEUP`"]
pub type RTC_CNTL_SLP_WAKEUP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_SLP_WAKEUP`"]
pub struct RTC_CNTL_SLP_WAKEUP_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_SLP_WAKEUP_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_SDIO_ACTIVE_IND`"]
pub type RTC_CNTL_SDIO_ACTIVE_IND_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_SDIO_ACTIVE_IND`"]
pub struct RTC_CNTL_SDIO_ACTIVE_IND_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_SDIO_ACTIVE_IND_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_ULP_CP_SLP_TIMER_EN`"]
pub type RTC_CNTL_ULP_CP_SLP_TIMER_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_ULP_CP_SLP_TIMER_EN`"]
pub struct RTC_CNTL_ULP_CP_SLP_TIMER_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_ULP_CP_SLP_TIMER_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_TOUCH_SLP_TIMER_EN`"]
pub type RTC_CNTL_TOUCH_SLP_TIMER_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_TOUCH_SLP_TIMER_EN`"]
pub struct RTC_CNTL_TOUCH_SLP_TIMER_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_TOUCH_SLP_TIMER_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_APB2RTC_BRIDGE_SEL`"]
pub type RTC_CNTL_APB2RTC_BRIDGE_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_APB2RTC_BRIDGE_SEL`"]
pub struct RTC_CNTL_APB2RTC_BRIDGE_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_APB2RTC_BRIDGE_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_ULP_CP_WAKEUP_FORCE_EN`"]
pub type RTC_CNTL_ULP_CP_WAKEUP_FORCE_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_ULP_CP_WAKEUP_FORCE_EN`"]
pub struct RTC_CNTL_ULP_CP_WAKEUP_FORCE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_ULP_CP_WAKEUP_FORCE_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_TOUCH_WAKEUP_FORCE_EN`"]
pub type RTC_CNTL_TOUCH_WAKEUP_FORCE_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_TOUCH_WAKEUP_FORCE_EN`"]
pub struct RTC_CNTL_TOUCH_WAKEUP_FORCE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_TOUCH_WAKEUP_FORCE_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - sleep enable bit"]
    #[inline(always)]
    pub fn rtc_cntl_sleep_en(&self) -> RTC_CNTL_SLEEP_EN_R {
        RTC_CNTL_SLEEP_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - sleep reject bit"]
    #[inline(always)]
    pub fn rtc_cntl_slp_reject(&self) -> RTC_CNTL_SLP_REJECT_R {
        RTC_CNTL_SLP_REJECT_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - sleep wakeup bit"]
    #[inline(always)]
    pub fn rtc_cntl_slp_wakeup(&self) -> RTC_CNTL_SLP_WAKEUP_R {
        RTC_CNTL_SLP_WAKEUP_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - SDIO active indication"]
    #[inline(always)]
    pub fn rtc_cntl_sdio_active_ind(&self) -> RTC_CNTL_SDIO_ACTIVE_IND_R {
        RTC_CNTL_SDIO_ACTIVE_IND_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 24 - ULP-coprocessor timer enable bit"]
    #[inline(always)]
    pub fn rtc_cntl_ulp_cp_slp_timer_en(&self) -> RTC_CNTL_ULP_CP_SLP_TIMER_EN_R {
        RTC_CNTL_ULP_CP_SLP_TIMER_EN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - touch timer enable bit"]
    #[inline(always)]
    pub fn rtc_cntl_touch_slp_timer_en(&self) -> RTC_CNTL_TOUCH_SLP_TIMER_EN_R {
        RTC_CNTL_TOUCH_SLP_TIMER_EN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - 1: APB to RTC using bridge 0: APB to RTC using sync"]
    #[inline(always)]
    pub fn rtc_cntl_apb2rtc_bridge_sel(&self) -> RTC_CNTL_APB2RTC_BRIDGE_SEL_R {
        RTC_CNTL_APB2RTC_BRIDGE_SEL_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - ULP-coprocessor force wake up"]
    #[inline(always)]
    pub fn rtc_cntl_ulp_cp_wakeup_force_en(&self) -> RTC_CNTL_ULP_CP_WAKEUP_FORCE_EN_R {
        RTC_CNTL_ULP_CP_WAKEUP_FORCE_EN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - touch controller force wake up"]
    #[inline(always)]
    pub fn rtc_cntl_touch_wakeup_force_en(&self) -> RTC_CNTL_TOUCH_WAKEUP_FORCE_EN_R {
        RTC_CNTL_TOUCH_WAKEUP_FORCE_EN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - sleep enable bit"]
    #[inline(always)]
    pub fn rtc_cntl_sleep_en(&mut self) -> RTC_CNTL_SLEEP_EN_W {
        RTC_CNTL_SLEEP_EN_W { w: self }
    }
    #[doc = "Bit 30 - sleep reject bit"]
    #[inline(always)]
    pub fn rtc_cntl_slp_reject(&mut self) -> RTC_CNTL_SLP_REJECT_W {
        RTC_CNTL_SLP_REJECT_W { w: self }
    }
    #[doc = "Bit 29 - sleep wakeup bit"]
    #[inline(always)]
    pub fn rtc_cntl_slp_wakeup(&mut self) -> RTC_CNTL_SLP_WAKEUP_W {
        RTC_CNTL_SLP_WAKEUP_W { w: self }
    }
    #[doc = "Bit 28 - SDIO active indication"]
    #[inline(always)]
    pub fn rtc_cntl_sdio_active_ind(&mut self) -> RTC_CNTL_SDIO_ACTIVE_IND_W {
        RTC_CNTL_SDIO_ACTIVE_IND_W { w: self }
    }
    #[doc = "Bit 24 - ULP-coprocessor timer enable bit"]
    #[inline(always)]
    pub fn rtc_cntl_ulp_cp_slp_timer_en(&mut self) -> RTC_CNTL_ULP_CP_SLP_TIMER_EN_W {
        RTC_CNTL_ULP_CP_SLP_TIMER_EN_W { w: self }
    }
    #[doc = "Bit 23 - touch timer enable bit"]
    #[inline(always)]
    pub fn rtc_cntl_touch_slp_timer_en(&mut self) -> RTC_CNTL_TOUCH_SLP_TIMER_EN_W {
        RTC_CNTL_TOUCH_SLP_TIMER_EN_W { w: self }
    }
    #[doc = "Bit 22 - 1: APB to RTC using bridge 0: APB to RTC using sync"]
    #[inline(always)]
    pub fn rtc_cntl_apb2rtc_bridge_sel(&mut self) -> RTC_CNTL_APB2RTC_BRIDGE_SEL_W {
        RTC_CNTL_APB2RTC_BRIDGE_SEL_W { w: self }
    }
    #[doc = "Bit 21 - ULP-coprocessor force wake up"]
    #[inline(always)]
    pub fn rtc_cntl_ulp_cp_wakeup_force_en(&mut self) -> RTC_CNTL_ULP_CP_WAKEUP_FORCE_EN_W {
        RTC_CNTL_ULP_CP_WAKEUP_FORCE_EN_W { w: self }
    }
    #[doc = "Bit 20 - touch controller force wake up"]
    #[inline(always)]
    pub fn rtc_cntl_touch_wakeup_force_en(&mut self) -> RTC_CNTL_TOUCH_WAKEUP_FORCE_EN_W {
        RTC_CNTL_TOUCH_WAKEUP_FORCE_EN_W { w: self }
    }
}
