#[doc = "Reader of register DPORT_CACHE_IA_INT_EN_REG"]
pub type R = crate::R<u32, super::DPORT_CACHE_IA_INT_EN_REG>;
#[doc = "Writer for register DPORT_CACHE_IA_INT_EN_REG"]
pub type W = crate::W<u32, super::DPORT_CACHE_IA_INT_EN_REG>;
#[doc = "Register DPORT_CACHE_IA_INT_EN_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::DPORT_CACHE_IA_INT_EN_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DPORT_CACHE_IA_INT_EN`"]
pub type DPORT_CACHE_IA_INT_EN_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DPORT_CACHE_IA_INT_EN`"]
pub struct DPORT_CACHE_IA_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_CACHE_IA_INT_EN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff_ffff) | ((value as u32) & 0x0fff_ffff);
        self.w
    }
}
#[doc = "Reader of field `DPORT_CACHE_IA_INT_PRO_OPPOSITE`"]
pub type DPORT_CACHE_IA_INT_PRO_OPPOSITE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPORT_CACHE_IA_INT_PRO_OPPOSITE`"]
pub struct DPORT_CACHE_IA_INT_PRO_OPPOSITE_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_CACHE_IA_INT_PRO_OPPOSITE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `DPORT_CACHE_IA_INT_PRO_DRAM1`"]
pub type DPORT_CACHE_IA_INT_PRO_DRAM1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPORT_CACHE_IA_INT_PRO_DRAM1`"]
pub struct DPORT_CACHE_IA_INT_PRO_DRAM1_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_CACHE_IA_INT_PRO_DRAM1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `DPORT_CACHE_IA_INT_PRO_IROM0`"]
pub type DPORT_CACHE_IA_INT_PRO_IROM0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPORT_CACHE_IA_INT_PRO_IROM0`"]
pub struct DPORT_CACHE_IA_INT_PRO_IROM0_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_CACHE_IA_INT_PRO_IROM0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `DPORT_CACHE_IA_INT_PRO_IRAM1`"]
pub type DPORT_CACHE_IA_INT_PRO_IRAM1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPORT_CACHE_IA_INT_PRO_IRAM1`"]
pub struct DPORT_CACHE_IA_INT_PRO_IRAM1_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_CACHE_IA_INT_PRO_IRAM1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `DPORT_CACHE_IA_INT_PRO_IRAM0`"]
pub type DPORT_CACHE_IA_INT_PRO_IRAM0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPORT_CACHE_IA_INT_PRO_IRAM0`"]
pub struct DPORT_CACHE_IA_INT_PRO_IRAM0_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_CACHE_IA_INT_PRO_IRAM0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `DPORT_CACHE_IA_INT_PRO_DROM0`"]
pub type DPORT_CACHE_IA_INT_PRO_DROM0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPORT_CACHE_IA_INT_PRO_DROM0`"]
pub struct DPORT_CACHE_IA_INT_PRO_DROM0_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_CACHE_IA_INT_PRO_DROM0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `DPORT_CACHE_IA_INT_APP_OPPOSITE`"]
pub type DPORT_CACHE_IA_INT_APP_OPPOSITE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPORT_CACHE_IA_INT_APP_OPPOSITE`"]
pub struct DPORT_CACHE_IA_INT_APP_OPPOSITE_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_CACHE_IA_INT_APP_OPPOSITE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `DPORT_CACHE_IA_INT_APP_IROM0`"]
pub type DPORT_CACHE_IA_INT_APP_IROM0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPORT_CACHE_IA_INT_APP_IROM0`"]
pub struct DPORT_CACHE_IA_INT_APP_IROM0_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_CACHE_IA_INT_APP_IROM0_W<'a> {
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
#[doc = "Reader of field `DPORT_CACHE_IA_INT_APP_IRAM1`"]
pub type DPORT_CACHE_IA_INT_APP_IRAM1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPORT_CACHE_IA_INT_APP_IRAM1`"]
pub struct DPORT_CACHE_IA_INT_APP_IRAM1_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_CACHE_IA_INT_APP_IRAM1_W<'a> {
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
#[doc = "Reader of field `DPORT_CACHE_IA_INT_APP_IRAM0`"]
pub type DPORT_CACHE_IA_INT_APP_IRAM0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPORT_CACHE_IA_INT_APP_IRAM0`"]
pub struct DPORT_CACHE_IA_INT_APP_IRAM0_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_CACHE_IA_INT_APP_IRAM0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `DPORT_CACHE_IA_INT_APP_DROM0`"]
pub type DPORT_CACHE_IA_INT_APP_DROM0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPORT_CACHE_IA_INT_APP_DROM0`"]
pub struct DPORT_CACHE_IA_INT_APP_DROM0_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_CACHE_IA_INT_APP_DROM0_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:27 - Interrupt enable bits for various invalid cache access reasons"]
    #[inline(always)]
    pub fn dport_cache_ia_int_en(&self) -> DPORT_CACHE_IA_INT_EN_R {
        DPORT_CACHE_IA_INT_EN_R::new((self.bits & 0x0fff_ffff) as u32)
    }
    #[doc = "Bit 19 - PRO CPU invalid access to APP CPU cache when cache disabled"]
    #[inline(always)]
    pub fn dport_cache_ia_int_pro_opposite(&self) -> DPORT_CACHE_IA_INT_PRO_OPPOSITE_R {
        DPORT_CACHE_IA_INT_PRO_OPPOSITE_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - PRO CPU invalid access to DRAM1 when cache is disabled"]
    #[inline(always)]
    pub fn dport_cache_ia_int_pro_dram1(&self) -> DPORT_CACHE_IA_INT_PRO_DRAM1_R {
        DPORT_CACHE_IA_INT_PRO_DRAM1_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - PRO CPU invalid access to IROM0 when cache is disabled"]
    #[inline(always)]
    pub fn dport_cache_ia_int_pro_irom0(&self) -> DPORT_CACHE_IA_INT_PRO_IROM0_R {
        DPORT_CACHE_IA_INT_PRO_IROM0_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - PRO CPU invalid access to IRAM1 when cache is disabled"]
    #[inline(always)]
    pub fn dport_cache_ia_int_pro_iram1(&self) -> DPORT_CACHE_IA_INT_PRO_IRAM1_R {
        DPORT_CACHE_IA_INT_PRO_IRAM1_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - PRO CPU invalid access to IRAM0 when cache is disabled"]
    #[inline(always)]
    pub fn dport_cache_ia_int_pro_iram0(&self) -> DPORT_CACHE_IA_INT_PRO_IRAM0_R {
        DPORT_CACHE_IA_INT_PRO_IRAM0_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - PRO CPU invalid access to DROM0 when cache is disabled"]
    #[inline(always)]
    pub fn dport_cache_ia_int_pro_drom0(&self) -> DPORT_CACHE_IA_INT_PRO_DROM0_R {
        DPORT_CACHE_IA_INT_PRO_DROM0_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 5 - APP CPU invalid access to APP CPU cache when cache disabled"]
    #[inline(always)]
    pub fn dport_cache_ia_int_app_opposite(&self) -> DPORT_CACHE_IA_INT_APP_OPPOSITE_R {
        DPORT_CACHE_IA_INT_APP_OPPOSITE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 3 - APP CPU invalid access to IROM0 when cache is disabled"]
    #[inline(always)]
    pub fn dport_cache_ia_int_app_irom0(&self) -> DPORT_CACHE_IA_INT_APP_IROM0_R {
        DPORT_CACHE_IA_INT_APP_IROM0_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - APP CPU invalid access to IRAM1 when cache is disabled"]
    #[inline(always)]
    pub fn dport_cache_ia_int_app_iram1(&self) -> DPORT_CACHE_IA_INT_APP_IRAM1_R {
        DPORT_CACHE_IA_INT_APP_IRAM1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - APP CPU invalid access to IRAM0 when cache is disabled"]
    #[inline(always)]
    pub fn dport_cache_ia_int_app_iram0(&self) -> DPORT_CACHE_IA_INT_APP_IRAM0_R {
        DPORT_CACHE_IA_INT_APP_IRAM0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - APP CPU invalid access to DROM0 when cache is disabled"]
    #[inline(always)]
    pub fn dport_cache_ia_int_app_drom0(&self) -> DPORT_CACHE_IA_INT_APP_DROM0_R {
        DPORT_CACHE_IA_INT_APP_DROM0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:27 - Interrupt enable bits for various invalid cache access reasons"]
    #[inline(always)]
    pub fn dport_cache_ia_int_en(&mut self) -> DPORT_CACHE_IA_INT_EN_W {
        DPORT_CACHE_IA_INT_EN_W { w: self }
    }
    #[doc = "Bit 19 - PRO CPU invalid access to APP CPU cache when cache disabled"]
    #[inline(always)]
    pub fn dport_cache_ia_int_pro_opposite(&mut self) -> DPORT_CACHE_IA_INT_PRO_OPPOSITE_W {
        DPORT_CACHE_IA_INT_PRO_OPPOSITE_W { w: self }
    }
    #[doc = "Bit 18 - PRO CPU invalid access to DRAM1 when cache is disabled"]
    #[inline(always)]
    pub fn dport_cache_ia_int_pro_dram1(&mut self) -> DPORT_CACHE_IA_INT_PRO_DRAM1_W {
        DPORT_CACHE_IA_INT_PRO_DRAM1_W { w: self }
    }
    #[doc = "Bit 17 - PRO CPU invalid access to IROM0 when cache is disabled"]
    #[inline(always)]
    pub fn dport_cache_ia_int_pro_irom0(&mut self) -> DPORT_CACHE_IA_INT_PRO_IROM0_W {
        DPORT_CACHE_IA_INT_PRO_IROM0_W { w: self }
    }
    #[doc = "Bit 16 - PRO CPU invalid access to IRAM1 when cache is disabled"]
    #[inline(always)]
    pub fn dport_cache_ia_int_pro_iram1(&mut self) -> DPORT_CACHE_IA_INT_PRO_IRAM1_W {
        DPORT_CACHE_IA_INT_PRO_IRAM1_W { w: self }
    }
    #[doc = "Bit 15 - PRO CPU invalid access to IRAM0 when cache is disabled"]
    #[inline(always)]
    pub fn dport_cache_ia_int_pro_iram0(&mut self) -> DPORT_CACHE_IA_INT_PRO_IRAM0_W {
        DPORT_CACHE_IA_INT_PRO_IRAM0_W { w: self }
    }
    #[doc = "Bit 14 - PRO CPU invalid access to DROM0 when cache is disabled"]
    #[inline(always)]
    pub fn dport_cache_ia_int_pro_drom0(&mut self) -> DPORT_CACHE_IA_INT_PRO_DROM0_W {
        DPORT_CACHE_IA_INT_PRO_DROM0_W { w: self }
    }
    #[doc = "Bit 5 - APP CPU invalid access to APP CPU cache when cache disabled"]
    #[inline(always)]
    pub fn dport_cache_ia_int_app_opposite(&mut self) -> DPORT_CACHE_IA_INT_APP_OPPOSITE_W {
        DPORT_CACHE_IA_INT_APP_OPPOSITE_W { w: self }
    }
    #[doc = "Bit 3 - APP CPU invalid access to IROM0 when cache is disabled"]
    #[inline(always)]
    pub fn dport_cache_ia_int_app_irom0(&mut self) -> DPORT_CACHE_IA_INT_APP_IROM0_W {
        DPORT_CACHE_IA_INT_APP_IROM0_W { w: self }
    }
    #[doc = "Bit 2 - APP CPU invalid access to IRAM1 when cache is disabled"]
    #[inline(always)]
    pub fn dport_cache_ia_int_app_iram1(&mut self) -> DPORT_CACHE_IA_INT_APP_IRAM1_W {
        DPORT_CACHE_IA_INT_APP_IRAM1_W { w: self }
    }
    #[doc = "Bit 1 - APP CPU invalid access to IRAM0 when cache is disabled"]
    #[inline(always)]
    pub fn dport_cache_ia_int_app_iram0(&mut self) -> DPORT_CACHE_IA_INT_APP_IRAM0_W {
        DPORT_CACHE_IA_INT_APP_IRAM0_W { w: self }
    }
    #[doc = "Bit 0 - APP CPU invalid access to DROM0 when cache is disabled"]
    #[inline(always)]
    pub fn dport_cache_ia_int_app_drom0(&mut self) -> DPORT_CACHE_IA_INT_APP_DROM0_W {
        DPORT_CACHE_IA_INT_APP_DROM0_W { w: self }
    }
}
