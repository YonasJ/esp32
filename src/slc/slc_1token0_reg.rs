#[doc = "Reader of register SLC_1TOKEN0_REG"]
pub type R = crate::R<u32, super::SLC_1TOKEN0_REG>;
#[doc = "Writer for register SLC_1TOKEN0_REG"]
pub type W = crate::W<u32, super::SLC_1TOKEN0_REG>;
#[doc = "Register SLC_1TOKEN0_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::SLC_1TOKEN0_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SLC_SLC1_TOKEN0`"]
pub type SLC_SLC1_TOKEN0_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SLC_SLC1_TOKEN0`"]
pub struct SLC_SLC1_TOKEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC1_TOKEN0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | (((value as u32) & 0x0fff) << 16);
        self.w
    }
}
#[doc = "Reader of field `SLC_SLC1_TOKEN0_INC_MORE`"]
pub type SLC_SLC1_TOKEN0_INC_MORE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_SLC1_TOKEN0_INC_MORE`"]
pub struct SLC_SLC1_TOKEN0_INC_MORE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC1_TOKEN0_INC_MORE_W<'a> {
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
#[doc = "Reader of field `SLC_SLC1_TOKEN0_INC`"]
pub type SLC_SLC1_TOKEN0_INC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_SLC1_TOKEN0_INC`"]
pub struct SLC_SLC1_TOKEN0_INC_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC1_TOKEN0_INC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `SLC_SLC1_TOKEN0_WR`"]
pub type SLC_SLC1_TOKEN0_WR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_SLC1_TOKEN0_WR`"]
pub struct SLC_SLC1_TOKEN0_WR_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC1_TOKEN0_WR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `SLC_SLC1_TOKEN0_WDATA`"]
pub type SLC_SLC1_TOKEN0_WDATA_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SLC_SLC1_TOKEN0_WDATA`"]
pub struct SLC_SLC1_TOKEN0_WDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC1_TOKEN0_WDATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:27"]
    #[inline(always)]
    pub fn slc_slc1_token0(&self) -> SLC_SLC1_TOKEN0_R {
        SLC_SLC1_TOKEN0_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn slc_slc1_token0_inc_more(&self) -> SLC_SLC1_TOKEN0_INC_MORE_R {
        SLC_SLC1_TOKEN0_INC_MORE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn slc_slc1_token0_inc(&self) -> SLC_SLC1_TOKEN0_INC_R {
        SLC_SLC1_TOKEN0_INC_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn slc_slc1_token0_wr(&self) -> SLC_SLC1_TOKEN0_WR_R {
        SLC_SLC1_TOKEN0_WR_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn slc_slc1_token0_wdata(&self) -> SLC_SLC1_TOKEN0_WDATA_R {
        SLC_SLC1_TOKEN0_WDATA_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:27"]
    #[inline(always)]
    pub fn slc_slc1_token0(&mut self) -> SLC_SLC1_TOKEN0_W {
        SLC_SLC1_TOKEN0_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn slc_slc1_token0_inc_more(&mut self) -> SLC_SLC1_TOKEN0_INC_MORE_W {
        SLC_SLC1_TOKEN0_INC_MORE_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn slc_slc1_token0_inc(&mut self) -> SLC_SLC1_TOKEN0_INC_W {
        SLC_SLC1_TOKEN0_INC_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn slc_slc1_token0_wr(&mut self) -> SLC_SLC1_TOKEN0_WR_W {
        SLC_SLC1_TOKEN0_WR_W { w: self }
    }
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn slc_slc1_token0_wdata(&mut self) -> SLC_SLC1_TOKEN0_WDATA_W {
        SLC_SLC1_TOKEN0_WDATA_W { w: self }
    }
}