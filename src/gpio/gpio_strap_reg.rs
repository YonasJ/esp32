#[doc = "Reader of register GPIO_STRAP_REG"]
pub type R = crate::R<u32, super::GPIO_STRAP_REG>;
#[doc = "Writer for register GPIO_STRAP_REG"]
pub type W = crate::W<u32, super::GPIO_STRAP_REG>;
#[doc = "Register GPIO_STRAP_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::GPIO_STRAP_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GPIO_STRAPPING`"]
pub type GPIO_STRAPPING_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `GPIO_STRAPPING`"]
pub struct GPIO_STRAPPING_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_STRAPPING_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - {10'b0, MTDI, GPIO0, GPIO2, GPIO4, MTDO, GPIO5}"]
    #[inline(always)]
    pub fn gpio_strapping(&self) -> GPIO_STRAPPING_R {
        GPIO_STRAPPING_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - {10'b0, MTDI, GPIO0, GPIO2, GPIO4, MTDO, GPIO5}"]
    #[inline(always)]
    pub fn gpio_strapping(&mut self) -> GPIO_STRAPPING_W {
        GPIO_STRAPPING_W { w: self }
    }
}