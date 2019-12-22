#[doc = "Reader of register GPIO_OUT_W1TS_REG"]
pub type R = crate::R<u32, super::GPIO_OUT_W1TS_REG>;
#[doc = "Writer for register GPIO_OUT_W1TS_REG"]
pub type W = crate::W<u32, super::GPIO_OUT_W1TS_REG>;
#[doc = "Register GPIO_OUT_W1TS_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::GPIO_OUT_W1TS_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GPIO_OUT_DATA_W1TS`"]
pub type GPIO_OUT_DATA_W1TS_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `GPIO_OUT_DATA_W1TS`"]
pub struct GPIO_OUT_DATA_W1TS_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_OUT_DATA_W1TS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - GPIO0~31 output value write 1 to set"]
    #[inline(always)]
    pub fn gpio_out_data_w1ts(&self) -> GPIO_OUT_DATA_W1TS_R {
        GPIO_OUT_DATA_W1TS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - GPIO0~31 output value write 1 to set"]
    #[inline(always)]
    pub fn gpio_out_data_w1ts(&mut self) -> GPIO_OUT_DATA_W1TS_W {
        GPIO_OUT_DATA_W1TS_W { w: self }
    }
}
