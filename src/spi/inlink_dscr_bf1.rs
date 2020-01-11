#[doc = "Reader of register INLINK_DSCR_BF1"]
pub type R = crate::R<u32, super::INLINK_DSCR_BF1>;
#[doc = "Writer for register INLINK_DSCR_BF1"]
pub type W = crate::W<u32, super::INLINK_DSCR_BF1>;
#[doc = "Register INLINK_DSCR_BF1 `reset()`'s with value 0"]
impl crate::ResetValue for super::INLINK_DSCR_BF1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPI_DMA_INLINK_DSCR_BF1`"]
pub type SPI_DMA_INLINK_DSCR_BF1_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SPI_DMA_INLINK_DSCR_BF1`"]
pub struct SPI_DMA_INLINK_DSCR_BF1_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_DMA_INLINK_DSCR_BF1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - The content of current in descriptor data buffer pointer."]
    #[inline(always)]
    pub fn spi_dma_inlink_dscr_bf1(&self) -> SPI_DMA_INLINK_DSCR_BF1_R {
        SPI_DMA_INLINK_DSCR_BF1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - The content of current in descriptor data buffer pointer."]
    #[inline(always)]
    pub fn spi_dma_inlink_dscr_bf1(&mut self) -> SPI_DMA_INLINK_DSCR_BF1_W {
        SPI_DMA_INLINK_DSCR_BF1_W { w: self }
    }
}