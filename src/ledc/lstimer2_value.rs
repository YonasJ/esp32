#[doc = "Reader of register LSTIMER2_VALUE"]
pub type R = crate::R<u32, super::LSTIMER2_VALUE>;
#[doc = "Writer for register LSTIMER2_VALUE"]
pub type W = crate::W<u32, super::LSTIMER2_VALUE>;
#[doc = "Register LSTIMER2_VALUE `reset()`'s with value 0"]
impl crate::ResetValue for super::LSTIMER2_VALUE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LEDC_LSTIMER2_CNT`"]
pub type LEDC_LSTIMER2_CNT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `LEDC_LSTIMER2_CNT`"]
pub struct LEDC_LSTIMER2_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_LSTIMER2_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | ((value as u32) & 0x000f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:19 - software can read this register to get the current counter value in low speed timer2."]
    #[inline(always)]
    pub fn ledc_lstimer2_cnt(&self) -> LEDC_LSTIMER2_CNT_R {
        LEDC_LSTIMER2_CNT_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:19 - software can read this register to get the current counter value in low speed timer2."]
    #[inline(always)]
    pub fn ledc_lstimer2_cnt(&mut self) -> LEDC_LSTIMER2_CNT_W {
        LEDC_LSTIMER2_CNT_W { w: self }
    }
}