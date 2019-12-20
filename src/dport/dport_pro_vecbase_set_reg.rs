#[doc = "Reader of register DPORT_PRO_VECBASE_SET_REG"]
pub type R = crate::R<u32, super::DPORT_PRO_VECBASE_SET_REG>;
#[doc = "Writer for register DPORT_PRO_VECBASE_SET_REG"]
pub type W = crate::W<u32, super::DPORT_PRO_VECBASE_SET_REG>;
#[doc = "Register DPORT_PRO_VECBASE_SET_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::DPORT_PRO_VECBASE_SET_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DPORT_PRO_OUT_VECBASE_REG`"]
pub type DPORT_PRO_OUT_VECBASE_REG_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DPORT_PRO_OUT_VECBASE_REG`"]
pub struct DPORT_PRO_OUT_VECBASE_REG_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_PRO_OUT_VECBASE_REG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x003f_ffff) | ((value as u32) & 0x003f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:21"]
    #[inline(always)]
    pub fn dport_pro_out_vecbase_reg(&self) -> DPORT_PRO_OUT_VECBASE_REG_R {
        DPORT_PRO_OUT_VECBASE_REG_R::new((self.bits & 0x003f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:21"]
    #[inline(always)]
    pub fn dport_pro_out_vecbase_reg(&mut self) -> DPORT_PRO_OUT_VECBASE_REG_W {
        DPORT_PRO_OUT_VECBASE_REG_W { w: self }
    }
}