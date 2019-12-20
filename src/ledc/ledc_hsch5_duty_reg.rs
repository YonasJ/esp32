#[doc = "Reader of register LEDC_HSCH5_DUTY_REG"]
pub type R = crate::R<u32, super::LEDC_HSCH5_DUTY_REG>;
#[doc = "Writer for register LEDC_HSCH5_DUTY_REG"]
pub type W = crate::W<u32, super::LEDC_HSCH5_DUTY_REG>;
#[doc = "Register LEDC_HSCH5_DUTY_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::LEDC_HSCH5_DUTY_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LEDC_DUTY_HSCH5`"]
pub type LEDC_DUTY_HSCH5_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `LEDC_DUTY_HSCH5`"]
pub struct LEDC_DUTY_HSCH5_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_DUTY_HSCH5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff_ffff) | ((value as u32) & 0x01ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:24 - The register is used to control output duty. When hstimerx(x=\\[0 3\\]) choosed by high speed channel5 has reached reg_lpoint_hsch5 the output signal changes to low. reg_lpoint_hsch5=(reg_hpoint_hsch5\\[19:0\\]+reg_duty_hsch5\\[24:4\\]) (1) reg_lpoint_hsch5=(reg_hpoint_hsch5\\[19:0\\]+reg_duty_hsch5\\[24:4\\] +1) (2) The least four bits in this register represent the decimal part and determines when to choose (1) or (2)"]
    #[inline(always)]
    pub fn ledc_duty_hsch5(&self) -> LEDC_DUTY_HSCH5_R {
        LEDC_DUTY_HSCH5_R::new((self.bits & 0x01ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:24 - The register is used to control output duty. When hstimerx(x=\\[0 3\\]) choosed by high speed channel5 has reached reg_lpoint_hsch5 the output signal changes to low. reg_lpoint_hsch5=(reg_hpoint_hsch5\\[19:0\\]+reg_duty_hsch5\\[24:4\\]) (1) reg_lpoint_hsch5=(reg_hpoint_hsch5\\[19:0\\]+reg_duty_hsch5\\[24:4\\] +1) (2) The least four bits in this register represent the decimal part and determines when to choose (1) or (2)"]
    #[inline(always)]
    pub fn ledc_duty_hsch5(&mut self) -> LEDC_DUTY_HSCH5_W {
        LEDC_DUTY_HSCH5_W { w: self }
    }
}