#[doc = "Reader of register HOST_SLCHOST_CONF_W0_REG"]
pub type R = crate::R<u32, super::HOST_SLCHOST_CONF_W0_REG>;
#[doc = "Writer for register HOST_SLCHOST_CONF_W0_REG"]
pub type W = crate::W<u32, super::HOST_SLCHOST_CONF_W0_REG>;
#[doc = "Register HOST_SLCHOST_CONF_W0_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::HOST_SLCHOST_CONF_W0_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HOST_SLCHOST_CONF3`"]
pub type HOST_SLCHOST_CONF3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HOST_SLCHOST_CONF3`"]
pub struct HOST_SLCHOST_CONF3_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLCHOST_CONF3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `HOST_SLCHOST_CONF2`"]
pub type HOST_SLCHOST_CONF2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HOST_SLCHOST_CONF2`"]
pub struct HOST_SLCHOST_CONF2_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLCHOST_CONF2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `HOST_SLCHOST_CONF1`"]
pub type HOST_SLCHOST_CONF1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HOST_SLCHOST_CONF1`"]
pub struct HOST_SLCHOST_CONF1_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLCHOST_CONF1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `HOST_SLCHOST_CONF0`"]
pub type HOST_SLCHOST_CONF0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HOST_SLCHOST_CONF0`"]
pub struct HOST_SLCHOST_CONF0_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLCHOST_CONF0_W<'a> {
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
    pub fn host_slchost_conf3(&self) -> HOST_SLCHOST_CONF3_R {
        HOST_SLCHOST_CONF3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn host_slchost_conf2(&self) -> HOST_SLCHOST_CONF2_R {
        HOST_SLCHOST_CONF2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn host_slchost_conf1(&self) -> HOST_SLCHOST_CONF1_R {
        HOST_SLCHOST_CONF1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn host_slchost_conf0(&self) -> HOST_SLCHOST_CONF0_R {
        HOST_SLCHOST_CONF0_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn host_slchost_conf3(&mut self) -> HOST_SLCHOST_CONF3_W {
        HOST_SLCHOST_CONF3_W { w: self }
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn host_slchost_conf2(&mut self) -> HOST_SLCHOST_CONF2_W {
        HOST_SLCHOST_CONF2_W { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn host_slchost_conf1(&mut self) -> HOST_SLCHOST_CONF1_W {
        HOST_SLCHOST_CONF1_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn host_slchost_conf0(&mut self) -> HOST_SLCHOST_CONF0_W {
        HOST_SLCHOST_CONF0_W { w: self }
    }
}
