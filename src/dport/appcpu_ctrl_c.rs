#[doc = "Reader of register APPCPU_CTRL_C"]
pub type R = crate::R<u32, super::APPCPU_CTRL_C>;
#[doc = "Writer for register APPCPU_CTRL_C"]
pub type W = crate::W<u32, super::APPCPU_CTRL_C>;
#[doc = "Register APPCPU_CTRL_C `reset()`'s with value 0"]
impl crate::ResetValue for super::APPCPU_CTRL_C {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DPORT_APPCPU_RUNSTALL`"]
pub type DPORT_APPCPU_RUNSTALL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPORT_APPCPU_RUNSTALL`"]
pub struct DPORT_APPCPU_RUNSTALL_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_APPCPU_RUNSTALL_W<'a> {
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
    pub fn dport_appcpu_runstall(&self) -> DPORT_APPCPU_RUNSTALL_R {
        DPORT_APPCPU_RUNSTALL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dport_appcpu_runstall(&mut self) -> DPORT_APPCPU_RUNSTALL_W {
        DPORT_APPCPU_RUNSTALL_W { w: self }
    }
}