#[doc = "Reader of register SARADC_SAR2_PATT_TAB2"]
pub type R = crate::R<u32, super::SARADC_SAR2_PATT_TAB2>;
#[doc = "Writer for register SARADC_SAR2_PATT_TAB2"]
pub type W = crate::W<u32, super::SARADC_SAR2_PATT_TAB2>;
#[doc = "Register SARADC_SAR2_PATT_TAB2 `reset()`'s with value 0"]
impl crate::ResetValue for super::SARADC_SAR2_PATT_TAB2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SYSCON_SARADC_SAR2_PATT_TAB2`"]
pub type SYSCON_SARADC_SAR2_PATT_TAB2_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SYSCON_SARADC_SAR2_PATT_TAB2`"]
pub struct SYSCON_SARADC_SAR2_PATT_TAB2_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCON_SARADC_SAR2_PATT_TAB2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Item 4 ~ 7 for pattern table 2 (each item one byte)"]
    #[inline(always)]
    pub fn syscon_saradc_sar2_patt_tab2(&self) -> SYSCON_SARADC_SAR2_PATT_TAB2_R {
        SYSCON_SARADC_SAR2_PATT_TAB2_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Item 4 ~ 7 for pattern table 2 (each item one byte)"]
    #[inline(always)]
    pub fn syscon_saradc_sar2_patt_tab2(&mut self) -> SYSCON_SARADC_SAR2_PATT_TAB2_W {
        SYSCON_SARADC_SAR2_PATT_TAB2_W { w: self }
    }
}