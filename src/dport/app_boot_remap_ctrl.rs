#[doc = "Reader of register APP_BOOT_REMAP_CTRL"]
pub type R = crate::R<u32, super::APP_BOOT_REMAP_CTRL>;
#[doc = "Writer for register APP_BOOT_REMAP_CTRL"]
pub type W = crate::W<u32, super::APP_BOOT_REMAP_CTRL>;
#[doc = "Register APP_BOOT_REMAP_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::APP_BOOT_REMAP_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DPORT_APP_BOOT_REMAP`"]
pub type DPORT_APP_BOOT_REMAP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPORT_APP_BOOT_REMAP`"]
pub struct DPORT_APP_BOOT_REMAP_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_APP_BOOT_REMAP_W<'a> {
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
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dport_app_boot_remap(&self) -> DPORT_APP_BOOT_REMAP_R {
        DPORT_APP_BOOT_REMAP_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dport_app_boot_remap(&mut self) -> DPORT_APP_BOOT_REMAP_W {
        DPORT_APP_BOOT_REMAP_W { w: self }
    }
}