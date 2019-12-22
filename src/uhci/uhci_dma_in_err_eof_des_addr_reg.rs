#[doc = "Reader of register UHCI_DMA_IN_ERR_EOF_DES_ADDR_REG"]
pub type R = crate::R<u32, super::UHCI_DMA_IN_ERR_EOF_DES_ADDR_REG>;
#[doc = "Writer for register UHCI_DMA_IN_ERR_EOF_DES_ADDR_REG"]
pub type W = crate::W<u32, super::UHCI_DMA_IN_ERR_EOF_DES_ADDR_REG>;
#[doc = "Register UHCI_DMA_IN_ERR_EOF_DES_ADDR_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::UHCI_DMA_IN_ERR_EOF_DES_ADDR_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UHCI_IN_ERR_EOF_DES_ADDR`"]
pub type UHCI_IN_ERR_EOF_DES_ADDR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `UHCI_IN_ERR_EOF_DES_ADDR`"]
pub struct UHCI_IN_ERR_EOF_DES_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_IN_ERR_EOF_DES_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - This register stores the address of in link descriptor when there are some errors in this descriptor."]
    #[inline(always)]
    pub fn uhci_in_err_eof_des_addr(&self) -> UHCI_IN_ERR_EOF_DES_ADDR_R {
        UHCI_IN_ERR_EOF_DES_ADDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - This register stores the address of in link descriptor when there are some errors in this descriptor."]
    #[inline(always)]
    pub fn uhci_in_err_eof_des_addr(&mut self) -> UHCI_IN_ERR_EOF_DES_ADDR_W {
        UHCI_IN_ERR_EOF_DES_ADDR_W { w: self }
    }
}
