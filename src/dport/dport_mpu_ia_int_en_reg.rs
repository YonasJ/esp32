#[doc = "Reader of register DPORT_MPU_IA_INT_EN_REG"]
pub type R = crate::R<u32, super::DPORT_MPU_IA_INT_EN_REG>;
#[doc = "Writer for register DPORT_MPU_IA_INT_EN_REG"]
pub type W = crate::W<u32, super::DPORT_MPU_IA_INT_EN_REG>;
#[doc = "Register DPORT_MPU_IA_INT_EN_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::DPORT_MPU_IA_INT_EN_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DPORT_MPU_IA_INT_EN`"]
pub type DPORT_MPU_IA_INT_EN_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DPORT_MPU_IA_INT_EN`"]
pub struct DPORT_MPU_IA_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_MPU_IA_INT_EN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0001_ffff) | ((value as u32) & 0x0001_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:16"]
    #[inline(always)]
    pub fn dport_mpu_ia_int_en(&self) -> DPORT_MPU_IA_INT_EN_R {
        DPORT_MPU_IA_INT_EN_R::new((self.bits & 0x0001_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:16"]
    #[inline(always)]
    pub fn dport_mpu_ia_int_en(&mut self) -> DPORT_MPU_IA_INT_EN_W {
        DPORT_MPU_IA_INT_EN_W { w: self }
    }
}
