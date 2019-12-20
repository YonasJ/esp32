#[doc = "Reader of register HOST_SLCHOST_RDCLR0_REG"]
pub type R = crate::R<u32, super::HOST_SLCHOST_RDCLR0_REG>;
#[doc = "Writer for register HOST_SLCHOST_RDCLR0_REG"]
pub type W = crate::W<u32, super::HOST_SLCHOST_RDCLR0_REG>;
#[doc = "Register HOST_SLCHOST_RDCLR0_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::HOST_SLCHOST_RDCLR0_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HOST_SLCHOST_SLC0_BIT6_CLRADDR`"]
pub type HOST_SLCHOST_SLC0_BIT6_CLRADDR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `HOST_SLCHOST_SLC0_BIT6_CLRADDR`"]
pub struct HOST_SLCHOST_SLC0_BIT6_CLRADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLCHOST_SLC0_BIT6_CLRADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 9)) | (((value as u32) & 0x01ff) << 9);
        self.w
    }
}
#[doc = "Reader of field `HOST_SLCHOST_SLC0_BIT7_CLRADDR`"]
pub type HOST_SLCHOST_SLC0_BIT7_CLRADDR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `HOST_SLCHOST_SLC0_BIT7_CLRADDR`"]
pub struct HOST_SLCHOST_SLC0_BIT7_CLRADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLCHOST_SLC0_BIT7_CLRADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | ((value as u32) & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 9:17"]
    #[inline(always)]
    pub fn host_slchost_slc0_bit6_clraddr(&self) -> HOST_SLCHOST_SLC0_BIT6_CLRADDR_R {
        HOST_SLCHOST_SLC0_BIT6_CLRADDR_R::new(((self.bits >> 9) & 0x01ff) as u16)
    }
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn host_slchost_slc0_bit7_clraddr(&self) -> HOST_SLCHOST_SLC0_BIT7_CLRADDR_R {
        HOST_SLCHOST_SLC0_BIT7_CLRADDR_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 9:17"]
    #[inline(always)]
    pub fn host_slchost_slc0_bit6_clraddr(&mut self) -> HOST_SLCHOST_SLC0_BIT6_CLRADDR_W {
        HOST_SLCHOST_SLC0_BIT6_CLRADDR_W { w: self }
    }
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn host_slchost_slc0_bit7_clraddr(&mut self) -> HOST_SLCHOST_SLC0_BIT7_CLRADDR_W {
        HOST_SLCHOST_SLC0_BIT7_CLRADDR_W { w: self }
    }
}