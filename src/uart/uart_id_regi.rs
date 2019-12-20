#[doc = "Reader of register UART_ID_REG(i)"]
pub type R = crate::R<u32, super::UART_ID_REGI>;
#[doc = "Writer for register UART_ID_REG(i)"]
pub type W = crate::W<u32, super::UART_ID_REGI>;
#[doc = "Register UART_ID_REG(i) `reset()`'s with value 0"]
impl crate::ResetValue for super::UART_ID_REGI {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UART_ID`"]
pub type UART_ID_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `UART_ID`"]
pub struct UART_ID_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_ID_W<'a> {
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
    pub fn uart_id(&self) -> UART_ID_R {
        UART_ID_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn uart_id(&mut self) -> UART_ID_W {
        UART_ID_W { w: self }
    }
}