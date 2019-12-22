#[doc = "Reader of register DPORT_PRO_INTRUSION_CTRL_REG"]
pub type R = crate::R<u32, super::DPORT_PRO_INTRUSION_CTRL_REG>;
#[doc = "Writer for register DPORT_PRO_INTRUSION_CTRL_REG"]
pub type W = crate::W<u32, super::DPORT_PRO_INTRUSION_CTRL_REG>;
#[doc = "Register DPORT_PRO_INTRUSION_CTRL_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::DPORT_PRO_INTRUSION_CTRL_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DPORT_PRO_INTRUSION_RECORD_RESET_N`"]
pub type DPORT_PRO_INTRUSION_RECORD_RESET_N_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPORT_PRO_INTRUSION_RECORD_RESET_N`"]
pub struct DPORT_PRO_INTRUSION_RECORD_RESET_N_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_PRO_INTRUSION_RECORD_RESET_N_W<'a> {
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
    pub fn dport_pro_intrusion_record_reset_n(&self) -> DPORT_PRO_INTRUSION_RECORD_RESET_N_R {
        DPORT_PRO_INTRUSION_RECORD_RESET_N_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dport_pro_intrusion_record_reset_n(&mut self) -> DPORT_PRO_INTRUSION_RECORD_RESET_N_W {
        DPORT_PRO_INTRUSION_RECORD_RESET_N_W { w: self }
    }
}
