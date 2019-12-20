#[doc = "Reader of register LEDC_LSTIMER1_CONF_REG"]
pub type R = crate::R<u32, super::LEDC_LSTIMER1_CONF_REG>;
#[doc = "Writer for register LEDC_LSTIMER1_CONF_REG"]
pub type W = crate::W<u32, super::LEDC_LSTIMER1_CONF_REG>;
#[doc = "Register LEDC_LSTIMER1_CONF_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::LEDC_LSTIMER1_CONF_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LEDC_LSTIMER1_PARA_UP`"]
pub type LEDC_LSTIMER1_PARA_UP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEDC_LSTIMER1_PARA_UP`"]
pub struct LEDC_LSTIMER1_PARA_UP_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_LSTIMER1_PARA_UP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `LEDC_TICK_SEL_LSTIMER1`"]
pub type LEDC_TICK_SEL_LSTIMER1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEDC_TICK_SEL_LSTIMER1`"]
pub struct LEDC_TICK_SEL_LSTIMER1_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_TICK_SEL_LSTIMER1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `LEDC_LSTIMER1_RST`"]
pub type LEDC_LSTIMER1_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEDC_LSTIMER1_RST`"]
pub struct LEDC_LSTIMER1_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_LSTIMER1_RST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `LEDC_LSTIMER1_PAUSE`"]
pub type LEDC_LSTIMER1_PAUSE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEDC_LSTIMER1_PAUSE`"]
pub struct LEDC_LSTIMER1_PAUSE_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_LSTIMER1_PAUSE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `LEDC_DIV_NUM_LSTIMER1`"]
pub type LEDC_DIV_NUM_LSTIMER1_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `LEDC_DIV_NUM_LSTIMER1`"]
pub struct LEDC_DIV_NUM_LSTIMER1_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_DIV_NUM_LSTIMER1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0003_ffff << 5)) | (((value as u32) & 0x0003_ffff) << 5);
        self.w
    }
}
#[doc = "Reader of field `LEDC_LSTIMER1_LIM`"]
pub type LEDC_LSTIMER1_LIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LEDC_LSTIMER1_LIM`"]
pub struct LEDC_LSTIMER1_LIM_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_LSTIMER1_LIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bit 26 - Set this bit to update reg_div_num_lstime1 and reg_lstimer1_lim."]
    #[inline(always)]
    pub fn ledc_lstimer1_para_up(&self) -> LEDC_LSTIMER1_PARA_UP_R {
        LEDC_LSTIMER1_PARA_UP_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - This bit is used to choose slow_clk or ref_tick for low speed timer1. 1'b1:slow_clk 0:ref_tick"]
    #[inline(always)]
    pub fn ledc_tick_sel_lstimer1(&self) -> LEDC_TICK_SEL_LSTIMER1_R {
        LEDC_TICK_SEL_LSTIMER1_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - This bit is used to reset low speed timer1 the counter will be 0 after reset."]
    #[inline(always)]
    pub fn ledc_lstimer1_rst(&self) -> LEDC_LSTIMER1_RST_R {
        LEDC_LSTIMER1_RST_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - This bit is used to pause the counter in low speed timer1."]
    #[inline(always)]
    pub fn ledc_lstimer1_pause(&self) -> LEDC_LSTIMER1_PAUSE_R {
        LEDC_LSTIMER1_PAUSE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 5:22 - This register is used to configure parameter for divider in low speed timer1 the least significant eight bits represent the decimal part."]
    #[inline(always)]
    pub fn ledc_div_num_lstimer1(&self) -> LEDC_DIV_NUM_LSTIMER1_R {
        LEDC_DIV_NUM_LSTIMER1_R::new(((self.bits >> 5) & 0x0003_ffff) as u32)
    }
    #[doc = "Bits 0:4 - This register controls the range of the counter in low speed timer1. the counter range is \\[0 2**reg_lstimer1_lim\\] the max bit width for counter is 20."]
    #[inline(always)]
    pub fn ledc_lstimer1_lim(&self) -> LEDC_LSTIMER1_LIM_R {
        LEDC_LSTIMER1_LIM_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 26 - Set this bit to update reg_div_num_lstime1 and reg_lstimer1_lim."]
    #[inline(always)]
    pub fn ledc_lstimer1_para_up(&mut self) -> LEDC_LSTIMER1_PARA_UP_W {
        LEDC_LSTIMER1_PARA_UP_W { w: self }
    }
    #[doc = "Bit 25 - This bit is used to choose slow_clk or ref_tick for low speed timer1. 1'b1:slow_clk 0:ref_tick"]
    #[inline(always)]
    pub fn ledc_tick_sel_lstimer1(&mut self) -> LEDC_TICK_SEL_LSTIMER1_W {
        LEDC_TICK_SEL_LSTIMER1_W { w: self }
    }
    #[doc = "Bit 24 - This bit is used to reset low speed timer1 the counter will be 0 after reset."]
    #[inline(always)]
    pub fn ledc_lstimer1_rst(&mut self) -> LEDC_LSTIMER1_RST_W {
        LEDC_LSTIMER1_RST_W { w: self }
    }
    #[doc = "Bit 23 - This bit is used to pause the counter in low speed timer1."]
    #[inline(always)]
    pub fn ledc_lstimer1_pause(&mut self) -> LEDC_LSTIMER1_PAUSE_W {
        LEDC_LSTIMER1_PAUSE_W { w: self }
    }
    #[doc = "Bits 5:22 - This register is used to configure parameter for divider in low speed timer1 the least significant eight bits represent the decimal part."]
    #[inline(always)]
    pub fn ledc_div_num_lstimer1(&mut self) -> LEDC_DIV_NUM_LSTIMER1_W {
        LEDC_DIV_NUM_LSTIMER1_W { w: self }
    }
    #[doc = "Bits 0:4 - This register controls the range of the counter in low speed timer1. the counter range is \\[0 2**reg_lstimer1_lim\\] the max bit width for counter is 20."]
    #[inline(always)]
    pub fn ledc_lstimer1_lim(&mut self) -> LEDC_LSTIMER1_LIM_W {
        LEDC_LSTIMER1_LIM_W { w: self }
    }
}