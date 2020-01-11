#[doc = "Reader of register DMA_IN_DSCR"]
pub type R = crate::R<u32, super::DMA_IN_DSCR>;
#[doc = "Writer for register DMA_IN_DSCR"]
pub type W = crate::W<u32, super::DMA_IN_DSCR>;
#[doc = "Register DMA_IN_DSCR `reset()`'s with value 0"]
impl crate::ResetValue for super::DMA_IN_DSCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UHCI_INLINK_DSCR`"]
pub type UHCI_INLINK_DSCR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `UHCI_INLINK_DSCR`"]
pub struct UHCI_INLINK_DSCR_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_INLINK_DSCR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - The content of current in link descriptor's third dword"]
    #[inline(always)]
    pub fn uhci_inlink_dscr(&self) -> UHCI_INLINK_DSCR_R {
        UHCI_INLINK_DSCR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - The content of current in link descriptor's third dword"]
    #[inline(always)]
    pub fn uhci_inlink_dscr(&mut self) -> UHCI_INLINK_DSCR_W {
        UHCI_INLINK_DSCR_W { w: self }
    }
}