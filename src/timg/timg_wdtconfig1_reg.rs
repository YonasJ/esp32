#[doc = "Reader of register TIMG_WDTCONFIG1_REG"]
pub type R = crate::R<u32, super::TIMG_WDTCONFIG1_REG>;
#[doc = "Writer for register TIMG_WDTCONFIG1_REG"]
pub type W = crate::W<u32, super::TIMG_WDTCONFIG1_REG>;
#[doc = "Register TIMG_WDTCONFIG1_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::TIMG_WDTCONFIG1_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TIMG_WDT_CLK_PRESCALE`"]
pub type TIMG_WDT_CLK_PRESCALE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TIMG_WDT_CLK_PRESCALE`"]
pub struct TIMG_WDT_CLK_PRESCALE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMG_WDT_CLK_PRESCALE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - SWDT clock prescale value. Period = 12.5ns * value stored in this register"]
    #[inline(always)]
    pub fn timg_wdt_clk_prescale(&self) -> TIMG_WDT_CLK_PRESCALE_R {
        TIMG_WDT_CLK_PRESCALE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - SWDT clock prescale value. Period = 12.5ns * value stored in this register"]
    #[inline(always)]
    pub fn timg_wdt_clk_prescale(&mut self) -> TIMG_WDT_CLK_PRESCALE_W {
        TIMG_WDT_CLK_PRESCALE_W { w: self }
    }
}
