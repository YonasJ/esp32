#[doc = "Reader of register HOST_SLC0HOST_TOKEN_WDATA_REG"]
pub type R = crate::R<u32, super::HOST_SLC0HOST_TOKEN_WDATA_REG>;
#[doc = "Writer for register HOST_SLC0HOST_TOKEN_WDATA_REG"]
pub type W = crate::W<u32, super::HOST_SLC0HOST_TOKEN_WDATA_REG>;
#[doc = "Register HOST_SLC0HOST_TOKEN_WDATA_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::HOST_SLC0HOST_TOKEN_WDATA_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HOST_SLC0HOST_TOKEN1_WD`"]
pub type HOST_SLC0HOST_TOKEN1_WD_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `HOST_SLC0HOST_TOKEN1_WD`"]
pub struct HOST_SLC0HOST_TOKEN1_WD_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLC0HOST_TOKEN1_WD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | (((value as u32) & 0x0fff) << 16);
        self.w
    }
}
#[doc = "Reader of field `HOST_SLC0HOST_TOKEN0_WD`"]
pub type HOST_SLC0HOST_TOKEN0_WD_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `HOST_SLC0HOST_TOKEN0_WD`"]
pub struct HOST_SLC0HOST_TOKEN0_WD_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLC0HOST_TOKEN0_WD_W<'a> {
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
    pub fn host_slc0host_token1_wd(&self) -> HOST_SLC0HOST_TOKEN1_WD_R {
        HOST_SLC0HOST_TOKEN1_WD_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn host_slc0host_token0_wd(&self) -> HOST_SLC0HOST_TOKEN0_WD_R {
        HOST_SLC0HOST_TOKEN0_WD_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:27"]
    #[inline(always)]
    pub fn host_slc0host_token1_wd(&mut self) -> HOST_SLC0HOST_TOKEN1_WD_W {
        HOST_SLC0HOST_TOKEN1_WD_W { w: self }
    }
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn host_slc0host_token0_wd(&mut self) -> HOST_SLC0HOST_TOKEN0_WD_W {
        HOST_SLC0HOST_TOKEN0_WD_W { w: self }
    }
}
