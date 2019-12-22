#[doc = "Reader of register MCPWM_TIMER0_CFG1_REG"]
pub type R = crate::R<u32, super::MCPWM_TIMER0_CFG1_REG>;
#[doc = "Writer for register MCPWM_TIMER0_CFG1_REG"]
pub type W = crate::W<u32, super::MCPWM_TIMER0_CFG1_REG>;
#[doc = "Register MCPWM_TIMER0_CFG1_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::MCPWM_TIMER0_CFG1_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MCPWM_TIMER0_MOD`"]
pub type MCPWM_TIMER0_MOD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MCPWM_TIMER0_MOD`"]
pub struct MCPWM_TIMER0_MOD_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_TIMER0_MOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u32) & 0x03) << 3);
        self.w
    }
}
#[doc = "Reader of field `MCPWM_TIMER0_START`"]
pub type MCPWM_TIMER0_START_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MCPWM_TIMER0_START`"]
pub struct MCPWM_TIMER0_START_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_TIMER0_START_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:4 - PWM timer0 working mode 0: freeze 1: increase mod 2: decrease mod 3: up-down mod"]
    #[inline(always)]
    pub fn mcpwm_timer0_mod(&self) -> MCPWM_TIMER0_MOD_R {
        MCPWM_TIMER0_MOD_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bits 0:2 - PWM timer0 start and stop control. 0: stop @ TEZ 1: stop @ TEP 2: free run 3: start and stop @ next TEZ 4: start and stop @ next TEP. TEP here and below means timer equal period event"]
    #[inline(always)]
    pub fn mcpwm_timer0_start(&self) -> MCPWM_TIMER0_START_R {
        MCPWM_TIMER0_START_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 3:4 - PWM timer0 working mode 0: freeze 1: increase mod 2: decrease mod 3: up-down mod"]
    #[inline(always)]
    pub fn mcpwm_timer0_mod(&mut self) -> MCPWM_TIMER0_MOD_W {
        MCPWM_TIMER0_MOD_W { w: self }
    }
    #[doc = "Bits 0:2 - PWM timer0 start and stop control. 0: stop @ TEZ 1: stop @ TEP 2: free run 3: start and stop @ next TEZ 4: start and stop @ next TEP. TEP here and below means timer equal period event"]
    #[inline(always)]
    pub fn mcpwm_timer0_start(&mut self) -> MCPWM_TIMER0_START_W {
        MCPWM_TIMER0_START_W { w: self }
    }
}
