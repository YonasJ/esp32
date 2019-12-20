#[doc = "Reader of register EFUSE_INT_RAW_REG"]
pub type R = crate::R<u32, super::EFUSE_INT_RAW_REG>;
#[doc = "Writer for register EFUSE_INT_RAW_REG"]
pub type W = crate::W<u32, super::EFUSE_INT_RAW_REG>;
#[doc = "Register EFUSE_INT_RAW_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::EFUSE_INT_RAW_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EFUSE_PGM_DONE_INT_RAW`"]
pub type EFUSE_PGM_DONE_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EFUSE_PGM_DONE_INT_RAW`"]
pub struct EFUSE_PGM_DONE_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_PGM_DONE_INT_RAW_W<'a> {
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
#[doc = "Reader of field `EFUSE_READ_DONE_INT_RAW`"]
pub type EFUSE_READ_DONE_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EFUSE_READ_DONE_INT_RAW`"]
pub struct EFUSE_READ_DONE_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_READ_DONE_INT_RAW_W<'a> {
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
    #[doc = "Bit 1 - program done interrupt raw status"]
    #[inline(always)]
    pub fn efuse_pgm_done_int_raw(&self) -> EFUSE_PGM_DONE_INT_RAW_R {
        EFUSE_PGM_DONE_INT_RAW_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - read done interrupt raw status"]
    #[inline(always)]
    pub fn efuse_read_done_int_raw(&self) -> EFUSE_READ_DONE_INT_RAW_R {
        EFUSE_READ_DONE_INT_RAW_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - program done interrupt raw status"]
    #[inline(always)]
    pub fn efuse_pgm_done_int_raw(&mut self) -> EFUSE_PGM_DONE_INT_RAW_W {
        EFUSE_PGM_DONE_INT_RAW_W { w: self }
    }
    #[doc = "Bit 0 - read done interrupt raw status"]
    #[inline(always)]
    pub fn efuse_read_done_int_raw(&mut self) -> EFUSE_READ_DONE_INT_RAW_W {
        EFUSE_READ_DONE_INT_RAW_W { w: self }
    }
}