#[doc = "Reader of register DMA_OUT_LINK"]
pub type R = crate::R<u32, super::DMA_OUT_LINK>;
#[doc = "Writer for register DMA_OUT_LINK"]
pub type W = crate::W<u32, super::DMA_OUT_LINK>;
#[doc = "Register DMA_OUT_LINK `reset()`'s with value 0"]
impl crate::ResetValue for super::DMA_OUT_LINK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPI_OUTLINK_RESTART`"]
pub type SPI_OUTLINK_RESTART_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_OUTLINK_RESTART`"]
pub struct SPI_OUTLINK_RESTART_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_OUTLINK_RESTART_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `SPI_OUTLINK_START`"]
pub type SPI_OUTLINK_START_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_OUTLINK_START`"]
pub struct SPI_OUTLINK_START_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_OUTLINK_START_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `SPI_OUTLINK_STOP`"]
pub type SPI_OUTLINK_STOP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_OUTLINK_STOP`"]
pub struct SPI_OUTLINK_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_OUTLINK_STOP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `SPI_OUTLINK_ADDR`"]
pub type SPI_OUTLINK_ADDR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SPI_OUTLINK_ADDR`"]
pub struct SPI_OUTLINK_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_OUTLINK_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | ((value as u32) & 0x000f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bit 30 - Set the bit to mount on new outlink descriptors."]
    #[inline(always)]
    pub fn spi_outlink_restart(&self) -> SPI_OUTLINK_RESTART_R {
        SPI_OUTLINK_RESTART_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Set the bit to start to use outlink descriptor."]
    #[inline(always)]
    pub fn spi_outlink_start(&self) -> SPI_OUTLINK_START_R {
        SPI_OUTLINK_START_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Set the bit to stop to use outlink descriptor."]
    #[inline(always)]
    pub fn spi_outlink_stop(&self) -> SPI_OUTLINK_STOP_R {
        SPI_OUTLINK_STOP_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bits 0:19 - The address of the first outlink descriptor."]
    #[inline(always)]
    pub fn spi_outlink_addr(&self) -> SPI_OUTLINK_ADDR_R {
        SPI_OUTLINK_ADDR_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 30 - Set the bit to mount on new outlink descriptors."]
    #[inline(always)]
    pub fn spi_outlink_restart(&mut self) -> SPI_OUTLINK_RESTART_W {
        SPI_OUTLINK_RESTART_W { w: self }
    }
    #[doc = "Bit 29 - Set the bit to start to use outlink descriptor."]
    #[inline(always)]
    pub fn spi_outlink_start(&mut self) -> SPI_OUTLINK_START_W {
        SPI_OUTLINK_START_W { w: self }
    }
    #[doc = "Bit 28 - Set the bit to stop to use outlink descriptor."]
    #[inline(always)]
    pub fn spi_outlink_stop(&mut self) -> SPI_OUTLINK_STOP_W {
        SPI_OUTLINK_STOP_W { w: self }
    }
    #[doc = "Bits 0:19 - The address of the first outlink descriptor."]
    #[inline(always)]
    pub fn spi_outlink_addr(&mut self) -> SPI_OUTLINK_ADDR_W {
        SPI_OUTLINK_ADDR_W { w: self }
    }
}