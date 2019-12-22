#[doc = "Reader of register DPORT_BT_LPCK_DIV_INT_REG"]
pub type R = crate::R<u32, super::DPORT_BT_LPCK_DIV_INT_REG>;
#[doc = "Writer for register DPORT_BT_LPCK_DIV_INT_REG"]
pub type W = crate::W<u32, super::DPORT_BT_LPCK_DIV_INT_REG>;
#[doc = "Register DPORT_BT_LPCK_DIV_INT_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::DPORT_BT_LPCK_DIV_INT_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DPORT_BTEXTWAKEUP_REQ`"]
pub type DPORT_BTEXTWAKEUP_REQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPORT_BTEXTWAKEUP_REQ`"]
pub struct DPORT_BTEXTWAKEUP_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_BTEXTWAKEUP_REQ_W<'a> {
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
#[doc = "Reader of field `DPORT_BT_LPCK_DIV_NUM`"]
pub type DPORT_BT_LPCK_DIV_NUM_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DPORT_BT_LPCK_DIV_NUM`"]
pub struct DPORT_BT_LPCK_DIV_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_BT_LPCK_DIV_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn dport_btextwakeup_req(&self) -> DPORT_BTEXTWAKEUP_REQ_R {
        DPORT_BTEXTWAKEUP_REQ_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn dport_bt_lpck_div_num(&self) -> DPORT_BT_LPCK_DIV_NUM_R {
        DPORT_BT_LPCK_DIV_NUM_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn dport_btextwakeup_req(&mut self) -> DPORT_BTEXTWAKEUP_REQ_W {
        DPORT_BTEXTWAKEUP_REQ_W { w: self }
    }
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn dport_bt_lpck_div_num(&mut self) -> DPORT_BT_LPCK_DIV_NUM_W {
        DPORT_BT_LPCK_DIV_NUM_W { w: self }
    }
}
