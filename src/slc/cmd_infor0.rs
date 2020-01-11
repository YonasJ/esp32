#[doc = "Reader of register CMD_INFOR0"]
pub type R = crate::R<u32, super::CMD_INFOR0>;
#[doc = "Writer for register CMD_INFOR0"]
pub type W = crate::W<u32, super::CMD_INFOR0>;
#[doc = "Register CMD_INFOR0 `reset()`'s with value 0"]
impl crate::ResetValue for super::CMD_INFOR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SLC_CMD_CONTENT0`"]
pub type SLC_CMD_CONTENT0_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SLC_CMD_CONTENT0`"]
pub struct SLC_CMD_CONTENT0_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_CMD_CONTENT0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn slc_cmd_content0(&self) -> SLC_CMD_CONTENT0_R {
        SLC_CMD_CONTENT0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn slc_cmd_content0(&mut self) -> SLC_CMD_CONTENT0_W {
        SLC_CMD_CONTENT0_W { w: self }
    }
}