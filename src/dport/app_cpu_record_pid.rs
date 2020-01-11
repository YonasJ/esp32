#[doc = "Reader of register APP_CPU_RECORD_PID"]
pub type R = crate::R<u32, super::APP_CPU_RECORD_PID>;
#[doc = "Writer for register APP_CPU_RECORD_PID"]
pub type W = crate::W<u32, super::APP_CPU_RECORD_PID>;
#[doc = "Register APP_CPU_RECORD_PID `reset()`'s with value 0"]
impl crate::ResetValue for super::APP_CPU_RECORD_PID {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DPORT_RECORD_APP_PID`"]
pub type DPORT_RECORD_APP_PID_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DPORT_RECORD_APP_PID`"]
pub struct DPORT_RECORD_APP_PID_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_RECORD_APP_PID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn dport_record_app_pid(&self) -> DPORT_RECORD_APP_PID_R {
        DPORT_RECORD_APP_PID_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn dport_record_app_pid(&mut self) -> DPORT_RECORD_APP_PID_W {
        DPORT_RECORD_APP_PID_W { w: self }
    }
}