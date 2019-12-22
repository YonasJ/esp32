#[doc = "Reader of register SPI_TX_CRC_REG"]
pub type R = crate::R<u32, super::SPI_TX_CRC_REG>;
#[doc = "Writer for register SPI_TX_CRC_REG"]
pub type W = crate::W<u32, super::SPI_TX_CRC_REG>;
#[doc = "Register SPI_TX_CRC_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::SPI_TX_CRC_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPI_TX_CRC_DATA`"]
pub type SPI_TX_CRC_DATA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SPI_TX_CRC_DATA`"]
pub struct SPI_TX_CRC_DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_TX_CRC_DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - For SPI1 the value of crc32 for 256 bits data."]
    #[inline(always)]
    pub fn spi_tx_crc_data(&self) -> SPI_TX_CRC_DATA_R {
        SPI_TX_CRC_DATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - For SPI1 the value of crc32 for 256 bits data."]
    #[inline(always)]
    pub fn spi_tx_crc_data(&mut self) -> SPI_TX_CRC_DATA_W {
        SPI_TX_CRC_DATA_W { w: self }
    }
}
