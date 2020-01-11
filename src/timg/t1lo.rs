#[doc = "Reader of register T1LO"]
pub type R = crate::R<u32, super::T1LO>;
#[doc = "Writer for register T1LO"]
pub type W = crate::W<u32, super::T1LO>;
#[doc = "Register T1LO `reset()`'s with value 0"]
impl crate::ResetValue for super::T1LO {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TIMG_T1_LO`"]
pub type TIMG_T1_LO_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `TIMG_T1_LO`"]
pub struct TIMG_T1_LO_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMG_T1_LO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Register to store timer 1 time-base counter current value lower 32 bits."]
    #[inline(always)]
    pub fn timg_t1_lo(&self) -> TIMG_T1_LO_R {
        TIMG_T1_LO_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Register to store timer 1 time-base counter current value lower 32 bits."]
    #[inline(always)]
    pub fn timg_t1_lo(&mut self) -> TIMG_T1_LO_W {
        TIMG_T1_LO_W { w: self }
    }
}