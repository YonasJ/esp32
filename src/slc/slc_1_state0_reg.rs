#[doc = "Reader of register SLC_1_STATE0_REG"]
pub type R = crate::R<u32, super::SLC_1_STATE0_REG>;
#[doc = "Writer for register SLC_1_STATE0_REG"]
pub type W = crate::W<u32, super::SLC_1_STATE0_REG>;
#[doc = "Register SLC_1_STATE0_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::SLC_1_STATE0_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SLC_SLC1_STATE0`"]
pub type SLC_SLC1_STATE0_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SLC_SLC1_STATE0`"]
pub struct SLC_SLC1_STATE0_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC1_STATE0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn slc_slc1_state0(&self) -> SLC_SLC1_STATE0_R {
        SLC_SLC1_STATE0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn slc_slc1_state0(&mut self) -> SLC_SLC1_STATE0_W {
        SLC_SLC1_STATE0_W { w: self }
    }
}