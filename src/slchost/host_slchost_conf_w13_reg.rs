#[doc = "Reader of register HOST_SLCHOST_CONF_W13_REG"]
pub type R = crate::R<u32, super::HOST_SLCHOST_CONF_W13_REG>;
#[doc = "Writer for register HOST_SLCHOST_CONF_W13_REG"]
pub type W = crate::W<u32, super::HOST_SLCHOST_CONF_W13_REG>;
#[doc = "Register HOST_SLCHOST_CONF_W13_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::HOST_SLCHOST_CONF_W13_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HOST_SLCHOST_CONF55`"]
pub type HOST_SLCHOST_CONF55_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HOST_SLCHOST_CONF55`"]
pub struct HOST_SLCHOST_CONF55_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLCHOST_CONF55_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `HOST_SLCHOST_CONF54`"]
pub type HOST_SLCHOST_CONF54_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HOST_SLCHOST_CONF54`"]
pub struct HOST_SLCHOST_CONF54_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLCHOST_CONF54_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `HOST_SLCHOST_CONF53`"]
pub type HOST_SLCHOST_CONF53_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HOST_SLCHOST_CONF53`"]
pub struct HOST_SLCHOST_CONF53_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLCHOST_CONF53_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `HOST_SLCHOST_CONF52`"]
pub type HOST_SLCHOST_CONF52_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HOST_SLCHOST_CONF52`"]
pub struct HOST_SLCHOST_CONF52_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLCHOST_CONF52_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn host_slchost_conf55(&self) -> HOST_SLCHOST_CONF55_R {
        HOST_SLCHOST_CONF55_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn host_slchost_conf54(&self) -> HOST_SLCHOST_CONF54_R {
        HOST_SLCHOST_CONF54_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn host_slchost_conf53(&self) -> HOST_SLCHOST_CONF53_R {
        HOST_SLCHOST_CONF53_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn host_slchost_conf52(&self) -> HOST_SLCHOST_CONF52_R {
        HOST_SLCHOST_CONF52_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn host_slchost_conf55(&mut self) -> HOST_SLCHOST_CONF55_W {
        HOST_SLCHOST_CONF55_W { w: self }
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn host_slchost_conf54(&mut self) -> HOST_SLCHOST_CONF54_W {
        HOST_SLCHOST_CONF54_W { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn host_slchost_conf53(&mut self) -> HOST_SLCHOST_CONF53_W {
        HOST_SLCHOST_CONF53_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn host_slchost_conf52(&mut self) -> HOST_SLCHOST_CONF52_W {
        HOST_SLCHOST_CONF52_W { w: self }
    }
}