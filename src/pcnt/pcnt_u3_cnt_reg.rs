#[doc = "Reader of register PCNT_U3_CNT_REG"]
pub type R = crate::R<u32, super::PCNT_U3_CNT_REG>;
#[doc = "Writer for register PCNT_U3_CNT_REG"]
pub type W = crate::W<u32, super::PCNT_U3_CNT_REG>;
#[doc = "Register PCNT_U3_CNT_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::PCNT_U3_CNT_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PCNT_PLUS_CNT_U3`"]
pub type PCNT_PLUS_CNT_U3_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PCNT_PLUS_CNT_U3`"]
pub struct PCNT_PLUS_CNT_U3_W<'a> {
    w: &'a mut W,
}
impl<'a> PCNT_PLUS_CNT_U3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - This register stores the current pulse count value for unit3."]
    #[inline(always)]
    pub fn pcnt_plus_cnt_u3(&self) -> PCNT_PLUS_CNT_U3_R {
        PCNT_PLUS_CNT_U3_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This register stores the current pulse count value for unit3."]
    #[inline(always)]
    pub fn pcnt_plus_cnt_u3(&mut self) -> PCNT_PLUS_CNT_U3_W {
        PCNT_PLUS_CNT_U3_W { w: self }
    }
}