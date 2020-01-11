#[doc = "Reader of register HSCH4_CONF0"]
pub type R = crate::R<u32, super::HSCH4_CONF0>;
#[doc = "Writer for register HSCH4_CONF0"]
pub type W = crate::W<u32, super::HSCH4_CONF0>;
#[doc = "Register HSCH4_CONF0 `reset()`'s with value 0"]
impl crate::ResetValue for super::HSCH4_CONF0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LEDC_IDLE_LV_HSCH4`"]
pub type LEDC_IDLE_LV_HSCH4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEDC_IDLE_LV_HSCH4`"]
pub struct LEDC_IDLE_LV_HSCH4_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_IDLE_LV_HSCH4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `LEDC_SIG_OUT_EN_HSCH4`"]
pub type LEDC_SIG_OUT_EN_HSCH4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEDC_SIG_OUT_EN_HSCH4`"]
pub struct LEDC_SIG_OUT_EN_HSCH4_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_SIG_OUT_EN_HSCH4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `LEDC_TIMER_SEL_HSCH4`"]
pub type LEDC_TIMER_SEL_HSCH4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LEDC_TIMER_SEL_HSCH4`"]
pub struct LEDC_TIMER_SEL_HSCH4_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_TIMER_SEL_HSCH4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bit 3 - This bit is used to control the output value when high speed channel4 is off."]
    #[inline(always)]
    pub fn ledc_idle_lv_hsch4(&self) -> LEDC_IDLE_LV_HSCH4_R {
        LEDC_IDLE_LV_HSCH4_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - This is the output enable control bit for high speed channel4"]
    #[inline(always)]
    pub fn ledc_sig_out_en_hsch4(&self) -> LEDC_SIG_OUT_EN_HSCH4_R {
        LEDC_SIG_OUT_EN_HSCH4_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 0:1 - There are four high speed timers the two bits are used to select one of them for high speed channel4. 2'b00: seletc hstimer0. 2'b01: select hstimer1. 2'b10: select hstimer2. 2'b11: select hstimer3."]
    #[inline(always)]
    pub fn ledc_timer_sel_hsch4(&self) -> LEDC_TIMER_SEL_HSCH4_R {
        LEDC_TIMER_SEL_HSCH4_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 3 - This bit is used to control the output value when high speed channel4 is off."]
    #[inline(always)]
    pub fn ledc_idle_lv_hsch4(&mut self) -> LEDC_IDLE_LV_HSCH4_W {
        LEDC_IDLE_LV_HSCH4_W { w: self }
    }
    #[doc = "Bit 2 - This is the output enable control bit for high speed channel4"]
    #[inline(always)]
    pub fn ledc_sig_out_en_hsch4(&mut self) -> LEDC_SIG_OUT_EN_HSCH4_W {
        LEDC_SIG_OUT_EN_HSCH4_W { w: self }
    }
    #[doc = "Bits 0:1 - There are four high speed timers the two bits are used to select one of them for high speed channel4. 2'b00: seletc hstimer0. 2'b01: select hstimer1. 2'b10: select hstimer2. 2'b11: select hstimer3."]
    #[inline(always)]
    pub fn ledc_timer_sel_hsch4(&mut self) -> LEDC_TIMER_SEL_HSCH4_W {
        LEDC_TIMER_SEL_HSCH4_W { w: self }
    }
}