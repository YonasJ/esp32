#[doc = "Reader of register DPORT_CPU_INTR_FROM_CPU_0_REG"]
pub type R = crate::R<u32, super::DPORT_CPU_INTR_FROM_CPU_0_REG>;
#[doc = "Writer for register DPORT_CPU_INTR_FROM_CPU_0_REG"]
pub type W = crate::W<u32, super::DPORT_CPU_INTR_FROM_CPU_0_REG>;
#[doc = "Register DPORT_CPU_INTR_FROM_CPU_0_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::DPORT_CPU_INTR_FROM_CPU_0_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DPORT_CPU_INTR_FROM_CPU_0`"]
pub type DPORT_CPU_INTR_FROM_CPU_0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPORT_CPU_INTR_FROM_CPU_0`"]
pub struct DPORT_CPU_INTR_FROM_CPU_0_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_CPU_INTR_FROM_CPU_0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dport_cpu_intr_from_cpu_0(&self) -> DPORT_CPU_INTR_FROM_CPU_0_R {
        DPORT_CPU_INTR_FROM_CPU_0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dport_cpu_intr_from_cpu_0(&mut self) -> DPORT_CPU_INTR_FROM_CPU_0_W {
        DPORT_CPU_INTR_FROM_CPU_0_W { w: self }
    }
}