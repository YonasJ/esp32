#[doc = "Reader of register SPI_USER_REG"]
pub type R = crate::R<u32, super::SPI_USER_REG>;
#[doc = "Writer for register SPI_USER_REG"]
pub type W = crate::W<u32, super::SPI_USER_REG>;
#[doc = "Register SPI_USER_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::SPI_USER_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPI_USR_COMMAND`"]
pub type SPI_USR_COMMAND_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_USR_COMMAND`"]
pub struct SPI_USR_COMMAND_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_USR_COMMAND_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `SPI_USR_ADDR`"]
pub type SPI_USR_ADDR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_USR_ADDR`"]
pub struct SPI_USR_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_USR_ADDR_W<'a> {
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
#[doc = "Reader of field `SPI_USR_DUMMY`"]
pub type SPI_USR_DUMMY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_USR_DUMMY`"]
pub struct SPI_USR_DUMMY_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_USR_DUMMY_W<'a> {
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
#[doc = "Reader of field `SPI_USR_MISO`"]
pub type SPI_USR_MISO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_USR_MISO`"]
pub struct SPI_USR_MISO_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_USR_MISO_W<'a> {
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
#[doc = "Reader of field `SPI_USR_MOSI`"]
pub type SPI_USR_MOSI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_USR_MOSI`"]
pub struct SPI_USR_MOSI_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_USR_MOSI_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `SPI_USR_DUMMY_IDLE`"]
pub type SPI_USR_DUMMY_IDLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_USR_DUMMY_IDLE`"]
pub struct SPI_USR_DUMMY_IDLE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_USR_DUMMY_IDLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `SPI_USR_MOSI_HIGHPART`"]
pub type SPI_USR_MOSI_HIGHPART_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_USR_MOSI_HIGHPART`"]
pub struct SPI_USR_MOSI_HIGHPART_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_USR_MOSI_HIGHPART_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `SPI_USR_MISO_HIGHPART`"]
pub type SPI_USR_MISO_HIGHPART_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_USR_MISO_HIGHPART`"]
pub struct SPI_USR_MISO_HIGHPART_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_USR_MISO_HIGHPART_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `SPI_USR_PREP_HOLD`"]
pub type SPI_USR_PREP_HOLD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_USR_PREP_HOLD`"]
pub struct SPI_USR_PREP_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_USR_PREP_HOLD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `SPI_USR_CMD_HOLD`"]
pub type SPI_USR_CMD_HOLD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_USR_CMD_HOLD`"]
pub struct SPI_USR_CMD_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_USR_CMD_HOLD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `SPI_USR_ADDR_HOLD`"]
pub type SPI_USR_ADDR_HOLD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_USR_ADDR_HOLD`"]
pub struct SPI_USR_ADDR_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_USR_ADDR_HOLD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `SPI_USR_DUMMY_HOLD`"]
pub type SPI_USR_DUMMY_HOLD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_USR_DUMMY_HOLD`"]
pub struct SPI_USR_DUMMY_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_USR_DUMMY_HOLD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `SPI_USR_DIN_HOLD`"]
pub type SPI_USR_DIN_HOLD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_USR_DIN_HOLD`"]
pub struct SPI_USR_DIN_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_USR_DIN_HOLD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `SPI_USR_DOUT_HOLD`"]
pub type SPI_USR_DOUT_HOLD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_USR_DOUT_HOLD`"]
pub struct SPI_USR_DOUT_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_USR_DOUT_HOLD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `SPI_USR_HOLD_POL`"]
pub type SPI_USR_HOLD_POL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_USR_HOLD_POL`"]
pub struct SPI_USR_HOLD_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_USR_HOLD_POL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `SPI_SIO`"]
pub type SPI_SIO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_SIO`"]
pub struct SPI_SIO_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SIO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `SPI_FWRITE_QIO`"]
pub type SPI_FWRITE_QIO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_FWRITE_QIO`"]
pub struct SPI_FWRITE_QIO_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_FWRITE_QIO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `SPI_FWRITE_DIO`"]
pub type SPI_FWRITE_DIO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_FWRITE_DIO`"]
pub struct SPI_FWRITE_DIO_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_FWRITE_DIO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `SPI_FWRITE_QUAD`"]
pub type SPI_FWRITE_QUAD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_FWRITE_QUAD`"]
pub struct SPI_FWRITE_QUAD_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_FWRITE_QUAD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `SPI_FWRITE_DUAL`"]
pub type SPI_FWRITE_DUAL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_FWRITE_DUAL`"]
pub struct SPI_FWRITE_DUAL_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_FWRITE_DUAL_W<'a> {
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
#[doc = "Reader of field `SPI_WR_BYTE_ORDER`"]
pub type SPI_WR_BYTE_ORDER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_WR_BYTE_ORDER`"]
pub struct SPI_WR_BYTE_ORDER_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_WR_BYTE_ORDER_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `SPI_RD_BYTE_ORDER`"]
pub type SPI_RD_BYTE_ORDER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_RD_BYTE_ORDER`"]
pub struct SPI_RD_BYTE_ORDER_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_RD_BYTE_ORDER_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `SPI_CK_OUT_EDGE`"]
pub type SPI_CK_OUT_EDGE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_CK_OUT_EDGE`"]
pub struct SPI_CK_OUT_EDGE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_CK_OUT_EDGE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `SPI_CK_I_EDGE`"]
pub type SPI_CK_I_EDGE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_CK_I_EDGE`"]
pub struct SPI_CK_I_EDGE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_CK_I_EDGE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `SPI_CS_SETUP`"]
pub type SPI_CS_SETUP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_CS_SETUP`"]
pub struct SPI_CS_SETUP_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_CS_SETUP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `SPI_CS_HOLD`"]
pub type SPI_CS_HOLD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_CS_HOLD`"]
pub struct SPI_CS_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_CS_HOLD_W<'a> {
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
#[doc = "Reader of field `SPI_DOUTDIN`"]
pub type SPI_DOUTDIN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_DOUTDIN`"]
pub struct SPI_DOUTDIN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_DOUTDIN_W<'a> {
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
    #[doc = "Bit 31 - This bit enable the command phase of an operation."]
    #[inline(always)]
    pub fn spi_usr_command(&self) -> SPI_USR_COMMAND_R {
        SPI_USR_COMMAND_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - This bit enable the address phase of an operation."]
    #[inline(always)]
    pub fn spi_usr_addr(&self) -> SPI_USR_ADDR_R {
        SPI_USR_ADDR_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - This bit enable the dummy phase of an operation."]
    #[inline(always)]
    pub fn spi_usr_dummy(&self) -> SPI_USR_DUMMY_R {
        SPI_USR_DUMMY_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - This bit enable the read-data phase of an operation."]
    #[inline(always)]
    pub fn spi_usr_miso(&self) -> SPI_USR_MISO_R {
        SPI_USR_MISO_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - This bit enable the write-data phase of an operation."]
    #[inline(always)]
    pub fn spi_usr_mosi(&self) -> SPI_USR_MOSI_R {
        SPI_USR_MOSI_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - spi clock is disable in dummy phase when the bit is enable."]
    #[inline(always)]
    pub fn spi_usr_dummy_idle(&self) -> SPI_USR_DUMMY_IDLE_R {
        SPI_USR_DUMMY_IDLE_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - write-data phase only access to high-part of the buffer spi_w8~spi_w15. 1: enable 0: disable."]
    #[inline(always)]
    pub fn spi_usr_mosi_highpart(&self) -> SPI_USR_MOSI_HIGHPART_R {
        SPI_USR_MOSI_HIGHPART_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - read-data phase only access to high-part of the buffer spi_w8~spi_w15. 1: enable 0: disable."]
    #[inline(always)]
    pub fn spi_usr_miso_highpart(&self) -> SPI_USR_MISO_HIGHPART_R {
        SPI_USR_MISO_HIGHPART_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - spi is hold at prepare state the bit combined with spi_usr_hold_pol bit."]
    #[inline(always)]
    pub fn spi_usr_prep_hold(&self) -> SPI_USR_PREP_HOLD_R {
        SPI_USR_PREP_HOLD_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - spi is hold at command state the bit combined with spi_usr_hold_pol bit."]
    #[inline(always)]
    pub fn spi_usr_cmd_hold(&self) -> SPI_USR_CMD_HOLD_R {
        SPI_USR_CMD_HOLD_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - spi is hold at address state the bit combined with spi_usr_hold_pol bit."]
    #[inline(always)]
    pub fn spi_usr_addr_hold(&self) -> SPI_USR_ADDR_HOLD_R {
        SPI_USR_ADDR_HOLD_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - spi is hold at dummy state the bit combined with spi_usr_hold_pol bit."]
    #[inline(always)]
    pub fn spi_usr_dummy_hold(&self) -> SPI_USR_DUMMY_HOLD_R {
        SPI_USR_DUMMY_HOLD_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - spi is hold at data in state the bit combined with spi_usr_hold_pol bit."]
    #[inline(always)]
    pub fn spi_usr_din_hold(&self) -> SPI_USR_DIN_HOLD_R {
        SPI_USR_DIN_HOLD_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - spi is hold at data out state the bit combined with spi_usr_hold_pol bit."]
    #[inline(always)]
    pub fn spi_usr_dout_hold(&self) -> SPI_USR_DOUT_HOLD_R {
        SPI_USR_DOUT_HOLD_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - It is combined with hold bits to set the polarity of spi hold line 1: spi will be held when spi hold line is high 0: spi will be held when spi hold line is low"]
    #[inline(always)]
    pub fn spi_usr_hold_pol(&self) -> SPI_USR_HOLD_POL_R {
        SPI_USR_HOLD_POL_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Set the bit to enable 3-line half duplex communication mosi and miso signals share the same pin. 1: enable 0: disable."]
    #[inline(always)]
    pub fn spi_sio(&self) -> SPI_SIO_R {
        SPI_SIO_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - In the write operations address phase and read-data phase apply 4 signals."]
    #[inline(always)]
    pub fn spi_fwrite_qio(&self) -> SPI_FWRITE_QIO_R {
        SPI_FWRITE_QIO_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - In the write operations address phase and read-data phase apply 2 signals."]
    #[inline(always)]
    pub fn spi_fwrite_dio(&self) -> SPI_FWRITE_DIO_R {
        SPI_FWRITE_DIO_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - In the write operations read-data phase apply 4 signals"]
    #[inline(always)]
    pub fn spi_fwrite_quad(&self) -> SPI_FWRITE_QUAD_R {
        SPI_FWRITE_QUAD_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - In the write operations read-data phase apply 2 signals"]
    #[inline(always)]
    pub fn spi_fwrite_dual(&self) -> SPI_FWRITE_DUAL_R {
        SPI_FWRITE_DUAL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - In command address write-data (MOSI) phases 1: big-endian 0: litte_endian"]
    #[inline(always)]
    pub fn spi_wr_byte_order(&self) -> SPI_WR_BYTE_ORDER_R {
        SPI_WR_BYTE_ORDER_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - In read-data (MISO) phase 1: big-endian 0: little_endian"]
    #[inline(always)]
    pub fn spi_rd_byte_order(&self) -> SPI_RD_BYTE_ORDER_R {
        SPI_RD_BYTE_ORDER_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 7 - the bit combined with spi_mosi_delay_mode bits to set mosi signal delay mode."]
    #[inline(always)]
    pub fn spi_ck_out_edge(&self) -> SPI_CK_OUT_EDGE_R {
        SPI_CK_OUT_EDGE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - In the slave mode the bit is same as spi_ck_out_edge in master mode. It is combined with spi_miso_delay_mode bits."]
    #[inline(always)]
    pub fn spi_ck_i_edge(&self) -> SPI_CK_I_EDGE_R {
        SPI_CK_I_EDGE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - spi cs is enable when spi is in \u{a1}\u{b0}prepare\u{a1}\u{b1} phase. 1: enable 0: disable."]
    #[inline(always)]
    pub fn spi_cs_setup(&self) -> SPI_CS_SETUP_R {
        SPI_CS_SETUP_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - spi cs keep low when spi is in \u{a1}\u{b0}done\u{a1}\u{b1} phase. 1: enable 0: disable."]
    #[inline(always)]
    pub fn spi_cs_hold(&self) -> SPI_CS_HOLD_R {
        SPI_CS_HOLD_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Set the bit to enable full duplex communication. 1: enable 0: disable."]
    #[inline(always)]
    pub fn spi_doutdin(&self) -> SPI_DOUTDIN_R {
        SPI_DOUTDIN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - This bit enable the command phase of an operation."]
    #[inline(always)]
    pub fn spi_usr_command(&mut self) -> SPI_USR_COMMAND_W {
        SPI_USR_COMMAND_W { w: self }
    }
    #[doc = "Bit 30 - This bit enable the address phase of an operation."]
    #[inline(always)]
    pub fn spi_usr_addr(&mut self) -> SPI_USR_ADDR_W {
        SPI_USR_ADDR_W { w: self }
    }
    #[doc = "Bit 29 - This bit enable the dummy phase of an operation."]
    #[inline(always)]
    pub fn spi_usr_dummy(&mut self) -> SPI_USR_DUMMY_W {
        SPI_USR_DUMMY_W { w: self }
    }
    #[doc = "Bit 28 - This bit enable the read-data phase of an operation."]
    #[inline(always)]
    pub fn spi_usr_miso(&mut self) -> SPI_USR_MISO_W {
        SPI_USR_MISO_W { w: self }
    }
    #[doc = "Bit 27 - This bit enable the write-data phase of an operation."]
    #[inline(always)]
    pub fn spi_usr_mosi(&mut self) -> SPI_USR_MOSI_W {
        SPI_USR_MOSI_W { w: self }
    }
    #[doc = "Bit 26 - spi clock is disable in dummy phase when the bit is enable."]
    #[inline(always)]
    pub fn spi_usr_dummy_idle(&mut self) -> SPI_USR_DUMMY_IDLE_W {
        SPI_USR_DUMMY_IDLE_W { w: self }
    }
    #[doc = "Bit 25 - write-data phase only access to high-part of the buffer spi_w8~spi_w15. 1: enable 0: disable."]
    #[inline(always)]
    pub fn spi_usr_mosi_highpart(&mut self) -> SPI_USR_MOSI_HIGHPART_W {
        SPI_USR_MOSI_HIGHPART_W { w: self }
    }
    #[doc = "Bit 24 - read-data phase only access to high-part of the buffer spi_w8~spi_w15. 1: enable 0: disable."]
    #[inline(always)]
    pub fn spi_usr_miso_highpart(&mut self) -> SPI_USR_MISO_HIGHPART_W {
        SPI_USR_MISO_HIGHPART_W { w: self }
    }
    #[doc = "Bit 23 - spi is hold at prepare state the bit combined with spi_usr_hold_pol bit."]
    #[inline(always)]
    pub fn spi_usr_prep_hold(&mut self) -> SPI_USR_PREP_HOLD_W {
        SPI_USR_PREP_HOLD_W { w: self }
    }
    #[doc = "Bit 22 - spi is hold at command state the bit combined with spi_usr_hold_pol bit."]
    #[inline(always)]
    pub fn spi_usr_cmd_hold(&mut self) -> SPI_USR_CMD_HOLD_W {
        SPI_USR_CMD_HOLD_W { w: self }
    }
    #[doc = "Bit 21 - spi is hold at address state the bit combined with spi_usr_hold_pol bit."]
    #[inline(always)]
    pub fn spi_usr_addr_hold(&mut self) -> SPI_USR_ADDR_HOLD_W {
        SPI_USR_ADDR_HOLD_W { w: self }
    }
    #[doc = "Bit 20 - spi is hold at dummy state the bit combined with spi_usr_hold_pol bit."]
    #[inline(always)]
    pub fn spi_usr_dummy_hold(&mut self) -> SPI_USR_DUMMY_HOLD_W {
        SPI_USR_DUMMY_HOLD_W { w: self }
    }
    #[doc = "Bit 19 - spi is hold at data in state the bit combined with spi_usr_hold_pol bit."]
    #[inline(always)]
    pub fn spi_usr_din_hold(&mut self) -> SPI_USR_DIN_HOLD_W {
        SPI_USR_DIN_HOLD_W { w: self }
    }
    #[doc = "Bit 18 - spi is hold at data out state the bit combined with spi_usr_hold_pol bit."]
    #[inline(always)]
    pub fn spi_usr_dout_hold(&mut self) -> SPI_USR_DOUT_HOLD_W {
        SPI_USR_DOUT_HOLD_W { w: self }
    }
    #[doc = "Bit 17 - It is combined with hold bits to set the polarity of spi hold line 1: spi will be held when spi hold line is high 0: spi will be held when spi hold line is low"]
    #[inline(always)]
    pub fn spi_usr_hold_pol(&mut self) -> SPI_USR_HOLD_POL_W {
        SPI_USR_HOLD_POL_W { w: self }
    }
    #[doc = "Bit 16 - Set the bit to enable 3-line half duplex communication mosi and miso signals share the same pin. 1: enable 0: disable."]
    #[inline(always)]
    pub fn spi_sio(&mut self) -> SPI_SIO_W {
        SPI_SIO_W { w: self }
    }
    #[doc = "Bit 15 - In the write operations address phase and read-data phase apply 4 signals."]
    #[inline(always)]
    pub fn spi_fwrite_qio(&mut self) -> SPI_FWRITE_QIO_W {
        SPI_FWRITE_QIO_W { w: self }
    }
    #[doc = "Bit 14 - In the write operations address phase and read-data phase apply 2 signals."]
    #[inline(always)]
    pub fn spi_fwrite_dio(&mut self) -> SPI_FWRITE_DIO_W {
        SPI_FWRITE_DIO_W { w: self }
    }
    #[doc = "Bit 13 - In the write operations read-data phase apply 4 signals"]
    #[inline(always)]
    pub fn spi_fwrite_quad(&mut self) -> SPI_FWRITE_QUAD_W {
        SPI_FWRITE_QUAD_W { w: self }
    }
    #[doc = "Bit 12 - In the write operations read-data phase apply 2 signals"]
    #[inline(always)]
    pub fn spi_fwrite_dual(&mut self) -> SPI_FWRITE_DUAL_W {
        SPI_FWRITE_DUAL_W { w: self }
    }
    #[doc = "Bit 11 - In command address write-data (MOSI) phases 1: big-endian 0: litte_endian"]
    #[inline(always)]
    pub fn spi_wr_byte_order(&mut self) -> SPI_WR_BYTE_ORDER_W {
        SPI_WR_BYTE_ORDER_W { w: self }
    }
    #[doc = "Bit 10 - In read-data (MISO) phase 1: big-endian 0: little_endian"]
    #[inline(always)]
    pub fn spi_rd_byte_order(&mut self) -> SPI_RD_BYTE_ORDER_W {
        SPI_RD_BYTE_ORDER_W { w: self }
    }
    #[doc = "Bit 7 - the bit combined with spi_mosi_delay_mode bits to set mosi signal delay mode."]
    #[inline(always)]
    pub fn spi_ck_out_edge(&mut self) -> SPI_CK_OUT_EDGE_W {
        SPI_CK_OUT_EDGE_W { w: self }
    }
    #[doc = "Bit 6 - In the slave mode the bit is same as spi_ck_out_edge in master mode. It is combined with spi_miso_delay_mode bits."]
    #[inline(always)]
    pub fn spi_ck_i_edge(&mut self) -> SPI_CK_I_EDGE_W {
        SPI_CK_I_EDGE_W { w: self }
    }
    #[doc = "Bit 5 - spi cs is enable when spi is in \u{a1}\u{b0}prepare\u{a1}\u{b1} phase. 1: enable 0: disable."]
    #[inline(always)]
    pub fn spi_cs_setup(&mut self) -> SPI_CS_SETUP_W {
        SPI_CS_SETUP_W { w: self }
    }
    #[doc = "Bit 4 - spi cs keep low when spi is in \u{a1}\u{b0}done\u{a1}\u{b1} phase. 1: enable 0: disable."]
    #[inline(always)]
    pub fn spi_cs_hold(&mut self) -> SPI_CS_HOLD_W {
        SPI_CS_HOLD_W { w: self }
    }
    #[doc = "Bit 0 - Set the bit to enable full duplex communication. 1: enable 0: disable."]
    #[inline(always)]
    pub fn spi_doutdin(&mut self) -> SPI_DOUTDIN_W {
        SPI_DOUTDIN_W { w: self }
    }
}
