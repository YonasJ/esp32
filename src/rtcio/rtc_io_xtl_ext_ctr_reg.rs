#[doc = "Reader of register RTC_IO_XTL_EXT_CTR_REG"]
pub type R = crate::R<u32, super::RTC_IO_XTL_EXT_CTR_REG>;
#[doc = "Writer for register RTC_IO_XTL_EXT_CTR_REG"]
pub type W = crate::W<u32, super::RTC_IO_XTL_EXT_CTR_REG>;
#[doc = "Register RTC_IO_XTL_EXT_CTR_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::RTC_IO_XTL_EXT_CTR_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTC_IO_XTL_EXT_CTR_SEL`"]
pub type RTC_IO_XTL_EXT_CTR_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_IO_XTL_EXT_CTR_SEL`"]
pub struct RTC_IO_XTL_EXT_CTR_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_XTL_EXT_CTR_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 27)) | (((value as u32) & 0x1f) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bits 27:31 - select the external xtl power source \u{d3}0\u{d3} select GPIO0 \u{d3}1\u{d3} select GPIO2 ...\u{d2}17\u{d3} select GPIO17"]
    #[inline(always)]
    pub fn rtc_io_xtl_ext_ctr_sel(&self) -> RTC_IO_XTL_EXT_CTR_SEL_R {
        RTC_IO_XTL_EXT_CTR_SEL_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 27:31 - select the external xtl power source \u{d3}0\u{d3} select GPIO0 \u{d3}1\u{d3} select GPIO2 ...\u{d2}17\u{d3} select GPIO17"]
    #[inline(always)]
    pub fn rtc_io_xtl_ext_ctr_sel(&mut self) -> RTC_IO_XTL_EXT_CTR_SEL_W {
        RTC_IO_XTL_EXT_CTR_SEL_W { w: self }
    }
}