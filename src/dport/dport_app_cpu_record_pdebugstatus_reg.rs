#[doc = "Reader of register DPORT_APP_CPU_RECORD_PDEBUGSTATUS_REG"]
pub type R = crate::R<u32, super::DPORT_APP_CPU_RECORD_PDEBUGSTATUS_REG>;
#[doc = "Writer for register DPORT_APP_CPU_RECORD_PDEBUGSTATUS_REG"]
pub type W = crate::W<u32, super::DPORT_APP_CPU_RECORD_PDEBUGSTATUS_REG>;
#[doc = "Register DPORT_APP_CPU_RECORD_PDEBUGSTATUS_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::DPORT_APP_CPU_RECORD_PDEBUGSTATUS_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DPORT_RECORD_APP_PDEBUGSTATUS`"]
pub type DPORT_RECORD_APP_PDEBUGSTATUS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DPORT_RECORD_APP_PDEBUGSTATUS`"]
pub struct DPORT_RECORD_APP_PDEBUGSTATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_RECORD_APP_PDEBUGSTATUS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn dport_record_app_pdebugstatus(&self) -> DPORT_RECORD_APP_PDEBUGSTATUS_R {
        DPORT_RECORD_APP_PDEBUGSTATUS_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn dport_record_app_pdebugstatus(&mut self) -> DPORT_RECORD_APP_PDEBUGSTATUS_W {
        DPORT_RECORD_APP_PDEBUGSTATUS_W { w: self }
    }
}