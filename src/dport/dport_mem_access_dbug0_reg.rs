#[doc = "Reader of register DPORT_MEM_ACCESS_DBUG0_REG"]
pub type R = crate::R<u32, super::DPORT_MEM_ACCESS_DBUG0_REG>;
#[doc = "Writer for register DPORT_MEM_ACCESS_DBUG0_REG"]
pub type W = crate::W<u32, super::DPORT_MEM_ACCESS_DBUG0_REG>;
#[doc = "Register DPORT_MEM_ACCESS_DBUG0_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::DPORT_MEM_ACCESS_DBUG0_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DPORT_INTERNAL_SRAM_MMU_MULTI_HIT`"]
pub type DPORT_INTERNAL_SRAM_MMU_MULTI_HIT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DPORT_INTERNAL_SRAM_MMU_MULTI_HIT`"]
pub struct DPORT_INTERNAL_SRAM_MMU_MULTI_HIT_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_INTERNAL_SRAM_MMU_MULTI_HIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 26)) | (((value as u32) & 0x0f) << 26);
        self.w
    }
}
#[doc = "Reader of field `DPORT_INTERNAL_SRAM_IA`"]
pub type DPORT_INTERNAL_SRAM_IA_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DPORT_INTERNAL_SRAM_IA`"]
pub struct DPORT_INTERNAL_SRAM_IA_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_INTERNAL_SRAM_IA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 14)) | (((value as u32) & 0x0fff) << 14);
        self.w
    }
}
#[doc = "Reader of field `DPORT_INTERNAL_SRAM_MMU_AD`"]
pub type DPORT_INTERNAL_SRAM_MMU_AD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DPORT_INTERNAL_SRAM_MMU_AD`"]
pub struct DPORT_INTERNAL_SRAM_MMU_AD_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_INTERNAL_SRAM_MMU_AD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 10)) | (((value as u32) & 0x0f) << 10);
        self.w
    }
}
#[doc = "Reader of field `DPORT_SHARE_ROM_IA`"]
pub type DPORT_SHARE_ROM_IA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DPORT_SHARE_ROM_IA`"]
pub struct DPORT_SHARE_ROM_IA_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_SHARE_ROM_IA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 6)) | (((value as u32) & 0x0f) << 6);
        self.w
    }
}
#[doc = "Reader of field `DPORT_SHARE_ROM_MPU_AD`"]
pub type DPORT_SHARE_ROM_MPU_AD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DPORT_SHARE_ROM_MPU_AD`"]
pub struct DPORT_SHARE_ROM_MPU_AD_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_SHARE_ROM_MPU_AD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `DPORT_APP_ROM_IA`"]
pub type DPORT_APP_ROM_IA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPORT_APP_ROM_IA`"]
pub struct DPORT_APP_ROM_IA_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_APP_ROM_IA_W<'a> {
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
#[doc = "Reader of field `DPORT_APP_ROM_MPU_AD`"]
pub type DPORT_APP_ROM_MPU_AD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPORT_APP_ROM_MPU_AD`"]
pub struct DPORT_APP_ROM_MPU_AD_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_APP_ROM_MPU_AD_W<'a> {
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
#[doc = "Reader of field `DPORT_PRO_ROM_IA`"]
pub type DPORT_PRO_ROM_IA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPORT_PRO_ROM_IA`"]
pub struct DPORT_PRO_ROM_IA_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_PRO_ROM_IA_W<'a> {
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
#[doc = "Reader of field `DPORT_PRO_ROM_MPU_AD`"]
pub type DPORT_PRO_ROM_MPU_AD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPORT_PRO_ROM_MPU_AD`"]
pub struct DPORT_PRO_ROM_MPU_AD_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_PRO_ROM_MPU_AD_W<'a> {
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
    #[doc = "Bits 26:29"]
    #[inline(always)]
    pub fn dport_internal_sram_mmu_multi_hit(&self) -> DPORT_INTERNAL_SRAM_MMU_MULTI_HIT_R {
        DPORT_INTERNAL_SRAM_MMU_MULTI_HIT_R::new(((self.bits >> 26) & 0x0f) as u8)
    }
    #[doc = "Bits 14:25"]
    #[inline(always)]
    pub fn dport_internal_sram_ia(&self) -> DPORT_INTERNAL_SRAM_IA_R {
        DPORT_INTERNAL_SRAM_IA_R::new(((self.bits >> 14) & 0x0fff) as u16)
    }
    #[doc = "Bits 10:13"]
    #[inline(always)]
    pub fn dport_internal_sram_mmu_ad(&self) -> DPORT_INTERNAL_SRAM_MMU_AD_R {
        DPORT_INTERNAL_SRAM_MMU_AD_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bits 6:9"]
    #[inline(always)]
    pub fn dport_share_rom_ia(&self) -> DPORT_SHARE_ROM_IA_R {
        DPORT_SHARE_ROM_IA_R::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn dport_share_rom_mpu_ad(&self) -> DPORT_SHARE_ROM_MPU_AD_R {
        DPORT_SHARE_ROM_MPU_AD_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dport_app_rom_ia(&self) -> DPORT_APP_ROM_IA_R {
        DPORT_APP_ROM_IA_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dport_app_rom_mpu_ad(&self) -> DPORT_APP_ROM_MPU_AD_R {
        DPORT_APP_ROM_MPU_AD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dport_pro_rom_ia(&self) -> DPORT_PRO_ROM_IA_R {
        DPORT_PRO_ROM_IA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dport_pro_rom_mpu_ad(&self) -> DPORT_PRO_ROM_MPU_AD_R {
        DPORT_PRO_ROM_MPU_AD_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 26:29"]
    #[inline(always)]
    pub fn dport_internal_sram_mmu_multi_hit(&mut self) -> DPORT_INTERNAL_SRAM_MMU_MULTI_HIT_W {
        DPORT_INTERNAL_SRAM_MMU_MULTI_HIT_W { w: self }
    }
    #[doc = "Bits 14:25"]
    #[inline(always)]
    pub fn dport_internal_sram_ia(&mut self) -> DPORT_INTERNAL_SRAM_IA_W {
        DPORT_INTERNAL_SRAM_IA_W { w: self }
    }
    #[doc = "Bits 10:13"]
    #[inline(always)]
    pub fn dport_internal_sram_mmu_ad(&mut self) -> DPORT_INTERNAL_SRAM_MMU_AD_W {
        DPORT_INTERNAL_SRAM_MMU_AD_W { w: self }
    }
    #[doc = "Bits 6:9"]
    #[inline(always)]
    pub fn dport_share_rom_ia(&mut self) -> DPORT_SHARE_ROM_IA_W {
        DPORT_SHARE_ROM_IA_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn dport_share_rom_mpu_ad(&mut self) -> DPORT_SHARE_ROM_MPU_AD_W {
        DPORT_SHARE_ROM_MPU_AD_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dport_app_rom_ia(&mut self) -> DPORT_APP_ROM_IA_W {
        DPORT_APP_ROM_IA_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dport_app_rom_mpu_ad(&mut self) -> DPORT_APP_ROM_MPU_AD_W {
        DPORT_APP_ROM_MPU_AD_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dport_pro_rom_ia(&mut self) -> DPORT_PRO_ROM_IA_W {
        DPORT_PRO_ROM_IA_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dport_pro_rom_mpu_ad(&mut self) -> DPORT_PRO_ROM_MPU_AD_W {
        DPORT_PRO_ROM_MPU_AD_W { w: self }
    }
}