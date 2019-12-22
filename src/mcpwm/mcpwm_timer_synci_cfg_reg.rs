#[doc = "Reader of register MCPWM_TIMER_SYNCI_CFG_REG"]
pub type R = crate::R<u32, super::MCPWM_TIMER_SYNCI_CFG_REG>;
#[doc = "Writer for register MCPWM_TIMER_SYNCI_CFG_REG"]
pub type W = crate::W<u32, super::MCPWM_TIMER_SYNCI_CFG_REG>;
#[doc = "Register MCPWM_TIMER_SYNCI_CFG_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::MCPWM_TIMER_SYNCI_CFG_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MCPWM_EXTERNAL_SYNCI2_INVERT`"]
pub type MCPWM_EXTERNAL_SYNCI2_INVERT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCPWM_EXTERNAL_SYNCI2_INVERT`"]
pub struct MCPWM_EXTERNAL_SYNCI2_INVERT_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_EXTERNAL_SYNCI2_INVERT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `MCPWM_EXTERNAL_SYNCI1_INVERT`"]
pub type MCPWM_EXTERNAL_SYNCI1_INVERT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCPWM_EXTERNAL_SYNCI1_INVERT`"]
pub struct MCPWM_EXTERNAL_SYNCI1_INVERT_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_EXTERNAL_SYNCI1_INVERT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `MCPWM_EXTERNAL_SYNCI0_INVERT`"]
pub type MCPWM_EXTERNAL_SYNCI0_INVERT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCPWM_EXTERNAL_SYNCI0_INVERT`"]
pub struct MCPWM_EXTERNAL_SYNCI0_INVERT_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_EXTERNAL_SYNCI0_INVERT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `MCPWM_TIMER2_SYNCISEL`"]
pub type MCPWM_TIMER2_SYNCISEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MCPWM_TIMER2_SYNCISEL`"]
pub struct MCPWM_TIMER2_SYNCISEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_TIMER2_SYNCISEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 6)) | (((value as u32) & 0x07) << 6);
        self.w
    }
}
#[doc = "Reader of field `MCPWM_TIMER1_SYNCISEL`"]
pub type MCPWM_TIMER1_SYNCISEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MCPWM_TIMER1_SYNCISEL`"]
pub struct MCPWM_TIMER1_SYNCISEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_TIMER1_SYNCISEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | (((value as u32) & 0x07) << 3);
        self.w
    }
}
#[doc = "Reader of field `MCPWM_TIMER0_SYNCISEL`"]
pub type MCPWM_TIMER0_SYNCISEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MCPWM_TIMER0_SYNCISEL`"]
pub struct MCPWM_TIMER0_SYNCISEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_TIMER0_SYNCISEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bit 11 - Onvert SYNC2 from GPIO matrix"]
    #[inline(always)]
    pub fn mcpwm_external_synci2_invert(&self) -> MCPWM_EXTERNAL_SYNCI2_INVERT_R {
        MCPWM_EXTERNAL_SYNCI2_INVERT_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Invert SYNC1 from GPIO matrix"]
    #[inline(always)]
    pub fn mcpwm_external_synci1_invert(&self) -> MCPWM_EXTERNAL_SYNCI1_INVERT_R {
        MCPWM_EXTERNAL_SYNCI1_INVERT_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Invert SYNC0 from GPIO matrix"]
    #[inline(always)]
    pub fn mcpwm_external_synci0_invert(&self) -> MCPWM_EXTERNAL_SYNCI0_INVERT_R {
        MCPWM_EXTERNAL_SYNCI0_INVERT_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 6:8 - Select sync input for PWM timer2 1: PWM timer0 synco 2: PWM timer1 synco 3: PWM timer2 synco 4: SYNC0 from GPIO matrix 5: SYNC1 from GPIO matrix 6: SYNC2 from GPIO matrix other values: no sync input selected"]
    #[inline(always)]
    pub fn mcpwm_timer2_syncisel(&self) -> MCPWM_TIMER2_SYNCISEL_R {
        MCPWM_TIMER2_SYNCISEL_R::new(((self.bits >> 6) & 0x07) as u8)
    }
    #[doc = "Bits 3:5 - Select sync input for PWM timer1 1: PWM timer0 synco 2: PWM timer1 synco 3: PWM timer2 synco 4: SYNC0 from GPIO matrix 5: SYNC1 from GPIO matrix 6: SYNC2 from GPIO matrix other values: no sync input selected"]
    #[inline(always)]
    pub fn mcpwm_timer1_syncisel(&self) -> MCPWM_TIMER1_SYNCISEL_R {
        MCPWM_TIMER1_SYNCISEL_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bits 0:2 - Select sync input for PWM timer0 1: PWM timer0 synco 2: PWM timer1 synco 3: PWM timer2 synco 4: SYNC0 from GPIO matrix 5: SYNC1 from GPIO matrix 6: SYNC2 from GPIO matrix other values: no sync input selected"]
    #[inline(always)]
    pub fn mcpwm_timer0_syncisel(&self) -> MCPWM_TIMER0_SYNCISEL_R {
        MCPWM_TIMER0_SYNCISEL_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 11 - Onvert SYNC2 from GPIO matrix"]
    #[inline(always)]
    pub fn mcpwm_external_synci2_invert(&mut self) -> MCPWM_EXTERNAL_SYNCI2_INVERT_W {
        MCPWM_EXTERNAL_SYNCI2_INVERT_W { w: self }
    }
    #[doc = "Bit 10 - Invert SYNC1 from GPIO matrix"]
    #[inline(always)]
    pub fn mcpwm_external_synci1_invert(&mut self) -> MCPWM_EXTERNAL_SYNCI1_INVERT_W {
        MCPWM_EXTERNAL_SYNCI1_INVERT_W { w: self }
    }
    #[doc = "Bit 9 - Invert SYNC0 from GPIO matrix"]
    #[inline(always)]
    pub fn mcpwm_external_synci0_invert(&mut self) -> MCPWM_EXTERNAL_SYNCI0_INVERT_W {
        MCPWM_EXTERNAL_SYNCI0_INVERT_W { w: self }
    }
    #[doc = "Bits 6:8 - Select sync input for PWM timer2 1: PWM timer0 synco 2: PWM timer1 synco 3: PWM timer2 synco 4: SYNC0 from GPIO matrix 5: SYNC1 from GPIO matrix 6: SYNC2 from GPIO matrix other values: no sync input selected"]
    #[inline(always)]
    pub fn mcpwm_timer2_syncisel(&mut self) -> MCPWM_TIMER2_SYNCISEL_W {
        MCPWM_TIMER2_SYNCISEL_W { w: self }
    }
    #[doc = "Bits 3:5 - Select sync input for PWM timer1 1: PWM timer0 synco 2: PWM timer1 synco 3: PWM timer2 synco 4: SYNC0 from GPIO matrix 5: SYNC1 from GPIO matrix 6: SYNC2 from GPIO matrix other values: no sync input selected"]
    #[inline(always)]
    pub fn mcpwm_timer1_syncisel(&mut self) -> MCPWM_TIMER1_SYNCISEL_W {
        MCPWM_TIMER1_SYNCISEL_W { w: self }
    }
    #[doc = "Bits 0:2 - Select sync input for PWM timer0 1: PWM timer0 synco 2: PWM timer1 synco 3: PWM timer2 synco 4: SYNC0 from GPIO matrix 5: SYNC1 from GPIO matrix 6: SYNC2 from GPIO matrix other values: no sync input selected"]
    #[inline(always)]
    pub fn mcpwm_timer0_syncisel(&mut self) -> MCPWM_TIMER0_SYNCISEL_W {
        MCPWM_TIMER0_SYNCISEL_W { w: self }
    }
}
