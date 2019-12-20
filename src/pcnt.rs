#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PCNT_U0_CONF0_REG"]
    pub pcnt_u0_conf0_reg: PCNT_U0_CONF0_REG,
    #[doc = "0x04 - PCNT_U0_CONF1_REG"]
    pub pcnt_u0_conf1_reg: PCNT_U0_CONF1_REG,
    #[doc = "0x08 - PCNT_U0_CONF2_REG"]
    pub pcnt_u0_conf2_reg: PCNT_U0_CONF2_REG,
    #[doc = "0x0c - PCNT_U1_CONF0_REG"]
    pub pcnt_u1_conf0_reg: PCNT_U1_CONF0_REG,
    #[doc = "0x10 - PCNT_U1_CONF1_REG"]
    pub pcnt_u1_conf1_reg: PCNT_U1_CONF1_REG,
    #[doc = "0x14 - PCNT_U1_CONF2_REG"]
    pub pcnt_u1_conf2_reg: PCNT_U1_CONF2_REG,
    #[doc = "0x18 - PCNT_U2_CONF0_REG"]
    pub pcnt_u2_conf0_reg: PCNT_U2_CONF0_REG,
    #[doc = "0x1c - PCNT_U2_CONF1_REG"]
    pub pcnt_u2_conf1_reg: PCNT_U2_CONF1_REG,
    #[doc = "0x20 - PCNT_U2_CONF2_REG"]
    pub pcnt_u2_conf2_reg: PCNT_U2_CONF2_REG,
    #[doc = "0x24 - PCNT_U3_CONF0_REG"]
    pub pcnt_u3_conf0_reg: PCNT_U3_CONF0_REG,
    #[doc = "0x28 - PCNT_U3_CONF1_REG"]
    pub pcnt_u3_conf1_reg: PCNT_U3_CONF1_REG,
    #[doc = "0x2c - PCNT_U3_CONF2_REG"]
    pub pcnt_u3_conf2_reg: PCNT_U3_CONF2_REG,
    #[doc = "0x30 - PCNT_U4_CONF0_REG"]
    pub pcnt_u4_conf0_reg: PCNT_U4_CONF0_REG,
    #[doc = "0x34 - PCNT_U4_CONF1_REG"]
    pub pcnt_u4_conf1_reg: PCNT_U4_CONF1_REG,
    #[doc = "0x38 - PCNT_U4_CONF2_REG"]
    pub pcnt_u4_conf2_reg: PCNT_U4_CONF2_REG,
    #[doc = "0x3c - PCNT_U5_CONF0_REG"]
    pub pcnt_u5_conf0_reg: PCNT_U5_CONF0_REG,
    #[doc = "0x40 - PCNT_U5_CONF1_REG"]
    pub pcnt_u5_conf1_reg: PCNT_U5_CONF1_REG,
    #[doc = "0x44 - PCNT_U5_CONF2_REG"]
    pub pcnt_u5_conf2_reg: PCNT_U5_CONF2_REG,
    #[doc = "0x48 - PCNT_U6_CONF0_REG"]
    pub pcnt_u6_conf0_reg: PCNT_U6_CONF0_REG,
    #[doc = "0x4c - PCNT_U6_CONF1_REG"]
    pub pcnt_u6_conf1_reg: PCNT_U6_CONF1_REG,
    #[doc = "0x50 - PCNT_U6_CONF2_REG"]
    pub pcnt_u6_conf2_reg: PCNT_U6_CONF2_REG,
    #[doc = "0x54 - PCNT_U7_CONF0_REG"]
    pub pcnt_u7_conf0_reg: PCNT_U7_CONF0_REG,
    #[doc = "0x58 - PCNT_U7_CONF1_REG"]
    pub pcnt_u7_conf1_reg: PCNT_U7_CONF1_REG,
    #[doc = "0x5c - PCNT_U7_CONF2_REG"]
    pub pcnt_u7_conf2_reg: PCNT_U7_CONF2_REG,
    #[doc = "0x60 - PCNT_U0_CNT_REG"]
    pub pcnt_u0_cnt_reg: PCNT_U0_CNT_REG,
    #[doc = "0x64 - PCNT_U1_CNT_REG"]
    pub pcnt_u1_cnt_reg: PCNT_U1_CNT_REG,
    #[doc = "0x68 - PCNT_U2_CNT_REG"]
    pub pcnt_u2_cnt_reg: PCNT_U2_CNT_REG,
    #[doc = "0x6c - PCNT_U3_CNT_REG"]
    pub pcnt_u3_cnt_reg: PCNT_U3_CNT_REG,
    #[doc = "0x70 - PCNT_U4_CNT_REG"]
    pub pcnt_u4_cnt_reg: PCNT_U4_CNT_REG,
    #[doc = "0x74 - PCNT_U5_CNT_REG"]
    pub pcnt_u5_cnt_reg: PCNT_U5_CNT_REG,
    #[doc = "0x78 - PCNT_U6_CNT_REG"]
    pub pcnt_u6_cnt_reg: PCNT_U6_CNT_REG,
    #[doc = "0x7c - PCNT_U7_CNT_REG"]
    pub pcnt_u7_cnt_reg: PCNT_U7_CNT_REG,
    #[doc = "0x80 - PCNT_INT_RAW_REG"]
    pub pcnt_int_raw_reg: PCNT_INT_RAW_REG,
    #[doc = "0x84 - PCNT_INT_ST_REG"]
    pub pcnt_int_st_reg: PCNT_INT_ST_REG,
    #[doc = "0x88 - PCNT_INT_ENA_REG"]
    pub pcnt_int_ena_reg: PCNT_INT_ENA_REG,
    #[doc = "0x8c - PCNT_INT_CLR_REG"]
    pub pcnt_int_clr_reg: PCNT_INT_CLR_REG,
    #[doc = "0x90 - PCNT_U0_STATUS_REG"]
    pub pcnt_u0_status_reg: PCNT_U0_STATUS_REG,
    #[doc = "0x94 - PCNT_U1_STATUS_REG"]
    pub pcnt_u1_status_reg: PCNT_U1_STATUS_REG,
    #[doc = "0x98 - PCNT_U2_STATUS_REG"]
    pub pcnt_u2_status_reg: PCNT_U2_STATUS_REG,
    #[doc = "0x9c - PCNT_U3_STATUS_REG"]
    pub pcnt_u3_status_reg: PCNT_U3_STATUS_REG,
    #[doc = "0xa0 - PCNT_U4_STATUS_REG"]
    pub pcnt_u4_status_reg: PCNT_U4_STATUS_REG,
    #[doc = "0xa4 - PCNT_U5_STATUS_REG"]
    pub pcnt_u5_status_reg: PCNT_U5_STATUS_REG,
    #[doc = "0xa8 - PCNT_U6_STATUS_REG"]
    pub pcnt_u6_status_reg: PCNT_U6_STATUS_REG,
    #[doc = "0xac - PCNT_U7_STATUS_REG"]
    pub pcnt_u7_status_reg: PCNT_U7_STATUS_REG,
    #[doc = "0xb0 - PCNT_CTRL_REG"]
    pub pcnt_ctrl_reg: PCNT_CTRL_REG,
    _reserved45: [u8; 72usize],
    #[doc = "0xfc - PCNT_DATE_REG"]
    pub pcnt_date_reg: PCNT_DATE_REG,
}
#[doc = "PCNT_U0_CONF0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcnt_u0_conf0_reg](pcnt_u0_conf0_reg) module"]
pub type PCNT_U0_CONF0_REG = crate::Reg<u32, _PCNT_U0_CONF0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCNT_U0_CONF0_REG;
#[doc = "`read()` method returns [pcnt_u0_conf0_reg::R](pcnt_u0_conf0_reg::R) reader structure"]
impl crate::Readable for PCNT_U0_CONF0_REG {}
#[doc = "`write(|w| ..)` method takes [pcnt_u0_conf0_reg::W](pcnt_u0_conf0_reg::W) writer structure"]
impl crate::Writable for PCNT_U0_CONF0_REG {}
#[doc = "PCNT_U0_CONF0_REG"]
pub mod pcnt_u0_conf0_reg;
#[doc = "PCNT_U0_CONF1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcnt_u0_conf1_reg](pcnt_u0_conf1_reg) module"]
pub type PCNT_U0_CONF1_REG = crate::Reg<u32, _PCNT_U0_CONF1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCNT_U0_CONF1_REG;
#[doc = "`read()` method returns [pcnt_u0_conf1_reg::R](pcnt_u0_conf1_reg::R) reader structure"]
impl crate::Readable for PCNT_U0_CONF1_REG {}
#[doc = "`write(|w| ..)` method takes [pcnt_u0_conf1_reg::W](pcnt_u0_conf1_reg::W) writer structure"]
impl crate::Writable for PCNT_U0_CONF1_REG {}
#[doc = "PCNT_U0_CONF1_REG"]
pub mod pcnt_u0_conf1_reg;
#[doc = "PCNT_U0_CONF2_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcnt_u0_conf2_reg](pcnt_u0_conf2_reg) module"]
pub type PCNT_U0_CONF2_REG = crate::Reg<u32, _PCNT_U0_CONF2_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCNT_U0_CONF2_REG;
#[doc = "`read()` method returns [pcnt_u0_conf2_reg::R](pcnt_u0_conf2_reg::R) reader structure"]
impl crate::Readable for PCNT_U0_CONF2_REG {}
#[doc = "`write(|w| ..)` method takes [pcnt_u0_conf2_reg::W](pcnt_u0_conf2_reg::W) writer structure"]
impl crate::Writable for PCNT_U0_CONF2_REG {}
#[doc = "PCNT_U0_CONF2_REG"]
pub mod pcnt_u0_conf2_reg;
#[doc = "PCNT_U1_CONF0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcnt_u1_conf0_reg](pcnt_u1_conf0_reg) module"]
pub type PCNT_U1_CONF0_REG = crate::Reg<u32, _PCNT_U1_CONF0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCNT_U1_CONF0_REG;
#[doc = "`read()` method returns [pcnt_u1_conf0_reg::R](pcnt_u1_conf0_reg::R) reader structure"]
impl crate::Readable for PCNT_U1_CONF0_REG {}
#[doc = "`write(|w| ..)` method takes [pcnt_u1_conf0_reg::W](pcnt_u1_conf0_reg::W) writer structure"]
impl crate::Writable for PCNT_U1_CONF0_REG {}
#[doc = "PCNT_U1_CONF0_REG"]
pub mod pcnt_u1_conf0_reg;
#[doc = "PCNT_U1_CONF1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcnt_u1_conf1_reg](pcnt_u1_conf1_reg) module"]
pub type PCNT_U1_CONF1_REG = crate::Reg<u32, _PCNT_U1_CONF1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCNT_U1_CONF1_REG;
#[doc = "`read()` method returns [pcnt_u1_conf1_reg::R](pcnt_u1_conf1_reg::R) reader structure"]
impl crate::Readable for PCNT_U1_CONF1_REG {}
#[doc = "`write(|w| ..)` method takes [pcnt_u1_conf1_reg::W](pcnt_u1_conf1_reg::W) writer structure"]
impl crate::Writable for PCNT_U1_CONF1_REG {}
#[doc = "PCNT_U1_CONF1_REG"]
pub mod pcnt_u1_conf1_reg;
#[doc = "PCNT_U1_CONF2_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcnt_u1_conf2_reg](pcnt_u1_conf2_reg) module"]
pub type PCNT_U1_CONF2_REG = crate::Reg<u32, _PCNT_U1_CONF2_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCNT_U1_CONF2_REG;
#[doc = "`read()` method returns [pcnt_u1_conf2_reg::R](pcnt_u1_conf2_reg::R) reader structure"]
impl crate::Readable for PCNT_U1_CONF2_REG {}
#[doc = "`write(|w| ..)` method takes [pcnt_u1_conf2_reg::W](pcnt_u1_conf2_reg::W) writer structure"]
impl crate::Writable for PCNT_U1_CONF2_REG {}
#[doc = "PCNT_U1_CONF2_REG"]
pub mod pcnt_u1_conf2_reg;
#[doc = "PCNT_U2_CONF0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcnt_u2_conf0_reg](pcnt_u2_conf0_reg) module"]
pub type PCNT_U2_CONF0_REG = crate::Reg<u32, _PCNT_U2_CONF0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCNT_U2_CONF0_REG;
#[doc = "`read()` method returns [pcnt_u2_conf0_reg::R](pcnt_u2_conf0_reg::R) reader structure"]
impl crate::Readable for PCNT_U2_CONF0_REG {}
#[doc = "`write(|w| ..)` method takes [pcnt_u2_conf0_reg::W](pcnt_u2_conf0_reg::W) writer structure"]
impl crate::Writable for PCNT_U2_CONF0_REG {}
#[doc = "PCNT_U2_CONF0_REG"]
pub mod pcnt_u2_conf0_reg;
#[doc = "PCNT_U2_CONF1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcnt_u2_conf1_reg](pcnt_u2_conf1_reg) module"]
pub type PCNT_U2_CONF1_REG = crate::Reg<u32, _PCNT_U2_CONF1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCNT_U2_CONF1_REG;
#[doc = "`read()` method returns [pcnt_u2_conf1_reg::R](pcnt_u2_conf1_reg::R) reader structure"]
impl crate::Readable for PCNT_U2_CONF1_REG {}
#[doc = "`write(|w| ..)` method takes [pcnt_u2_conf1_reg::W](pcnt_u2_conf1_reg::W) writer structure"]
impl crate::Writable for PCNT_U2_CONF1_REG {}
#[doc = "PCNT_U2_CONF1_REG"]
pub mod pcnt_u2_conf1_reg;
#[doc = "PCNT_U2_CONF2_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcnt_u2_conf2_reg](pcnt_u2_conf2_reg) module"]
pub type PCNT_U2_CONF2_REG = crate::Reg<u32, _PCNT_U2_CONF2_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCNT_U2_CONF2_REG;
#[doc = "`read()` method returns [pcnt_u2_conf2_reg::R](pcnt_u2_conf2_reg::R) reader structure"]
impl crate::Readable for PCNT_U2_CONF2_REG {}
#[doc = "`write(|w| ..)` method takes [pcnt_u2_conf2_reg::W](pcnt_u2_conf2_reg::W) writer structure"]
impl crate::Writable for PCNT_U2_CONF2_REG {}
#[doc = "PCNT_U2_CONF2_REG"]
pub mod pcnt_u2_conf2_reg;
#[doc = "PCNT_U3_CONF0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcnt_u3_conf0_reg](pcnt_u3_conf0_reg) module"]
pub type PCNT_U3_CONF0_REG = crate::Reg<u32, _PCNT_U3_CONF0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCNT_U3_CONF0_REG;
#[doc = "`read()` method returns [pcnt_u3_conf0_reg::R](pcnt_u3_conf0_reg::R) reader structure"]
impl crate::Readable for PCNT_U3_CONF0_REG {}
#[doc = "`write(|w| ..)` method takes [pcnt_u3_conf0_reg::W](pcnt_u3_conf0_reg::W) writer structure"]
impl crate::Writable for PCNT_U3_CONF0_REG {}
#[doc = "PCNT_U3_CONF0_REG"]
pub mod pcnt_u3_conf0_reg;
#[doc = "PCNT_U3_CONF1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcnt_u3_conf1_reg](pcnt_u3_conf1_reg) module"]
pub type PCNT_U3_CONF1_REG = crate::Reg<u32, _PCNT_U3_CONF1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCNT_U3_CONF1_REG;
#[doc = "`read()` method returns [pcnt_u3_conf1_reg::R](pcnt_u3_conf1_reg::R) reader structure"]
impl crate::Readable for PCNT_U3_CONF1_REG {}
#[doc = "`write(|w| ..)` method takes [pcnt_u3_conf1_reg::W](pcnt_u3_conf1_reg::W) writer structure"]
impl crate::Writable for PCNT_U3_CONF1_REG {}
#[doc = "PCNT_U3_CONF1_REG"]
pub mod pcnt_u3_conf1_reg;
#[doc = "PCNT_U3_CONF2_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcnt_u3_conf2_reg](pcnt_u3_conf2_reg) module"]
pub type PCNT_U3_CONF2_REG = crate::Reg<u32, _PCNT_U3_CONF2_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCNT_U3_CONF2_REG;
#[doc = "`read()` method returns [pcnt_u3_conf2_reg::R](pcnt_u3_conf2_reg::R) reader structure"]
impl crate::Readable for PCNT_U3_CONF2_REG {}
#[doc = "`write(|w| ..)` method takes [pcnt_u3_conf2_reg::W](pcnt_u3_conf2_reg::W) writer structure"]
impl crate::Writable for PCNT_U3_CONF2_REG {}
#[doc = "PCNT_U3_CONF2_REG"]
pub mod pcnt_u3_conf2_reg;
#[doc = "PCNT_U4_CONF0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcnt_u4_conf0_reg](pcnt_u4_conf0_reg) module"]
pub type PCNT_U4_CONF0_REG = crate::Reg<u32, _PCNT_U4_CONF0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCNT_U4_CONF0_REG;
#[doc = "`read()` method returns [pcnt_u4_conf0_reg::R](pcnt_u4_conf0_reg::R) reader structure"]
impl crate::Readable for PCNT_U4_CONF0_REG {}
#[doc = "`write(|w| ..)` method takes [pcnt_u4_conf0_reg::W](pcnt_u4_conf0_reg::W) writer structure"]
impl crate::Writable for PCNT_U4_CONF0_REG {}
#[doc = "PCNT_U4_CONF0_REG"]
pub mod pcnt_u4_conf0_reg;
#[doc = "PCNT_U4_CONF1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcnt_u4_conf1_reg](pcnt_u4_conf1_reg) module"]
pub type PCNT_U4_CONF1_REG = crate::Reg<u32, _PCNT_U4_CONF1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCNT_U4_CONF1_REG;
#[doc = "`read()` method returns [pcnt_u4_conf1_reg::R](pcnt_u4_conf1_reg::R) reader structure"]
impl crate::Readable for PCNT_U4_CONF1_REG {}
#[doc = "`write(|w| ..)` method takes [pcnt_u4_conf1_reg::W](pcnt_u4_conf1_reg::W) writer structure"]
impl crate::Writable for PCNT_U4_CONF1_REG {}
#[doc = "PCNT_U4_CONF1_REG"]
pub mod pcnt_u4_conf1_reg;
#[doc = "PCNT_U4_CONF2_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcnt_u4_conf2_reg](pcnt_u4_conf2_reg) module"]
pub type PCNT_U4_CONF2_REG = crate::Reg<u32, _PCNT_U4_CONF2_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCNT_U4_CONF2_REG;
#[doc = "`read()` method returns [pcnt_u4_conf2_reg::R](pcnt_u4_conf2_reg::R) reader structure"]
impl crate::Readable for PCNT_U4_CONF2_REG {}
#[doc = "`write(|w| ..)` method takes [pcnt_u4_conf2_reg::W](pcnt_u4_conf2_reg::W) writer structure"]
impl crate::Writable for PCNT_U4_CONF2_REG {}
#[doc = "PCNT_U4_CONF2_REG"]
pub mod pcnt_u4_conf2_reg;
#[doc = "PCNT_U5_CONF0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcnt_u5_conf0_reg](pcnt_u5_conf0_reg) module"]
pub type PCNT_U5_CONF0_REG = crate::Reg<u32, _PCNT_U5_CONF0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCNT_U5_CONF0_REG;
#[doc = "`read()` method returns [pcnt_u5_conf0_reg::R](pcnt_u5_conf0_reg::R) reader structure"]
impl crate::Readable for PCNT_U5_CONF0_REG {}
#[doc = "`write(|w| ..)` method takes [pcnt_u5_conf0_reg::W](pcnt_u5_conf0_reg::W) writer structure"]
impl crate::Writable for PCNT_U5_CONF0_REG {}
#[doc = "PCNT_U5_CONF0_REG"]
pub mod pcnt_u5_conf0_reg;
#[doc = "PCNT_U5_CONF1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcnt_u5_conf1_reg](pcnt_u5_conf1_reg) module"]
pub type PCNT_U5_CONF1_REG = crate::Reg<u32, _PCNT_U5_CONF1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCNT_U5_CONF1_REG;
#[doc = "`read()` method returns [pcnt_u5_conf1_reg::R](pcnt_u5_conf1_reg::R) reader structure"]
impl crate::Readable for PCNT_U5_CONF1_REG {}
#[doc = "`write(|w| ..)` method takes [pcnt_u5_conf1_reg::W](pcnt_u5_conf1_reg::W) writer structure"]
impl crate::Writable for PCNT_U5_CONF1_REG {}
#[doc = "PCNT_U5_CONF1_REG"]
pub mod pcnt_u5_conf1_reg;
#[doc = "PCNT_U5_CONF2_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcnt_u5_conf2_reg](pcnt_u5_conf2_reg) module"]
pub type PCNT_U5_CONF2_REG = crate::Reg<u32, _PCNT_U5_CONF2_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCNT_U5_CONF2_REG;
#[doc = "`read()` method returns [pcnt_u5_conf2_reg::R](pcnt_u5_conf2_reg::R) reader structure"]
impl crate::Readable for PCNT_U5_CONF2_REG {}
#[doc = "`write(|w| ..)` method takes [pcnt_u5_conf2_reg::W](pcnt_u5_conf2_reg::W) writer structure"]
impl crate::Writable for PCNT_U5_CONF2_REG {}
#[doc = "PCNT_U5_CONF2_REG"]
pub mod pcnt_u5_conf2_reg;
#[doc = "PCNT_U6_CONF0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcnt_u6_conf0_reg](pcnt_u6_conf0_reg) module"]
pub type PCNT_U6_CONF0_REG = crate::Reg<u32, _PCNT_U6_CONF0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCNT_U6_CONF0_REG;
#[doc = "`read()` method returns [pcnt_u6_conf0_reg::R](pcnt_u6_conf0_reg::R) reader structure"]
impl crate::Readable for PCNT_U6_CONF0_REG {}
#[doc = "`write(|w| ..)` method takes [pcnt_u6_conf0_reg::W](pcnt_u6_conf0_reg::W) writer structure"]
impl crate::Writable for PCNT_U6_CONF0_REG {}
#[doc = "PCNT_U6_CONF0_REG"]
pub mod pcnt_u6_conf0_reg;
#[doc = "PCNT_U6_CONF1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcnt_u6_conf1_reg](pcnt_u6_conf1_reg) module"]
pub type PCNT_U6_CONF1_REG = crate::Reg<u32, _PCNT_U6_CONF1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCNT_U6_CONF1_REG;
#[doc = "`read()` method returns [pcnt_u6_conf1_reg::R](pcnt_u6_conf1_reg::R) reader structure"]
impl crate::Readable for PCNT_U6_CONF1_REG {}
#[doc = "`write(|w| ..)` method takes [pcnt_u6_conf1_reg::W](pcnt_u6_conf1_reg::W) writer structure"]
impl crate::Writable for PCNT_U6_CONF1_REG {}
#[doc = "PCNT_U6_CONF1_REG"]
pub mod pcnt_u6_conf1_reg;
#[doc = "PCNT_U6_CONF2_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcnt_u6_conf2_reg](pcnt_u6_conf2_reg) module"]
pub type PCNT_U6_CONF2_REG = crate::Reg<u32, _PCNT_U6_CONF2_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCNT_U6_CONF2_REG;
#[doc = "`read()` method returns [pcnt_u6_conf2_reg::R](pcnt_u6_conf2_reg::R) reader structure"]
impl crate::Readable for PCNT_U6_CONF2_REG {}
#[doc = "`write(|w| ..)` method takes [pcnt_u6_conf2_reg::W](pcnt_u6_conf2_reg::W) writer structure"]
impl crate::Writable for PCNT_U6_CONF2_REG {}
#[doc = "PCNT_U6_CONF2_REG"]
pub mod pcnt_u6_conf2_reg;
#[doc = "PCNT_U7_CONF0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcnt_u7_conf0_reg](pcnt_u7_conf0_reg) module"]
pub type PCNT_U7_CONF0_REG = crate::Reg<u32, _PCNT_U7_CONF0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCNT_U7_CONF0_REG;
#[doc = "`read()` method returns [pcnt_u7_conf0_reg::R](pcnt_u7_conf0_reg::R) reader structure"]
impl crate::Readable for PCNT_U7_CONF0_REG {}
#[doc = "`write(|w| ..)` method takes [pcnt_u7_conf0_reg::W](pcnt_u7_conf0_reg::W) writer structure"]
impl crate::Writable for PCNT_U7_CONF0_REG {}
#[doc = "PCNT_U7_CONF0_REG"]
pub mod pcnt_u7_conf0_reg;
#[doc = "PCNT_U7_CONF1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcnt_u7_conf1_reg](pcnt_u7_conf1_reg) module"]
pub type PCNT_U7_CONF1_REG = crate::Reg<u32, _PCNT_U7_CONF1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCNT_U7_CONF1_REG;
#[doc = "`read()` method returns [pcnt_u7_conf1_reg::R](pcnt_u7_conf1_reg::R) reader structure"]
impl crate::Readable for PCNT_U7_CONF1_REG {}
#[doc = "`write(|w| ..)` method takes [pcnt_u7_conf1_reg::W](pcnt_u7_conf1_reg::W) writer structure"]
impl crate::Writable for PCNT_U7_CONF1_REG {}
#[doc = "PCNT_U7_CONF1_REG"]
pub mod pcnt_u7_conf1_reg;
#[doc = "PCNT_U7_CONF2_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcnt_u7_conf2_reg](pcnt_u7_conf2_reg) module"]
pub type PCNT_U7_CONF2_REG = crate::Reg<u32, _PCNT_U7_CONF2_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCNT_U7_CONF2_REG;
#[doc = "`read()` method returns [pcnt_u7_conf2_reg::R](pcnt_u7_conf2_reg::R) reader structure"]
impl crate::Readable for PCNT_U7_CONF2_REG {}
#[doc = "`write(|w| ..)` method takes [pcnt_u7_conf2_reg::W](pcnt_u7_conf2_reg::W) writer structure"]
impl crate::Writable for PCNT_U7_CONF2_REG {}
#[doc = "PCNT_U7_CONF2_REG"]
pub mod pcnt_u7_conf2_reg;
#[doc = "PCNT_U0_CNT_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcnt_u0_cnt_reg](pcnt_u0_cnt_reg) module"]
pub type PCNT_U0_CNT_REG = crate::Reg<u32, _PCNT_U0_CNT_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCNT_U0_CNT_REG;
#[doc = "`read()` method returns [pcnt_u0_cnt_reg::R](pcnt_u0_cnt_reg::R) reader structure"]
impl crate::Readable for PCNT_U0_CNT_REG {}
#[doc = "`write(|w| ..)` method takes [pcnt_u0_cnt_reg::W](pcnt_u0_cnt_reg::W) writer structure"]
impl crate::Writable for PCNT_U0_CNT_REG {}
#[doc = "PCNT_U0_CNT_REG"]
pub mod pcnt_u0_cnt_reg;
#[doc = "PCNT_U1_CNT_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcnt_u1_cnt_reg](pcnt_u1_cnt_reg) module"]
pub type PCNT_U1_CNT_REG = crate::Reg<u32, _PCNT_U1_CNT_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCNT_U1_CNT_REG;
#[doc = "`read()` method returns [pcnt_u1_cnt_reg::R](pcnt_u1_cnt_reg::R) reader structure"]
impl crate::Readable for PCNT_U1_CNT_REG {}
#[doc = "`write(|w| ..)` method takes [pcnt_u1_cnt_reg::W](pcnt_u1_cnt_reg::W) writer structure"]
impl crate::Writable for PCNT_U1_CNT_REG {}
#[doc = "PCNT_U1_CNT_REG"]
pub mod pcnt_u1_cnt_reg;
#[doc = "PCNT_U2_CNT_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcnt_u2_cnt_reg](pcnt_u2_cnt_reg) module"]
pub type PCNT_U2_CNT_REG = crate::Reg<u32, _PCNT_U2_CNT_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCNT_U2_CNT_REG;
#[doc = "`read()` method returns [pcnt_u2_cnt_reg::R](pcnt_u2_cnt_reg::R) reader structure"]
impl crate::Readable for PCNT_U2_CNT_REG {}
#[doc = "`write(|w| ..)` method takes [pcnt_u2_cnt_reg::W](pcnt_u2_cnt_reg::W) writer structure"]
impl crate::Writable for PCNT_U2_CNT_REG {}
#[doc = "PCNT_U2_CNT_REG"]
pub mod pcnt_u2_cnt_reg;
#[doc = "PCNT_U3_CNT_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcnt_u3_cnt_reg](pcnt_u3_cnt_reg) module"]
pub type PCNT_U3_CNT_REG = crate::Reg<u32, _PCNT_U3_CNT_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCNT_U3_CNT_REG;
#[doc = "`read()` method returns [pcnt_u3_cnt_reg::R](pcnt_u3_cnt_reg::R) reader structure"]
impl crate::Readable for PCNT_U3_CNT_REG {}
#[doc = "`write(|w| ..)` method takes [pcnt_u3_cnt_reg::W](pcnt_u3_cnt_reg::W) writer structure"]
impl crate::Writable for PCNT_U3_CNT_REG {}
#[doc = "PCNT_U3_CNT_REG"]
pub mod pcnt_u3_cnt_reg;
#[doc = "PCNT_U4_CNT_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcnt_u4_cnt_reg](pcnt_u4_cnt_reg) module"]
pub type PCNT_U4_CNT_REG = crate::Reg<u32, _PCNT_U4_CNT_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCNT_U4_CNT_REG;
#[doc = "`read()` method returns [pcnt_u4_cnt_reg::R](pcnt_u4_cnt_reg::R) reader structure"]
impl crate::Readable for PCNT_U4_CNT_REG {}
#[doc = "`write(|w| ..)` method takes [pcnt_u4_cnt_reg::W](pcnt_u4_cnt_reg::W) writer structure"]
impl crate::Writable for PCNT_U4_CNT_REG {}
#[doc = "PCNT_U4_CNT_REG"]
pub mod pcnt_u4_cnt_reg;
#[doc = "PCNT_U5_CNT_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcnt_u5_cnt_reg](pcnt_u5_cnt_reg) module"]
pub type PCNT_U5_CNT_REG = crate::Reg<u32, _PCNT_U5_CNT_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCNT_U5_CNT_REG;
#[doc = "`read()` method returns [pcnt_u5_cnt_reg::R](pcnt_u5_cnt_reg::R) reader structure"]
impl crate::Readable for PCNT_U5_CNT_REG {}
#[doc = "`write(|w| ..)` method takes [pcnt_u5_cnt_reg::W](pcnt_u5_cnt_reg::W) writer structure"]
impl crate::Writable for PCNT_U5_CNT_REG {}
#[doc = "PCNT_U5_CNT_REG"]
pub mod pcnt_u5_cnt_reg;
#[doc = "PCNT_U6_CNT_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcnt_u6_cnt_reg](pcnt_u6_cnt_reg) module"]
pub type PCNT_U6_CNT_REG = crate::Reg<u32, _PCNT_U6_CNT_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCNT_U6_CNT_REG;
#[doc = "`read()` method returns [pcnt_u6_cnt_reg::R](pcnt_u6_cnt_reg::R) reader structure"]
impl crate::Readable for PCNT_U6_CNT_REG {}
#[doc = "`write(|w| ..)` method takes [pcnt_u6_cnt_reg::W](pcnt_u6_cnt_reg::W) writer structure"]
impl crate::Writable for PCNT_U6_CNT_REG {}
#[doc = "PCNT_U6_CNT_REG"]
pub mod pcnt_u6_cnt_reg;
#[doc = "PCNT_U7_CNT_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcnt_u7_cnt_reg](pcnt_u7_cnt_reg) module"]
pub type PCNT_U7_CNT_REG = crate::Reg<u32, _PCNT_U7_CNT_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCNT_U7_CNT_REG;
#[doc = "`read()` method returns [pcnt_u7_cnt_reg::R](pcnt_u7_cnt_reg::R) reader structure"]
impl crate::Readable for PCNT_U7_CNT_REG {}
#[doc = "`write(|w| ..)` method takes [pcnt_u7_cnt_reg::W](pcnt_u7_cnt_reg::W) writer structure"]
impl crate::Writable for PCNT_U7_CNT_REG {}
#[doc = "PCNT_U7_CNT_REG"]
pub mod pcnt_u7_cnt_reg;
#[doc = "PCNT_INT_RAW_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcnt_int_raw_reg](pcnt_int_raw_reg) module"]
pub type PCNT_INT_RAW_REG = crate::Reg<u32, _PCNT_INT_RAW_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCNT_INT_RAW_REG;
#[doc = "`read()` method returns [pcnt_int_raw_reg::R](pcnt_int_raw_reg::R) reader structure"]
impl crate::Readable for PCNT_INT_RAW_REG {}
#[doc = "`write(|w| ..)` method takes [pcnt_int_raw_reg::W](pcnt_int_raw_reg::W) writer structure"]
impl crate::Writable for PCNT_INT_RAW_REG {}
#[doc = "PCNT_INT_RAW_REG"]
pub mod pcnt_int_raw_reg;
#[doc = "PCNT_INT_ST_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcnt_int_st_reg](pcnt_int_st_reg) module"]
pub type PCNT_INT_ST_REG = crate::Reg<u32, _PCNT_INT_ST_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCNT_INT_ST_REG;
#[doc = "`read()` method returns [pcnt_int_st_reg::R](pcnt_int_st_reg::R) reader structure"]
impl crate::Readable for PCNT_INT_ST_REG {}
#[doc = "`write(|w| ..)` method takes [pcnt_int_st_reg::W](pcnt_int_st_reg::W) writer structure"]
impl crate::Writable for PCNT_INT_ST_REG {}
#[doc = "PCNT_INT_ST_REG"]
pub mod pcnt_int_st_reg;
#[doc = "PCNT_INT_ENA_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcnt_int_ena_reg](pcnt_int_ena_reg) module"]
pub type PCNT_INT_ENA_REG = crate::Reg<u32, _PCNT_INT_ENA_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCNT_INT_ENA_REG;
#[doc = "`read()` method returns [pcnt_int_ena_reg::R](pcnt_int_ena_reg::R) reader structure"]
impl crate::Readable for PCNT_INT_ENA_REG {}
#[doc = "`write(|w| ..)` method takes [pcnt_int_ena_reg::W](pcnt_int_ena_reg::W) writer structure"]
impl crate::Writable for PCNT_INT_ENA_REG {}
#[doc = "PCNT_INT_ENA_REG"]
pub mod pcnt_int_ena_reg;
#[doc = "PCNT_INT_CLR_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcnt_int_clr_reg](pcnt_int_clr_reg) module"]
pub type PCNT_INT_CLR_REG = crate::Reg<u32, _PCNT_INT_CLR_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCNT_INT_CLR_REG;
#[doc = "`read()` method returns [pcnt_int_clr_reg::R](pcnt_int_clr_reg::R) reader structure"]
impl crate::Readable for PCNT_INT_CLR_REG {}
#[doc = "`write(|w| ..)` method takes [pcnt_int_clr_reg::W](pcnt_int_clr_reg::W) writer structure"]
impl crate::Writable for PCNT_INT_CLR_REG {}
#[doc = "PCNT_INT_CLR_REG"]
pub mod pcnt_int_clr_reg;
#[doc = "PCNT_U0_STATUS_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcnt_u0_status_reg](pcnt_u0_status_reg) module"]
pub type PCNT_U0_STATUS_REG = crate::Reg<u32, _PCNT_U0_STATUS_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCNT_U0_STATUS_REG;
#[doc = "`read()` method returns [pcnt_u0_status_reg::R](pcnt_u0_status_reg::R) reader structure"]
impl crate::Readable for PCNT_U0_STATUS_REG {}
#[doc = "`write(|w| ..)` method takes [pcnt_u0_status_reg::W](pcnt_u0_status_reg::W) writer structure"]
impl crate::Writable for PCNT_U0_STATUS_REG {}
#[doc = "PCNT_U0_STATUS_REG"]
pub mod pcnt_u0_status_reg;
#[doc = "PCNT_U1_STATUS_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcnt_u1_status_reg](pcnt_u1_status_reg) module"]
pub type PCNT_U1_STATUS_REG = crate::Reg<u32, _PCNT_U1_STATUS_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCNT_U1_STATUS_REG;
#[doc = "`read()` method returns [pcnt_u1_status_reg::R](pcnt_u1_status_reg::R) reader structure"]
impl crate::Readable for PCNT_U1_STATUS_REG {}
#[doc = "`write(|w| ..)` method takes [pcnt_u1_status_reg::W](pcnt_u1_status_reg::W) writer structure"]
impl crate::Writable for PCNT_U1_STATUS_REG {}
#[doc = "PCNT_U1_STATUS_REG"]
pub mod pcnt_u1_status_reg;
#[doc = "PCNT_U2_STATUS_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcnt_u2_status_reg](pcnt_u2_status_reg) module"]
pub type PCNT_U2_STATUS_REG = crate::Reg<u32, _PCNT_U2_STATUS_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCNT_U2_STATUS_REG;
#[doc = "`read()` method returns [pcnt_u2_status_reg::R](pcnt_u2_status_reg::R) reader structure"]
impl crate::Readable for PCNT_U2_STATUS_REG {}
#[doc = "`write(|w| ..)` method takes [pcnt_u2_status_reg::W](pcnt_u2_status_reg::W) writer structure"]
impl crate::Writable for PCNT_U2_STATUS_REG {}
#[doc = "PCNT_U2_STATUS_REG"]
pub mod pcnt_u2_status_reg;
#[doc = "PCNT_U3_STATUS_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcnt_u3_status_reg](pcnt_u3_status_reg) module"]
pub type PCNT_U3_STATUS_REG = crate::Reg<u32, _PCNT_U3_STATUS_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCNT_U3_STATUS_REG;
#[doc = "`read()` method returns [pcnt_u3_status_reg::R](pcnt_u3_status_reg::R) reader structure"]
impl crate::Readable for PCNT_U3_STATUS_REG {}
#[doc = "`write(|w| ..)` method takes [pcnt_u3_status_reg::W](pcnt_u3_status_reg::W) writer structure"]
impl crate::Writable for PCNT_U3_STATUS_REG {}
#[doc = "PCNT_U3_STATUS_REG"]
pub mod pcnt_u3_status_reg;
#[doc = "PCNT_U4_STATUS_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcnt_u4_status_reg](pcnt_u4_status_reg) module"]
pub type PCNT_U4_STATUS_REG = crate::Reg<u32, _PCNT_U4_STATUS_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCNT_U4_STATUS_REG;
#[doc = "`read()` method returns [pcnt_u4_status_reg::R](pcnt_u4_status_reg::R) reader structure"]
impl crate::Readable for PCNT_U4_STATUS_REG {}
#[doc = "`write(|w| ..)` method takes [pcnt_u4_status_reg::W](pcnt_u4_status_reg::W) writer structure"]
impl crate::Writable for PCNT_U4_STATUS_REG {}
#[doc = "PCNT_U4_STATUS_REG"]
pub mod pcnt_u4_status_reg;
#[doc = "PCNT_U5_STATUS_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcnt_u5_status_reg](pcnt_u5_status_reg) module"]
pub type PCNT_U5_STATUS_REG = crate::Reg<u32, _PCNT_U5_STATUS_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCNT_U5_STATUS_REG;
#[doc = "`read()` method returns [pcnt_u5_status_reg::R](pcnt_u5_status_reg::R) reader structure"]
impl crate::Readable for PCNT_U5_STATUS_REG {}
#[doc = "`write(|w| ..)` method takes [pcnt_u5_status_reg::W](pcnt_u5_status_reg::W) writer structure"]
impl crate::Writable for PCNT_U5_STATUS_REG {}
#[doc = "PCNT_U5_STATUS_REG"]
pub mod pcnt_u5_status_reg;
#[doc = "PCNT_U6_STATUS_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcnt_u6_status_reg](pcnt_u6_status_reg) module"]
pub type PCNT_U6_STATUS_REG = crate::Reg<u32, _PCNT_U6_STATUS_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCNT_U6_STATUS_REG;
#[doc = "`read()` method returns [pcnt_u6_status_reg::R](pcnt_u6_status_reg::R) reader structure"]
impl crate::Readable for PCNT_U6_STATUS_REG {}
#[doc = "`write(|w| ..)` method takes [pcnt_u6_status_reg::W](pcnt_u6_status_reg::W) writer structure"]
impl crate::Writable for PCNT_U6_STATUS_REG {}
#[doc = "PCNT_U6_STATUS_REG"]
pub mod pcnt_u6_status_reg;
#[doc = "PCNT_U7_STATUS_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcnt_u7_status_reg](pcnt_u7_status_reg) module"]
pub type PCNT_U7_STATUS_REG = crate::Reg<u32, _PCNT_U7_STATUS_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCNT_U7_STATUS_REG;
#[doc = "`read()` method returns [pcnt_u7_status_reg::R](pcnt_u7_status_reg::R) reader structure"]
impl crate::Readable for PCNT_U7_STATUS_REG {}
#[doc = "`write(|w| ..)` method takes [pcnt_u7_status_reg::W](pcnt_u7_status_reg::W) writer structure"]
impl crate::Writable for PCNT_U7_STATUS_REG {}
#[doc = "PCNT_U7_STATUS_REG"]
pub mod pcnt_u7_status_reg;
#[doc = "PCNT_CTRL_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcnt_ctrl_reg](pcnt_ctrl_reg) module"]
pub type PCNT_CTRL_REG = crate::Reg<u32, _PCNT_CTRL_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCNT_CTRL_REG;
#[doc = "`read()` method returns [pcnt_ctrl_reg::R](pcnt_ctrl_reg::R) reader structure"]
impl crate::Readable for PCNT_CTRL_REG {}
#[doc = "`write(|w| ..)` method takes [pcnt_ctrl_reg::W](pcnt_ctrl_reg::W) writer structure"]
impl crate::Writable for PCNT_CTRL_REG {}
#[doc = "PCNT_CTRL_REG"]
pub mod pcnt_ctrl_reg;
#[doc = "PCNT_DATE_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcnt_date_reg](pcnt_date_reg) module"]
pub type PCNT_DATE_REG = crate::Reg<u32, _PCNT_DATE_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCNT_DATE_REG;
#[doc = "`read()` method returns [pcnt_date_reg::R](pcnt_date_reg::R) reader structure"]
impl crate::Readable for PCNT_DATE_REG {}
#[doc = "`write(|w| ..)` method takes [pcnt_date_reg::W](pcnt_date_reg::W) writer structure"]
impl crate::Writable for PCNT_DATE_REG {}
#[doc = "PCNT_DATE_REG"]
pub mod pcnt_date_reg;