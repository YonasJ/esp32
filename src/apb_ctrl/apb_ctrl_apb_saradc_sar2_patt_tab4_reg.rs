#[doc = "Reader of register APB_CTRL_APB_SARADC_SAR2_PATT_TAB4_REG"]
pub type R = crate::R<u32, super::APB_CTRL_APB_SARADC_SAR2_PATT_TAB4_REG>;
#[doc = "Writer for register APB_CTRL_APB_SARADC_SAR2_PATT_TAB4_REG"]
pub type W = crate::W<u32, super::APB_CTRL_APB_SARADC_SAR2_PATT_TAB4_REG>;
#[doc = "Register APB_CTRL_APB_SARADC_SAR2_PATT_TAB4_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::APB_CTRL_APB_SARADC_SAR2_PATT_TAB4_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `APB_CTRL_SARADC_SAR2_PATT_TAB4`"]
pub type APB_CTRL_SARADC_SAR2_PATT_TAB4_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `APB_CTRL_SARADC_SAR2_PATT_TAB4`"]
pub struct APB_CTRL_SARADC_SAR2_PATT_TAB4_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_CTRL_SARADC_SAR2_PATT_TAB4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Item 12 ~ 15 for pattern table 2 (each item one byte)"]
    #[inline(always)]
    pub fn apb_ctrl_saradc_sar2_patt_tab4(&self) -> APB_CTRL_SARADC_SAR2_PATT_TAB4_R {
        APB_CTRL_SARADC_SAR2_PATT_TAB4_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Item 12 ~ 15 for pattern table 2 (each item one byte)"]
    #[inline(always)]
    pub fn apb_ctrl_saradc_sar2_patt_tab4(&mut self) -> APB_CTRL_SARADC_SAR2_PATT_TAB4_W {
        APB_CTRL_SARADC_SAR2_PATT_TAB4_W { w: self }
    }
}
