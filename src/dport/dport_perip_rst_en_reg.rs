#[doc = "Reader of register DPORT_PERIP_RST_EN_REG"]
pub type R = crate::R<u32, super::DPORT_PERIP_RST_EN_REG>;
#[doc = "Writer for register DPORT_PERIP_RST_EN_REG"]
pub type W = crate::W<u32, super::DPORT_PERIP_RST_EN_REG>;
#[doc = "Register DPORT_PERIP_RST_EN_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::DPORT_PERIP_RST_EN_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DPORT_PERIP_RST`"]
pub type DPORT_PERIP_RST_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DPORT_PERIP_RST`"]
pub struct DPORT_PERIP_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_PERIP_RST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
#[doc = "Reader of field `DPORT_SPI_DECRYPT_ENABLE`"]
pub type DPORT_SPI_DECRYPT_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPORT_SPI_DECRYPT_ENABLE`"]
pub struct DPORT_SPI_DECRYPT_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_SPI_DECRYPT_ENABLE_W<'a> {
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
#[doc = "Reader of field `DPORT_SPI_ENCRYPT_ENABLE`"]
pub type DPORT_SPI_ENCRYPT_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPORT_SPI_ENCRYPT_ENABLE`"]
pub struct DPORT_SPI_ENCRYPT_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_SPI_ENCRYPT_ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `DPORT_SLAVE_SPI_MASK_APP`"]
pub type DPORT_SLAVE_SPI_MASK_APP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPORT_SLAVE_SPI_MASK_APP`"]
pub struct DPORT_SLAVE_SPI_MASK_APP_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_SLAVE_SPI_MASK_APP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `DPORT_SLAVE_SPI_MASK_PRO`"]
pub type DPORT_SLAVE_SPI_MASK_PRO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPORT_SLAVE_SPI_MASK_PRO`"]
pub struct DPORT_SLAVE_SPI_MASK_PRO_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_SLAVE_SPI_MASK_PRO_W<'a> {
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
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dport_perip_rst(&self) -> DPORT_PERIP_RST_R {
        DPORT_PERIP_RST_R::new((self.bits & 0xffff_ffff) as u32)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn dport_spi_decrypt_enable(&self) -> DPORT_SPI_DECRYPT_ENABLE_R {
        DPORT_SPI_DECRYPT_ENABLE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn dport_spi_encrypt_enable(&self) -> DPORT_SPI_ENCRYPT_ENABLE_R {
        DPORT_SPI_ENCRYPT_ENABLE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn dport_slave_spi_mask_app(&self) -> DPORT_SLAVE_SPI_MASK_APP_R {
        DPORT_SLAVE_SPI_MASK_APP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dport_slave_spi_mask_pro(&self) -> DPORT_SLAVE_SPI_MASK_PRO_R {
        DPORT_SLAVE_SPI_MASK_PRO_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dport_perip_rst(&mut self) -> DPORT_PERIP_RST_W {
        DPORT_PERIP_RST_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn dport_spi_decrypt_enable(&mut self) -> DPORT_SPI_DECRYPT_ENABLE_W {
        DPORT_SPI_DECRYPT_ENABLE_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn dport_spi_encrypt_enable(&mut self) -> DPORT_SPI_ENCRYPT_ENABLE_W {
        DPORT_SPI_ENCRYPT_ENABLE_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn dport_slave_spi_mask_app(&mut self) -> DPORT_SLAVE_SPI_MASK_APP_W {
        DPORT_SLAVE_SPI_MASK_APP_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dport_slave_spi_mask_pro(&mut self) -> DPORT_SLAVE_SPI_MASK_PRO_W {
        DPORT_SLAVE_SPI_MASK_PRO_W { w: self }
    }
}