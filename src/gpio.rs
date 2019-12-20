#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIO_BT_SELECT_REG"]
    pub gpio_bt_select_reg: GPIO_BT_SELECT_REG,
    #[doc = "0x04 - GPIO_OUT_REG"]
    pub gpio_out_reg: GPIO_OUT_REG,
    #[doc = "0x08 - GPIO_OUT_W1TS_REG"]
    pub gpio_out_w1ts_reg: GPIO_OUT_W1TS_REG,
    #[doc = "0x0c - GPIO_OUT_W1TC_REG"]
    pub gpio_out_w1tc_reg: GPIO_OUT_W1TC_REG,
    #[doc = "0x10 - GPIO_OUT1_REG"]
    pub gpio_out1_reg: GPIO_OUT1_REG,
    #[doc = "0x14 - GPIO_OUT1_W1TS_REG"]
    pub gpio_out1_w1ts_reg: GPIO_OUT1_W1TS_REG,
    #[doc = "0x18 - GPIO_OUT1_W1TC_REG"]
    pub gpio_out1_w1tc_reg: GPIO_OUT1_W1TC_REG,
    #[doc = "0x1c - GPIO_SDIO_SELECT_REG"]
    pub gpio_sdio_select_reg: GPIO_SDIO_SELECT_REG,
    #[doc = "0x20 - GPIO_ENABLE_REG"]
    pub gpio_enable_reg: GPIO_ENABLE_REG,
    #[doc = "0x24 - GPIO_ENABLE_W1TS_REG"]
    pub gpio_enable_w1ts_reg: GPIO_ENABLE_W1TS_REG,
    #[doc = "0x28 - GPIO_ENABLE_W1TC_REG"]
    pub gpio_enable_w1tc_reg: GPIO_ENABLE_W1TC_REG,
    #[doc = "0x2c - GPIO_ENABLE1_REG"]
    pub gpio_enable1_reg: GPIO_ENABLE1_REG,
    #[doc = "0x30 - GPIO_ENABLE1_W1TS_REG"]
    pub gpio_enable1_w1ts_reg: GPIO_ENABLE1_W1TS_REG,
    #[doc = "0x34 - GPIO_ENABLE1_W1TC_REG"]
    pub gpio_enable1_w1tc_reg: GPIO_ENABLE1_W1TC_REG,
    #[doc = "0x38 - GPIO_STRAP_REG"]
    pub gpio_strap_reg: GPIO_STRAP_REG,
    #[doc = "0x3c - GPIO_IN_REG"]
    pub gpio_in_reg: GPIO_IN_REG,
    #[doc = "0x40 - GPIO_IN1_REG"]
    pub gpio_in1_reg: GPIO_IN1_REG,
    #[doc = "0x44 - GPIO_STATUS_REG"]
    pub gpio_status_reg: GPIO_STATUS_REG,
    #[doc = "0x48 - GPIO_STATUS_W1TS_REG"]
    pub gpio_status_w1ts_reg: GPIO_STATUS_W1TS_REG,
    #[doc = "0x4c - GPIO_STATUS_W1TC_REG"]
    pub gpio_status_w1tc_reg: GPIO_STATUS_W1TC_REG,
    #[doc = "0x50 - GPIO_STATUS1_REG"]
    pub gpio_status1_reg: GPIO_STATUS1_REG,
    #[doc = "0x54 - GPIO_STATUS1_W1TS_REG"]
    pub gpio_status1_w1ts_reg: GPIO_STATUS1_W1TS_REG,
    #[doc = "0x58 - GPIO_STATUS1_W1TC_REG"]
    pub gpio_status1_w1tc_reg: GPIO_STATUS1_W1TC_REG,
    _reserved23: [u8; 4usize],
    #[doc = "0x60 - GPIO_ACPU_INT_REG"]
    pub gpio_acpu_int_reg: GPIO_ACPU_INT_REG,
    #[doc = "0x64 - GPIO_ACPU_NMI_INT_REG"]
    pub gpio_acpu_nmi_int_reg: GPIO_ACPU_NMI_INT_REG,
    #[doc = "0x68 - GPIO_PCPU_INT_REG"]
    pub gpio_pcpu_int_reg: GPIO_PCPU_INT_REG,
    #[doc = "0x6c - GPIO_PCPU_NMI_INT_REG"]
    pub gpio_pcpu_nmi_int_reg: GPIO_PCPU_NMI_INT_REG,
    #[doc = "0x70 - GPIO_CPUSDIO_INT_REG"]
    pub gpio_cpusdio_int_reg: GPIO_CPUSDIO_INT_REG,
    #[doc = "0x74 - GPIO_ACPU_INT1_REG"]
    pub gpio_acpu_int1_reg: GPIO_ACPU_INT1_REG,
    #[doc = "0x78 - GPIO_ACPU_NMI_INT1_REG"]
    pub gpio_acpu_nmi_int1_reg: GPIO_ACPU_NMI_INT1_REG,
    #[doc = "0x7c - GPIO_PCPU_INT1_REG"]
    pub gpio_pcpu_int1_reg: GPIO_PCPU_INT1_REG,
    #[doc = "0x80 - GPIO_PCPU_NMI_INT1_REG"]
    pub gpio_pcpu_nmi_int1_reg: GPIO_PCPU_NMI_INT1_REG,
    #[doc = "0x84 - GPIO_CPUSDIO_INT1_REG"]
    pub gpio_cpusdio_int1_reg: GPIO_CPUSDIO_INT1_REG,
    #[doc = "0x88 - GPIO_PIN0_REG"]
    pub gpio_pin0_reg: GPIO_PIN0_REG,
    #[doc = "0x8c - GPIO_PIN1_REG"]
    pub gpio_pin1_reg: GPIO_PIN1_REG,
    #[doc = "0x90 - GPIO_PIN2_REG"]
    pub gpio_pin2_reg: GPIO_PIN2_REG,
    #[doc = "0x94 - GPIO_PIN3_REG"]
    pub gpio_pin3_reg: GPIO_PIN3_REG,
    #[doc = "0x98 - GPIO_PIN4_REG"]
    pub gpio_pin4_reg: GPIO_PIN4_REG,
    #[doc = "0x9c - GPIO_PIN5_REG"]
    pub gpio_pin5_reg: GPIO_PIN5_REG,
    #[doc = "0xa0 - GPIO_PIN6_REG"]
    pub gpio_pin6_reg: GPIO_PIN6_REG,
    #[doc = "0xa4 - GPIO_PIN7_REG"]
    pub gpio_pin7_reg: GPIO_PIN7_REG,
    #[doc = "0xa8 - GPIO_PIN8_REG"]
    pub gpio_pin8_reg: GPIO_PIN8_REG,
    #[doc = "0xac - GPIO_PIN9_REG"]
    pub gpio_pin9_reg: GPIO_PIN9_REG,
    #[doc = "0xb0 - GPIO_PIN10_REG"]
    pub gpio_pin10_reg: GPIO_PIN10_REG,
    #[doc = "0xb4 - GPIO_PIN11_REG"]
    pub gpio_pin11_reg: GPIO_PIN11_REG,
    #[doc = "0xb8 - GPIO_PIN12_REG"]
    pub gpio_pin12_reg: GPIO_PIN12_REG,
    #[doc = "0xbc - GPIO_PIN13_REG"]
    pub gpio_pin13_reg: GPIO_PIN13_REG,
    #[doc = "0xc0 - GPIO_PIN14_REG"]
    pub gpio_pin14_reg: GPIO_PIN14_REG,
    #[doc = "0xc4 - GPIO_PIN15_REG"]
    pub gpio_pin15_reg: GPIO_PIN15_REG,
    #[doc = "0xc8 - GPIO_PIN16_REG"]
    pub gpio_pin16_reg: GPIO_PIN16_REG,
    #[doc = "0xcc - GPIO_PIN17_REG"]
    pub gpio_pin17_reg: GPIO_PIN17_REG,
    #[doc = "0xd0 - GPIO_PIN18_REG"]
    pub gpio_pin18_reg: GPIO_PIN18_REG,
    #[doc = "0xd4 - GPIO_PIN19_REG"]
    pub gpio_pin19_reg: GPIO_PIN19_REG,
    #[doc = "0xd8 - GPIO_PIN20_REG"]
    pub gpio_pin20_reg: GPIO_PIN20_REG,
    #[doc = "0xdc - GPIO_PIN21_REG"]
    pub gpio_pin21_reg: GPIO_PIN21_REG,
    #[doc = "0xe0 - GPIO_PIN22_REG"]
    pub gpio_pin22_reg: GPIO_PIN22_REG,
    #[doc = "0xe4 - GPIO_PIN23_REG"]
    pub gpio_pin23_reg: GPIO_PIN23_REG,
    #[doc = "0xe8 - GPIO_PIN24_REG"]
    pub gpio_pin24_reg: GPIO_PIN24_REG,
    #[doc = "0xec - GPIO_PIN25_REG"]
    pub gpio_pin25_reg: GPIO_PIN25_REG,
    #[doc = "0xf0 - GPIO_PIN26_REG"]
    pub gpio_pin26_reg: GPIO_PIN26_REG,
    #[doc = "0xf4 - GPIO_PIN27_REG"]
    pub gpio_pin27_reg: GPIO_PIN27_REG,
    #[doc = "0xf8 - GPIO_PIN28_REG"]
    pub gpio_pin28_reg: GPIO_PIN28_REG,
    #[doc = "0xfc - GPIO_PIN29_REG"]
    pub gpio_pin29_reg: GPIO_PIN29_REG,
    #[doc = "0x100 - GPIO_PIN30_REG"]
    pub gpio_pin30_reg: GPIO_PIN30_REG,
    #[doc = "0x104 - GPIO_PIN31_REG"]
    pub gpio_pin31_reg: GPIO_PIN31_REG,
    #[doc = "0x108 - GPIO_PIN32_REG"]
    pub gpio_pin32_reg: GPIO_PIN32_REG,
    #[doc = "0x10c - GPIO_PIN33_REG"]
    pub gpio_pin33_reg: GPIO_PIN33_REG,
    #[doc = "0x110 - GPIO_PIN34_REG"]
    pub gpio_pin34_reg: GPIO_PIN34_REG,
    #[doc = "0x114 - GPIO_PIN35_REG"]
    pub gpio_pin35_reg: GPIO_PIN35_REG,
    #[doc = "0x118 - GPIO_PIN36_REG"]
    pub gpio_pin36_reg: GPIO_PIN36_REG,
    #[doc = "0x11c - GPIO_PIN37_REG"]
    pub gpio_pin37_reg: GPIO_PIN37_REG,
    #[doc = "0x120 - GPIO_PIN38_REG"]
    pub gpio_pin38_reg: GPIO_PIN38_REG,
    #[doc = "0x124 - GPIO_PIN39_REG"]
    pub gpio_pin39_reg: GPIO_PIN39_REG,
    #[doc = "0x128 - GPIO_cali_conf_REG"]
    pub gpio_cali_conf_reg: GPIO_CALI_CONF_REG,
    #[doc = "0x12c - GPIO_cali_data_REG"]
    pub gpio_cali_data_reg: GPIO_CALI_DATA_REG,
    #[doc = "0x130 - GPIO_FUNC0_IN_SEL_CFG_REG"]
    pub gpio_func0_in_sel_cfg_reg: GPIO_FUNC0_IN_SEL_CFG_REG,
    #[doc = "0x134 - GPIO_FUNC1_IN_SEL_CFG_REG"]
    pub gpio_func1_in_sel_cfg_reg: GPIO_FUNC1_IN_SEL_CFG_REG,
    #[doc = "0x138 - GPIO_FUNC2_IN_SEL_CFG_REG"]
    pub gpio_func2_in_sel_cfg_reg: GPIO_FUNC2_IN_SEL_CFG_REG,
    #[doc = "0x13c - GPIO_FUNC3_IN_SEL_CFG_REG"]
    pub gpio_func3_in_sel_cfg_reg: GPIO_FUNC3_IN_SEL_CFG_REG,
    #[doc = "0x140 - GPIO_FUNC4_IN_SEL_CFG_REG"]
    pub gpio_func4_in_sel_cfg_reg: GPIO_FUNC4_IN_SEL_CFG_REG,
    #[doc = "0x144 - GPIO_FUNC5_IN_SEL_CFG_REG"]
    pub gpio_func5_in_sel_cfg_reg: GPIO_FUNC5_IN_SEL_CFG_REG,
    #[doc = "0x148 - GPIO_FUNC6_IN_SEL_CFG_REG"]
    pub gpio_func6_in_sel_cfg_reg: GPIO_FUNC6_IN_SEL_CFG_REG,
    #[doc = "0x14c - GPIO_FUNC7_IN_SEL_CFG_REG"]
    pub gpio_func7_in_sel_cfg_reg: GPIO_FUNC7_IN_SEL_CFG_REG,
    #[doc = "0x150 - GPIO_FUNC8_IN_SEL_CFG_REG"]
    pub gpio_func8_in_sel_cfg_reg: GPIO_FUNC8_IN_SEL_CFG_REG,
    #[doc = "0x154 - GPIO_FUNC9_IN_SEL_CFG_REG"]
    pub gpio_func9_in_sel_cfg_reg: GPIO_FUNC9_IN_SEL_CFG_REG,
    #[doc = "0x158 - GPIO_FUNC10_IN_SEL_CFG_REG"]
    pub gpio_func10_in_sel_cfg_reg: GPIO_FUNC10_IN_SEL_CFG_REG,
    #[doc = "0x15c - GPIO_FUNC11_IN_SEL_CFG_REG"]
    pub gpio_func11_in_sel_cfg_reg: GPIO_FUNC11_IN_SEL_CFG_REG,
    #[doc = "0x160 - GPIO_FUNC12_IN_SEL_CFG_REG"]
    pub gpio_func12_in_sel_cfg_reg: GPIO_FUNC12_IN_SEL_CFG_REG,
    #[doc = "0x164 - GPIO_FUNC13_IN_SEL_CFG_REG"]
    pub gpio_func13_in_sel_cfg_reg: GPIO_FUNC13_IN_SEL_CFG_REG,
    #[doc = "0x168 - GPIO_FUNC14_IN_SEL_CFG_REG"]
    pub gpio_func14_in_sel_cfg_reg: GPIO_FUNC14_IN_SEL_CFG_REG,
    #[doc = "0x16c - GPIO_FUNC15_IN_SEL_CFG_REG"]
    pub gpio_func15_in_sel_cfg_reg: GPIO_FUNC15_IN_SEL_CFG_REG,
    #[doc = "0x170 - GPIO_FUNC16_IN_SEL_CFG_REG"]
    pub gpio_func16_in_sel_cfg_reg: GPIO_FUNC16_IN_SEL_CFG_REG,
    #[doc = "0x174 - GPIO_FUNC17_IN_SEL_CFG_REG"]
    pub gpio_func17_in_sel_cfg_reg: GPIO_FUNC17_IN_SEL_CFG_REG,
    #[doc = "0x178 - GPIO_FUNC18_IN_SEL_CFG_REG"]
    pub gpio_func18_in_sel_cfg_reg: GPIO_FUNC18_IN_SEL_CFG_REG,
    #[doc = "0x17c - GPIO_FUNC19_IN_SEL_CFG_REG"]
    pub gpio_func19_in_sel_cfg_reg: GPIO_FUNC19_IN_SEL_CFG_REG,
    #[doc = "0x180 - GPIO_FUNC20_IN_SEL_CFG_REG"]
    pub gpio_func20_in_sel_cfg_reg: GPIO_FUNC20_IN_SEL_CFG_REG,
    #[doc = "0x184 - GPIO_FUNC21_IN_SEL_CFG_REG"]
    pub gpio_func21_in_sel_cfg_reg: GPIO_FUNC21_IN_SEL_CFG_REG,
    #[doc = "0x188 - GPIO_FUNC22_IN_SEL_CFG_REG"]
    pub gpio_func22_in_sel_cfg_reg: GPIO_FUNC22_IN_SEL_CFG_REG,
    #[doc = "0x18c - GPIO_FUNC23_IN_SEL_CFG_REG"]
    pub gpio_func23_in_sel_cfg_reg: GPIO_FUNC23_IN_SEL_CFG_REG,
    #[doc = "0x190 - GPIO_FUNC24_IN_SEL_CFG_REG"]
    pub gpio_func24_in_sel_cfg_reg: GPIO_FUNC24_IN_SEL_CFG_REG,
    #[doc = "0x194 - GPIO_FUNC25_IN_SEL_CFG_REG"]
    pub gpio_func25_in_sel_cfg_reg: GPIO_FUNC25_IN_SEL_CFG_REG,
    #[doc = "0x198 - GPIO_FUNC26_IN_SEL_CFG_REG"]
    pub gpio_func26_in_sel_cfg_reg: GPIO_FUNC26_IN_SEL_CFG_REG,
    #[doc = "0x19c - GPIO_FUNC27_IN_SEL_CFG_REG"]
    pub gpio_func27_in_sel_cfg_reg: GPIO_FUNC27_IN_SEL_CFG_REG,
    #[doc = "0x1a0 - GPIO_FUNC28_IN_SEL_CFG_REG"]
    pub gpio_func28_in_sel_cfg_reg: GPIO_FUNC28_IN_SEL_CFG_REG,
    #[doc = "0x1a4 - GPIO_FUNC29_IN_SEL_CFG_REG"]
    pub gpio_func29_in_sel_cfg_reg: GPIO_FUNC29_IN_SEL_CFG_REG,
    #[doc = "0x1a8 - GPIO_FUNC30_IN_SEL_CFG_REG"]
    pub gpio_func30_in_sel_cfg_reg: GPIO_FUNC30_IN_SEL_CFG_REG,
    #[doc = "0x1ac - GPIO_FUNC31_IN_SEL_CFG_REG"]
    pub gpio_func31_in_sel_cfg_reg: GPIO_FUNC31_IN_SEL_CFG_REG,
    #[doc = "0x1b0 - GPIO_FUNC32_IN_SEL_CFG_REG"]
    pub gpio_func32_in_sel_cfg_reg: GPIO_FUNC32_IN_SEL_CFG_REG,
    #[doc = "0x1b4 - GPIO_FUNC33_IN_SEL_CFG_REG"]
    pub gpio_func33_in_sel_cfg_reg: GPIO_FUNC33_IN_SEL_CFG_REG,
    #[doc = "0x1b8 - GPIO_FUNC34_IN_SEL_CFG_REG"]
    pub gpio_func34_in_sel_cfg_reg: GPIO_FUNC34_IN_SEL_CFG_REG,
    #[doc = "0x1bc - GPIO_FUNC35_IN_SEL_CFG_REG"]
    pub gpio_func35_in_sel_cfg_reg: GPIO_FUNC35_IN_SEL_CFG_REG,
    #[doc = "0x1c0 - GPIO_FUNC36_IN_SEL_CFG_REG"]
    pub gpio_func36_in_sel_cfg_reg: GPIO_FUNC36_IN_SEL_CFG_REG,
    #[doc = "0x1c4 - GPIO_FUNC37_IN_SEL_CFG_REG"]
    pub gpio_func37_in_sel_cfg_reg: GPIO_FUNC37_IN_SEL_CFG_REG,
    #[doc = "0x1c8 - GPIO_FUNC38_IN_SEL_CFG_REG"]
    pub gpio_func38_in_sel_cfg_reg: GPIO_FUNC38_IN_SEL_CFG_REG,
    #[doc = "0x1cc - GPIO_FUNC39_IN_SEL_CFG_REG"]
    pub gpio_func39_in_sel_cfg_reg: GPIO_FUNC39_IN_SEL_CFG_REG,
    #[doc = "0x1d0 - GPIO_FUNC40_IN_SEL_CFG_REG"]
    pub gpio_func40_in_sel_cfg_reg: GPIO_FUNC40_IN_SEL_CFG_REG,
    #[doc = "0x1d4 - GPIO_FUNC41_IN_SEL_CFG_REG"]
    pub gpio_func41_in_sel_cfg_reg: GPIO_FUNC41_IN_SEL_CFG_REG,
    #[doc = "0x1d8 - GPIO_FUNC42_IN_SEL_CFG_REG"]
    pub gpio_func42_in_sel_cfg_reg: GPIO_FUNC42_IN_SEL_CFG_REG,
    #[doc = "0x1dc - GPIO_FUNC43_IN_SEL_CFG_REG"]
    pub gpio_func43_in_sel_cfg_reg: GPIO_FUNC43_IN_SEL_CFG_REG,
    #[doc = "0x1e0 - GPIO_FUNC44_IN_SEL_CFG_REG"]
    pub gpio_func44_in_sel_cfg_reg: GPIO_FUNC44_IN_SEL_CFG_REG,
    #[doc = "0x1e4 - GPIO_FUNC45_IN_SEL_CFG_REG"]
    pub gpio_func45_in_sel_cfg_reg: GPIO_FUNC45_IN_SEL_CFG_REG,
    #[doc = "0x1e8 - GPIO_FUNC46_IN_SEL_CFG_REG"]
    pub gpio_func46_in_sel_cfg_reg: GPIO_FUNC46_IN_SEL_CFG_REG,
    #[doc = "0x1ec - GPIO_FUNC47_IN_SEL_CFG_REG"]
    pub gpio_func47_in_sel_cfg_reg: GPIO_FUNC47_IN_SEL_CFG_REG,
    #[doc = "0x1f0 - GPIO_FUNC48_IN_SEL_CFG_REG"]
    pub gpio_func48_in_sel_cfg_reg: GPIO_FUNC48_IN_SEL_CFG_REG,
    #[doc = "0x1f4 - GPIO_FUNC49_IN_SEL_CFG_REG"]
    pub gpio_func49_in_sel_cfg_reg: GPIO_FUNC49_IN_SEL_CFG_REG,
    #[doc = "0x1f8 - GPIO_FUNC50_IN_SEL_CFG_REG"]
    pub gpio_func50_in_sel_cfg_reg: GPIO_FUNC50_IN_SEL_CFG_REG,
    #[doc = "0x1fc - GPIO_FUNC51_IN_SEL_CFG_REG"]
    pub gpio_func51_in_sel_cfg_reg: GPIO_FUNC51_IN_SEL_CFG_REG,
    #[doc = "0x200 - GPIO_FUNC52_IN_SEL_CFG_REG"]
    pub gpio_func52_in_sel_cfg_reg: GPIO_FUNC52_IN_SEL_CFG_REG,
    #[doc = "0x204 - GPIO_FUNC53_IN_SEL_CFG_REG"]
    pub gpio_func53_in_sel_cfg_reg: GPIO_FUNC53_IN_SEL_CFG_REG,
    #[doc = "0x208 - GPIO_FUNC54_IN_SEL_CFG_REG"]
    pub gpio_func54_in_sel_cfg_reg: GPIO_FUNC54_IN_SEL_CFG_REG,
    #[doc = "0x20c - GPIO_FUNC55_IN_SEL_CFG_REG"]
    pub gpio_func55_in_sel_cfg_reg: GPIO_FUNC55_IN_SEL_CFG_REG,
    #[doc = "0x210 - GPIO_FUNC56_IN_SEL_CFG_REG"]
    pub gpio_func56_in_sel_cfg_reg: GPIO_FUNC56_IN_SEL_CFG_REG,
    #[doc = "0x214 - GPIO_FUNC57_IN_SEL_CFG_REG"]
    pub gpio_func57_in_sel_cfg_reg: GPIO_FUNC57_IN_SEL_CFG_REG,
    #[doc = "0x218 - GPIO_FUNC58_IN_SEL_CFG_REG"]
    pub gpio_func58_in_sel_cfg_reg: GPIO_FUNC58_IN_SEL_CFG_REG,
    #[doc = "0x21c - GPIO_FUNC59_IN_SEL_CFG_REG"]
    pub gpio_func59_in_sel_cfg_reg: GPIO_FUNC59_IN_SEL_CFG_REG,
    #[doc = "0x220 - GPIO_FUNC60_IN_SEL_CFG_REG"]
    pub gpio_func60_in_sel_cfg_reg: GPIO_FUNC60_IN_SEL_CFG_REG,
    #[doc = "0x224 - GPIO_FUNC61_IN_SEL_CFG_REG"]
    pub gpio_func61_in_sel_cfg_reg: GPIO_FUNC61_IN_SEL_CFG_REG,
    #[doc = "0x228 - GPIO_FUNC62_IN_SEL_CFG_REG"]
    pub gpio_func62_in_sel_cfg_reg: GPIO_FUNC62_IN_SEL_CFG_REG,
    #[doc = "0x22c - GPIO_FUNC63_IN_SEL_CFG_REG"]
    pub gpio_func63_in_sel_cfg_reg: GPIO_FUNC63_IN_SEL_CFG_REG,
    #[doc = "0x230 - GPIO_FUNC64_IN_SEL_CFG_REG"]
    pub gpio_func64_in_sel_cfg_reg: GPIO_FUNC64_IN_SEL_CFG_REG,
    #[doc = "0x234 - GPIO_FUNC65_IN_SEL_CFG_REG"]
    pub gpio_func65_in_sel_cfg_reg: GPIO_FUNC65_IN_SEL_CFG_REG,
    #[doc = "0x238 - GPIO_FUNC66_IN_SEL_CFG_REG"]
    pub gpio_func66_in_sel_cfg_reg: GPIO_FUNC66_IN_SEL_CFG_REG,
    #[doc = "0x23c - GPIO_FUNC67_IN_SEL_CFG_REG"]
    pub gpio_func67_in_sel_cfg_reg: GPIO_FUNC67_IN_SEL_CFG_REG,
    #[doc = "0x240 - GPIO_FUNC68_IN_SEL_CFG_REG"]
    pub gpio_func68_in_sel_cfg_reg: GPIO_FUNC68_IN_SEL_CFG_REG,
    #[doc = "0x244 - GPIO_FUNC69_IN_SEL_CFG_REG"]
    pub gpio_func69_in_sel_cfg_reg: GPIO_FUNC69_IN_SEL_CFG_REG,
    #[doc = "0x248 - GPIO_FUNC70_IN_SEL_CFG_REG"]
    pub gpio_func70_in_sel_cfg_reg: GPIO_FUNC70_IN_SEL_CFG_REG,
    #[doc = "0x24c - GPIO_FUNC71_IN_SEL_CFG_REG"]
    pub gpio_func71_in_sel_cfg_reg: GPIO_FUNC71_IN_SEL_CFG_REG,
    #[doc = "0x250 - GPIO_FUNC72_IN_SEL_CFG_REG"]
    pub gpio_func72_in_sel_cfg_reg: GPIO_FUNC72_IN_SEL_CFG_REG,
    #[doc = "0x254 - GPIO_FUNC73_IN_SEL_CFG_REG"]
    pub gpio_func73_in_sel_cfg_reg: GPIO_FUNC73_IN_SEL_CFG_REG,
    #[doc = "0x258 - GPIO_FUNC74_IN_SEL_CFG_REG"]
    pub gpio_func74_in_sel_cfg_reg: GPIO_FUNC74_IN_SEL_CFG_REG,
    #[doc = "0x25c - GPIO_FUNC75_IN_SEL_CFG_REG"]
    pub gpio_func75_in_sel_cfg_reg: GPIO_FUNC75_IN_SEL_CFG_REG,
    #[doc = "0x260 - GPIO_FUNC76_IN_SEL_CFG_REG"]
    pub gpio_func76_in_sel_cfg_reg: GPIO_FUNC76_IN_SEL_CFG_REG,
    #[doc = "0x264 - GPIO_FUNC77_IN_SEL_CFG_REG"]
    pub gpio_func77_in_sel_cfg_reg: GPIO_FUNC77_IN_SEL_CFG_REG,
    #[doc = "0x268 - GPIO_FUNC78_IN_SEL_CFG_REG"]
    pub gpio_func78_in_sel_cfg_reg: GPIO_FUNC78_IN_SEL_CFG_REG,
    #[doc = "0x26c - GPIO_FUNC79_IN_SEL_CFG_REG"]
    pub gpio_func79_in_sel_cfg_reg: GPIO_FUNC79_IN_SEL_CFG_REG,
    #[doc = "0x270 - GPIO_FUNC80_IN_SEL_CFG_REG"]
    pub gpio_func80_in_sel_cfg_reg: GPIO_FUNC80_IN_SEL_CFG_REG,
    #[doc = "0x274 - GPIO_FUNC81_IN_SEL_CFG_REG"]
    pub gpio_func81_in_sel_cfg_reg: GPIO_FUNC81_IN_SEL_CFG_REG,
    #[doc = "0x278 - GPIO_FUNC82_IN_SEL_CFG_REG"]
    pub gpio_func82_in_sel_cfg_reg: GPIO_FUNC82_IN_SEL_CFG_REG,
    #[doc = "0x27c - GPIO_FUNC83_IN_SEL_CFG_REG"]
    pub gpio_func83_in_sel_cfg_reg: GPIO_FUNC83_IN_SEL_CFG_REG,
    #[doc = "0x280 - GPIO_FUNC84_IN_SEL_CFG_REG"]
    pub gpio_func84_in_sel_cfg_reg: GPIO_FUNC84_IN_SEL_CFG_REG,
    #[doc = "0x284 - GPIO_FUNC85_IN_SEL_CFG_REG"]
    pub gpio_func85_in_sel_cfg_reg: GPIO_FUNC85_IN_SEL_CFG_REG,
    #[doc = "0x288 - GPIO_FUNC86_IN_SEL_CFG_REG"]
    pub gpio_func86_in_sel_cfg_reg: GPIO_FUNC86_IN_SEL_CFG_REG,
    #[doc = "0x28c - GPIO_FUNC87_IN_SEL_CFG_REG"]
    pub gpio_func87_in_sel_cfg_reg: GPIO_FUNC87_IN_SEL_CFG_REG,
    #[doc = "0x290 - GPIO_FUNC88_IN_SEL_CFG_REG"]
    pub gpio_func88_in_sel_cfg_reg: GPIO_FUNC88_IN_SEL_CFG_REG,
    #[doc = "0x294 - GPIO_FUNC89_IN_SEL_CFG_REG"]
    pub gpio_func89_in_sel_cfg_reg: GPIO_FUNC89_IN_SEL_CFG_REG,
    #[doc = "0x298 - GPIO_FUNC90_IN_SEL_CFG_REG"]
    pub gpio_func90_in_sel_cfg_reg: GPIO_FUNC90_IN_SEL_CFG_REG,
    #[doc = "0x29c - GPIO_FUNC91_IN_SEL_CFG_REG"]
    pub gpio_func91_in_sel_cfg_reg: GPIO_FUNC91_IN_SEL_CFG_REG,
    #[doc = "0x2a0 - GPIO_FUNC92_IN_SEL_CFG_REG"]
    pub gpio_func92_in_sel_cfg_reg: GPIO_FUNC92_IN_SEL_CFG_REG,
    #[doc = "0x2a4 - GPIO_FUNC93_IN_SEL_CFG_REG"]
    pub gpio_func93_in_sel_cfg_reg: GPIO_FUNC93_IN_SEL_CFG_REG,
    #[doc = "0x2a8 - GPIO_FUNC94_IN_SEL_CFG_REG"]
    pub gpio_func94_in_sel_cfg_reg: GPIO_FUNC94_IN_SEL_CFG_REG,
    #[doc = "0x2ac - GPIO_FUNC95_IN_SEL_CFG_REG"]
    pub gpio_func95_in_sel_cfg_reg: GPIO_FUNC95_IN_SEL_CFG_REG,
    #[doc = "0x2b0 - GPIO_FUNC96_IN_SEL_CFG_REG"]
    pub gpio_func96_in_sel_cfg_reg: GPIO_FUNC96_IN_SEL_CFG_REG,
    #[doc = "0x2b4 - GPIO_FUNC97_IN_SEL_CFG_REG"]
    pub gpio_func97_in_sel_cfg_reg: GPIO_FUNC97_IN_SEL_CFG_REG,
    #[doc = "0x2b8 - GPIO_FUNC98_IN_SEL_CFG_REG"]
    pub gpio_func98_in_sel_cfg_reg: GPIO_FUNC98_IN_SEL_CFG_REG,
    #[doc = "0x2bc - GPIO_FUNC99_IN_SEL_CFG_REG"]
    pub gpio_func99_in_sel_cfg_reg: GPIO_FUNC99_IN_SEL_CFG_REG,
    #[doc = "0x2c0 - GPIO_FUNC100_IN_SEL_CFG_REG"]
    pub gpio_func100_in_sel_cfg_reg: GPIO_FUNC100_IN_SEL_CFG_REG,
    #[doc = "0x2c4 - GPIO_FUNC101_IN_SEL_CFG_REG"]
    pub gpio_func101_in_sel_cfg_reg: GPIO_FUNC101_IN_SEL_CFG_REG,
    #[doc = "0x2c8 - GPIO_FUNC102_IN_SEL_CFG_REG"]
    pub gpio_func102_in_sel_cfg_reg: GPIO_FUNC102_IN_SEL_CFG_REG,
    #[doc = "0x2cc - GPIO_FUNC103_IN_SEL_CFG_REG"]
    pub gpio_func103_in_sel_cfg_reg: GPIO_FUNC103_IN_SEL_CFG_REG,
    #[doc = "0x2d0 - GPIO_FUNC104_IN_SEL_CFG_REG"]
    pub gpio_func104_in_sel_cfg_reg: GPIO_FUNC104_IN_SEL_CFG_REG,
    #[doc = "0x2d4 - GPIO_FUNC105_IN_SEL_CFG_REG"]
    pub gpio_func105_in_sel_cfg_reg: GPIO_FUNC105_IN_SEL_CFG_REG,
    #[doc = "0x2d8 - GPIO_FUNC106_IN_SEL_CFG_REG"]
    pub gpio_func106_in_sel_cfg_reg: GPIO_FUNC106_IN_SEL_CFG_REG,
    #[doc = "0x2dc - GPIO_FUNC107_IN_SEL_CFG_REG"]
    pub gpio_func107_in_sel_cfg_reg: GPIO_FUNC107_IN_SEL_CFG_REG,
    #[doc = "0x2e0 - GPIO_FUNC108_IN_SEL_CFG_REG"]
    pub gpio_func108_in_sel_cfg_reg: GPIO_FUNC108_IN_SEL_CFG_REG,
    #[doc = "0x2e4 - GPIO_FUNC109_IN_SEL_CFG_REG"]
    pub gpio_func109_in_sel_cfg_reg: GPIO_FUNC109_IN_SEL_CFG_REG,
    #[doc = "0x2e8 - GPIO_FUNC110_IN_SEL_CFG_REG"]
    pub gpio_func110_in_sel_cfg_reg: GPIO_FUNC110_IN_SEL_CFG_REG,
    #[doc = "0x2ec - GPIO_FUNC111_IN_SEL_CFG_REG"]
    pub gpio_func111_in_sel_cfg_reg: GPIO_FUNC111_IN_SEL_CFG_REG,
    #[doc = "0x2f0 - GPIO_FUNC112_IN_SEL_CFG_REG"]
    pub gpio_func112_in_sel_cfg_reg: GPIO_FUNC112_IN_SEL_CFG_REG,
    #[doc = "0x2f4 - GPIO_FUNC113_IN_SEL_CFG_REG"]
    pub gpio_func113_in_sel_cfg_reg: GPIO_FUNC113_IN_SEL_CFG_REG,
    #[doc = "0x2f8 - GPIO_FUNC114_IN_SEL_CFG_REG"]
    pub gpio_func114_in_sel_cfg_reg: GPIO_FUNC114_IN_SEL_CFG_REG,
    #[doc = "0x2fc - GPIO_FUNC115_IN_SEL_CFG_REG"]
    pub gpio_func115_in_sel_cfg_reg: GPIO_FUNC115_IN_SEL_CFG_REG,
    #[doc = "0x300 - GPIO_FUNC116_IN_SEL_CFG_REG"]
    pub gpio_func116_in_sel_cfg_reg: GPIO_FUNC116_IN_SEL_CFG_REG,
    #[doc = "0x304 - GPIO_FUNC117_IN_SEL_CFG_REG"]
    pub gpio_func117_in_sel_cfg_reg: GPIO_FUNC117_IN_SEL_CFG_REG,
    #[doc = "0x308 - GPIO_FUNC118_IN_SEL_CFG_REG"]
    pub gpio_func118_in_sel_cfg_reg: GPIO_FUNC118_IN_SEL_CFG_REG,
    #[doc = "0x30c - GPIO_FUNC119_IN_SEL_CFG_REG"]
    pub gpio_func119_in_sel_cfg_reg: GPIO_FUNC119_IN_SEL_CFG_REG,
    #[doc = "0x310 - GPIO_FUNC120_IN_SEL_CFG_REG"]
    pub gpio_func120_in_sel_cfg_reg: GPIO_FUNC120_IN_SEL_CFG_REG,
    #[doc = "0x314 - GPIO_FUNC121_IN_SEL_CFG_REG"]
    pub gpio_func121_in_sel_cfg_reg: GPIO_FUNC121_IN_SEL_CFG_REG,
    #[doc = "0x318 - GPIO_FUNC122_IN_SEL_CFG_REG"]
    pub gpio_func122_in_sel_cfg_reg: GPIO_FUNC122_IN_SEL_CFG_REG,
    #[doc = "0x31c - GPIO_FUNC123_IN_SEL_CFG_REG"]
    pub gpio_func123_in_sel_cfg_reg: GPIO_FUNC123_IN_SEL_CFG_REG,
    #[doc = "0x320 - GPIO_FUNC124_IN_SEL_CFG_REG"]
    pub gpio_func124_in_sel_cfg_reg: GPIO_FUNC124_IN_SEL_CFG_REG,
    #[doc = "0x324 - GPIO_FUNC125_IN_SEL_CFG_REG"]
    pub gpio_func125_in_sel_cfg_reg: GPIO_FUNC125_IN_SEL_CFG_REG,
    #[doc = "0x328 - GPIO_FUNC126_IN_SEL_CFG_REG"]
    pub gpio_func126_in_sel_cfg_reg: GPIO_FUNC126_IN_SEL_CFG_REG,
    #[doc = "0x32c - GPIO_FUNC127_IN_SEL_CFG_REG"]
    pub gpio_func127_in_sel_cfg_reg: GPIO_FUNC127_IN_SEL_CFG_REG,
    #[doc = "0x330 - GPIO_FUNC128_IN_SEL_CFG_REG"]
    pub gpio_func128_in_sel_cfg_reg: GPIO_FUNC128_IN_SEL_CFG_REG,
    #[doc = "0x334 - GPIO_FUNC129_IN_SEL_CFG_REG"]
    pub gpio_func129_in_sel_cfg_reg: GPIO_FUNC129_IN_SEL_CFG_REG,
    #[doc = "0x338 - GPIO_FUNC130_IN_SEL_CFG_REG"]
    pub gpio_func130_in_sel_cfg_reg: GPIO_FUNC130_IN_SEL_CFG_REG,
    #[doc = "0x33c - GPIO_FUNC131_IN_SEL_CFG_REG"]
    pub gpio_func131_in_sel_cfg_reg: GPIO_FUNC131_IN_SEL_CFG_REG,
    #[doc = "0x340 - GPIO_FUNC132_IN_SEL_CFG_REG"]
    pub gpio_func132_in_sel_cfg_reg: GPIO_FUNC132_IN_SEL_CFG_REG,
    #[doc = "0x344 - GPIO_FUNC133_IN_SEL_CFG_REG"]
    pub gpio_func133_in_sel_cfg_reg: GPIO_FUNC133_IN_SEL_CFG_REG,
    #[doc = "0x348 - GPIO_FUNC134_IN_SEL_CFG_REG"]
    pub gpio_func134_in_sel_cfg_reg: GPIO_FUNC134_IN_SEL_CFG_REG,
    #[doc = "0x34c - GPIO_FUNC135_IN_SEL_CFG_REG"]
    pub gpio_func135_in_sel_cfg_reg: GPIO_FUNC135_IN_SEL_CFG_REG,
    #[doc = "0x350 - GPIO_FUNC136_IN_SEL_CFG_REG"]
    pub gpio_func136_in_sel_cfg_reg: GPIO_FUNC136_IN_SEL_CFG_REG,
    #[doc = "0x354 - GPIO_FUNC137_IN_SEL_CFG_REG"]
    pub gpio_func137_in_sel_cfg_reg: GPIO_FUNC137_IN_SEL_CFG_REG,
    #[doc = "0x358 - GPIO_FUNC138_IN_SEL_CFG_REG"]
    pub gpio_func138_in_sel_cfg_reg: GPIO_FUNC138_IN_SEL_CFG_REG,
    #[doc = "0x35c - GPIO_FUNC139_IN_SEL_CFG_REG"]
    pub gpio_func139_in_sel_cfg_reg: GPIO_FUNC139_IN_SEL_CFG_REG,
    #[doc = "0x360 - GPIO_FUNC140_IN_SEL_CFG_REG"]
    pub gpio_func140_in_sel_cfg_reg: GPIO_FUNC140_IN_SEL_CFG_REG,
    #[doc = "0x364 - GPIO_FUNC141_IN_SEL_CFG_REG"]
    pub gpio_func141_in_sel_cfg_reg: GPIO_FUNC141_IN_SEL_CFG_REG,
    #[doc = "0x368 - GPIO_FUNC142_IN_SEL_CFG_REG"]
    pub gpio_func142_in_sel_cfg_reg: GPIO_FUNC142_IN_SEL_CFG_REG,
    #[doc = "0x36c - GPIO_FUNC143_IN_SEL_CFG_REG"]
    pub gpio_func143_in_sel_cfg_reg: GPIO_FUNC143_IN_SEL_CFG_REG,
    #[doc = "0x370 - GPIO_FUNC144_IN_SEL_CFG_REG"]
    pub gpio_func144_in_sel_cfg_reg: GPIO_FUNC144_IN_SEL_CFG_REG,
    #[doc = "0x374 - GPIO_FUNC145_IN_SEL_CFG_REG"]
    pub gpio_func145_in_sel_cfg_reg: GPIO_FUNC145_IN_SEL_CFG_REG,
    #[doc = "0x378 - GPIO_FUNC146_IN_SEL_CFG_REG"]
    pub gpio_func146_in_sel_cfg_reg: GPIO_FUNC146_IN_SEL_CFG_REG,
    #[doc = "0x37c - GPIO_FUNC147_IN_SEL_CFG_REG"]
    pub gpio_func147_in_sel_cfg_reg: GPIO_FUNC147_IN_SEL_CFG_REG,
    #[doc = "0x380 - GPIO_FUNC148_IN_SEL_CFG_REG"]
    pub gpio_func148_in_sel_cfg_reg: GPIO_FUNC148_IN_SEL_CFG_REG,
    #[doc = "0x384 - GPIO_FUNC149_IN_SEL_CFG_REG"]
    pub gpio_func149_in_sel_cfg_reg: GPIO_FUNC149_IN_SEL_CFG_REG,
    #[doc = "0x388 - GPIO_FUNC150_IN_SEL_CFG_REG"]
    pub gpio_func150_in_sel_cfg_reg: GPIO_FUNC150_IN_SEL_CFG_REG,
    #[doc = "0x38c - GPIO_FUNC151_IN_SEL_CFG_REG"]
    pub gpio_func151_in_sel_cfg_reg: GPIO_FUNC151_IN_SEL_CFG_REG,
    #[doc = "0x390 - GPIO_FUNC152_IN_SEL_CFG_REG"]
    pub gpio_func152_in_sel_cfg_reg: GPIO_FUNC152_IN_SEL_CFG_REG,
    #[doc = "0x394 - GPIO_FUNC153_IN_SEL_CFG_REG"]
    pub gpio_func153_in_sel_cfg_reg: GPIO_FUNC153_IN_SEL_CFG_REG,
    #[doc = "0x398 - GPIO_FUNC154_IN_SEL_CFG_REG"]
    pub gpio_func154_in_sel_cfg_reg: GPIO_FUNC154_IN_SEL_CFG_REG,
    #[doc = "0x39c - GPIO_FUNC155_IN_SEL_CFG_REG"]
    pub gpio_func155_in_sel_cfg_reg: GPIO_FUNC155_IN_SEL_CFG_REG,
    #[doc = "0x3a0 - GPIO_FUNC156_IN_SEL_CFG_REG"]
    pub gpio_func156_in_sel_cfg_reg: GPIO_FUNC156_IN_SEL_CFG_REG,
    #[doc = "0x3a4 - GPIO_FUNC157_IN_SEL_CFG_REG"]
    pub gpio_func157_in_sel_cfg_reg: GPIO_FUNC157_IN_SEL_CFG_REG,
    #[doc = "0x3a8 - GPIO_FUNC158_IN_SEL_CFG_REG"]
    pub gpio_func158_in_sel_cfg_reg: GPIO_FUNC158_IN_SEL_CFG_REG,
    #[doc = "0x3ac - GPIO_FUNC159_IN_SEL_CFG_REG"]
    pub gpio_func159_in_sel_cfg_reg: GPIO_FUNC159_IN_SEL_CFG_REG,
    #[doc = "0x3b0 - GPIO_FUNC160_IN_SEL_CFG_REG"]
    pub gpio_func160_in_sel_cfg_reg: GPIO_FUNC160_IN_SEL_CFG_REG,
    #[doc = "0x3b4 - GPIO_FUNC161_IN_SEL_CFG_REG"]
    pub gpio_func161_in_sel_cfg_reg: GPIO_FUNC161_IN_SEL_CFG_REG,
    #[doc = "0x3b8 - GPIO_FUNC162_IN_SEL_CFG_REG"]
    pub gpio_func162_in_sel_cfg_reg: GPIO_FUNC162_IN_SEL_CFG_REG,
    #[doc = "0x3bc - GPIO_FUNC163_IN_SEL_CFG_REG"]
    pub gpio_func163_in_sel_cfg_reg: GPIO_FUNC163_IN_SEL_CFG_REG,
    #[doc = "0x3c0 - GPIO_FUNC164_IN_SEL_CFG_REG"]
    pub gpio_func164_in_sel_cfg_reg: GPIO_FUNC164_IN_SEL_CFG_REG,
    #[doc = "0x3c4 - GPIO_FUNC165_IN_SEL_CFG_REG"]
    pub gpio_func165_in_sel_cfg_reg: GPIO_FUNC165_IN_SEL_CFG_REG,
    #[doc = "0x3c8 - GPIO_FUNC166_IN_SEL_CFG_REG"]
    pub gpio_func166_in_sel_cfg_reg: GPIO_FUNC166_IN_SEL_CFG_REG,
    #[doc = "0x3cc - GPIO_FUNC167_IN_SEL_CFG_REG"]
    pub gpio_func167_in_sel_cfg_reg: GPIO_FUNC167_IN_SEL_CFG_REG,
    #[doc = "0x3d0 - GPIO_FUNC168_IN_SEL_CFG_REG"]
    pub gpio_func168_in_sel_cfg_reg: GPIO_FUNC168_IN_SEL_CFG_REG,
    #[doc = "0x3d4 - GPIO_FUNC169_IN_SEL_CFG_REG"]
    pub gpio_func169_in_sel_cfg_reg: GPIO_FUNC169_IN_SEL_CFG_REG,
    #[doc = "0x3d8 - GPIO_FUNC170_IN_SEL_CFG_REG"]
    pub gpio_func170_in_sel_cfg_reg: GPIO_FUNC170_IN_SEL_CFG_REG,
    #[doc = "0x3dc - GPIO_FUNC171_IN_SEL_CFG_REG"]
    pub gpio_func171_in_sel_cfg_reg: GPIO_FUNC171_IN_SEL_CFG_REG,
    #[doc = "0x3e0 - GPIO_FUNC172_IN_SEL_CFG_REG"]
    pub gpio_func172_in_sel_cfg_reg: GPIO_FUNC172_IN_SEL_CFG_REG,
    #[doc = "0x3e4 - GPIO_FUNC173_IN_SEL_CFG_REG"]
    pub gpio_func173_in_sel_cfg_reg: GPIO_FUNC173_IN_SEL_CFG_REG,
    #[doc = "0x3e8 - GPIO_FUNC174_IN_SEL_CFG_REG"]
    pub gpio_func174_in_sel_cfg_reg: GPIO_FUNC174_IN_SEL_CFG_REG,
    #[doc = "0x3ec - GPIO_FUNC175_IN_SEL_CFG_REG"]
    pub gpio_func175_in_sel_cfg_reg: GPIO_FUNC175_IN_SEL_CFG_REG,
    #[doc = "0x3f0 - GPIO_FUNC176_IN_SEL_CFG_REG"]
    pub gpio_func176_in_sel_cfg_reg: GPIO_FUNC176_IN_SEL_CFG_REG,
    #[doc = "0x3f4 - GPIO_FUNC177_IN_SEL_CFG_REG"]
    pub gpio_func177_in_sel_cfg_reg: GPIO_FUNC177_IN_SEL_CFG_REG,
    #[doc = "0x3f8 - GPIO_FUNC178_IN_SEL_CFG_REG"]
    pub gpio_func178_in_sel_cfg_reg: GPIO_FUNC178_IN_SEL_CFG_REG,
    #[doc = "0x3fc - GPIO_FUNC179_IN_SEL_CFG_REG"]
    pub gpio_func179_in_sel_cfg_reg: GPIO_FUNC179_IN_SEL_CFG_REG,
    #[doc = "0x400 - GPIO_FUNC180_IN_SEL_CFG_REG"]
    pub gpio_func180_in_sel_cfg_reg: GPIO_FUNC180_IN_SEL_CFG_REG,
    #[doc = "0x404 - GPIO_FUNC181_IN_SEL_CFG_REG"]
    pub gpio_func181_in_sel_cfg_reg: GPIO_FUNC181_IN_SEL_CFG_REG,
    #[doc = "0x408 - GPIO_FUNC182_IN_SEL_CFG_REG"]
    pub gpio_func182_in_sel_cfg_reg: GPIO_FUNC182_IN_SEL_CFG_REG,
    #[doc = "0x40c - GPIO_FUNC183_IN_SEL_CFG_REG"]
    pub gpio_func183_in_sel_cfg_reg: GPIO_FUNC183_IN_SEL_CFG_REG,
    #[doc = "0x410 - GPIO_FUNC184_IN_SEL_CFG_REG"]
    pub gpio_func184_in_sel_cfg_reg: GPIO_FUNC184_IN_SEL_CFG_REG,
    #[doc = "0x414 - GPIO_FUNC185_IN_SEL_CFG_REG"]
    pub gpio_func185_in_sel_cfg_reg: GPIO_FUNC185_IN_SEL_CFG_REG,
    #[doc = "0x418 - GPIO_FUNC186_IN_SEL_CFG_REG"]
    pub gpio_func186_in_sel_cfg_reg: GPIO_FUNC186_IN_SEL_CFG_REG,
    #[doc = "0x41c - GPIO_FUNC187_IN_SEL_CFG_REG"]
    pub gpio_func187_in_sel_cfg_reg: GPIO_FUNC187_IN_SEL_CFG_REG,
    #[doc = "0x420 - GPIO_FUNC188_IN_SEL_CFG_REG"]
    pub gpio_func188_in_sel_cfg_reg: GPIO_FUNC188_IN_SEL_CFG_REG,
    #[doc = "0x424 - GPIO_FUNC189_IN_SEL_CFG_REG"]
    pub gpio_func189_in_sel_cfg_reg: GPIO_FUNC189_IN_SEL_CFG_REG,
    #[doc = "0x428 - GPIO_FUNC190_IN_SEL_CFG_REG"]
    pub gpio_func190_in_sel_cfg_reg: GPIO_FUNC190_IN_SEL_CFG_REG,
    #[doc = "0x42c - GPIO_FUNC191_IN_SEL_CFG_REG"]
    pub gpio_func191_in_sel_cfg_reg: GPIO_FUNC191_IN_SEL_CFG_REG,
    #[doc = "0x430 - GPIO_FUNC192_IN_SEL_CFG_REG"]
    pub gpio_func192_in_sel_cfg_reg: GPIO_FUNC192_IN_SEL_CFG_REG,
    #[doc = "0x434 - GPIO_FUNC193_IN_SEL_CFG_REG"]
    pub gpio_func193_in_sel_cfg_reg: GPIO_FUNC193_IN_SEL_CFG_REG,
    #[doc = "0x438 - GPIO_FUNC194_IN_SEL_CFG_REG"]
    pub gpio_func194_in_sel_cfg_reg: GPIO_FUNC194_IN_SEL_CFG_REG,
    #[doc = "0x43c - GPIO_FUNC195_IN_SEL_CFG_REG"]
    pub gpio_func195_in_sel_cfg_reg: GPIO_FUNC195_IN_SEL_CFG_REG,
    #[doc = "0x440 - GPIO_FUNC196_IN_SEL_CFG_REG"]
    pub gpio_func196_in_sel_cfg_reg: GPIO_FUNC196_IN_SEL_CFG_REG,
    #[doc = "0x444 - GPIO_FUNC197_IN_SEL_CFG_REG"]
    pub gpio_func197_in_sel_cfg_reg: GPIO_FUNC197_IN_SEL_CFG_REG,
    #[doc = "0x448 - GPIO_FUNC198_IN_SEL_CFG_REG"]
    pub gpio_func198_in_sel_cfg_reg: GPIO_FUNC198_IN_SEL_CFG_REG,
    #[doc = "0x44c - GPIO_FUNC199_IN_SEL_CFG_REG"]
    pub gpio_func199_in_sel_cfg_reg: GPIO_FUNC199_IN_SEL_CFG_REG,
    #[doc = "0x450 - GPIO_FUNC200_IN_SEL_CFG_REG"]
    pub gpio_func200_in_sel_cfg_reg: GPIO_FUNC200_IN_SEL_CFG_REG,
    #[doc = "0x454 - GPIO_FUNC201_IN_SEL_CFG_REG"]
    pub gpio_func201_in_sel_cfg_reg: GPIO_FUNC201_IN_SEL_CFG_REG,
    #[doc = "0x458 - GPIO_FUNC202_IN_SEL_CFG_REG"]
    pub gpio_func202_in_sel_cfg_reg: GPIO_FUNC202_IN_SEL_CFG_REG,
    #[doc = "0x45c - GPIO_FUNC203_IN_SEL_CFG_REG"]
    pub gpio_func203_in_sel_cfg_reg: GPIO_FUNC203_IN_SEL_CFG_REG,
    #[doc = "0x460 - GPIO_FUNC204_IN_SEL_CFG_REG"]
    pub gpio_func204_in_sel_cfg_reg: GPIO_FUNC204_IN_SEL_CFG_REG,
    #[doc = "0x464 - GPIO_FUNC205_IN_SEL_CFG_REG"]
    pub gpio_func205_in_sel_cfg_reg: GPIO_FUNC205_IN_SEL_CFG_REG,
    #[doc = "0x468 - GPIO_FUNC206_IN_SEL_CFG_REG"]
    pub gpio_func206_in_sel_cfg_reg: GPIO_FUNC206_IN_SEL_CFG_REG,
    #[doc = "0x46c - GPIO_FUNC207_IN_SEL_CFG_REG"]
    pub gpio_func207_in_sel_cfg_reg: GPIO_FUNC207_IN_SEL_CFG_REG,
    #[doc = "0x470 - GPIO_FUNC208_IN_SEL_CFG_REG"]
    pub gpio_func208_in_sel_cfg_reg: GPIO_FUNC208_IN_SEL_CFG_REG,
    #[doc = "0x474 - GPIO_FUNC209_IN_SEL_CFG_REG"]
    pub gpio_func209_in_sel_cfg_reg: GPIO_FUNC209_IN_SEL_CFG_REG,
    #[doc = "0x478 - GPIO_FUNC210_IN_SEL_CFG_REG"]
    pub gpio_func210_in_sel_cfg_reg: GPIO_FUNC210_IN_SEL_CFG_REG,
    #[doc = "0x47c - GPIO_FUNC211_IN_SEL_CFG_REG"]
    pub gpio_func211_in_sel_cfg_reg: GPIO_FUNC211_IN_SEL_CFG_REG,
    #[doc = "0x480 - GPIO_FUNC212_IN_SEL_CFG_REG"]
    pub gpio_func212_in_sel_cfg_reg: GPIO_FUNC212_IN_SEL_CFG_REG,
    #[doc = "0x484 - GPIO_FUNC213_IN_SEL_CFG_REG"]
    pub gpio_func213_in_sel_cfg_reg: GPIO_FUNC213_IN_SEL_CFG_REG,
    #[doc = "0x488 - GPIO_FUNC214_IN_SEL_CFG_REG"]
    pub gpio_func214_in_sel_cfg_reg: GPIO_FUNC214_IN_SEL_CFG_REG,
    #[doc = "0x48c - GPIO_FUNC215_IN_SEL_CFG_REG"]
    pub gpio_func215_in_sel_cfg_reg: GPIO_FUNC215_IN_SEL_CFG_REG,
    #[doc = "0x490 - GPIO_FUNC216_IN_SEL_CFG_REG"]
    pub gpio_func216_in_sel_cfg_reg: GPIO_FUNC216_IN_SEL_CFG_REG,
    #[doc = "0x494 - GPIO_FUNC217_IN_SEL_CFG_REG"]
    pub gpio_func217_in_sel_cfg_reg: GPIO_FUNC217_IN_SEL_CFG_REG,
    #[doc = "0x498 - GPIO_FUNC218_IN_SEL_CFG_REG"]
    pub gpio_func218_in_sel_cfg_reg: GPIO_FUNC218_IN_SEL_CFG_REG,
    #[doc = "0x49c - GPIO_FUNC219_IN_SEL_CFG_REG"]
    pub gpio_func219_in_sel_cfg_reg: GPIO_FUNC219_IN_SEL_CFG_REG,
    #[doc = "0x4a0 - GPIO_FUNC220_IN_SEL_CFG_REG"]
    pub gpio_func220_in_sel_cfg_reg: GPIO_FUNC220_IN_SEL_CFG_REG,
    #[doc = "0x4a4 - GPIO_FUNC221_IN_SEL_CFG_REG"]
    pub gpio_func221_in_sel_cfg_reg: GPIO_FUNC221_IN_SEL_CFG_REG,
    #[doc = "0x4a8 - GPIO_FUNC222_IN_SEL_CFG_REG"]
    pub gpio_func222_in_sel_cfg_reg: GPIO_FUNC222_IN_SEL_CFG_REG,
    #[doc = "0x4ac - GPIO_FUNC223_IN_SEL_CFG_REG"]
    pub gpio_func223_in_sel_cfg_reg: GPIO_FUNC223_IN_SEL_CFG_REG,
    #[doc = "0x4b0 - GPIO_FUNC224_IN_SEL_CFG_REG"]
    pub gpio_func224_in_sel_cfg_reg: GPIO_FUNC224_IN_SEL_CFG_REG,
    #[doc = "0x4b4 - GPIO_FUNC225_IN_SEL_CFG_REG"]
    pub gpio_func225_in_sel_cfg_reg: GPIO_FUNC225_IN_SEL_CFG_REG,
    #[doc = "0x4b8 - GPIO_FUNC226_IN_SEL_CFG_REG"]
    pub gpio_func226_in_sel_cfg_reg: GPIO_FUNC226_IN_SEL_CFG_REG,
    #[doc = "0x4bc - GPIO_FUNC227_IN_SEL_CFG_REG"]
    pub gpio_func227_in_sel_cfg_reg: GPIO_FUNC227_IN_SEL_CFG_REG,
    #[doc = "0x4c0 - GPIO_FUNC228_IN_SEL_CFG_REG"]
    pub gpio_func228_in_sel_cfg_reg: GPIO_FUNC228_IN_SEL_CFG_REG,
    #[doc = "0x4c4 - GPIO_FUNC229_IN_SEL_CFG_REG"]
    pub gpio_func229_in_sel_cfg_reg: GPIO_FUNC229_IN_SEL_CFG_REG,
    #[doc = "0x4c8 - GPIO_FUNC230_IN_SEL_CFG_REG"]
    pub gpio_func230_in_sel_cfg_reg: GPIO_FUNC230_IN_SEL_CFG_REG,
    #[doc = "0x4cc - GPIO_FUNC231_IN_SEL_CFG_REG"]
    pub gpio_func231_in_sel_cfg_reg: GPIO_FUNC231_IN_SEL_CFG_REG,
    #[doc = "0x4d0 - GPIO_FUNC232_IN_SEL_CFG_REG"]
    pub gpio_func232_in_sel_cfg_reg: GPIO_FUNC232_IN_SEL_CFG_REG,
    #[doc = "0x4d4 - GPIO_FUNC233_IN_SEL_CFG_REG"]
    pub gpio_func233_in_sel_cfg_reg: GPIO_FUNC233_IN_SEL_CFG_REG,
    #[doc = "0x4d8 - GPIO_FUNC234_IN_SEL_CFG_REG"]
    pub gpio_func234_in_sel_cfg_reg: GPIO_FUNC234_IN_SEL_CFG_REG,
    #[doc = "0x4dc - GPIO_FUNC235_IN_SEL_CFG_REG"]
    pub gpio_func235_in_sel_cfg_reg: GPIO_FUNC235_IN_SEL_CFG_REG,
    #[doc = "0x4e0 - GPIO_FUNC236_IN_SEL_CFG_REG"]
    pub gpio_func236_in_sel_cfg_reg: GPIO_FUNC236_IN_SEL_CFG_REG,
    #[doc = "0x4e4 - GPIO_FUNC237_IN_SEL_CFG_REG"]
    pub gpio_func237_in_sel_cfg_reg: GPIO_FUNC237_IN_SEL_CFG_REG,
    #[doc = "0x4e8 - GPIO_FUNC238_IN_SEL_CFG_REG"]
    pub gpio_func238_in_sel_cfg_reg: GPIO_FUNC238_IN_SEL_CFG_REG,
    #[doc = "0x4ec - GPIO_FUNC239_IN_SEL_CFG_REG"]
    pub gpio_func239_in_sel_cfg_reg: GPIO_FUNC239_IN_SEL_CFG_REG,
    #[doc = "0x4f0 - GPIO_FUNC240_IN_SEL_CFG_REG"]
    pub gpio_func240_in_sel_cfg_reg: GPIO_FUNC240_IN_SEL_CFG_REG,
    #[doc = "0x4f4 - GPIO_FUNC241_IN_SEL_CFG_REG"]
    pub gpio_func241_in_sel_cfg_reg: GPIO_FUNC241_IN_SEL_CFG_REG,
    #[doc = "0x4f8 - GPIO_FUNC242_IN_SEL_CFG_REG"]
    pub gpio_func242_in_sel_cfg_reg: GPIO_FUNC242_IN_SEL_CFG_REG,
    #[doc = "0x4fc - GPIO_FUNC243_IN_SEL_CFG_REG"]
    pub gpio_func243_in_sel_cfg_reg: GPIO_FUNC243_IN_SEL_CFG_REG,
    #[doc = "0x500 - GPIO_FUNC244_IN_SEL_CFG_REG"]
    pub gpio_func244_in_sel_cfg_reg: GPIO_FUNC244_IN_SEL_CFG_REG,
    #[doc = "0x504 - GPIO_FUNC245_IN_SEL_CFG_REG"]
    pub gpio_func245_in_sel_cfg_reg: GPIO_FUNC245_IN_SEL_CFG_REG,
    #[doc = "0x508 - GPIO_FUNC246_IN_SEL_CFG_REG"]
    pub gpio_func246_in_sel_cfg_reg: GPIO_FUNC246_IN_SEL_CFG_REG,
    #[doc = "0x50c - GPIO_FUNC247_IN_SEL_CFG_REG"]
    pub gpio_func247_in_sel_cfg_reg: GPIO_FUNC247_IN_SEL_CFG_REG,
    #[doc = "0x510 - GPIO_FUNC248_IN_SEL_CFG_REG"]
    pub gpio_func248_in_sel_cfg_reg: GPIO_FUNC248_IN_SEL_CFG_REG,
    #[doc = "0x514 - GPIO_FUNC249_IN_SEL_CFG_REG"]
    pub gpio_func249_in_sel_cfg_reg: GPIO_FUNC249_IN_SEL_CFG_REG,
    #[doc = "0x518 - GPIO_FUNC250_IN_SEL_CFG_REG"]
    pub gpio_func250_in_sel_cfg_reg: GPIO_FUNC250_IN_SEL_CFG_REG,
    #[doc = "0x51c - GPIO_FUNC251_IN_SEL_CFG_REG"]
    pub gpio_func251_in_sel_cfg_reg: GPIO_FUNC251_IN_SEL_CFG_REG,
    #[doc = "0x520 - GPIO_FUNC252_IN_SEL_CFG_REG"]
    pub gpio_func252_in_sel_cfg_reg: GPIO_FUNC252_IN_SEL_CFG_REG,
    #[doc = "0x524 - GPIO_FUNC253_IN_SEL_CFG_REG"]
    pub gpio_func253_in_sel_cfg_reg: GPIO_FUNC253_IN_SEL_CFG_REG,
    #[doc = "0x528 - GPIO_FUNC254_IN_SEL_CFG_REG"]
    pub gpio_func254_in_sel_cfg_reg: GPIO_FUNC254_IN_SEL_CFG_REG,
    #[doc = "0x52c - GPIO_FUNC255_IN_SEL_CFG_REG"]
    pub gpio_func255_in_sel_cfg_reg: GPIO_FUNC255_IN_SEL_CFG_REG,
    #[doc = "0x530 - GPIO_FUNC0_OUT_SEL_CFG_REG"]
    pub gpio_func0_out_sel_cfg_reg: GPIO_FUNC0_OUT_SEL_CFG_REG,
    #[doc = "0x534 - GPIO_FUNC1_OUT_SEL_CFG_REG"]
    pub gpio_func1_out_sel_cfg_reg: GPIO_FUNC1_OUT_SEL_CFG_REG,
    #[doc = "0x538 - GPIO_FUNC2_OUT_SEL_CFG_REG"]
    pub gpio_func2_out_sel_cfg_reg: GPIO_FUNC2_OUT_SEL_CFG_REG,
    #[doc = "0x53c - GPIO_FUNC3_OUT_SEL_CFG_REG"]
    pub gpio_func3_out_sel_cfg_reg: GPIO_FUNC3_OUT_SEL_CFG_REG,
    #[doc = "0x540 - GPIO_FUNC4_OUT_SEL_CFG_REG"]
    pub gpio_func4_out_sel_cfg_reg: GPIO_FUNC4_OUT_SEL_CFG_REG,
    #[doc = "0x544 - GPIO_FUNC5_OUT_SEL_CFG_REG"]
    pub gpio_func5_out_sel_cfg_reg: GPIO_FUNC5_OUT_SEL_CFG_REG,
    #[doc = "0x548 - GPIO_FUNC6_OUT_SEL_CFG_REG"]
    pub gpio_func6_out_sel_cfg_reg: GPIO_FUNC6_OUT_SEL_CFG_REG,
    #[doc = "0x54c - GPIO_FUNC7_OUT_SEL_CFG_REG"]
    pub gpio_func7_out_sel_cfg_reg: GPIO_FUNC7_OUT_SEL_CFG_REG,
    #[doc = "0x550 - GPIO_FUNC8_OUT_SEL_CFG_REG"]
    pub gpio_func8_out_sel_cfg_reg: GPIO_FUNC8_OUT_SEL_CFG_REG,
    #[doc = "0x554 - GPIO_FUNC9_OUT_SEL_CFG_REG"]
    pub gpio_func9_out_sel_cfg_reg: GPIO_FUNC9_OUT_SEL_CFG_REG,
    #[doc = "0x558 - GPIO_FUNC10_OUT_SEL_CFG_REG"]
    pub gpio_func10_out_sel_cfg_reg: GPIO_FUNC10_OUT_SEL_CFG_REG,
    #[doc = "0x55c - GPIO_FUNC11_OUT_SEL_CFG_REG"]
    pub gpio_func11_out_sel_cfg_reg: GPIO_FUNC11_OUT_SEL_CFG_REG,
    #[doc = "0x560 - GPIO_FUNC12_OUT_SEL_CFG_REG"]
    pub gpio_func12_out_sel_cfg_reg: GPIO_FUNC12_OUT_SEL_CFG_REG,
    #[doc = "0x564 - GPIO_FUNC13_OUT_SEL_CFG_REG"]
    pub gpio_func13_out_sel_cfg_reg: GPIO_FUNC13_OUT_SEL_CFG_REG,
    #[doc = "0x568 - GPIO_FUNC14_OUT_SEL_CFG_REG"]
    pub gpio_func14_out_sel_cfg_reg: GPIO_FUNC14_OUT_SEL_CFG_REG,
    #[doc = "0x56c - GPIO_FUNC15_OUT_SEL_CFG_REG"]
    pub gpio_func15_out_sel_cfg_reg: GPIO_FUNC15_OUT_SEL_CFG_REG,
    #[doc = "0x570 - GPIO_FUNC16_OUT_SEL_CFG_REG"]
    pub gpio_func16_out_sel_cfg_reg: GPIO_FUNC16_OUT_SEL_CFG_REG,
    #[doc = "0x574 - GPIO_FUNC17_OUT_SEL_CFG_REG"]
    pub gpio_func17_out_sel_cfg_reg: GPIO_FUNC17_OUT_SEL_CFG_REG,
    #[doc = "0x578 - GPIO_FUNC18_OUT_SEL_CFG_REG"]
    pub gpio_func18_out_sel_cfg_reg: GPIO_FUNC18_OUT_SEL_CFG_REG,
    #[doc = "0x57c - GPIO_FUNC19_OUT_SEL_CFG_REG"]
    pub gpio_func19_out_sel_cfg_reg: GPIO_FUNC19_OUT_SEL_CFG_REG,
    #[doc = "0x580 - GPIO_FUNC20_OUT_SEL_CFG_REG"]
    pub gpio_func20_out_sel_cfg_reg: GPIO_FUNC20_OUT_SEL_CFG_REG,
    #[doc = "0x584 - GPIO_FUNC21_OUT_SEL_CFG_REG"]
    pub gpio_func21_out_sel_cfg_reg: GPIO_FUNC21_OUT_SEL_CFG_REG,
    #[doc = "0x588 - GPIO_FUNC22_OUT_SEL_CFG_REG"]
    pub gpio_func22_out_sel_cfg_reg: GPIO_FUNC22_OUT_SEL_CFG_REG,
    #[doc = "0x58c - GPIO_FUNC23_OUT_SEL_CFG_REG"]
    pub gpio_func23_out_sel_cfg_reg: GPIO_FUNC23_OUT_SEL_CFG_REG,
    #[doc = "0x590 - GPIO_FUNC24_OUT_SEL_CFG_REG"]
    pub gpio_func24_out_sel_cfg_reg: GPIO_FUNC24_OUT_SEL_CFG_REG,
    #[doc = "0x594 - GPIO_FUNC25_OUT_SEL_CFG_REG"]
    pub gpio_func25_out_sel_cfg_reg: GPIO_FUNC25_OUT_SEL_CFG_REG,
    #[doc = "0x598 - GPIO_FUNC26_OUT_SEL_CFG_REG"]
    pub gpio_func26_out_sel_cfg_reg: GPIO_FUNC26_OUT_SEL_CFG_REG,
    #[doc = "0x59c - GPIO_FUNC27_OUT_SEL_CFG_REG"]
    pub gpio_func27_out_sel_cfg_reg: GPIO_FUNC27_OUT_SEL_CFG_REG,
    #[doc = "0x5a0 - GPIO_FUNC28_OUT_SEL_CFG_REG"]
    pub gpio_func28_out_sel_cfg_reg: GPIO_FUNC28_OUT_SEL_CFG_REG,
    #[doc = "0x5a4 - GPIO_FUNC29_OUT_SEL_CFG_REG"]
    pub gpio_func29_out_sel_cfg_reg: GPIO_FUNC29_OUT_SEL_CFG_REG,
    #[doc = "0x5a8 - GPIO_FUNC30_OUT_SEL_CFG_REG"]
    pub gpio_func30_out_sel_cfg_reg: GPIO_FUNC30_OUT_SEL_CFG_REG,
    #[doc = "0x5ac - GPIO_FUNC31_OUT_SEL_CFG_REG"]
    pub gpio_func31_out_sel_cfg_reg: GPIO_FUNC31_OUT_SEL_CFG_REG,
    #[doc = "0x5b0 - GPIO_FUNC32_OUT_SEL_CFG_REG"]
    pub gpio_func32_out_sel_cfg_reg: GPIO_FUNC32_OUT_SEL_CFG_REG,
    #[doc = "0x5b4 - GPIO_FUNC33_OUT_SEL_CFG_REG"]
    pub gpio_func33_out_sel_cfg_reg: GPIO_FUNC33_OUT_SEL_CFG_REG,
    #[doc = "0x5b8 - GPIO_FUNC34_OUT_SEL_CFG_REG"]
    pub gpio_func34_out_sel_cfg_reg: GPIO_FUNC34_OUT_SEL_CFG_REG,
    #[doc = "0x5bc - GPIO_FUNC35_OUT_SEL_CFG_REG"]
    pub gpio_func35_out_sel_cfg_reg: GPIO_FUNC35_OUT_SEL_CFG_REG,
    #[doc = "0x5c0 - GPIO_FUNC36_OUT_SEL_CFG_REG"]
    pub gpio_func36_out_sel_cfg_reg: GPIO_FUNC36_OUT_SEL_CFG_REG,
    #[doc = "0x5c4 - GPIO_FUNC37_OUT_SEL_CFG_REG"]
    pub gpio_func37_out_sel_cfg_reg: GPIO_FUNC37_OUT_SEL_CFG_REG,
    #[doc = "0x5c8 - GPIO_FUNC38_OUT_SEL_CFG_REG"]
    pub gpio_func38_out_sel_cfg_reg: GPIO_FUNC38_OUT_SEL_CFG_REG,
    #[doc = "0x5cc - GPIO_FUNC39_OUT_SEL_CFG_REG"]
    pub gpio_func39_out_sel_cfg_reg: GPIO_FUNC39_OUT_SEL_CFG_REG,
}
#[doc = "GPIO_BT_SELECT_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_bt_select_reg](gpio_bt_select_reg) module"]
pub type GPIO_BT_SELECT_REG = crate::Reg<u32, _GPIO_BT_SELECT_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_BT_SELECT_REG;
#[doc = "`read()` method returns [gpio_bt_select_reg::R](gpio_bt_select_reg::R) reader structure"]
impl crate::Readable for GPIO_BT_SELECT_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_bt_select_reg::W](gpio_bt_select_reg::W) writer structure"]
impl crate::Writable for GPIO_BT_SELECT_REG {}
#[doc = "GPIO_BT_SELECT_REG"]
pub mod gpio_bt_select_reg;
#[doc = "GPIO_OUT_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_out_reg](gpio_out_reg) module"]
pub type GPIO_OUT_REG = crate::Reg<u32, _GPIO_OUT_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_OUT_REG;
#[doc = "`read()` method returns [gpio_out_reg::R](gpio_out_reg::R) reader structure"]
impl crate::Readable for GPIO_OUT_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_out_reg::W](gpio_out_reg::W) writer structure"]
impl crate::Writable for GPIO_OUT_REG {}
#[doc = "GPIO_OUT_REG"]
pub mod gpio_out_reg;
#[doc = "GPIO_OUT_W1TS_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_out_w1ts_reg](gpio_out_w1ts_reg) module"]
pub type GPIO_OUT_W1TS_REG = crate::Reg<u32, _GPIO_OUT_W1TS_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_OUT_W1TS_REG;
#[doc = "`read()` method returns [gpio_out_w1ts_reg::R](gpio_out_w1ts_reg::R) reader structure"]
impl crate::Readable for GPIO_OUT_W1TS_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_out_w1ts_reg::W](gpio_out_w1ts_reg::W) writer structure"]
impl crate::Writable for GPIO_OUT_W1TS_REG {}
#[doc = "GPIO_OUT_W1TS_REG"]
pub mod gpio_out_w1ts_reg;
#[doc = "GPIO_OUT_W1TC_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_out_w1tc_reg](gpio_out_w1tc_reg) module"]
pub type GPIO_OUT_W1TC_REG = crate::Reg<u32, _GPIO_OUT_W1TC_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_OUT_W1TC_REG;
#[doc = "`read()` method returns [gpio_out_w1tc_reg::R](gpio_out_w1tc_reg::R) reader structure"]
impl crate::Readable for GPIO_OUT_W1TC_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_out_w1tc_reg::W](gpio_out_w1tc_reg::W) writer structure"]
impl crate::Writable for GPIO_OUT_W1TC_REG {}
#[doc = "GPIO_OUT_W1TC_REG"]
pub mod gpio_out_w1tc_reg;
#[doc = "GPIO_OUT1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_out1_reg](gpio_out1_reg) module"]
pub type GPIO_OUT1_REG = crate::Reg<u32, _GPIO_OUT1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_OUT1_REG;
#[doc = "`read()` method returns [gpio_out1_reg::R](gpio_out1_reg::R) reader structure"]
impl crate::Readable for GPIO_OUT1_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_out1_reg::W](gpio_out1_reg::W) writer structure"]
impl crate::Writable for GPIO_OUT1_REG {}
#[doc = "GPIO_OUT1_REG"]
pub mod gpio_out1_reg;
#[doc = "GPIO_OUT1_W1TS_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_out1_w1ts_reg](gpio_out1_w1ts_reg) module"]
pub type GPIO_OUT1_W1TS_REG = crate::Reg<u32, _GPIO_OUT1_W1TS_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_OUT1_W1TS_REG;
#[doc = "`read()` method returns [gpio_out1_w1ts_reg::R](gpio_out1_w1ts_reg::R) reader structure"]
impl crate::Readable for GPIO_OUT1_W1TS_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_out1_w1ts_reg::W](gpio_out1_w1ts_reg::W) writer structure"]
impl crate::Writable for GPIO_OUT1_W1TS_REG {}
#[doc = "GPIO_OUT1_W1TS_REG"]
pub mod gpio_out1_w1ts_reg;
#[doc = "GPIO_OUT1_W1TC_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_out1_w1tc_reg](gpio_out1_w1tc_reg) module"]
pub type GPIO_OUT1_W1TC_REG = crate::Reg<u32, _GPIO_OUT1_W1TC_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_OUT1_W1TC_REG;
#[doc = "`read()` method returns [gpio_out1_w1tc_reg::R](gpio_out1_w1tc_reg::R) reader structure"]
impl crate::Readable for GPIO_OUT1_W1TC_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_out1_w1tc_reg::W](gpio_out1_w1tc_reg::W) writer structure"]
impl crate::Writable for GPIO_OUT1_W1TC_REG {}
#[doc = "GPIO_OUT1_W1TC_REG"]
pub mod gpio_out1_w1tc_reg;
#[doc = "GPIO_SDIO_SELECT_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_sdio_select_reg](gpio_sdio_select_reg) module"]
pub type GPIO_SDIO_SELECT_REG = crate::Reg<u32, _GPIO_SDIO_SELECT_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_SDIO_SELECT_REG;
#[doc = "`read()` method returns [gpio_sdio_select_reg::R](gpio_sdio_select_reg::R) reader structure"]
impl crate::Readable for GPIO_SDIO_SELECT_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_sdio_select_reg::W](gpio_sdio_select_reg::W) writer structure"]
impl crate::Writable for GPIO_SDIO_SELECT_REG {}
#[doc = "GPIO_SDIO_SELECT_REG"]
pub mod gpio_sdio_select_reg;
#[doc = "GPIO_ENABLE_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_enable_reg](gpio_enable_reg) module"]
pub type GPIO_ENABLE_REG = crate::Reg<u32, _GPIO_ENABLE_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_ENABLE_REG;
#[doc = "`read()` method returns [gpio_enable_reg::R](gpio_enable_reg::R) reader structure"]
impl crate::Readable for GPIO_ENABLE_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_enable_reg::W](gpio_enable_reg::W) writer structure"]
impl crate::Writable for GPIO_ENABLE_REG {}
#[doc = "GPIO_ENABLE_REG"]
pub mod gpio_enable_reg;
#[doc = "GPIO_ENABLE_W1TS_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_enable_w1ts_reg](gpio_enable_w1ts_reg) module"]
pub type GPIO_ENABLE_W1TS_REG = crate::Reg<u32, _GPIO_ENABLE_W1TS_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_ENABLE_W1TS_REG;
#[doc = "`read()` method returns [gpio_enable_w1ts_reg::R](gpio_enable_w1ts_reg::R) reader structure"]
impl crate::Readable for GPIO_ENABLE_W1TS_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_enable_w1ts_reg::W](gpio_enable_w1ts_reg::W) writer structure"]
impl crate::Writable for GPIO_ENABLE_W1TS_REG {}
#[doc = "GPIO_ENABLE_W1TS_REG"]
pub mod gpio_enable_w1ts_reg;
#[doc = "GPIO_ENABLE_W1TC_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_enable_w1tc_reg](gpio_enable_w1tc_reg) module"]
pub type GPIO_ENABLE_W1TC_REG = crate::Reg<u32, _GPIO_ENABLE_W1TC_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_ENABLE_W1TC_REG;
#[doc = "`read()` method returns [gpio_enable_w1tc_reg::R](gpio_enable_w1tc_reg::R) reader structure"]
impl crate::Readable for GPIO_ENABLE_W1TC_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_enable_w1tc_reg::W](gpio_enable_w1tc_reg::W) writer structure"]
impl crate::Writable for GPIO_ENABLE_W1TC_REG {}
#[doc = "GPIO_ENABLE_W1TC_REG"]
pub mod gpio_enable_w1tc_reg;
#[doc = "GPIO_ENABLE1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_enable1_reg](gpio_enable1_reg) module"]
pub type GPIO_ENABLE1_REG = crate::Reg<u32, _GPIO_ENABLE1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_ENABLE1_REG;
#[doc = "`read()` method returns [gpio_enable1_reg::R](gpio_enable1_reg::R) reader structure"]
impl crate::Readable for GPIO_ENABLE1_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_enable1_reg::W](gpio_enable1_reg::W) writer structure"]
impl crate::Writable for GPIO_ENABLE1_REG {}
#[doc = "GPIO_ENABLE1_REG"]
pub mod gpio_enable1_reg;
#[doc = "GPIO_ENABLE1_W1TS_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_enable1_w1ts_reg](gpio_enable1_w1ts_reg) module"]
pub type GPIO_ENABLE1_W1TS_REG = crate::Reg<u32, _GPIO_ENABLE1_W1TS_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_ENABLE1_W1TS_REG;
#[doc = "`read()` method returns [gpio_enable1_w1ts_reg::R](gpio_enable1_w1ts_reg::R) reader structure"]
impl crate::Readable for GPIO_ENABLE1_W1TS_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_enable1_w1ts_reg::W](gpio_enable1_w1ts_reg::W) writer structure"]
impl crate::Writable for GPIO_ENABLE1_W1TS_REG {}
#[doc = "GPIO_ENABLE1_W1TS_REG"]
pub mod gpio_enable1_w1ts_reg;
#[doc = "GPIO_ENABLE1_W1TC_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_enable1_w1tc_reg](gpio_enable1_w1tc_reg) module"]
pub type GPIO_ENABLE1_W1TC_REG = crate::Reg<u32, _GPIO_ENABLE1_W1TC_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_ENABLE1_W1TC_REG;
#[doc = "`read()` method returns [gpio_enable1_w1tc_reg::R](gpio_enable1_w1tc_reg::R) reader structure"]
impl crate::Readable for GPIO_ENABLE1_W1TC_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_enable1_w1tc_reg::W](gpio_enable1_w1tc_reg::W) writer structure"]
impl crate::Writable for GPIO_ENABLE1_W1TC_REG {}
#[doc = "GPIO_ENABLE1_W1TC_REG"]
pub mod gpio_enable1_w1tc_reg;
#[doc = "GPIO_STRAP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_strap_reg](gpio_strap_reg) module"]
pub type GPIO_STRAP_REG = crate::Reg<u32, _GPIO_STRAP_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_STRAP_REG;
#[doc = "`read()` method returns [gpio_strap_reg::R](gpio_strap_reg::R) reader structure"]
impl crate::Readable for GPIO_STRAP_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_strap_reg::W](gpio_strap_reg::W) writer structure"]
impl crate::Writable for GPIO_STRAP_REG {}
#[doc = "GPIO_STRAP_REG"]
pub mod gpio_strap_reg;
#[doc = "GPIO_IN_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_in_reg](gpio_in_reg) module"]
pub type GPIO_IN_REG = crate::Reg<u32, _GPIO_IN_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_IN_REG;
#[doc = "`read()` method returns [gpio_in_reg::R](gpio_in_reg::R) reader structure"]
impl crate::Readable for GPIO_IN_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_in_reg::W](gpio_in_reg::W) writer structure"]
impl crate::Writable for GPIO_IN_REG {}
#[doc = "GPIO_IN_REG"]
pub mod gpio_in_reg;
#[doc = "GPIO_IN1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_in1_reg](gpio_in1_reg) module"]
pub type GPIO_IN1_REG = crate::Reg<u32, _GPIO_IN1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_IN1_REG;
#[doc = "`read()` method returns [gpio_in1_reg::R](gpio_in1_reg::R) reader structure"]
impl crate::Readable for GPIO_IN1_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_in1_reg::W](gpio_in1_reg::W) writer structure"]
impl crate::Writable for GPIO_IN1_REG {}
#[doc = "GPIO_IN1_REG"]
pub mod gpio_in1_reg;
#[doc = "GPIO_STATUS_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_status_reg](gpio_status_reg) module"]
pub type GPIO_STATUS_REG = crate::Reg<u32, _GPIO_STATUS_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_STATUS_REG;
#[doc = "`read()` method returns [gpio_status_reg::R](gpio_status_reg::R) reader structure"]
impl crate::Readable for GPIO_STATUS_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_status_reg::W](gpio_status_reg::W) writer structure"]
impl crate::Writable for GPIO_STATUS_REG {}
#[doc = "GPIO_STATUS_REG"]
pub mod gpio_status_reg;
#[doc = "GPIO_STATUS_W1TS_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_status_w1ts_reg](gpio_status_w1ts_reg) module"]
pub type GPIO_STATUS_W1TS_REG = crate::Reg<u32, _GPIO_STATUS_W1TS_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_STATUS_W1TS_REG;
#[doc = "`read()` method returns [gpio_status_w1ts_reg::R](gpio_status_w1ts_reg::R) reader structure"]
impl crate::Readable for GPIO_STATUS_W1TS_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_status_w1ts_reg::W](gpio_status_w1ts_reg::W) writer structure"]
impl crate::Writable for GPIO_STATUS_W1TS_REG {}
#[doc = "GPIO_STATUS_W1TS_REG"]
pub mod gpio_status_w1ts_reg;
#[doc = "GPIO_STATUS_W1TC_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_status_w1tc_reg](gpio_status_w1tc_reg) module"]
pub type GPIO_STATUS_W1TC_REG = crate::Reg<u32, _GPIO_STATUS_W1TC_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_STATUS_W1TC_REG;
#[doc = "`read()` method returns [gpio_status_w1tc_reg::R](gpio_status_w1tc_reg::R) reader structure"]
impl crate::Readable for GPIO_STATUS_W1TC_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_status_w1tc_reg::W](gpio_status_w1tc_reg::W) writer structure"]
impl crate::Writable for GPIO_STATUS_W1TC_REG {}
#[doc = "GPIO_STATUS_W1TC_REG"]
pub mod gpio_status_w1tc_reg;
#[doc = "GPIO_STATUS1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_status1_reg](gpio_status1_reg) module"]
pub type GPIO_STATUS1_REG = crate::Reg<u32, _GPIO_STATUS1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_STATUS1_REG;
#[doc = "`read()` method returns [gpio_status1_reg::R](gpio_status1_reg::R) reader structure"]
impl crate::Readable for GPIO_STATUS1_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_status1_reg::W](gpio_status1_reg::W) writer structure"]
impl crate::Writable for GPIO_STATUS1_REG {}
#[doc = "GPIO_STATUS1_REG"]
pub mod gpio_status1_reg;
#[doc = "GPIO_STATUS1_W1TS_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_status1_w1ts_reg](gpio_status1_w1ts_reg) module"]
pub type GPIO_STATUS1_W1TS_REG = crate::Reg<u32, _GPIO_STATUS1_W1TS_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_STATUS1_W1TS_REG;
#[doc = "`read()` method returns [gpio_status1_w1ts_reg::R](gpio_status1_w1ts_reg::R) reader structure"]
impl crate::Readable for GPIO_STATUS1_W1TS_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_status1_w1ts_reg::W](gpio_status1_w1ts_reg::W) writer structure"]
impl crate::Writable for GPIO_STATUS1_W1TS_REG {}
#[doc = "GPIO_STATUS1_W1TS_REG"]
pub mod gpio_status1_w1ts_reg;
#[doc = "GPIO_STATUS1_W1TC_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_status1_w1tc_reg](gpio_status1_w1tc_reg) module"]
pub type GPIO_STATUS1_W1TC_REG = crate::Reg<u32, _GPIO_STATUS1_W1TC_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_STATUS1_W1TC_REG;
#[doc = "`read()` method returns [gpio_status1_w1tc_reg::R](gpio_status1_w1tc_reg::R) reader structure"]
impl crate::Readable for GPIO_STATUS1_W1TC_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_status1_w1tc_reg::W](gpio_status1_w1tc_reg::W) writer structure"]
impl crate::Writable for GPIO_STATUS1_W1TC_REG {}
#[doc = "GPIO_STATUS1_W1TC_REG"]
pub mod gpio_status1_w1tc_reg;
#[doc = "GPIO_ACPU_INT_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_acpu_int_reg](gpio_acpu_int_reg) module"]
pub type GPIO_ACPU_INT_REG = crate::Reg<u32, _GPIO_ACPU_INT_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_ACPU_INT_REG;
#[doc = "`read()` method returns [gpio_acpu_int_reg::R](gpio_acpu_int_reg::R) reader structure"]
impl crate::Readable for GPIO_ACPU_INT_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_acpu_int_reg::W](gpio_acpu_int_reg::W) writer structure"]
impl crate::Writable for GPIO_ACPU_INT_REG {}
#[doc = "GPIO_ACPU_INT_REG"]
pub mod gpio_acpu_int_reg;
#[doc = "GPIO_ACPU_NMI_INT_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_acpu_nmi_int_reg](gpio_acpu_nmi_int_reg) module"]
pub type GPIO_ACPU_NMI_INT_REG = crate::Reg<u32, _GPIO_ACPU_NMI_INT_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_ACPU_NMI_INT_REG;
#[doc = "`read()` method returns [gpio_acpu_nmi_int_reg::R](gpio_acpu_nmi_int_reg::R) reader structure"]
impl crate::Readable for GPIO_ACPU_NMI_INT_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_acpu_nmi_int_reg::W](gpio_acpu_nmi_int_reg::W) writer structure"]
impl crate::Writable for GPIO_ACPU_NMI_INT_REG {}
#[doc = "GPIO_ACPU_NMI_INT_REG"]
pub mod gpio_acpu_nmi_int_reg;
#[doc = "GPIO_PCPU_INT_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_pcpu_int_reg](gpio_pcpu_int_reg) module"]
pub type GPIO_PCPU_INT_REG = crate::Reg<u32, _GPIO_PCPU_INT_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PCPU_INT_REG;
#[doc = "`read()` method returns [gpio_pcpu_int_reg::R](gpio_pcpu_int_reg::R) reader structure"]
impl crate::Readable for GPIO_PCPU_INT_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_pcpu_int_reg::W](gpio_pcpu_int_reg::W) writer structure"]
impl crate::Writable for GPIO_PCPU_INT_REG {}
#[doc = "GPIO_PCPU_INT_REG"]
pub mod gpio_pcpu_int_reg;
#[doc = "GPIO_PCPU_NMI_INT_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_pcpu_nmi_int_reg](gpio_pcpu_nmi_int_reg) module"]
pub type GPIO_PCPU_NMI_INT_REG = crate::Reg<u32, _GPIO_PCPU_NMI_INT_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PCPU_NMI_INT_REG;
#[doc = "`read()` method returns [gpio_pcpu_nmi_int_reg::R](gpio_pcpu_nmi_int_reg::R) reader structure"]
impl crate::Readable for GPIO_PCPU_NMI_INT_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_pcpu_nmi_int_reg::W](gpio_pcpu_nmi_int_reg::W) writer structure"]
impl crate::Writable for GPIO_PCPU_NMI_INT_REG {}
#[doc = "GPIO_PCPU_NMI_INT_REG"]
pub mod gpio_pcpu_nmi_int_reg;
#[doc = "GPIO_CPUSDIO_INT_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_cpusdio_int_reg](gpio_cpusdio_int_reg) module"]
pub type GPIO_CPUSDIO_INT_REG = crate::Reg<u32, _GPIO_CPUSDIO_INT_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_CPUSDIO_INT_REG;
#[doc = "`read()` method returns [gpio_cpusdio_int_reg::R](gpio_cpusdio_int_reg::R) reader structure"]
impl crate::Readable for GPIO_CPUSDIO_INT_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_cpusdio_int_reg::W](gpio_cpusdio_int_reg::W) writer structure"]
impl crate::Writable for GPIO_CPUSDIO_INT_REG {}
#[doc = "GPIO_CPUSDIO_INT_REG"]
pub mod gpio_cpusdio_int_reg;
#[doc = "GPIO_ACPU_INT1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_acpu_int1_reg](gpio_acpu_int1_reg) module"]
pub type GPIO_ACPU_INT1_REG = crate::Reg<u32, _GPIO_ACPU_INT1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_ACPU_INT1_REG;
#[doc = "`read()` method returns [gpio_acpu_int1_reg::R](gpio_acpu_int1_reg::R) reader structure"]
impl crate::Readable for GPIO_ACPU_INT1_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_acpu_int1_reg::W](gpio_acpu_int1_reg::W) writer structure"]
impl crate::Writable for GPIO_ACPU_INT1_REG {}
#[doc = "GPIO_ACPU_INT1_REG"]
pub mod gpio_acpu_int1_reg;
#[doc = "GPIO_ACPU_NMI_INT1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_acpu_nmi_int1_reg](gpio_acpu_nmi_int1_reg) module"]
pub type GPIO_ACPU_NMI_INT1_REG = crate::Reg<u32, _GPIO_ACPU_NMI_INT1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_ACPU_NMI_INT1_REG;
#[doc = "`read()` method returns [gpio_acpu_nmi_int1_reg::R](gpio_acpu_nmi_int1_reg::R) reader structure"]
impl crate::Readable for GPIO_ACPU_NMI_INT1_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_acpu_nmi_int1_reg::W](gpio_acpu_nmi_int1_reg::W) writer structure"]
impl crate::Writable for GPIO_ACPU_NMI_INT1_REG {}
#[doc = "GPIO_ACPU_NMI_INT1_REG"]
pub mod gpio_acpu_nmi_int1_reg;
#[doc = "GPIO_PCPU_INT1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_pcpu_int1_reg](gpio_pcpu_int1_reg) module"]
pub type GPIO_PCPU_INT1_REG = crate::Reg<u32, _GPIO_PCPU_INT1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PCPU_INT1_REG;
#[doc = "`read()` method returns [gpio_pcpu_int1_reg::R](gpio_pcpu_int1_reg::R) reader structure"]
impl crate::Readable for GPIO_PCPU_INT1_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_pcpu_int1_reg::W](gpio_pcpu_int1_reg::W) writer structure"]
impl crate::Writable for GPIO_PCPU_INT1_REG {}
#[doc = "GPIO_PCPU_INT1_REG"]
pub mod gpio_pcpu_int1_reg;
#[doc = "GPIO_PCPU_NMI_INT1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_pcpu_nmi_int1_reg](gpio_pcpu_nmi_int1_reg) module"]
pub type GPIO_PCPU_NMI_INT1_REG = crate::Reg<u32, _GPIO_PCPU_NMI_INT1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PCPU_NMI_INT1_REG;
#[doc = "`read()` method returns [gpio_pcpu_nmi_int1_reg::R](gpio_pcpu_nmi_int1_reg::R) reader structure"]
impl crate::Readable for GPIO_PCPU_NMI_INT1_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_pcpu_nmi_int1_reg::W](gpio_pcpu_nmi_int1_reg::W) writer structure"]
impl crate::Writable for GPIO_PCPU_NMI_INT1_REG {}
#[doc = "GPIO_PCPU_NMI_INT1_REG"]
pub mod gpio_pcpu_nmi_int1_reg;
#[doc = "GPIO_CPUSDIO_INT1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_cpusdio_int1_reg](gpio_cpusdio_int1_reg) module"]
pub type GPIO_CPUSDIO_INT1_REG = crate::Reg<u32, _GPIO_CPUSDIO_INT1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_CPUSDIO_INT1_REG;
#[doc = "`read()` method returns [gpio_cpusdio_int1_reg::R](gpio_cpusdio_int1_reg::R) reader structure"]
impl crate::Readable for GPIO_CPUSDIO_INT1_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_cpusdio_int1_reg::W](gpio_cpusdio_int1_reg::W) writer structure"]
impl crate::Writable for GPIO_CPUSDIO_INT1_REG {}
#[doc = "GPIO_CPUSDIO_INT1_REG"]
pub mod gpio_cpusdio_int1_reg;
#[doc = "GPIO_PIN0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_pin0_reg](gpio_pin0_reg) module"]
pub type GPIO_PIN0_REG = crate::Reg<u32, _GPIO_PIN0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PIN0_REG;
#[doc = "`read()` method returns [gpio_pin0_reg::R](gpio_pin0_reg::R) reader structure"]
impl crate::Readable for GPIO_PIN0_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_pin0_reg::W](gpio_pin0_reg::W) writer structure"]
impl crate::Writable for GPIO_PIN0_REG {}
#[doc = "GPIO_PIN0_REG"]
pub mod gpio_pin0_reg;
#[doc = "GPIO_PIN1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_pin1_reg](gpio_pin1_reg) module"]
pub type GPIO_PIN1_REG = crate::Reg<u32, _GPIO_PIN1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PIN1_REG;
#[doc = "`read()` method returns [gpio_pin1_reg::R](gpio_pin1_reg::R) reader structure"]
impl crate::Readable for GPIO_PIN1_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_pin1_reg::W](gpio_pin1_reg::W) writer structure"]
impl crate::Writable for GPIO_PIN1_REG {}
#[doc = "GPIO_PIN1_REG"]
pub mod gpio_pin1_reg;
#[doc = "GPIO_PIN2_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_pin2_reg](gpio_pin2_reg) module"]
pub type GPIO_PIN2_REG = crate::Reg<u32, _GPIO_PIN2_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PIN2_REG;
#[doc = "`read()` method returns [gpio_pin2_reg::R](gpio_pin2_reg::R) reader structure"]
impl crate::Readable for GPIO_PIN2_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_pin2_reg::W](gpio_pin2_reg::W) writer structure"]
impl crate::Writable for GPIO_PIN2_REG {}
#[doc = "GPIO_PIN2_REG"]
pub mod gpio_pin2_reg;
#[doc = "GPIO_PIN3_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_pin3_reg](gpio_pin3_reg) module"]
pub type GPIO_PIN3_REG = crate::Reg<u32, _GPIO_PIN3_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PIN3_REG;
#[doc = "`read()` method returns [gpio_pin3_reg::R](gpio_pin3_reg::R) reader structure"]
impl crate::Readable for GPIO_PIN3_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_pin3_reg::W](gpio_pin3_reg::W) writer structure"]
impl crate::Writable for GPIO_PIN3_REG {}
#[doc = "GPIO_PIN3_REG"]
pub mod gpio_pin3_reg;
#[doc = "GPIO_PIN4_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_pin4_reg](gpio_pin4_reg) module"]
pub type GPIO_PIN4_REG = crate::Reg<u32, _GPIO_PIN4_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PIN4_REG;
#[doc = "`read()` method returns [gpio_pin4_reg::R](gpio_pin4_reg::R) reader structure"]
impl crate::Readable for GPIO_PIN4_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_pin4_reg::W](gpio_pin4_reg::W) writer structure"]
impl crate::Writable for GPIO_PIN4_REG {}
#[doc = "GPIO_PIN4_REG"]
pub mod gpio_pin4_reg;
#[doc = "GPIO_PIN5_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_pin5_reg](gpio_pin5_reg) module"]
pub type GPIO_PIN5_REG = crate::Reg<u32, _GPIO_PIN5_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PIN5_REG;
#[doc = "`read()` method returns [gpio_pin5_reg::R](gpio_pin5_reg::R) reader structure"]
impl crate::Readable for GPIO_PIN5_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_pin5_reg::W](gpio_pin5_reg::W) writer structure"]
impl crate::Writable for GPIO_PIN5_REG {}
#[doc = "GPIO_PIN5_REG"]
pub mod gpio_pin5_reg;
#[doc = "GPIO_PIN6_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_pin6_reg](gpio_pin6_reg) module"]
pub type GPIO_PIN6_REG = crate::Reg<u32, _GPIO_PIN6_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PIN6_REG;
#[doc = "`read()` method returns [gpio_pin6_reg::R](gpio_pin6_reg::R) reader structure"]
impl crate::Readable for GPIO_PIN6_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_pin6_reg::W](gpio_pin6_reg::W) writer structure"]
impl crate::Writable for GPIO_PIN6_REG {}
#[doc = "GPIO_PIN6_REG"]
pub mod gpio_pin6_reg;
#[doc = "GPIO_PIN7_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_pin7_reg](gpio_pin7_reg) module"]
pub type GPIO_PIN7_REG = crate::Reg<u32, _GPIO_PIN7_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PIN7_REG;
#[doc = "`read()` method returns [gpio_pin7_reg::R](gpio_pin7_reg::R) reader structure"]
impl crate::Readable for GPIO_PIN7_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_pin7_reg::W](gpio_pin7_reg::W) writer structure"]
impl crate::Writable for GPIO_PIN7_REG {}
#[doc = "GPIO_PIN7_REG"]
pub mod gpio_pin7_reg;
#[doc = "GPIO_PIN8_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_pin8_reg](gpio_pin8_reg) module"]
pub type GPIO_PIN8_REG = crate::Reg<u32, _GPIO_PIN8_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PIN8_REG;
#[doc = "`read()` method returns [gpio_pin8_reg::R](gpio_pin8_reg::R) reader structure"]
impl crate::Readable for GPIO_PIN8_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_pin8_reg::W](gpio_pin8_reg::W) writer structure"]
impl crate::Writable for GPIO_PIN8_REG {}
#[doc = "GPIO_PIN8_REG"]
pub mod gpio_pin8_reg;
#[doc = "GPIO_PIN9_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_pin9_reg](gpio_pin9_reg) module"]
pub type GPIO_PIN9_REG = crate::Reg<u32, _GPIO_PIN9_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PIN9_REG;
#[doc = "`read()` method returns [gpio_pin9_reg::R](gpio_pin9_reg::R) reader structure"]
impl crate::Readable for GPIO_PIN9_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_pin9_reg::W](gpio_pin9_reg::W) writer structure"]
impl crate::Writable for GPIO_PIN9_REG {}
#[doc = "GPIO_PIN9_REG"]
pub mod gpio_pin9_reg;
#[doc = "GPIO_PIN10_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_pin10_reg](gpio_pin10_reg) module"]
pub type GPIO_PIN10_REG = crate::Reg<u32, _GPIO_PIN10_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PIN10_REG;
#[doc = "`read()` method returns [gpio_pin10_reg::R](gpio_pin10_reg::R) reader structure"]
impl crate::Readable for GPIO_PIN10_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_pin10_reg::W](gpio_pin10_reg::W) writer structure"]
impl crate::Writable for GPIO_PIN10_REG {}
#[doc = "GPIO_PIN10_REG"]
pub mod gpio_pin10_reg;
#[doc = "GPIO_PIN11_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_pin11_reg](gpio_pin11_reg) module"]
pub type GPIO_PIN11_REG = crate::Reg<u32, _GPIO_PIN11_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PIN11_REG;
#[doc = "`read()` method returns [gpio_pin11_reg::R](gpio_pin11_reg::R) reader structure"]
impl crate::Readable for GPIO_PIN11_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_pin11_reg::W](gpio_pin11_reg::W) writer structure"]
impl crate::Writable for GPIO_PIN11_REG {}
#[doc = "GPIO_PIN11_REG"]
pub mod gpio_pin11_reg;
#[doc = "GPIO_PIN12_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_pin12_reg](gpio_pin12_reg) module"]
pub type GPIO_PIN12_REG = crate::Reg<u32, _GPIO_PIN12_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PIN12_REG;
#[doc = "`read()` method returns [gpio_pin12_reg::R](gpio_pin12_reg::R) reader structure"]
impl crate::Readable for GPIO_PIN12_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_pin12_reg::W](gpio_pin12_reg::W) writer structure"]
impl crate::Writable for GPIO_PIN12_REG {}
#[doc = "GPIO_PIN12_REG"]
pub mod gpio_pin12_reg;
#[doc = "GPIO_PIN13_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_pin13_reg](gpio_pin13_reg) module"]
pub type GPIO_PIN13_REG = crate::Reg<u32, _GPIO_PIN13_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PIN13_REG;
#[doc = "`read()` method returns [gpio_pin13_reg::R](gpio_pin13_reg::R) reader structure"]
impl crate::Readable for GPIO_PIN13_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_pin13_reg::W](gpio_pin13_reg::W) writer structure"]
impl crate::Writable for GPIO_PIN13_REG {}
#[doc = "GPIO_PIN13_REG"]
pub mod gpio_pin13_reg;
#[doc = "GPIO_PIN14_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_pin14_reg](gpio_pin14_reg) module"]
pub type GPIO_PIN14_REG = crate::Reg<u32, _GPIO_PIN14_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PIN14_REG;
#[doc = "`read()` method returns [gpio_pin14_reg::R](gpio_pin14_reg::R) reader structure"]
impl crate::Readable for GPIO_PIN14_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_pin14_reg::W](gpio_pin14_reg::W) writer structure"]
impl crate::Writable for GPIO_PIN14_REG {}
#[doc = "GPIO_PIN14_REG"]
pub mod gpio_pin14_reg;
#[doc = "GPIO_PIN15_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_pin15_reg](gpio_pin15_reg) module"]
pub type GPIO_PIN15_REG = crate::Reg<u32, _GPIO_PIN15_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PIN15_REG;
#[doc = "`read()` method returns [gpio_pin15_reg::R](gpio_pin15_reg::R) reader structure"]
impl crate::Readable for GPIO_PIN15_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_pin15_reg::W](gpio_pin15_reg::W) writer structure"]
impl crate::Writable for GPIO_PIN15_REG {}
#[doc = "GPIO_PIN15_REG"]
pub mod gpio_pin15_reg;
#[doc = "GPIO_PIN16_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_pin16_reg](gpio_pin16_reg) module"]
pub type GPIO_PIN16_REG = crate::Reg<u32, _GPIO_PIN16_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PIN16_REG;
#[doc = "`read()` method returns [gpio_pin16_reg::R](gpio_pin16_reg::R) reader structure"]
impl crate::Readable for GPIO_PIN16_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_pin16_reg::W](gpio_pin16_reg::W) writer structure"]
impl crate::Writable for GPIO_PIN16_REG {}
#[doc = "GPIO_PIN16_REG"]
pub mod gpio_pin16_reg;
#[doc = "GPIO_PIN17_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_pin17_reg](gpio_pin17_reg) module"]
pub type GPIO_PIN17_REG = crate::Reg<u32, _GPIO_PIN17_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PIN17_REG;
#[doc = "`read()` method returns [gpio_pin17_reg::R](gpio_pin17_reg::R) reader structure"]
impl crate::Readable for GPIO_PIN17_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_pin17_reg::W](gpio_pin17_reg::W) writer structure"]
impl crate::Writable for GPIO_PIN17_REG {}
#[doc = "GPIO_PIN17_REG"]
pub mod gpio_pin17_reg;
#[doc = "GPIO_PIN18_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_pin18_reg](gpio_pin18_reg) module"]
pub type GPIO_PIN18_REG = crate::Reg<u32, _GPIO_PIN18_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PIN18_REG;
#[doc = "`read()` method returns [gpio_pin18_reg::R](gpio_pin18_reg::R) reader structure"]
impl crate::Readable for GPIO_PIN18_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_pin18_reg::W](gpio_pin18_reg::W) writer structure"]
impl crate::Writable for GPIO_PIN18_REG {}
#[doc = "GPIO_PIN18_REG"]
pub mod gpio_pin18_reg;
#[doc = "GPIO_PIN19_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_pin19_reg](gpio_pin19_reg) module"]
pub type GPIO_PIN19_REG = crate::Reg<u32, _GPIO_PIN19_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PIN19_REG;
#[doc = "`read()` method returns [gpio_pin19_reg::R](gpio_pin19_reg::R) reader structure"]
impl crate::Readable for GPIO_PIN19_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_pin19_reg::W](gpio_pin19_reg::W) writer structure"]
impl crate::Writable for GPIO_PIN19_REG {}
#[doc = "GPIO_PIN19_REG"]
pub mod gpio_pin19_reg;
#[doc = "GPIO_PIN20_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_pin20_reg](gpio_pin20_reg) module"]
pub type GPIO_PIN20_REG = crate::Reg<u32, _GPIO_PIN20_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PIN20_REG;
#[doc = "`read()` method returns [gpio_pin20_reg::R](gpio_pin20_reg::R) reader structure"]
impl crate::Readable for GPIO_PIN20_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_pin20_reg::W](gpio_pin20_reg::W) writer structure"]
impl crate::Writable for GPIO_PIN20_REG {}
#[doc = "GPIO_PIN20_REG"]
pub mod gpio_pin20_reg;
#[doc = "GPIO_PIN21_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_pin21_reg](gpio_pin21_reg) module"]
pub type GPIO_PIN21_REG = crate::Reg<u32, _GPIO_PIN21_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PIN21_REG;
#[doc = "`read()` method returns [gpio_pin21_reg::R](gpio_pin21_reg::R) reader structure"]
impl crate::Readable for GPIO_PIN21_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_pin21_reg::W](gpio_pin21_reg::W) writer structure"]
impl crate::Writable for GPIO_PIN21_REG {}
#[doc = "GPIO_PIN21_REG"]
pub mod gpio_pin21_reg;
#[doc = "GPIO_PIN22_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_pin22_reg](gpio_pin22_reg) module"]
pub type GPIO_PIN22_REG = crate::Reg<u32, _GPIO_PIN22_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PIN22_REG;
#[doc = "`read()` method returns [gpio_pin22_reg::R](gpio_pin22_reg::R) reader structure"]
impl crate::Readable for GPIO_PIN22_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_pin22_reg::W](gpio_pin22_reg::W) writer structure"]
impl crate::Writable for GPIO_PIN22_REG {}
#[doc = "GPIO_PIN22_REG"]
pub mod gpio_pin22_reg;
#[doc = "GPIO_PIN23_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_pin23_reg](gpio_pin23_reg) module"]
pub type GPIO_PIN23_REG = crate::Reg<u32, _GPIO_PIN23_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PIN23_REG;
#[doc = "`read()` method returns [gpio_pin23_reg::R](gpio_pin23_reg::R) reader structure"]
impl crate::Readable for GPIO_PIN23_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_pin23_reg::W](gpio_pin23_reg::W) writer structure"]
impl crate::Writable for GPIO_PIN23_REG {}
#[doc = "GPIO_PIN23_REG"]
pub mod gpio_pin23_reg;
#[doc = "GPIO_PIN24_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_pin24_reg](gpio_pin24_reg) module"]
pub type GPIO_PIN24_REG = crate::Reg<u32, _GPIO_PIN24_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PIN24_REG;
#[doc = "`read()` method returns [gpio_pin24_reg::R](gpio_pin24_reg::R) reader structure"]
impl crate::Readable for GPIO_PIN24_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_pin24_reg::W](gpio_pin24_reg::W) writer structure"]
impl crate::Writable for GPIO_PIN24_REG {}
#[doc = "GPIO_PIN24_REG"]
pub mod gpio_pin24_reg;
#[doc = "GPIO_PIN25_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_pin25_reg](gpio_pin25_reg) module"]
pub type GPIO_PIN25_REG = crate::Reg<u32, _GPIO_PIN25_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PIN25_REG;
#[doc = "`read()` method returns [gpio_pin25_reg::R](gpio_pin25_reg::R) reader structure"]
impl crate::Readable for GPIO_PIN25_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_pin25_reg::W](gpio_pin25_reg::W) writer structure"]
impl crate::Writable for GPIO_PIN25_REG {}
#[doc = "GPIO_PIN25_REG"]
pub mod gpio_pin25_reg;
#[doc = "GPIO_PIN26_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_pin26_reg](gpio_pin26_reg) module"]
pub type GPIO_PIN26_REG = crate::Reg<u32, _GPIO_PIN26_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PIN26_REG;
#[doc = "`read()` method returns [gpio_pin26_reg::R](gpio_pin26_reg::R) reader structure"]
impl crate::Readable for GPIO_PIN26_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_pin26_reg::W](gpio_pin26_reg::W) writer structure"]
impl crate::Writable for GPIO_PIN26_REG {}
#[doc = "GPIO_PIN26_REG"]
pub mod gpio_pin26_reg;
#[doc = "GPIO_PIN27_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_pin27_reg](gpio_pin27_reg) module"]
pub type GPIO_PIN27_REG = crate::Reg<u32, _GPIO_PIN27_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PIN27_REG;
#[doc = "`read()` method returns [gpio_pin27_reg::R](gpio_pin27_reg::R) reader structure"]
impl crate::Readable for GPIO_PIN27_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_pin27_reg::W](gpio_pin27_reg::W) writer structure"]
impl crate::Writable for GPIO_PIN27_REG {}
#[doc = "GPIO_PIN27_REG"]
pub mod gpio_pin27_reg;
#[doc = "GPIO_PIN28_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_pin28_reg](gpio_pin28_reg) module"]
pub type GPIO_PIN28_REG = crate::Reg<u32, _GPIO_PIN28_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PIN28_REG;
#[doc = "`read()` method returns [gpio_pin28_reg::R](gpio_pin28_reg::R) reader structure"]
impl crate::Readable for GPIO_PIN28_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_pin28_reg::W](gpio_pin28_reg::W) writer structure"]
impl crate::Writable for GPIO_PIN28_REG {}
#[doc = "GPIO_PIN28_REG"]
pub mod gpio_pin28_reg;
#[doc = "GPIO_PIN29_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_pin29_reg](gpio_pin29_reg) module"]
pub type GPIO_PIN29_REG = crate::Reg<u32, _GPIO_PIN29_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PIN29_REG;
#[doc = "`read()` method returns [gpio_pin29_reg::R](gpio_pin29_reg::R) reader structure"]
impl crate::Readable for GPIO_PIN29_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_pin29_reg::W](gpio_pin29_reg::W) writer structure"]
impl crate::Writable for GPIO_PIN29_REG {}
#[doc = "GPIO_PIN29_REG"]
pub mod gpio_pin29_reg;
#[doc = "GPIO_PIN30_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_pin30_reg](gpio_pin30_reg) module"]
pub type GPIO_PIN30_REG = crate::Reg<u32, _GPIO_PIN30_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PIN30_REG;
#[doc = "`read()` method returns [gpio_pin30_reg::R](gpio_pin30_reg::R) reader structure"]
impl crate::Readable for GPIO_PIN30_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_pin30_reg::W](gpio_pin30_reg::W) writer structure"]
impl crate::Writable for GPIO_PIN30_REG {}
#[doc = "GPIO_PIN30_REG"]
pub mod gpio_pin30_reg;
#[doc = "GPIO_PIN31_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_pin31_reg](gpio_pin31_reg) module"]
pub type GPIO_PIN31_REG = crate::Reg<u32, _GPIO_PIN31_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PIN31_REG;
#[doc = "`read()` method returns [gpio_pin31_reg::R](gpio_pin31_reg::R) reader structure"]
impl crate::Readable for GPIO_PIN31_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_pin31_reg::W](gpio_pin31_reg::W) writer structure"]
impl crate::Writable for GPIO_PIN31_REG {}
#[doc = "GPIO_PIN31_REG"]
pub mod gpio_pin31_reg;
#[doc = "GPIO_PIN32_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_pin32_reg](gpio_pin32_reg) module"]
pub type GPIO_PIN32_REG = crate::Reg<u32, _GPIO_PIN32_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PIN32_REG;
#[doc = "`read()` method returns [gpio_pin32_reg::R](gpio_pin32_reg::R) reader structure"]
impl crate::Readable for GPIO_PIN32_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_pin32_reg::W](gpio_pin32_reg::W) writer structure"]
impl crate::Writable for GPIO_PIN32_REG {}
#[doc = "GPIO_PIN32_REG"]
pub mod gpio_pin32_reg;
#[doc = "GPIO_PIN33_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_pin33_reg](gpio_pin33_reg) module"]
pub type GPIO_PIN33_REG = crate::Reg<u32, _GPIO_PIN33_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PIN33_REG;
#[doc = "`read()` method returns [gpio_pin33_reg::R](gpio_pin33_reg::R) reader structure"]
impl crate::Readable for GPIO_PIN33_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_pin33_reg::W](gpio_pin33_reg::W) writer structure"]
impl crate::Writable for GPIO_PIN33_REG {}
#[doc = "GPIO_PIN33_REG"]
pub mod gpio_pin33_reg;
#[doc = "GPIO_PIN34_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_pin34_reg](gpio_pin34_reg) module"]
pub type GPIO_PIN34_REG = crate::Reg<u32, _GPIO_PIN34_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PIN34_REG;
#[doc = "`read()` method returns [gpio_pin34_reg::R](gpio_pin34_reg::R) reader structure"]
impl crate::Readable for GPIO_PIN34_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_pin34_reg::W](gpio_pin34_reg::W) writer structure"]
impl crate::Writable for GPIO_PIN34_REG {}
#[doc = "GPIO_PIN34_REG"]
pub mod gpio_pin34_reg;
#[doc = "GPIO_PIN35_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_pin35_reg](gpio_pin35_reg) module"]
pub type GPIO_PIN35_REG = crate::Reg<u32, _GPIO_PIN35_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PIN35_REG;
#[doc = "`read()` method returns [gpio_pin35_reg::R](gpio_pin35_reg::R) reader structure"]
impl crate::Readable for GPIO_PIN35_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_pin35_reg::W](gpio_pin35_reg::W) writer structure"]
impl crate::Writable for GPIO_PIN35_REG {}
#[doc = "GPIO_PIN35_REG"]
pub mod gpio_pin35_reg;
#[doc = "GPIO_PIN36_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_pin36_reg](gpio_pin36_reg) module"]
pub type GPIO_PIN36_REG = crate::Reg<u32, _GPIO_PIN36_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PIN36_REG;
#[doc = "`read()` method returns [gpio_pin36_reg::R](gpio_pin36_reg::R) reader structure"]
impl crate::Readable for GPIO_PIN36_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_pin36_reg::W](gpio_pin36_reg::W) writer structure"]
impl crate::Writable for GPIO_PIN36_REG {}
#[doc = "GPIO_PIN36_REG"]
pub mod gpio_pin36_reg;
#[doc = "GPIO_PIN37_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_pin37_reg](gpio_pin37_reg) module"]
pub type GPIO_PIN37_REG = crate::Reg<u32, _GPIO_PIN37_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PIN37_REG;
#[doc = "`read()` method returns [gpio_pin37_reg::R](gpio_pin37_reg::R) reader structure"]
impl crate::Readable for GPIO_PIN37_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_pin37_reg::W](gpio_pin37_reg::W) writer structure"]
impl crate::Writable for GPIO_PIN37_REG {}
#[doc = "GPIO_PIN37_REG"]
pub mod gpio_pin37_reg;
#[doc = "GPIO_PIN38_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_pin38_reg](gpio_pin38_reg) module"]
pub type GPIO_PIN38_REG = crate::Reg<u32, _GPIO_PIN38_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PIN38_REG;
#[doc = "`read()` method returns [gpio_pin38_reg::R](gpio_pin38_reg::R) reader structure"]
impl crate::Readable for GPIO_PIN38_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_pin38_reg::W](gpio_pin38_reg::W) writer structure"]
impl crate::Writable for GPIO_PIN38_REG {}
#[doc = "GPIO_PIN38_REG"]
pub mod gpio_pin38_reg;
#[doc = "GPIO_PIN39_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_pin39_reg](gpio_pin39_reg) module"]
pub type GPIO_PIN39_REG = crate::Reg<u32, _GPIO_PIN39_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PIN39_REG;
#[doc = "`read()` method returns [gpio_pin39_reg::R](gpio_pin39_reg::R) reader structure"]
impl crate::Readable for GPIO_PIN39_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_pin39_reg::W](gpio_pin39_reg::W) writer structure"]
impl crate::Writable for GPIO_PIN39_REG {}
#[doc = "GPIO_PIN39_REG"]
pub mod gpio_pin39_reg;
#[doc = "GPIO_cali_conf_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_cali_conf_reg](gpio_cali_conf_reg) module"]
pub type GPIO_CALI_CONF_REG = crate::Reg<u32, _GPIO_CALI_CONF_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_CALI_CONF_REG;
#[doc = "`read()` method returns [gpio_cali_conf_reg::R](gpio_cali_conf_reg::R) reader structure"]
impl crate::Readable for GPIO_CALI_CONF_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_cali_conf_reg::W](gpio_cali_conf_reg::W) writer structure"]
impl crate::Writable for GPIO_CALI_CONF_REG {}
#[doc = "GPIO_cali_conf_REG"]
pub mod gpio_cali_conf_reg;
#[doc = "GPIO_cali_data_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_cali_data_reg](gpio_cali_data_reg) module"]
pub type GPIO_CALI_DATA_REG = crate::Reg<u32, _GPIO_CALI_DATA_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_CALI_DATA_REG;
#[doc = "`read()` method returns [gpio_cali_data_reg::R](gpio_cali_data_reg::R) reader structure"]
impl crate::Readable for GPIO_CALI_DATA_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_cali_data_reg::W](gpio_cali_data_reg::W) writer structure"]
impl crate::Writable for GPIO_CALI_DATA_REG {}
#[doc = "GPIO_cali_data_REG"]
pub mod gpio_cali_data_reg;
#[doc = "GPIO_FUNC0_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func0_in_sel_cfg_reg](gpio_func0_in_sel_cfg_reg) module"]
pub type GPIO_FUNC0_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC0_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC0_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func0_in_sel_cfg_reg::R](gpio_func0_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC0_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func0_in_sel_cfg_reg::W](gpio_func0_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC0_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC0_IN_SEL_CFG_REG"]
pub mod gpio_func0_in_sel_cfg_reg;
#[doc = "GPIO_FUNC1_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func1_in_sel_cfg_reg](gpio_func1_in_sel_cfg_reg) module"]
pub type GPIO_FUNC1_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC1_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC1_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func1_in_sel_cfg_reg::R](gpio_func1_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC1_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func1_in_sel_cfg_reg::W](gpio_func1_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC1_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC1_IN_SEL_CFG_REG"]
pub mod gpio_func1_in_sel_cfg_reg;
#[doc = "GPIO_FUNC2_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func2_in_sel_cfg_reg](gpio_func2_in_sel_cfg_reg) module"]
pub type GPIO_FUNC2_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC2_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC2_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func2_in_sel_cfg_reg::R](gpio_func2_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC2_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func2_in_sel_cfg_reg::W](gpio_func2_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC2_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC2_IN_SEL_CFG_REG"]
pub mod gpio_func2_in_sel_cfg_reg;
#[doc = "GPIO_FUNC3_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func3_in_sel_cfg_reg](gpio_func3_in_sel_cfg_reg) module"]
pub type GPIO_FUNC3_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC3_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC3_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func3_in_sel_cfg_reg::R](gpio_func3_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC3_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func3_in_sel_cfg_reg::W](gpio_func3_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC3_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC3_IN_SEL_CFG_REG"]
pub mod gpio_func3_in_sel_cfg_reg;
#[doc = "GPIO_FUNC4_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func4_in_sel_cfg_reg](gpio_func4_in_sel_cfg_reg) module"]
pub type GPIO_FUNC4_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC4_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC4_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func4_in_sel_cfg_reg::R](gpio_func4_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC4_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func4_in_sel_cfg_reg::W](gpio_func4_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC4_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC4_IN_SEL_CFG_REG"]
pub mod gpio_func4_in_sel_cfg_reg;
#[doc = "GPIO_FUNC5_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func5_in_sel_cfg_reg](gpio_func5_in_sel_cfg_reg) module"]
pub type GPIO_FUNC5_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC5_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC5_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func5_in_sel_cfg_reg::R](gpio_func5_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC5_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func5_in_sel_cfg_reg::W](gpio_func5_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC5_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC5_IN_SEL_CFG_REG"]
pub mod gpio_func5_in_sel_cfg_reg;
#[doc = "GPIO_FUNC6_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func6_in_sel_cfg_reg](gpio_func6_in_sel_cfg_reg) module"]
pub type GPIO_FUNC6_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC6_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC6_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func6_in_sel_cfg_reg::R](gpio_func6_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC6_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func6_in_sel_cfg_reg::W](gpio_func6_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC6_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC6_IN_SEL_CFG_REG"]
pub mod gpio_func6_in_sel_cfg_reg;
#[doc = "GPIO_FUNC7_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func7_in_sel_cfg_reg](gpio_func7_in_sel_cfg_reg) module"]
pub type GPIO_FUNC7_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC7_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC7_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func7_in_sel_cfg_reg::R](gpio_func7_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC7_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func7_in_sel_cfg_reg::W](gpio_func7_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC7_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC7_IN_SEL_CFG_REG"]
pub mod gpio_func7_in_sel_cfg_reg;
#[doc = "GPIO_FUNC8_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func8_in_sel_cfg_reg](gpio_func8_in_sel_cfg_reg) module"]
pub type GPIO_FUNC8_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC8_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC8_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func8_in_sel_cfg_reg::R](gpio_func8_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC8_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func8_in_sel_cfg_reg::W](gpio_func8_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC8_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC8_IN_SEL_CFG_REG"]
pub mod gpio_func8_in_sel_cfg_reg;
#[doc = "GPIO_FUNC9_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func9_in_sel_cfg_reg](gpio_func9_in_sel_cfg_reg) module"]
pub type GPIO_FUNC9_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC9_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC9_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func9_in_sel_cfg_reg::R](gpio_func9_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC9_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func9_in_sel_cfg_reg::W](gpio_func9_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC9_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC9_IN_SEL_CFG_REG"]
pub mod gpio_func9_in_sel_cfg_reg;
#[doc = "GPIO_FUNC10_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func10_in_sel_cfg_reg](gpio_func10_in_sel_cfg_reg) module"]
pub type GPIO_FUNC10_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC10_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC10_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func10_in_sel_cfg_reg::R](gpio_func10_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC10_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func10_in_sel_cfg_reg::W](gpio_func10_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC10_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC10_IN_SEL_CFG_REG"]
pub mod gpio_func10_in_sel_cfg_reg;
#[doc = "GPIO_FUNC11_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func11_in_sel_cfg_reg](gpio_func11_in_sel_cfg_reg) module"]
pub type GPIO_FUNC11_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC11_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC11_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func11_in_sel_cfg_reg::R](gpio_func11_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC11_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func11_in_sel_cfg_reg::W](gpio_func11_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC11_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC11_IN_SEL_CFG_REG"]
pub mod gpio_func11_in_sel_cfg_reg;
#[doc = "GPIO_FUNC12_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func12_in_sel_cfg_reg](gpio_func12_in_sel_cfg_reg) module"]
pub type GPIO_FUNC12_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC12_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC12_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func12_in_sel_cfg_reg::R](gpio_func12_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC12_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func12_in_sel_cfg_reg::W](gpio_func12_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC12_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC12_IN_SEL_CFG_REG"]
pub mod gpio_func12_in_sel_cfg_reg;
#[doc = "GPIO_FUNC13_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func13_in_sel_cfg_reg](gpio_func13_in_sel_cfg_reg) module"]
pub type GPIO_FUNC13_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC13_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC13_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func13_in_sel_cfg_reg::R](gpio_func13_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC13_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func13_in_sel_cfg_reg::W](gpio_func13_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC13_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC13_IN_SEL_CFG_REG"]
pub mod gpio_func13_in_sel_cfg_reg;
#[doc = "GPIO_FUNC14_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func14_in_sel_cfg_reg](gpio_func14_in_sel_cfg_reg) module"]
pub type GPIO_FUNC14_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC14_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC14_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func14_in_sel_cfg_reg::R](gpio_func14_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC14_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func14_in_sel_cfg_reg::W](gpio_func14_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC14_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC14_IN_SEL_CFG_REG"]
pub mod gpio_func14_in_sel_cfg_reg;
#[doc = "GPIO_FUNC15_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func15_in_sel_cfg_reg](gpio_func15_in_sel_cfg_reg) module"]
pub type GPIO_FUNC15_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC15_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC15_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func15_in_sel_cfg_reg::R](gpio_func15_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC15_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func15_in_sel_cfg_reg::W](gpio_func15_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC15_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC15_IN_SEL_CFG_REG"]
pub mod gpio_func15_in_sel_cfg_reg;
#[doc = "GPIO_FUNC16_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func16_in_sel_cfg_reg](gpio_func16_in_sel_cfg_reg) module"]
pub type GPIO_FUNC16_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC16_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC16_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func16_in_sel_cfg_reg::R](gpio_func16_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC16_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func16_in_sel_cfg_reg::W](gpio_func16_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC16_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC16_IN_SEL_CFG_REG"]
pub mod gpio_func16_in_sel_cfg_reg;
#[doc = "GPIO_FUNC17_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func17_in_sel_cfg_reg](gpio_func17_in_sel_cfg_reg) module"]
pub type GPIO_FUNC17_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC17_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC17_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func17_in_sel_cfg_reg::R](gpio_func17_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC17_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func17_in_sel_cfg_reg::W](gpio_func17_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC17_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC17_IN_SEL_CFG_REG"]
pub mod gpio_func17_in_sel_cfg_reg;
#[doc = "GPIO_FUNC18_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func18_in_sel_cfg_reg](gpio_func18_in_sel_cfg_reg) module"]
pub type GPIO_FUNC18_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC18_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC18_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func18_in_sel_cfg_reg::R](gpio_func18_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC18_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func18_in_sel_cfg_reg::W](gpio_func18_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC18_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC18_IN_SEL_CFG_REG"]
pub mod gpio_func18_in_sel_cfg_reg;
#[doc = "GPIO_FUNC19_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func19_in_sel_cfg_reg](gpio_func19_in_sel_cfg_reg) module"]
pub type GPIO_FUNC19_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC19_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC19_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func19_in_sel_cfg_reg::R](gpio_func19_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC19_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func19_in_sel_cfg_reg::W](gpio_func19_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC19_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC19_IN_SEL_CFG_REG"]
pub mod gpio_func19_in_sel_cfg_reg;
#[doc = "GPIO_FUNC20_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func20_in_sel_cfg_reg](gpio_func20_in_sel_cfg_reg) module"]
pub type GPIO_FUNC20_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC20_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC20_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func20_in_sel_cfg_reg::R](gpio_func20_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC20_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func20_in_sel_cfg_reg::W](gpio_func20_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC20_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC20_IN_SEL_CFG_REG"]
pub mod gpio_func20_in_sel_cfg_reg;
#[doc = "GPIO_FUNC21_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func21_in_sel_cfg_reg](gpio_func21_in_sel_cfg_reg) module"]
pub type GPIO_FUNC21_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC21_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC21_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func21_in_sel_cfg_reg::R](gpio_func21_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC21_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func21_in_sel_cfg_reg::W](gpio_func21_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC21_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC21_IN_SEL_CFG_REG"]
pub mod gpio_func21_in_sel_cfg_reg;
#[doc = "GPIO_FUNC22_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func22_in_sel_cfg_reg](gpio_func22_in_sel_cfg_reg) module"]
pub type GPIO_FUNC22_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC22_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC22_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func22_in_sel_cfg_reg::R](gpio_func22_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC22_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func22_in_sel_cfg_reg::W](gpio_func22_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC22_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC22_IN_SEL_CFG_REG"]
pub mod gpio_func22_in_sel_cfg_reg;
#[doc = "GPIO_FUNC23_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func23_in_sel_cfg_reg](gpio_func23_in_sel_cfg_reg) module"]
pub type GPIO_FUNC23_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC23_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC23_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func23_in_sel_cfg_reg::R](gpio_func23_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC23_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func23_in_sel_cfg_reg::W](gpio_func23_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC23_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC23_IN_SEL_CFG_REG"]
pub mod gpio_func23_in_sel_cfg_reg;
#[doc = "GPIO_FUNC24_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func24_in_sel_cfg_reg](gpio_func24_in_sel_cfg_reg) module"]
pub type GPIO_FUNC24_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC24_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC24_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func24_in_sel_cfg_reg::R](gpio_func24_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC24_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func24_in_sel_cfg_reg::W](gpio_func24_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC24_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC24_IN_SEL_CFG_REG"]
pub mod gpio_func24_in_sel_cfg_reg;
#[doc = "GPIO_FUNC25_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func25_in_sel_cfg_reg](gpio_func25_in_sel_cfg_reg) module"]
pub type GPIO_FUNC25_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC25_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC25_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func25_in_sel_cfg_reg::R](gpio_func25_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC25_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func25_in_sel_cfg_reg::W](gpio_func25_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC25_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC25_IN_SEL_CFG_REG"]
pub mod gpio_func25_in_sel_cfg_reg;
#[doc = "GPIO_FUNC26_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func26_in_sel_cfg_reg](gpio_func26_in_sel_cfg_reg) module"]
pub type GPIO_FUNC26_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC26_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC26_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func26_in_sel_cfg_reg::R](gpio_func26_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC26_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func26_in_sel_cfg_reg::W](gpio_func26_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC26_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC26_IN_SEL_CFG_REG"]
pub mod gpio_func26_in_sel_cfg_reg;
#[doc = "GPIO_FUNC27_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func27_in_sel_cfg_reg](gpio_func27_in_sel_cfg_reg) module"]
pub type GPIO_FUNC27_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC27_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC27_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func27_in_sel_cfg_reg::R](gpio_func27_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC27_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func27_in_sel_cfg_reg::W](gpio_func27_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC27_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC27_IN_SEL_CFG_REG"]
pub mod gpio_func27_in_sel_cfg_reg;
#[doc = "GPIO_FUNC28_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func28_in_sel_cfg_reg](gpio_func28_in_sel_cfg_reg) module"]
pub type GPIO_FUNC28_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC28_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC28_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func28_in_sel_cfg_reg::R](gpio_func28_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC28_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func28_in_sel_cfg_reg::W](gpio_func28_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC28_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC28_IN_SEL_CFG_REG"]
pub mod gpio_func28_in_sel_cfg_reg;
#[doc = "GPIO_FUNC29_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func29_in_sel_cfg_reg](gpio_func29_in_sel_cfg_reg) module"]
pub type GPIO_FUNC29_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC29_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC29_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func29_in_sel_cfg_reg::R](gpio_func29_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC29_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func29_in_sel_cfg_reg::W](gpio_func29_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC29_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC29_IN_SEL_CFG_REG"]
pub mod gpio_func29_in_sel_cfg_reg;
#[doc = "GPIO_FUNC30_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func30_in_sel_cfg_reg](gpio_func30_in_sel_cfg_reg) module"]
pub type GPIO_FUNC30_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC30_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC30_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func30_in_sel_cfg_reg::R](gpio_func30_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC30_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func30_in_sel_cfg_reg::W](gpio_func30_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC30_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC30_IN_SEL_CFG_REG"]
pub mod gpio_func30_in_sel_cfg_reg;
#[doc = "GPIO_FUNC31_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func31_in_sel_cfg_reg](gpio_func31_in_sel_cfg_reg) module"]
pub type GPIO_FUNC31_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC31_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC31_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func31_in_sel_cfg_reg::R](gpio_func31_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC31_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func31_in_sel_cfg_reg::W](gpio_func31_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC31_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC31_IN_SEL_CFG_REG"]
pub mod gpio_func31_in_sel_cfg_reg;
#[doc = "GPIO_FUNC32_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func32_in_sel_cfg_reg](gpio_func32_in_sel_cfg_reg) module"]
pub type GPIO_FUNC32_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC32_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC32_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func32_in_sel_cfg_reg::R](gpio_func32_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC32_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func32_in_sel_cfg_reg::W](gpio_func32_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC32_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC32_IN_SEL_CFG_REG"]
pub mod gpio_func32_in_sel_cfg_reg;
#[doc = "GPIO_FUNC33_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func33_in_sel_cfg_reg](gpio_func33_in_sel_cfg_reg) module"]
pub type GPIO_FUNC33_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC33_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC33_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func33_in_sel_cfg_reg::R](gpio_func33_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC33_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func33_in_sel_cfg_reg::W](gpio_func33_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC33_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC33_IN_SEL_CFG_REG"]
pub mod gpio_func33_in_sel_cfg_reg;
#[doc = "GPIO_FUNC34_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func34_in_sel_cfg_reg](gpio_func34_in_sel_cfg_reg) module"]
pub type GPIO_FUNC34_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC34_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC34_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func34_in_sel_cfg_reg::R](gpio_func34_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC34_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func34_in_sel_cfg_reg::W](gpio_func34_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC34_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC34_IN_SEL_CFG_REG"]
pub mod gpio_func34_in_sel_cfg_reg;
#[doc = "GPIO_FUNC35_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func35_in_sel_cfg_reg](gpio_func35_in_sel_cfg_reg) module"]
pub type GPIO_FUNC35_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC35_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC35_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func35_in_sel_cfg_reg::R](gpio_func35_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC35_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func35_in_sel_cfg_reg::W](gpio_func35_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC35_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC35_IN_SEL_CFG_REG"]
pub mod gpio_func35_in_sel_cfg_reg;
#[doc = "GPIO_FUNC36_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func36_in_sel_cfg_reg](gpio_func36_in_sel_cfg_reg) module"]
pub type GPIO_FUNC36_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC36_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC36_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func36_in_sel_cfg_reg::R](gpio_func36_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC36_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func36_in_sel_cfg_reg::W](gpio_func36_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC36_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC36_IN_SEL_CFG_REG"]
pub mod gpio_func36_in_sel_cfg_reg;
#[doc = "GPIO_FUNC37_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func37_in_sel_cfg_reg](gpio_func37_in_sel_cfg_reg) module"]
pub type GPIO_FUNC37_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC37_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC37_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func37_in_sel_cfg_reg::R](gpio_func37_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC37_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func37_in_sel_cfg_reg::W](gpio_func37_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC37_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC37_IN_SEL_CFG_REG"]
pub mod gpio_func37_in_sel_cfg_reg;
#[doc = "GPIO_FUNC38_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func38_in_sel_cfg_reg](gpio_func38_in_sel_cfg_reg) module"]
pub type GPIO_FUNC38_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC38_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC38_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func38_in_sel_cfg_reg::R](gpio_func38_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC38_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func38_in_sel_cfg_reg::W](gpio_func38_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC38_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC38_IN_SEL_CFG_REG"]
pub mod gpio_func38_in_sel_cfg_reg;
#[doc = "GPIO_FUNC39_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func39_in_sel_cfg_reg](gpio_func39_in_sel_cfg_reg) module"]
pub type GPIO_FUNC39_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC39_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC39_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func39_in_sel_cfg_reg::R](gpio_func39_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC39_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func39_in_sel_cfg_reg::W](gpio_func39_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC39_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC39_IN_SEL_CFG_REG"]
pub mod gpio_func39_in_sel_cfg_reg;
#[doc = "GPIO_FUNC40_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func40_in_sel_cfg_reg](gpio_func40_in_sel_cfg_reg) module"]
pub type GPIO_FUNC40_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC40_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC40_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func40_in_sel_cfg_reg::R](gpio_func40_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC40_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func40_in_sel_cfg_reg::W](gpio_func40_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC40_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC40_IN_SEL_CFG_REG"]
pub mod gpio_func40_in_sel_cfg_reg;
#[doc = "GPIO_FUNC41_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func41_in_sel_cfg_reg](gpio_func41_in_sel_cfg_reg) module"]
pub type GPIO_FUNC41_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC41_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC41_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func41_in_sel_cfg_reg::R](gpio_func41_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC41_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func41_in_sel_cfg_reg::W](gpio_func41_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC41_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC41_IN_SEL_CFG_REG"]
pub mod gpio_func41_in_sel_cfg_reg;
#[doc = "GPIO_FUNC42_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func42_in_sel_cfg_reg](gpio_func42_in_sel_cfg_reg) module"]
pub type GPIO_FUNC42_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC42_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC42_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func42_in_sel_cfg_reg::R](gpio_func42_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC42_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func42_in_sel_cfg_reg::W](gpio_func42_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC42_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC42_IN_SEL_CFG_REG"]
pub mod gpio_func42_in_sel_cfg_reg;
#[doc = "GPIO_FUNC43_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func43_in_sel_cfg_reg](gpio_func43_in_sel_cfg_reg) module"]
pub type GPIO_FUNC43_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC43_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC43_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func43_in_sel_cfg_reg::R](gpio_func43_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC43_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func43_in_sel_cfg_reg::W](gpio_func43_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC43_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC43_IN_SEL_CFG_REG"]
pub mod gpio_func43_in_sel_cfg_reg;
#[doc = "GPIO_FUNC44_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func44_in_sel_cfg_reg](gpio_func44_in_sel_cfg_reg) module"]
pub type GPIO_FUNC44_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC44_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC44_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func44_in_sel_cfg_reg::R](gpio_func44_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC44_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func44_in_sel_cfg_reg::W](gpio_func44_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC44_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC44_IN_SEL_CFG_REG"]
pub mod gpio_func44_in_sel_cfg_reg;
#[doc = "GPIO_FUNC45_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func45_in_sel_cfg_reg](gpio_func45_in_sel_cfg_reg) module"]
pub type GPIO_FUNC45_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC45_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC45_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func45_in_sel_cfg_reg::R](gpio_func45_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC45_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func45_in_sel_cfg_reg::W](gpio_func45_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC45_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC45_IN_SEL_CFG_REG"]
pub mod gpio_func45_in_sel_cfg_reg;
#[doc = "GPIO_FUNC46_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func46_in_sel_cfg_reg](gpio_func46_in_sel_cfg_reg) module"]
pub type GPIO_FUNC46_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC46_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC46_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func46_in_sel_cfg_reg::R](gpio_func46_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC46_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func46_in_sel_cfg_reg::W](gpio_func46_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC46_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC46_IN_SEL_CFG_REG"]
pub mod gpio_func46_in_sel_cfg_reg;
#[doc = "GPIO_FUNC47_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func47_in_sel_cfg_reg](gpio_func47_in_sel_cfg_reg) module"]
pub type GPIO_FUNC47_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC47_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC47_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func47_in_sel_cfg_reg::R](gpio_func47_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC47_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func47_in_sel_cfg_reg::W](gpio_func47_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC47_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC47_IN_SEL_CFG_REG"]
pub mod gpio_func47_in_sel_cfg_reg;
#[doc = "GPIO_FUNC48_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func48_in_sel_cfg_reg](gpio_func48_in_sel_cfg_reg) module"]
pub type GPIO_FUNC48_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC48_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC48_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func48_in_sel_cfg_reg::R](gpio_func48_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC48_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func48_in_sel_cfg_reg::W](gpio_func48_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC48_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC48_IN_SEL_CFG_REG"]
pub mod gpio_func48_in_sel_cfg_reg;
#[doc = "GPIO_FUNC49_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func49_in_sel_cfg_reg](gpio_func49_in_sel_cfg_reg) module"]
pub type GPIO_FUNC49_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC49_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC49_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func49_in_sel_cfg_reg::R](gpio_func49_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC49_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func49_in_sel_cfg_reg::W](gpio_func49_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC49_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC49_IN_SEL_CFG_REG"]
pub mod gpio_func49_in_sel_cfg_reg;
#[doc = "GPIO_FUNC50_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func50_in_sel_cfg_reg](gpio_func50_in_sel_cfg_reg) module"]
pub type GPIO_FUNC50_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC50_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC50_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func50_in_sel_cfg_reg::R](gpio_func50_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC50_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func50_in_sel_cfg_reg::W](gpio_func50_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC50_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC50_IN_SEL_CFG_REG"]
pub mod gpio_func50_in_sel_cfg_reg;
#[doc = "GPIO_FUNC51_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func51_in_sel_cfg_reg](gpio_func51_in_sel_cfg_reg) module"]
pub type GPIO_FUNC51_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC51_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC51_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func51_in_sel_cfg_reg::R](gpio_func51_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC51_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func51_in_sel_cfg_reg::W](gpio_func51_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC51_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC51_IN_SEL_CFG_REG"]
pub mod gpio_func51_in_sel_cfg_reg;
#[doc = "GPIO_FUNC52_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func52_in_sel_cfg_reg](gpio_func52_in_sel_cfg_reg) module"]
pub type GPIO_FUNC52_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC52_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC52_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func52_in_sel_cfg_reg::R](gpio_func52_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC52_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func52_in_sel_cfg_reg::W](gpio_func52_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC52_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC52_IN_SEL_CFG_REG"]
pub mod gpio_func52_in_sel_cfg_reg;
#[doc = "GPIO_FUNC53_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func53_in_sel_cfg_reg](gpio_func53_in_sel_cfg_reg) module"]
pub type GPIO_FUNC53_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC53_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC53_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func53_in_sel_cfg_reg::R](gpio_func53_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC53_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func53_in_sel_cfg_reg::W](gpio_func53_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC53_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC53_IN_SEL_CFG_REG"]
pub mod gpio_func53_in_sel_cfg_reg;
#[doc = "GPIO_FUNC54_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func54_in_sel_cfg_reg](gpio_func54_in_sel_cfg_reg) module"]
pub type GPIO_FUNC54_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC54_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC54_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func54_in_sel_cfg_reg::R](gpio_func54_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC54_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func54_in_sel_cfg_reg::W](gpio_func54_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC54_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC54_IN_SEL_CFG_REG"]
pub mod gpio_func54_in_sel_cfg_reg;
#[doc = "GPIO_FUNC55_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func55_in_sel_cfg_reg](gpio_func55_in_sel_cfg_reg) module"]
pub type GPIO_FUNC55_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC55_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC55_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func55_in_sel_cfg_reg::R](gpio_func55_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC55_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func55_in_sel_cfg_reg::W](gpio_func55_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC55_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC55_IN_SEL_CFG_REG"]
pub mod gpio_func55_in_sel_cfg_reg;
#[doc = "GPIO_FUNC56_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func56_in_sel_cfg_reg](gpio_func56_in_sel_cfg_reg) module"]
pub type GPIO_FUNC56_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC56_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC56_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func56_in_sel_cfg_reg::R](gpio_func56_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC56_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func56_in_sel_cfg_reg::W](gpio_func56_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC56_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC56_IN_SEL_CFG_REG"]
pub mod gpio_func56_in_sel_cfg_reg;
#[doc = "GPIO_FUNC57_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func57_in_sel_cfg_reg](gpio_func57_in_sel_cfg_reg) module"]
pub type GPIO_FUNC57_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC57_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC57_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func57_in_sel_cfg_reg::R](gpio_func57_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC57_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func57_in_sel_cfg_reg::W](gpio_func57_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC57_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC57_IN_SEL_CFG_REG"]
pub mod gpio_func57_in_sel_cfg_reg;
#[doc = "GPIO_FUNC58_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func58_in_sel_cfg_reg](gpio_func58_in_sel_cfg_reg) module"]
pub type GPIO_FUNC58_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC58_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC58_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func58_in_sel_cfg_reg::R](gpio_func58_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC58_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func58_in_sel_cfg_reg::W](gpio_func58_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC58_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC58_IN_SEL_CFG_REG"]
pub mod gpio_func58_in_sel_cfg_reg;
#[doc = "GPIO_FUNC59_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func59_in_sel_cfg_reg](gpio_func59_in_sel_cfg_reg) module"]
pub type GPIO_FUNC59_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC59_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC59_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func59_in_sel_cfg_reg::R](gpio_func59_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC59_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func59_in_sel_cfg_reg::W](gpio_func59_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC59_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC59_IN_SEL_CFG_REG"]
pub mod gpio_func59_in_sel_cfg_reg;
#[doc = "GPIO_FUNC60_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func60_in_sel_cfg_reg](gpio_func60_in_sel_cfg_reg) module"]
pub type GPIO_FUNC60_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC60_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC60_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func60_in_sel_cfg_reg::R](gpio_func60_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC60_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func60_in_sel_cfg_reg::W](gpio_func60_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC60_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC60_IN_SEL_CFG_REG"]
pub mod gpio_func60_in_sel_cfg_reg;
#[doc = "GPIO_FUNC61_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func61_in_sel_cfg_reg](gpio_func61_in_sel_cfg_reg) module"]
pub type GPIO_FUNC61_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC61_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC61_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func61_in_sel_cfg_reg::R](gpio_func61_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC61_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func61_in_sel_cfg_reg::W](gpio_func61_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC61_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC61_IN_SEL_CFG_REG"]
pub mod gpio_func61_in_sel_cfg_reg;
#[doc = "GPIO_FUNC62_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func62_in_sel_cfg_reg](gpio_func62_in_sel_cfg_reg) module"]
pub type GPIO_FUNC62_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC62_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC62_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func62_in_sel_cfg_reg::R](gpio_func62_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC62_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func62_in_sel_cfg_reg::W](gpio_func62_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC62_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC62_IN_SEL_CFG_REG"]
pub mod gpio_func62_in_sel_cfg_reg;
#[doc = "GPIO_FUNC63_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func63_in_sel_cfg_reg](gpio_func63_in_sel_cfg_reg) module"]
pub type GPIO_FUNC63_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC63_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC63_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func63_in_sel_cfg_reg::R](gpio_func63_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC63_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func63_in_sel_cfg_reg::W](gpio_func63_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC63_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC63_IN_SEL_CFG_REG"]
pub mod gpio_func63_in_sel_cfg_reg;
#[doc = "GPIO_FUNC64_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func64_in_sel_cfg_reg](gpio_func64_in_sel_cfg_reg) module"]
pub type GPIO_FUNC64_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC64_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC64_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func64_in_sel_cfg_reg::R](gpio_func64_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC64_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func64_in_sel_cfg_reg::W](gpio_func64_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC64_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC64_IN_SEL_CFG_REG"]
pub mod gpio_func64_in_sel_cfg_reg;
#[doc = "GPIO_FUNC65_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func65_in_sel_cfg_reg](gpio_func65_in_sel_cfg_reg) module"]
pub type GPIO_FUNC65_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC65_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC65_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func65_in_sel_cfg_reg::R](gpio_func65_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC65_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func65_in_sel_cfg_reg::W](gpio_func65_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC65_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC65_IN_SEL_CFG_REG"]
pub mod gpio_func65_in_sel_cfg_reg;
#[doc = "GPIO_FUNC66_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func66_in_sel_cfg_reg](gpio_func66_in_sel_cfg_reg) module"]
pub type GPIO_FUNC66_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC66_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC66_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func66_in_sel_cfg_reg::R](gpio_func66_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC66_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func66_in_sel_cfg_reg::W](gpio_func66_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC66_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC66_IN_SEL_CFG_REG"]
pub mod gpio_func66_in_sel_cfg_reg;
#[doc = "GPIO_FUNC67_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func67_in_sel_cfg_reg](gpio_func67_in_sel_cfg_reg) module"]
pub type GPIO_FUNC67_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC67_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC67_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func67_in_sel_cfg_reg::R](gpio_func67_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC67_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func67_in_sel_cfg_reg::W](gpio_func67_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC67_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC67_IN_SEL_CFG_REG"]
pub mod gpio_func67_in_sel_cfg_reg;
#[doc = "GPIO_FUNC68_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func68_in_sel_cfg_reg](gpio_func68_in_sel_cfg_reg) module"]
pub type GPIO_FUNC68_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC68_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC68_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func68_in_sel_cfg_reg::R](gpio_func68_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC68_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func68_in_sel_cfg_reg::W](gpio_func68_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC68_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC68_IN_SEL_CFG_REG"]
pub mod gpio_func68_in_sel_cfg_reg;
#[doc = "GPIO_FUNC69_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func69_in_sel_cfg_reg](gpio_func69_in_sel_cfg_reg) module"]
pub type GPIO_FUNC69_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC69_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC69_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func69_in_sel_cfg_reg::R](gpio_func69_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC69_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func69_in_sel_cfg_reg::W](gpio_func69_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC69_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC69_IN_SEL_CFG_REG"]
pub mod gpio_func69_in_sel_cfg_reg;
#[doc = "GPIO_FUNC70_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func70_in_sel_cfg_reg](gpio_func70_in_sel_cfg_reg) module"]
pub type GPIO_FUNC70_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC70_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC70_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func70_in_sel_cfg_reg::R](gpio_func70_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC70_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func70_in_sel_cfg_reg::W](gpio_func70_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC70_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC70_IN_SEL_CFG_REG"]
pub mod gpio_func70_in_sel_cfg_reg;
#[doc = "GPIO_FUNC71_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func71_in_sel_cfg_reg](gpio_func71_in_sel_cfg_reg) module"]
pub type GPIO_FUNC71_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC71_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC71_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func71_in_sel_cfg_reg::R](gpio_func71_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC71_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func71_in_sel_cfg_reg::W](gpio_func71_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC71_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC71_IN_SEL_CFG_REG"]
pub mod gpio_func71_in_sel_cfg_reg;
#[doc = "GPIO_FUNC72_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func72_in_sel_cfg_reg](gpio_func72_in_sel_cfg_reg) module"]
pub type GPIO_FUNC72_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC72_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC72_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func72_in_sel_cfg_reg::R](gpio_func72_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC72_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func72_in_sel_cfg_reg::W](gpio_func72_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC72_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC72_IN_SEL_CFG_REG"]
pub mod gpio_func72_in_sel_cfg_reg;
#[doc = "GPIO_FUNC73_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func73_in_sel_cfg_reg](gpio_func73_in_sel_cfg_reg) module"]
pub type GPIO_FUNC73_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC73_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC73_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func73_in_sel_cfg_reg::R](gpio_func73_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC73_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func73_in_sel_cfg_reg::W](gpio_func73_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC73_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC73_IN_SEL_CFG_REG"]
pub mod gpio_func73_in_sel_cfg_reg;
#[doc = "GPIO_FUNC74_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func74_in_sel_cfg_reg](gpio_func74_in_sel_cfg_reg) module"]
pub type GPIO_FUNC74_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC74_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC74_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func74_in_sel_cfg_reg::R](gpio_func74_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC74_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func74_in_sel_cfg_reg::W](gpio_func74_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC74_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC74_IN_SEL_CFG_REG"]
pub mod gpio_func74_in_sel_cfg_reg;
#[doc = "GPIO_FUNC75_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func75_in_sel_cfg_reg](gpio_func75_in_sel_cfg_reg) module"]
pub type GPIO_FUNC75_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC75_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC75_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func75_in_sel_cfg_reg::R](gpio_func75_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC75_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func75_in_sel_cfg_reg::W](gpio_func75_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC75_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC75_IN_SEL_CFG_REG"]
pub mod gpio_func75_in_sel_cfg_reg;
#[doc = "GPIO_FUNC76_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func76_in_sel_cfg_reg](gpio_func76_in_sel_cfg_reg) module"]
pub type GPIO_FUNC76_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC76_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC76_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func76_in_sel_cfg_reg::R](gpio_func76_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC76_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func76_in_sel_cfg_reg::W](gpio_func76_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC76_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC76_IN_SEL_CFG_REG"]
pub mod gpio_func76_in_sel_cfg_reg;
#[doc = "GPIO_FUNC77_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func77_in_sel_cfg_reg](gpio_func77_in_sel_cfg_reg) module"]
pub type GPIO_FUNC77_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC77_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC77_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func77_in_sel_cfg_reg::R](gpio_func77_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC77_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func77_in_sel_cfg_reg::W](gpio_func77_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC77_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC77_IN_SEL_CFG_REG"]
pub mod gpio_func77_in_sel_cfg_reg;
#[doc = "GPIO_FUNC78_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func78_in_sel_cfg_reg](gpio_func78_in_sel_cfg_reg) module"]
pub type GPIO_FUNC78_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC78_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC78_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func78_in_sel_cfg_reg::R](gpio_func78_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC78_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func78_in_sel_cfg_reg::W](gpio_func78_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC78_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC78_IN_SEL_CFG_REG"]
pub mod gpio_func78_in_sel_cfg_reg;
#[doc = "GPIO_FUNC79_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func79_in_sel_cfg_reg](gpio_func79_in_sel_cfg_reg) module"]
pub type GPIO_FUNC79_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC79_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC79_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func79_in_sel_cfg_reg::R](gpio_func79_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC79_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func79_in_sel_cfg_reg::W](gpio_func79_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC79_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC79_IN_SEL_CFG_REG"]
pub mod gpio_func79_in_sel_cfg_reg;
#[doc = "GPIO_FUNC80_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func80_in_sel_cfg_reg](gpio_func80_in_sel_cfg_reg) module"]
pub type GPIO_FUNC80_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC80_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC80_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func80_in_sel_cfg_reg::R](gpio_func80_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC80_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func80_in_sel_cfg_reg::W](gpio_func80_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC80_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC80_IN_SEL_CFG_REG"]
pub mod gpio_func80_in_sel_cfg_reg;
#[doc = "GPIO_FUNC81_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func81_in_sel_cfg_reg](gpio_func81_in_sel_cfg_reg) module"]
pub type GPIO_FUNC81_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC81_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC81_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func81_in_sel_cfg_reg::R](gpio_func81_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC81_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func81_in_sel_cfg_reg::W](gpio_func81_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC81_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC81_IN_SEL_CFG_REG"]
pub mod gpio_func81_in_sel_cfg_reg;
#[doc = "GPIO_FUNC82_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func82_in_sel_cfg_reg](gpio_func82_in_sel_cfg_reg) module"]
pub type GPIO_FUNC82_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC82_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC82_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func82_in_sel_cfg_reg::R](gpio_func82_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC82_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func82_in_sel_cfg_reg::W](gpio_func82_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC82_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC82_IN_SEL_CFG_REG"]
pub mod gpio_func82_in_sel_cfg_reg;
#[doc = "GPIO_FUNC83_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func83_in_sel_cfg_reg](gpio_func83_in_sel_cfg_reg) module"]
pub type GPIO_FUNC83_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC83_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC83_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func83_in_sel_cfg_reg::R](gpio_func83_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC83_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func83_in_sel_cfg_reg::W](gpio_func83_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC83_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC83_IN_SEL_CFG_REG"]
pub mod gpio_func83_in_sel_cfg_reg;
#[doc = "GPIO_FUNC84_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func84_in_sel_cfg_reg](gpio_func84_in_sel_cfg_reg) module"]
pub type GPIO_FUNC84_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC84_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC84_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func84_in_sel_cfg_reg::R](gpio_func84_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC84_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func84_in_sel_cfg_reg::W](gpio_func84_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC84_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC84_IN_SEL_CFG_REG"]
pub mod gpio_func84_in_sel_cfg_reg;
#[doc = "GPIO_FUNC85_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func85_in_sel_cfg_reg](gpio_func85_in_sel_cfg_reg) module"]
pub type GPIO_FUNC85_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC85_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC85_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func85_in_sel_cfg_reg::R](gpio_func85_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC85_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func85_in_sel_cfg_reg::W](gpio_func85_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC85_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC85_IN_SEL_CFG_REG"]
pub mod gpio_func85_in_sel_cfg_reg;
#[doc = "GPIO_FUNC86_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func86_in_sel_cfg_reg](gpio_func86_in_sel_cfg_reg) module"]
pub type GPIO_FUNC86_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC86_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC86_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func86_in_sel_cfg_reg::R](gpio_func86_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC86_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func86_in_sel_cfg_reg::W](gpio_func86_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC86_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC86_IN_SEL_CFG_REG"]
pub mod gpio_func86_in_sel_cfg_reg;
#[doc = "GPIO_FUNC87_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func87_in_sel_cfg_reg](gpio_func87_in_sel_cfg_reg) module"]
pub type GPIO_FUNC87_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC87_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC87_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func87_in_sel_cfg_reg::R](gpio_func87_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC87_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func87_in_sel_cfg_reg::W](gpio_func87_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC87_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC87_IN_SEL_CFG_REG"]
pub mod gpio_func87_in_sel_cfg_reg;
#[doc = "GPIO_FUNC88_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func88_in_sel_cfg_reg](gpio_func88_in_sel_cfg_reg) module"]
pub type GPIO_FUNC88_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC88_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC88_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func88_in_sel_cfg_reg::R](gpio_func88_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC88_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func88_in_sel_cfg_reg::W](gpio_func88_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC88_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC88_IN_SEL_CFG_REG"]
pub mod gpio_func88_in_sel_cfg_reg;
#[doc = "GPIO_FUNC89_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func89_in_sel_cfg_reg](gpio_func89_in_sel_cfg_reg) module"]
pub type GPIO_FUNC89_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC89_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC89_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func89_in_sel_cfg_reg::R](gpio_func89_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC89_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func89_in_sel_cfg_reg::W](gpio_func89_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC89_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC89_IN_SEL_CFG_REG"]
pub mod gpio_func89_in_sel_cfg_reg;
#[doc = "GPIO_FUNC90_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func90_in_sel_cfg_reg](gpio_func90_in_sel_cfg_reg) module"]
pub type GPIO_FUNC90_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC90_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC90_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func90_in_sel_cfg_reg::R](gpio_func90_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC90_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func90_in_sel_cfg_reg::W](gpio_func90_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC90_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC90_IN_SEL_CFG_REG"]
pub mod gpio_func90_in_sel_cfg_reg;
#[doc = "GPIO_FUNC91_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func91_in_sel_cfg_reg](gpio_func91_in_sel_cfg_reg) module"]
pub type GPIO_FUNC91_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC91_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC91_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func91_in_sel_cfg_reg::R](gpio_func91_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC91_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func91_in_sel_cfg_reg::W](gpio_func91_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC91_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC91_IN_SEL_CFG_REG"]
pub mod gpio_func91_in_sel_cfg_reg;
#[doc = "GPIO_FUNC92_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func92_in_sel_cfg_reg](gpio_func92_in_sel_cfg_reg) module"]
pub type GPIO_FUNC92_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC92_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC92_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func92_in_sel_cfg_reg::R](gpio_func92_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC92_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func92_in_sel_cfg_reg::W](gpio_func92_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC92_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC92_IN_SEL_CFG_REG"]
pub mod gpio_func92_in_sel_cfg_reg;
#[doc = "GPIO_FUNC93_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func93_in_sel_cfg_reg](gpio_func93_in_sel_cfg_reg) module"]
pub type GPIO_FUNC93_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC93_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC93_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func93_in_sel_cfg_reg::R](gpio_func93_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC93_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func93_in_sel_cfg_reg::W](gpio_func93_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC93_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC93_IN_SEL_CFG_REG"]
pub mod gpio_func93_in_sel_cfg_reg;
#[doc = "GPIO_FUNC94_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func94_in_sel_cfg_reg](gpio_func94_in_sel_cfg_reg) module"]
pub type GPIO_FUNC94_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC94_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC94_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func94_in_sel_cfg_reg::R](gpio_func94_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC94_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func94_in_sel_cfg_reg::W](gpio_func94_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC94_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC94_IN_SEL_CFG_REG"]
pub mod gpio_func94_in_sel_cfg_reg;
#[doc = "GPIO_FUNC95_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func95_in_sel_cfg_reg](gpio_func95_in_sel_cfg_reg) module"]
pub type GPIO_FUNC95_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC95_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC95_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func95_in_sel_cfg_reg::R](gpio_func95_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC95_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func95_in_sel_cfg_reg::W](gpio_func95_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC95_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC95_IN_SEL_CFG_REG"]
pub mod gpio_func95_in_sel_cfg_reg;
#[doc = "GPIO_FUNC96_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func96_in_sel_cfg_reg](gpio_func96_in_sel_cfg_reg) module"]
pub type GPIO_FUNC96_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC96_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC96_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func96_in_sel_cfg_reg::R](gpio_func96_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC96_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func96_in_sel_cfg_reg::W](gpio_func96_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC96_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC96_IN_SEL_CFG_REG"]
pub mod gpio_func96_in_sel_cfg_reg;
#[doc = "GPIO_FUNC97_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func97_in_sel_cfg_reg](gpio_func97_in_sel_cfg_reg) module"]
pub type GPIO_FUNC97_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC97_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC97_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func97_in_sel_cfg_reg::R](gpio_func97_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC97_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func97_in_sel_cfg_reg::W](gpio_func97_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC97_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC97_IN_SEL_CFG_REG"]
pub mod gpio_func97_in_sel_cfg_reg;
#[doc = "GPIO_FUNC98_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func98_in_sel_cfg_reg](gpio_func98_in_sel_cfg_reg) module"]
pub type GPIO_FUNC98_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC98_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC98_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func98_in_sel_cfg_reg::R](gpio_func98_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC98_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func98_in_sel_cfg_reg::W](gpio_func98_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC98_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC98_IN_SEL_CFG_REG"]
pub mod gpio_func98_in_sel_cfg_reg;
#[doc = "GPIO_FUNC99_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func99_in_sel_cfg_reg](gpio_func99_in_sel_cfg_reg) module"]
pub type GPIO_FUNC99_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC99_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC99_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func99_in_sel_cfg_reg::R](gpio_func99_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC99_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func99_in_sel_cfg_reg::W](gpio_func99_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC99_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC99_IN_SEL_CFG_REG"]
pub mod gpio_func99_in_sel_cfg_reg;
#[doc = "GPIO_FUNC100_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func100_in_sel_cfg_reg](gpio_func100_in_sel_cfg_reg) module"]
pub type GPIO_FUNC100_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC100_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC100_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func100_in_sel_cfg_reg::R](gpio_func100_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC100_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func100_in_sel_cfg_reg::W](gpio_func100_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC100_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC100_IN_SEL_CFG_REG"]
pub mod gpio_func100_in_sel_cfg_reg;
#[doc = "GPIO_FUNC101_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func101_in_sel_cfg_reg](gpio_func101_in_sel_cfg_reg) module"]
pub type GPIO_FUNC101_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC101_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC101_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func101_in_sel_cfg_reg::R](gpio_func101_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC101_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func101_in_sel_cfg_reg::W](gpio_func101_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC101_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC101_IN_SEL_CFG_REG"]
pub mod gpio_func101_in_sel_cfg_reg;
#[doc = "GPIO_FUNC102_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func102_in_sel_cfg_reg](gpio_func102_in_sel_cfg_reg) module"]
pub type GPIO_FUNC102_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC102_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC102_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func102_in_sel_cfg_reg::R](gpio_func102_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC102_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func102_in_sel_cfg_reg::W](gpio_func102_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC102_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC102_IN_SEL_CFG_REG"]
pub mod gpio_func102_in_sel_cfg_reg;
#[doc = "GPIO_FUNC103_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func103_in_sel_cfg_reg](gpio_func103_in_sel_cfg_reg) module"]
pub type GPIO_FUNC103_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC103_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC103_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func103_in_sel_cfg_reg::R](gpio_func103_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC103_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func103_in_sel_cfg_reg::W](gpio_func103_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC103_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC103_IN_SEL_CFG_REG"]
pub mod gpio_func103_in_sel_cfg_reg;
#[doc = "GPIO_FUNC104_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func104_in_sel_cfg_reg](gpio_func104_in_sel_cfg_reg) module"]
pub type GPIO_FUNC104_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC104_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC104_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func104_in_sel_cfg_reg::R](gpio_func104_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC104_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func104_in_sel_cfg_reg::W](gpio_func104_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC104_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC104_IN_SEL_CFG_REG"]
pub mod gpio_func104_in_sel_cfg_reg;
#[doc = "GPIO_FUNC105_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func105_in_sel_cfg_reg](gpio_func105_in_sel_cfg_reg) module"]
pub type GPIO_FUNC105_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC105_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC105_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func105_in_sel_cfg_reg::R](gpio_func105_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC105_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func105_in_sel_cfg_reg::W](gpio_func105_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC105_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC105_IN_SEL_CFG_REG"]
pub mod gpio_func105_in_sel_cfg_reg;
#[doc = "GPIO_FUNC106_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func106_in_sel_cfg_reg](gpio_func106_in_sel_cfg_reg) module"]
pub type GPIO_FUNC106_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC106_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC106_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func106_in_sel_cfg_reg::R](gpio_func106_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC106_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func106_in_sel_cfg_reg::W](gpio_func106_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC106_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC106_IN_SEL_CFG_REG"]
pub mod gpio_func106_in_sel_cfg_reg;
#[doc = "GPIO_FUNC107_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func107_in_sel_cfg_reg](gpio_func107_in_sel_cfg_reg) module"]
pub type GPIO_FUNC107_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC107_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC107_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func107_in_sel_cfg_reg::R](gpio_func107_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC107_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func107_in_sel_cfg_reg::W](gpio_func107_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC107_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC107_IN_SEL_CFG_REG"]
pub mod gpio_func107_in_sel_cfg_reg;
#[doc = "GPIO_FUNC108_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func108_in_sel_cfg_reg](gpio_func108_in_sel_cfg_reg) module"]
pub type GPIO_FUNC108_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC108_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC108_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func108_in_sel_cfg_reg::R](gpio_func108_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC108_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func108_in_sel_cfg_reg::W](gpio_func108_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC108_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC108_IN_SEL_CFG_REG"]
pub mod gpio_func108_in_sel_cfg_reg;
#[doc = "GPIO_FUNC109_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func109_in_sel_cfg_reg](gpio_func109_in_sel_cfg_reg) module"]
pub type GPIO_FUNC109_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC109_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC109_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func109_in_sel_cfg_reg::R](gpio_func109_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC109_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func109_in_sel_cfg_reg::W](gpio_func109_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC109_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC109_IN_SEL_CFG_REG"]
pub mod gpio_func109_in_sel_cfg_reg;
#[doc = "GPIO_FUNC110_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func110_in_sel_cfg_reg](gpio_func110_in_sel_cfg_reg) module"]
pub type GPIO_FUNC110_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC110_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC110_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func110_in_sel_cfg_reg::R](gpio_func110_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC110_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func110_in_sel_cfg_reg::W](gpio_func110_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC110_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC110_IN_SEL_CFG_REG"]
pub mod gpio_func110_in_sel_cfg_reg;
#[doc = "GPIO_FUNC111_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func111_in_sel_cfg_reg](gpio_func111_in_sel_cfg_reg) module"]
pub type GPIO_FUNC111_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC111_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC111_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func111_in_sel_cfg_reg::R](gpio_func111_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC111_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func111_in_sel_cfg_reg::W](gpio_func111_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC111_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC111_IN_SEL_CFG_REG"]
pub mod gpio_func111_in_sel_cfg_reg;
#[doc = "GPIO_FUNC112_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func112_in_sel_cfg_reg](gpio_func112_in_sel_cfg_reg) module"]
pub type GPIO_FUNC112_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC112_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC112_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func112_in_sel_cfg_reg::R](gpio_func112_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC112_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func112_in_sel_cfg_reg::W](gpio_func112_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC112_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC112_IN_SEL_CFG_REG"]
pub mod gpio_func112_in_sel_cfg_reg;
#[doc = "GPIO_FUNC113_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func113_in_sel_cfg_reg](gpio_func113_in_sel_cfg_reg) module"]
pub type GPIO_FUNC113_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC113_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC113_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func113_in_sel_cfg_reg::R](gpio_func113_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC113_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func113_in_sel_cfg_reg::W](gpio_func113_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC113_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC113_IN_SEL_CFG_REG"]
pub mod gpio_func113_in_sel_cfg_reg;
#[doc = "GPIO_FUNC114_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func114_in_sel_cfg_reg](gpio_func114_in_sel_cfg_reg) module"]
pub type GPIO_FUNC114_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC114_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC114_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func114_in_sel_cfg_reg::R](gpio_func114_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC114_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func114_in_sel_cfg_reg::W](gpio_func114_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC114_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC114_IN_SEL_CFG_REG"]
pub mod gpio_func114_in_sel_cfg_reg;
#[doc = "GPIO_FUNC115_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func115_in_sel_cfg_reg](gpio_func115_in_sel_cfg_reg) module"]
pub type GPIO_FUNC115_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC115_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC115_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func115_in_sel_cfg_reg::R](gpio_func115_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC115_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func115_in_sel_cfg_reg::W](gpio_func115_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC115_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC115_IN_SEL_CFG_REG"]
pub mod gpio_func115_in_sel_cfg_reg;
#[doc = "GPIO_FUNC116_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func116_in_sel_cfg_reg](gpio_func116_in_sel_cfg_reg) module"]
pub type GPIO_FUNC116_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC116_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC116_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func116_in_sel_cfg_reg::R](gpio_func116_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC116_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func116_in_sel_cfg_reg::W](gpio_func116_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC116_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC116_IN_SEL_CFG_REG"]
pub mod gpio_func116_in_sel_cfg_reg;
#[doc = "GPIO_FUNC117_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func117_in_sel_cfg_reg](gpio_func117_in_sel_cfg_reg) module"]
pub type GPIO_FUNC117_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC117_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC117_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func117_in_sel_cfg_reg::R](gpio_func117_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC117_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func117_in_sel_cfg_reg::W](gpio_func117_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC117_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC117_IN_SEL_CFG_REG"]
pub mod gpio_func117_in_sel_cfg_reg;
#[doc = "GPIO_FUNC118_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func118_in_sel_cfg_reg](gpio_func118_in_sel_cfg_reg) module"]
pub type GPIO_FUNC118_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC118_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC118_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func118_in_sel_cfg_reg::R](gpio_func118_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC118_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func118_in_sel_cfg_reg::W](gpio_func118_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC118_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC118_IN_SEL_CFG_REG"]
pub mod gpio_func118_in_sel_cfg_reg;
#[doc = "GPIO_FUNC119_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func119_in_sel_cfg_reg](gpio_func119_in_sel_cfg_reg) module"]
pub type GPIO_FUNC119_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC119_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC119_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func119_in_sel_cfg_reg::R](gpio_func119_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC119_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func119_in_sel_cfg_reg::W](gpio_func119_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC119_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC119_IN_SEL_CFG_REG"]
pub mod gpio_func119_in_sel_cfg_reg;
#[doc = "GPIO_FUNC120_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func120_in_sel_cfg_reg](gpio_func120_in_sel_cfg_reg) module"]
pub type GPIO_FUNC120_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC120_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC120_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func120_in_sel_cfg_reg::R](gpio_func120_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC120_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func120_in_sel_cfg_reg::W](gpio_func120_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC120_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC120_IN_SEL_CFG_REG"]
pub mod gpio_func120_in_sel_cfg_reg;
#[doc = "GPIO_FUNC121_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func121_in_sel_cfg_reg](gpio_func121_in_sel_cfg_reg) module"]
pub type GPIO_FUNC121_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC121_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC121_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func121_in_sel_cfg_reg::R](gpio_func121_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC121_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func121_in_sel_cfg_reg::W](gpio_func121_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC121_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC121_IN_SEL_CFG_REG"]
pub mod gpio_func121_in_sel_cfg_reg;
#[doc = "GPIO_FUNC122_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func122_in_sel_cfg_reg](gpio_func122_in_sel_cfg_reg) module"]
pub type GPIO_FUNC122_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC122_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC122_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func122_in_sel_cfg_reg::R](gpio_func122_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC122_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func122_in_sel_cfg_reg::W](gpio_func122_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC122_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC122_IN_SEL_CFG_REG"]
pub mod gpio_func122_in_sel_cfg_reg;
#[doc = "GPIO_FUNC123_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func123_in_sel_cfg_reg](gpio_func123_in_sel_cfg_reg) module"]
pub type GPIO_FUNC123_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC123_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC123_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func123_in_sel_cfg_reg::R](gpio_func123_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC123_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func123_in_sel_cfg_reg::W](gpio_func123_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC123_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC123_IN_SEL_CFG_REG"]
pub mod gpio_func123_in_sel_cfg_reg;
#[doc = "GPIO_FUNC124_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func124_in_sel_cfg_reg](gpio_func124_in_sel_cfg_reg) module"]
pub type GPIO_FUNC124_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC124_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC124_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func124_in_sel_cfg_reg::R](gpio_func124_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC124_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func124_in_sel_cfg_reg::W](gpio_func124_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC124_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC124_IN_SEL_CFG_REG"]
pub mod gpio_func124_in_sel_cfg_reg;
#[doc = "GPIO_FUNC125_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func125_in_sel_cfg_reg](gpio_func125_in_sel_cfg_reg) module"]
pub type GPIO_FUNC125_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC125_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC125_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func125_in_sel_cfg_reg::R](gpio_func125_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC125_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func125_in_sel_cfg_reg::W](gpio_func125_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC125_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC125_IN_SEL_CFG_REG"]
pub mod gpio_func125_in_sel_cfg_reg;
#[doc = "GPIO_FUNC126_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func126_in_sel_cfg_reg](gpio_func126_in_sel_cfg_reg) module"]
pub type GPIO_FUNC126_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC126_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC126_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func126_in_sel_cfg_reg::R](gpio_func126_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC126_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func126_in_sel_cfg_reg::W](gpio_func126_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC126_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC126_IN_SEL_CFG_REG"]
pub mod gpio_func126_in_sel_cfg_reg;
#[doc = "GPIO_FUNC127_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func127_in_sel_cfg_reg](gpio_func127_in_sel_cfg_reg) module"]
pub type GPIO_FUNC127_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC127_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC127_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func127_in_sel_cfg_reg::R](gpio_func127_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC127_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func127_in_sel_cfg_reg::W](gpio_func127_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC127_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC127_IN_SEL_CFG_REG"]
pub mod gpio_func127_in_sel_cfg_reg;
#[doc = "GPIO_FUNC128_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func128_in_sel_cfg_reg](gpio_func128_in_sel_cfg_reg) module"]
pub type GPIO_FUNC128_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC128_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC128_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func128_in_sel_cfg_reg::R](gpio_func128_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC128_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func128_in_sel_cfg_reg::W](gpio_func128_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC128_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC128_IN_SEL_CFG_REG"]
pub mod gpio_func128_in_sel_cfg_reg;
#[doc = "GPIO_FUNC129_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func129_in_sel_cfg_reg](gpio_func129_in_sel_cfg_reg) module"]
pub type GPIO_FUNC129_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC129_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC129_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func129_in_sel_cfg_reg::R](gpio_func129_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC129_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func129_in_sel_cfg_reg::W](gpio_func129_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC129_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC129_IN_SEL_CFG_REG"]
pub mod gpio_func129_in_sel_cfg_reg;
#[doc = "GPIO_FUNC130_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func130_in_sel_cfg_reg](gpio_func130_in_sel_cfg_reg) module"]
pub type GPIO_FUNC130_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC130_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC130_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func130_in_sel_cfg_reg::R](gpio_func130_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC130_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func130_in_sel_cfg_reg::W](gpio_func130_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC130_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC130_IN_SEL_CFG_REG"]
pub mod gpio_func130_in_sel_cfg_reg;
#[doc = "GPIO_FUNC131_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func131_in_sel_cfg_reg](gpio_func131_in_sel_cfg_reg) module"]
pub type GPIO_FUNC131_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC131_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC131_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func131_in_sel_cfg_reg::R](gpio_func131_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC131_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func131_in_sel_cfg_reg::W](gpio_func131_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC131_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC131_IN_SEL_CFG_REG"]
pub mod gpio_func131_in_sel_cfg_reg;
#[doc = "GPIO_FUNC132_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func132_in_sel_cfg_reg](gpio_func132_in_sel_cfg_reg) module"]
pub type GPIO_FUNC132_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC132_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC132_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func132_in_sel_cfg_reg::R](gpio_func132_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC132_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func132_in_sel_cfg_reg::W](gpio_func132_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC132_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC132_IN_SEL_CFG_REG"]
pub mod gpio_func132_in_sel_cfg_reg;
#[doc = "GPIO_FUNC133_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func133_in_sel_cfg_reg](gpio_func133_in_sel_cfg_reg) module"]
pub type GPIO_FUNC133_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC133_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC133_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func133_in_sel_cfg_reg::R](gpio_func133_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC133_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func133_in_sel_cfg_reg::W](gpio_func133_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC133_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC133_IN_SEL_CFG_REG"]
pub mod gpio_func133_in_sel_cfg_reg;
#[doc = "GPIO_FUNC134_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func134_in_sel_cfg_reg](gpio_func134_in_sel_cfg_reg) module"]
pub type GPIO_FUNC134_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC134_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC134_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func134_in_sel_cfg_reg::R](gpio_func134_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC134_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func134_in_sel_cfg_reg::W](gpio_func134_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC134_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC134_IN_SEL_CFG_REG"]
pub mod gpio_func134_in_sel_cfg_reg;
#[doc = "GPIO_FUNC135_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func135_in_sel_cfg_reg](gpio_func135_in_sel_cfg_reg) module"]
pub type GPIO_FUNC135_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC135_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC135_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func135_in_sel_cfg_reg::R](gpio_func135_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC135_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func135_in_sel_cfg_reg::W](gpio_func135_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC135_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC135_IN_SEL_CFG_REG"]
pub mod gpio_func135_in_sel_cfg_reg;
#[doc = "GPIO_FUNC136_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func136_in_sel_cfg_reg](gpio_func136_in_sel_cfg_reg) module"]
pub type GPIO_FUNC136_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC136_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC136_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func136_in_sel_cfg_reg::R](gpio_func136_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC136_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func136_in_sel_cfg_reg::W](gpio_func136_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC136_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC136_IN_SEL_CFG_REG"]
pub mod gpio_func136_in_sel_cfg_reg;
#[doc = "GPIO_FUNC137_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func137_in_sel_cfg_reg](gpio_func137_in_sel_cfg_reg) module"]
pub type GPIO_FUNC137_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC137_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC137_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func137_in_sel_cfg_reg::R](gpio_func137_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC137_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func137_in_sel_cfg_reg::W](gpio_func137_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC137_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC137_IN_SEL_CFG_REG"]
pub mod gpio_func137_in_sel_cfg_reg;
#[doc = "GPIO_FUNC138_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func138_in_sel_cfg_reg](gpio_func138_in_sel_cfg_reg) module"]
pub type GPIO_FUNC138_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC138_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC138_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func138_in_sel_cfg_reg::R](gpio_func138_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC138_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func138_in_sel_cfg_reg::W](gpio_func138_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC138_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC138_IN_SEL_CFG_REG"]
pub mod gpio_func138_in_sel_cfg_reg;
#[doc = "GPIO_FUNC139_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func139_in_sel_cfg_reg](gpio_func139_in_sel_cfg_reg) module"]
pub type GPIO_FUNC139_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC139_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC139_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func139_in_sel_cfg_reg::R](gpio_func139_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC139_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func139_in_sel_cfg_reg::W](gpio_func139_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC139_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC139_IN_SEL_CFG_REG"]
pub mod gpio_func139_in_sel_cfg_reg;
#[doc = "GPIO_FUNC140_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func140_in_sel_cfg_reg](gpio_func140_in_sel_cfg_reg) module"]
pub type GPIO_FUNC140_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC140_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC140_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func140_in_sel_cfg_reg::R](gpio_func140_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC140_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func140_in_sel_cfg_reg::W](gpio_func140_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC140_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC140_IN_SEL_CFG_REG"]
pub mod gpio_func140_in_sel_cfg_reg;
#[doc = "GPIO_FUNC141_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func141_in_sel_cfg_reg](gpio_func141_in_sel_cfg_reg) module"]
pub type GPIO_FUNC141_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC141_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC141_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func141_in_sel_cfg_reg::R](gpio_func141_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC141_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func141_in_sel_cfg_reg::W](gpio_func141_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC141_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC141_IN_SEL_CFG_REG"]
pub mod gpio_func141_in_sel_cfg_reg;
#[doc = "GPIO_FUNC142_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func142_in_sel_cfg_reg](gpio_func142_in_sel_cfg_reg) module"]
pub type GPIO_FUNC142_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC142_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC142_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func142_in_sel_cfg_reg::R](gpio_func142_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC142_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func142_in_sel_cfg_reg::W](gpio_func142_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC142_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC142_IN_SEL_CFG_REG"]
pub mod gpio_func142_in_sel_cfg_reg;
#[doc = "GPIO_FUNC143_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func143_in_sel_cfg_reg](gpio_func143_in_sel_cfg_reg) module"]
pub type GPIO_FUNC143_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC143_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC143_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func143_in_sel_cfg_reg::R](gpio_func143_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC143_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func143_in_sel_cfg_reg::W](gpio_func143_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC143_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC143_IN_SEL_CFG_REG"]
pub mod gpio_func143_in_sel_cfg_reg;
#[doc = "GPIO_FUNC144_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func144_in_sel_cfg_reg](gpio_func144_in_sel_cfg_reg) module"]
pub type GPIO_FUNC144_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC144_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC144_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func144_in_sel_cfg_reg::R](gpio_func144_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC144_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func144_in_sel_cfg_reg::W](gpio_func144_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC144_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC144_IN_SEL_CFG_REG"]
pub mod gpio_func144_in_sel_cfg_reg;
#[doc = "GPIO_FUNC145_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func145_in_sel_cfg_reg](gpio_func145_in_sel_cfg_reg) module"]
pub type GPIO_FUNC145_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC145_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC145_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func145_in_sel_cfg_reg::R](gpio_func145_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC145_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func145_in_sel_cfg_reg::W](gpio_func145_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC145_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC145_IN_SEL_CFG_REG"]
pub mod gpio_func145_in_sel_cfg_reg;
#[doc = "GPIO_FUNC146_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func146_in_sel_cfg_reg](gpio_func146_in_sel_cfg_reg) module"]
pub type GPIO_FUNC146_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC146_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC146_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func146_in_sel_cfg_reg::R](gpio_func146_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC146_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func146_in_sel_cfg_reg::W](gpio_func146_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC146_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC146_IN_SEL_CFG_REG"]
pub mod gpio_func146_in_sel_cfg_reg;
#[doc = "GPIO_FUNC147_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func147_in_sel_cfg_reg](gpio_func147_in_sel_cfg_reg) module"]
pub type GPIO_FUNC147_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC147_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC147_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func147_in_sel_cfg_reg::R](gpio_func147_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC147_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func147_in_sel_cfg_reg::W](gpio_func147_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC147_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC147_IN_SEL_CFG_REG"]
pub mod gpio_func147_in_sel_cfg_reg;
#[doc = "GPIO_FUNC148_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func148_in_sel_cfg_reg](gpio_func148_in_sel_cfg_reg) module"]
pub type GPIO_FUNC148_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC148_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC148_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func148_in_sel_cfg_reg::R](gpio_func148_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC148_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func148_in_sel_cfg_reg::W](gpio_func148_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC148_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC148_IN_SEL_CFG_REG"]
pub mod gpio_func148_in_sel_cfg_reg;
#[doc = "GPIO_FUNC149_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func149_in_sel_cfg_reg](gpio_func149_in_sel_cfg_reg) module"]
pub type GPIO_FUNC149_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC149_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC149_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func149_in_sel_cfg_reg::R](gpio_func149_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC149_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func149_in_sel_cfg_reg::W](gpio_func149_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC149_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC149_IN_SEL_CFG_REG"]
pub mod gpio_func149_in_sel_cfg_reg;
#[doc = "GPIO_FUNC150_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func150_in_sel_cfg_reg](gpio_func150_in_sel_cfg_reg) module"]
pub type GPIO_FUNC150_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC150_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC150_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func150_in_sel_cfg_reg::R](gpio_func150_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC150_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func150_in_sel_cfg_reg::W](gpio_func150_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC150_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC150_IN_SEL_CFG_REG"]
pub mod gpio_func150_in_sel_cfg_reg;
#[doc = "GPIO_FUNC151_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func151_in_sel_cfg_reg](gpio_func151_in_sel_cfg_reg) module"]
pub type GPIO_FUNC151_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC151_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC151_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func151_in_sel_cfg_reg::R](gpio_func151_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC151_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func151_in_sel_cfg_reg::W](gpio_func151_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC151_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC151_IN_SEL_CFG_REG"]
pub mod gpio_func151_in_sel_cfg_reg;
#[doc = "GPIO_FUNC152_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func152_in_sel_cfg_reg](gpio_func152_in_sel_cfg_reg) module"]
pub type GPIO_FUNC152_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC152_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC152_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func152_in_sel_cfg_reg::R](gpio_func152_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC152_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func152_in_sel_cfg_reg::W](gpio_func152_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC152_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC152_IN_SEL_CFG_REG"]
pub mod gpio_func152_in_sel_cfg_reg;
#[doc = "GPIO_FUNC153_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func153_in_sel_cfg_reg](gpio_func153_in_sel_cfg_reg) module"]
pub type GPIO_FUNC153_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC153_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC153_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func153_in_sel_cfg_reg::R](gpio_func153_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC153_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func153_in_sel_cfg_reg::W](gpio_func153_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC153_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC153_IN_SEL_CFG_REG"]
pub mod gpio_func153_in_sel_cfg_reg;
#[doc = "GPIO_FUNC154_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func154_in_sel_cfg_reg](gpio_func154_in_sel_cfg_reg) module"]
pub type GPIO_FUNC154_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC154_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC154_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func154_in_sel_cfg_reg::R](gpio_func154_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC154_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func154_in_sel_cfg_reg::W](gpio_func154_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC154_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC154_IN_SEL_CFG_REG"]
pub mod gpio_func154_in_sel_cfg_reg;
#[doc = "GPIO_FUNC155_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func155_in_sel_cfg_reg](gpio_func155_in_sel_cfg_reg) module"]
pub type GPIO_FUNC155_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC155_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC155_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func155_in_sel_cfg_reg::R](gpio_func155_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC155_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func155_in_sel_cfg_reg::W](gpio_func155_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC155_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC155_IN_SEL_CFG_REG"]
pub mod gpio_func155_in_sel_cfg_reg;
#[doc = "GPIO_FUNC156_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func156_in_sel_cfg_reg](gpio_func156_in_sel_cfg_reg) module"]
pub type GPIO_FUNC156_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC156_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC156_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func156_in_sel_cfg_reg::R](gpio_func156_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC156_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func156_in_sel_cfg_reg::W](gpio_func156_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC156_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC156_IN_SEL_CFG_REG"]
pub mod gpio_func156_in_sel_cfg_reg;
#[doc = "GPIO_FUNC157_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func157_in_sel_cfg_reg](gpio_func157_in_sel_cfg_reg) module"]
pub type GPIO_FUNC157_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC157_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC157_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func157_in_sel_cfg_reg::R](gpio_func157_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC157_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func157_in_sel_cfg_reg::W](gpio_func157_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC157_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC157_IN_SEL_CFG_REG"]
pub mod gpio_func157_in_sel_cfg_reg;
#[doc = "GPIO_FUNC158_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func158_in_sel_cfg_reg](gpio_func158_in_sel_cfg_reg) module"]
pub type GPIO_FUNC158_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC158_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC158_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func158_in_sel_cfg_reg::R](gpio_func158_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC158_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func158_in_sel_cfg_reg::W](gpio_func158_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC158_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC158_IN_SEL_CFG_REG"]
pub mod gpio_func158_in_sel_cfg_reg;
#[doc = "GPIO_FUNC159_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func159_in_sel_cfg_reg](gpio_func159_in_sel_cfg_reg) module"]
pub type GPIO_FUNC159_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC159_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC159_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func159_in_sel_cfg_reg::R](gpio_func159_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC159_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func159_in_sel_cfg_reg::W](gpio_func159_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC159_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC159_IN_SEL_CFG_REG"]
pub mod gpio_func159_in_sel_cfg_reg;
#[doc = "GPIO_FUNC160_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func160_in_sel_cfg_reg](gpio_func160_in_sel_cfg_reg) module"]
pub type GPIO_FUNC160_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC160_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC160_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func160_in_sel_cfg_reg::R](gpio_func160_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC160_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func160_in_sel_cfg_reg::W](gpio_func160_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC160_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC160_IN_SEL_CFG_REG"]
pub mod gpio_func160_in_sel_cfg_reg;
#[doc = "GPIO_FUNC161_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func161_in_sel_cfg_reg](gpio_func161_in_sel_cfg_reg) module"]
pub type GPIO_FUNC161_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC161_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC161_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func161_in_sel_cfg_reg::R](gpio_func161_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC161_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func161_in_sel_cfg_reg::W](gpio_func161_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC161_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC161_IN_SEL_CFG_REG"]
pub mod gpio_func161_in_sel_cfg_reg;
#[doc = "GPIO_FUNC162_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func162_in_sel_cfg_reg](gpio_func162_in_sel_cfg_reg) module"]
pub type GPIO_FUNC162_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC162_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC162_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func162_in_sel_cfg_reg::R](gpio_func162_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC162_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func162_in_sel_cfg_reg::W](gpio_func162_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC162_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC162_IN_SEL_CFG_REG"]
pub mod gpio_func162_in_sel_cfg_reg;
#[doc = "GPIO_FUNC163_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func163_in_sel_cfg_reg](gpio_func163_in_sel_cfg_reg) module"]
pub type GPIO_FUNC163_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC163_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC163_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func163_in_sel_cfg_reg::R](gpio_func163_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC163_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func163_in_sel_cfg_reg::W](gpio_func163_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC163_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC163_IN_SEL_CFG_REG"]
pub mod gpio_func163_in_sel_cfg_reg;
#[doc = "GPIO_FUNC164_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func164_in_sel_cfg_reg](gpio_func164_in_sel_cfg_reg) module"]
pub type GPIO_FUNC164_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC164_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC164_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func164_in_sel_cfg_reg::R](gpio_func164_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC164_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func164_in_sel_cfg_reg::W](gpio_func164_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC164_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC164_IN_SEL_CFG_REG"]
pub mod gpio_func164_in_sel_cfg_reg;
#[doc = "GPIO_FUNC165_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func165_in_sel_cfg_reg](gpio_func165_in_sel_cfg_reg) module"]
pub type GPIO_FUNC165_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC165_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC165_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func165_in_sel_cfg_reg::R](gpio_func165_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC165_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func165_in_sel_cfg_reg::W](gpio_func165_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC165_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC165_IN_SEL_CFG_REG"]
pub mod gpio_func165_in_sel_cfg_reg;
#[doc = "GPIO_FUNC166_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func166_in_sel_cfg_reg](gpio_func166_in_sel_cfg_reg) module"]
pub type GPIO_FUNC166_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC166_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC166_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func166_in_sel_cfg_reg::R](gpio_func166_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC166_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func166_in_sel_cfg_reg::W](gpio_func166_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC166_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC166_IN_SEL_CFG_REG"]
pub mod gpio_func166_in_sel_cfg_reg;
#[doc = "GPIO_FUNC167_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func167_in_sel_cfg_reg](gpio_func167_in_sel_cfg_reg) module"]
pub type GPIO_FUNC167_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC167_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC167_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func167_in_sel_cfg_reg::R](gpio_func167_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC167_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func167_in_sel_cfg_reg::W](gpio_func167_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC167_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC167_IN_SEL_CFG_REG"]
pub mod gpio_func167_in_sel_cfg_reg;
#[doc = "GPIO_FUNC168_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func168_in_sel_cfg_reg](gpio_func168_in_sel_cfg_reg) module"]
pub type GPIO_FUNC168_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC168_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC168_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func168_in_sel_cfg_reg::R](gpio_func168_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC168_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func168_in_sel_cfg_reg::W](gpio_func168_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC168_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC168_IN_SEL_CFG_REG"]
pub mod gpio_func168_in_sel_cfg_reg;
#[doc = "GPIO_FUNC169_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func169_in_sel_cfg_reg](gpio_func169_in_sel_cfg_reg) module"]
pub type GPIO_FUNC169_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC169_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC169_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func169_in_sel_cfg_reg::R](gpio_func169_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC169_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func169_in_sel_cfg_reg::W](gpio_func169_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC169_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC169_IN_SEL_CFG_REG"]
pub mod gpio_func169_in_sel_cfg_reg;
#[doc = "GPIO_FUNC170_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func170_in_sel_cfg_reg](gpio_func170_in_sel_cfg_reg) module"]
pub type GPIO_FUNC170_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC170_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC170_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func170_in_sel_cfg_reg::R](gpio_func170_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC170_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func170_in_sel_cfg_reg::W](gpio_func170_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC170_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC170_IN_SEL_CFG_REG"]
pub mod gpio_func170_in_sel_cfg_reg;
#[doc = "GPIO_FUNC171_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func171_in_sel_cfg_reg](gpio_func171_in_sel_cfg_reg) module"]
pub type GPIO_FUNC171_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC171_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC171_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func171_in_sel_cfg_reg::R](gpio_func171_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC171_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func171_in_sel_cfg_reg::W](gpio_func171_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC171_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC171_IN_SEL_CFG_REG"]
pub mod gpio_func171_in_sel_cfg_reg;
#[doc = "GPIO_FUNC172_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func172_in_sel_cfg_reg](gpio_func172_in_sel_cfg_reg) module"]
pub type GPIO_FUNC172_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC172_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC172_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func172_in_sel_cfg_reg::R](gpio_func172_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC172_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func172_in_sel_cfg_reg::W](gpio_func172_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC172_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC172_IN_SEL_CFG_REG"]
pub mod gpio_func172_in_sel_cfg_reg;
#[doc = "GPIO_FUNC173_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func173_in_sel_cfg_reg](gpio_func173_in_sel_cfg_reg) module"]
pub type GPIO_FUNC173_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC173_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC173_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func173_in_sel_cfg_reg::R](gpio_func173_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC173_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func173_in_sel_cfg_reg::W](gpio_func173_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC173_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC173_IN_SEL_CFG_REG"]
pub mod gpio_func173_in_sel_cfg_reg;
#[doc = "GPIO_FUNC174_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func174_in_sel_cfg_reg](gpio_func174_in_sel_cfg_reg) module"]
pub type GPIO_FUNC174_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC174_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC174_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func174_in_sel_cfg_reg::R](gpio_func174_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC174_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func174_in_sel_cfg_reg::W](gpio_func174_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC174_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC174_IN_SEL_CFG_REG"]
pub mod gpio_func174_in_sel_cfg_reg;
#[doc = "GPIO_FUNC175_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func175_in_sel_cfg_reg](gpio_func175_in_sel_cfg_reg) module"]
pub type GPIO_FUNC175_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC175_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC175_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func175_in_sel_cfg_reg::R](gpio_func175_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC175_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func175_in_sel_cfg_reg::W](gpio_func175_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC175_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC175_IN_SEL_CFG_REG"]
pub mod gpio_func175_in_sel_cfg_reg;
#[doc = "GPIO_FUNC176_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func176_in_sel_cfg_reg](gpio_func176_in_sel_cfg_reg) module"]
pub type GPIO_FUNC176_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC176_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC176_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func176_in_sel_cfg_reg::R](gpio_func176_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC176_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func176_in_sel_cfg_reg::W](gpio_func176_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC176_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC176_IN_SEL_CFG_REG"]
pub mod gpio_func176_in_sel_cfg_reg;
#[doc = "GPIO_FUNC177_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func177_in_sel_cfg_reg](gpio_func177_in_sel_cfg_reg) module"]
pub type GPIO_FUNC177_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC177_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC177_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func177_in_sel_cfg_reg::R](gpio_func177_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC177_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func177_in_sel_cfg_reg::W](gpio_func177_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC177_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC177_IN_SEL_CFG_REG"]
pub mod gpio_func177_in_sel_cfg_reg;
#[doc = "GPIO_FUNC178_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func178_in_sel_cfg_reg](gpio_func178_in_sel_cfg_reg) module"]
pub type GPIO_FUNC178_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC178_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC178_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func178_in_sel_cfg_reg::R](gpio_func178_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC178_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func178_in_sel_cfg_reg::W](gpio_func178_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC178_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC178_IN_SEL_CFG_REG"]
pub mod gpio_func178_in_sel_cfg_reg;
#[doc = "GPIO_FUNC179_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func179_in_sel_cfg_reg](gpio_func179_in_sel_cfg_reg) module"]
pub type GPIO_FUNC179_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC179_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC179_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func179_in_sel_cfg_reg::R](gpio_func179_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC179_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func179_in_sel_cfg_reg::W](gpio_func179_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC179_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC179_IN_SEL_CFG_REG"]
pub mod gpio_func179_in_sel_cfg_reg;
#[doc = "GPIO_FUNC180_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func180_in_sel_cfg_reg](gpio_func180_in_sel_cfg_reg) module"]
pub type GPIO_FUNC180_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC180_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC180_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func180_in_sel_cfg_reg::R](gpio_func180_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC180_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func180_in_sel_cfg_reg::W](gpio_func180_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC180_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC180_IN_SEL_CFG_REG"]
pub mod gpio_func180_in_sel_cfg_reg;
#[doc = "GPIO_FUNC181_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func181_in_sel_cfg_reg](gpio_func181_in_sel_cfg_reg) module"]
pub type GPIO_FUNC181_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC181_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC181_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func181_in_sel_cfg_reg::R](gpio_func181_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC181_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func181_in_sel_cfg_reg::W](gpio_func181_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC181_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC181_IN_SEL_CFG_REG"]
pub mod gpio_func181_in_sel_cfg_reg;
#[doc = "GPIO_FUNC182_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func182_in_sel_cfg_reg](gpio_func182_in_sel_cfg_reg) module"]
pub type GPIO_FUNC182_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC182_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC182_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func182_in_sel_cfg_reg::R](gpio_func182_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC182_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func182_in_sel_cfg_reg::W](gpio_func182_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC182_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC182_IN_SEL_CFG_REG"]
pub mod gpio_func182_in_sel_cfg_reg;
#[doc = "GPIO_FUNC183_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func183_in_sel_cfg_reg](gpio_func183_in_sel_cfg_reg) module"]
pub type GPIO_FUNC183_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC183_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC183_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func183_in_sel_cfg_reg::R](gpio_func183_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC183_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func183_in_sel_cfg_reg::W](gpio_func183_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC183_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC183_IN_SEL_CFG_REG"]
pub mod gpio_func183_in_sel_cfg_reg;
#[doc = "GPIO_FUNC184_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func184_in_sel_cfg_reg](gpio_func184_in_sel_cfg_reg) module"]
pub type GPIO_FUNC184_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC184_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC184_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func184_in_sel_cfg_reg::R](gpio_func184_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC184_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func184_in_sel_cfg_reg::W](gpio_func184_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC184_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC184_IN_SEL_CFG_REG"]
pub mod gpio_func184_in_sel_cfg_reg;
#[doc = "GPIO_FUNC185_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func185_in_sel_cfg_reg](gpio_func185_in_sel_cfg_reg) module"]
pub type GPIO_FUNC185_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC185_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC185_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func185_in_sel_cfg_reg::R](gpio_func185_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC185_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func185_in_sel_cfg_reg::W](gpio_func185_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC185_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC185_IN_SEL_CFG_REG"]
pub mod gpio_func185_in_sel_cfg_reg;
#[doc = "GPIO_FUNC186_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func186_in_sel_cfg_reg](gpio_func186_in_sel_cfg_reg) module"]
pub type GPIO_FUNC186_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC186_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC186_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func186_in_sel_cfg_reg::R](gpio_func186_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC186_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func186_in_sel_cfg_reg::W](gpio_func186_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC186_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC186_IN_SEL_CFG_REG"]
pub mod gpio_func186_in_sel_cfg_reg;
#[doc = "GPIO_FUNC187_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func187_in_sel_cfg_reg](gpio_func187_in_sel_cfg_reg) module"]
pub type GPIO_FUNC187_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC187_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC187_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func187_in_sel_cfg_reg::R](gpio_func187_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC187_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func187_in_sel_cfg_reg::W](gpio_func187_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC187_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC187_IN_SEL_CFG_REG"]
pub mod gpio_func187_in_sel_cfg_reg;
#[doc = "GPIO_FUNC188_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func188_in_sel_cfg_reg](gpio_func188_in_sel_cfg_reg) module"]
pub type GPIO_FUNC188_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC188_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC188_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func188_in_sel_cfg_reg::R](gpio_func188_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC188_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func188_in_sel_cfg_reg::W](gpio_func188_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC188_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC188_IN_SEL_CFG_REG"]
pub mod gpio_func188_in_sel_cfg_reg;
#[doc = "GPIO_FUNC189_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func189_in_sel_cfg_reg](gpio_func189_in_sel_cfg_reg) module"]
pub type GPIO_FUNC189_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC189_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC189_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func189_in_sel_cfg_reg::R](gpio_func189_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC189_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func189_in_sel_cfg_reg::W](gpio_func189_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC189_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC189_IN_SEL_CFG_REG"]
pub mod gpio_func189_in_sel_cfg_reg;
#[doc = "GPIO_FUNC190_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func190_in_sel_cfg_reg](gpio_func190_in_sel_cfg_reg) module"]
pub type GPIO_FUNC190_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC190_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC190_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func190_in_sel_cfg_reg::R](gpio_func190_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC190_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func190_in_sel_cfg_reg::W](gpio_func190_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC190_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC190_IN_SEL_CFG_REG"]
pub mod gpio_func190_in_sel_cfg_reg;
#[doc = "GPIO_FUNC191_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func191_in_sel_cfg_reg](gpio_func191_in_sel_cfg_reg) module"]
pub type GPIO_FUNC191_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC191_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC191_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func191_in_sel_cfg_reg::R](gpio_func191_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC191_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func191_in_sel_cfg_reg::W](gpio_func191_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC191_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC191_IN_SEL_CFG_REG"]
pub mod gpio_func191_in_sel_cfg_reg;
#[doc = "GPIO_FUNC192_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func192_in_sel_cfg_reg](gpio_func192_in_sel_cfg_reg) module"]
pub type GPIO_FUNC192_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC192_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC192_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func192_in_sel_cfg_reg::R](gpio_func192_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC192_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func192_in_sel_cfg_reg::W](gpio_func192_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC192_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC192_IN_SEL_CFG_REG"]
pub mod gpio_func192_in_sel_cfg_reg;
#[doc = "GPIO_FUNC193_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func193_in_sel_cfg_reg](gpio_func193_in_sel_cfg_reg) module"]
pub type GPIO_FUNC193_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC193_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC193_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func193_in_sel_cfg_reg::R](gpio_func193_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC193_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func193_in_sel_cfg_reg::W](gpio_func193_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC193_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC193_IN_SEL_CFG_REG"]
pub mod gpio_func193_in_sel_cfg_reg;
#[doc = "GPIO_FUNC194_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func194_in_sel_cfg_reg](gpio_func194_in_sel_cfg_reg) module"]
pub type GPIO_FUNC194_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC194_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC194_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func194_in_sel_cfg_reg::R](gpio_func194_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC194_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func194_in_sel_cfg_reg::W](gpio_func194_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC194_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC194_IN_SEL_CFG_REG"]
pub mod gpio_func194_in_sel_cfg_reg;
#[doc = "GPIO_FUNC195_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func195_in_sel_cfg_reg](gpio_func195_in_sel_cfg_reg) module"]
pub type GPIO_FUNC195_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC195_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC195_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func195_in_sel_cfg_reg::R](gpio_func195_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC195_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func195_in_sel_cfg_reg::W](gpio_func195_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC195_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC195_IN_SEL_CFG_REG"]
pub mod gpio_func195_in_sel_cfg_reg;
#[doc = "GPIO_FUNC196_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func196_in_sel_cfg_reg](gpio_func196_in_sel_cfg_reg) module"]
pub type GPIO_FUNC196_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC196_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC196_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func196_in_sel_cfg_reg::R](gpio_func196_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC196_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func196_in_sel_cfg_reg::W](gpio_func196_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC196_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC196_IN_SEL_CFG_REG"]
pub mod gpio_func196_in_sel_cfg_reg;
#[doc = "GPIO_FUNC197_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func197_in_sel_cfg_reg](gpio_func197_in_sel_cfg_reg) module"]
pub type GPIO_FUNC197_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC197_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC197_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func197_in_sel_cfg_reg::R](gpio_func197_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC197_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func197_in_sel_cfg_reg::W](gpio_func197_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC197_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC197_IN_SEL_CFG_REG"]
pub mod gpio_func197_in_sel_cfg_reg;
#[doc = "GPIO_FUNC198_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func198_in_sel_cfg_reg](gpio_func198_in_sel_cfg_reg) module"]
pub type GPIO_FUNC198_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC198_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC198_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func198_in_sel_cfg_reg::R](gpio_func198_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC198_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func198_in_sel_cfg_reg::W](gpio_func198_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC198_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC198_IN_SEL_CFG_REG"]
pub mod gpio_func198_in_sel_cfg_reg;
#[doc = "GPIO_FUNC199_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func199_in_sel_cfg_reg](gpio_func199_in_sel_cfg_reg) module"]
pub type GPIO_FUNC199_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC199_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC199_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func199_in_sel_cfg_reg::R](gpio_func199_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC199_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func199_in_sel_cfg_reg::W](gpio_func199_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC199_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC199_IN_SEL_CFG_REG"]
pub mod gpio_func199_in_sel_cfg_reg;
#[doc = "GPIO_FUNC200_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func200_in_sel_cfg_reg](gpio_func200_in_sel_cfg_reg) module"]
pub type GPIO_FUNC200_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC200_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC200_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func200_in_sel_cfg_reg::R](gpio_func200_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC200_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func200_in_sel_cfg_reg::W](gpio_func200_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC200_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC200_IN_SEL_CFG_REG"]
pub mod gpio_func200_in_sel_cfg_reg;
#[doc = "GPIO_FUNC201_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func201_in_sel_cfg_reg](gpio_func201_in_sel_cfg_reg) module"]
pub type GPIO_FUNC201_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC201_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC201_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func201_in_sel_cfg_reg::R](gpio_func201_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC201_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func201_in_sel_cfg_reg::W](gpio_func201_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC201_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC201_IN_SEL_CFG_REG"]
pub mod gpio_func201_in_sel_cfg_reg;
#[doc = "GPIO_FUNC202_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func202_in_sel_cfg_reg](gpio_func202_in_sel_cfg_reg) module"]
pub type GPIO_FUNC202_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC202_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC202_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func202_in_sel_cfg_reg::R](gpio_func202_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC202_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func202_in_sel_cfg_reg::W](gpio_func202_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC202_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC202_IN_SEL_CFG_REG"]
pub mod gpio_func202_in_sel_cfg_reg;
#[doc = "GPIO_FUNC203_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func203_in_sel_cfg_reg](gpio_func203_in_sel_cfg_reg) module"]
pub type GPIO_FUNC203_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC203_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC203_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func203_in_sel_cfg_reg::R](gpio_func203_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC203_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func203_in_sel_cfg_reg::W](gpio_func203_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC203_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC203_IN_SEL_CFG_REG"]
pub mod gpio_func203_in_sel_cfg_reg;
#[doc = "GPIO_FUNC204_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func204_in_sel_cfg_reg](gpio_func204_in_sel_cfg_reg) module"]
pub type GPIO_FUNC204_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC204_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC204_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func204_in_sel_cfg_reg::R](gpio_func204_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC204_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func204_in_sel_cfg_reg::W](gpio_func204_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC204_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC204_IN_SEL_CFG_REG"]
pub mod gpio_func204_in_sel_cfg_reg;
#[doc = "GPIO_FUNC205_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func205_in_sel_cfg_reg](gpio_func205_in_sel_cfg_reg) module"]
pub type GPIO_FUNC205_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC205_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC205_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func205_in_sel_cfg_reg::R](gpio_func205_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC205_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func205_in_sel_cfg_reg::W](gpio_func205_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC205_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC205_IN_SEL_CFG_REG"]
pub mod gpio_func205_in_sel_cfg_reg;
#[doc = "GPIO_FUNC206_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func206_in_sel_cfg_reg](gpio_func206_in_sel_cfg_reg) module"]
pub type GPIO_FUNC206_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC206_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC206_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func206_in_sel_cfg_reg::R](gpio_func206_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC206_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func206_in_sel_cfg_reg::W](gpio_func206_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC206_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC206_IN_SEL_CFG_REG"]
pub mod gpio_func206_in_sel_cfg_reg;
#[doc = "GPIO_FUNC207_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func207_in_sel_cfg_reg](gpio_func207_in_sel_cfg_reg) module"]
pub type GPIO_FUNC207_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC207_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC207_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func207_in_sel_cfg_reg::R](gpio_func207_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC207_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func207_in_sel_cfg_reg::W](gpio_func207_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC207_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC207_IN_SEL_CFG_REG"]
pub mod gpio_func207_in_sel_cfg_reg;
#[doc = "GPIO_FUNC208_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func208_in_sel_cfg_reg](gpio_func208_in_sel_cfg_reg) module"]
pub type GPIO_FUNC208_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC208_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC208_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func208_in_sel_cfg_reg::R](gpio_func208_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC208_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func208_in_sel_cfg_reg::W](gpio_func208_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC208_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC208_IN_SEL_CFG_REG"]
pub mod gpio_func208_in_sel_cfg_reg;
#[doc = "GPIO_FUNC209_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func209_in_sel_cfg_reg](gpio_func209_in_sel_cfg_reg) module"]
pub type GPIO_FUNC209_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC209_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC209_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func209_in_sel_cfg_reg::R](gpio_func209_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC209_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func209_in_sel_cfg_reg::W](gpio_func209_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC209_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC209_IN_SEL_CFG_REG"]
pub mod gpio_func209_in_sel_cfg_reg;
#[doc = "GPIO_FUNC210_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func210_in_sel_cfg_reg](gpio_func210_in_sel_cfg_reg) module"]
pub type GPIO_FUNC210_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC210_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC210_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func210_in_sel_cfg_reg::R](gpio_func210_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC210_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func210_in_sel_cfg_reg::W](gpio_func210_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC210_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC210_IN_SEL_CFG_REG"]
pub mod gpio_func210_in_sel_cfg_reg;
#[doc = "GPIO_FUNC211_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func211_in_sel_cfg_reg](gpio_func211_in_sel_cfg_reg) module"]
pub type GPIO_FUNC211_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC211_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC211_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func211_in_sel_cfg_reg::R](gpio_func211_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC211_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func211_in_sel_cfg_reg::W](gpio_func211_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC211_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC211_IN_SEL_CFG_REG"]
pub mod gpio_func211_in_sel_cfg_reg;
#[doc = "GPIO_FUNC212_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func212_in_sel_cfg_reg](gpio_func212_in_sel_cfg_reg) module"]
pub type GPIO_FUNC212_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC212_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC212_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func212_in_sel_cfg_reg::R](gpio_func212_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC212_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func212_in_sel_cfg_reg::W](gpio_func212_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC212_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC212_IN_SEL_CFG_REG"]
pub mod gpio_func212_in_sel_cfg_reg;
#[doc = "GPIO_FUNC213_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func213_in_sel_cfg_reg](gpio_func213_in_sel_cfg_reg) module"]
pub type GPIO_FUNC213_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC213_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC213_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func213_in_sel_cfg_reg::R](gpio_func213_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC213_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func213_in_sel_cfg_reg::W](gpio_func213_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC213_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC213_IN_SEL_CFG_REG"]
pub mod gpio_func213_in_sel_cfg_reg;
#[doc = "GPIO_FUNC214_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func214_in_sel_cfg_reg](gpio_func214_in_sel_cfg_reg) module"]
pub type GPIO_FUNC214_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC214_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC214_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func214_in_sel_cfg_reg::R](gpio_func214_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC214_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func214_in_sel_cfg_reg::W](gpio_func214_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC214_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC214_IN_SEL_CFG_REG"]
pub mod gpio_func214_in_sel_cfg_reg;
#[doc = "GPIO_FUNC215_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func215_in_sel_cfg_reg](gpio_func215_in_sel_cfg_reg) module"]
pub type GPIO_FUNC215_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC215_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC215_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func215_in_sel_cfg_reg::R](gpio_func215_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC215_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func215_in_sel_cfg_reg::W](gpio_func215_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC215_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC215_IN_SEL_CFG_REG"]
pub mod gpio_func215_in_sel_cfg_reg;
#[doc = "GPIO_FUNC216_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func216_in_sel_cfg_reg](gpio_func216_in_sel_cfg_reg) module"]
pub type GPIO_FUNC216_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC216_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC216_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func216_in_sel_cfg_reg::R](gpio_func216_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC216_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func216_in_sel_cfg_reg::W](gpio_func216_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC216_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC216_IN_SEL_CFG_REG"]
pub mod gpio_func216_in_sel_cfg_reg;
#[doc = "GPIO_FUNC217_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func217_in_sel_cfg_reg](gpio_func217_in_sel_cfg_reg) module"]
pub type GPIO_FUNC217_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC217_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC217_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func217_in_sel_cfg_reg::R](gpio_func217_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC217_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func217_in_sel_cfg_reg::W](gpio_func217_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC217_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC217_IN_SEL_CFG_REG"]
pub mod gpio_func217_in_sel_cfg_reg;
#[doc = "GPIO_FUNC218_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func218_in_sel_cfg_reg](gpio_func218_in_sel_cfg_reg) module"]
pub type GPIO_FUNC218_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC218_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC218_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func218_in_sel_cfg_reg::R](gpio_func218_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC218_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func218_in_sel_cfg_reg::W](gpio_func218_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC218_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC218_IN_SEL_CFG_REG"]
pub mod gpio_func218_in_sel_cfg_reg;
#[doc = "GPIO_FUNC219_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func219_in_sel_cfg_reg](gpio_func219_in_sel_cfg_reg) module"]
pub type GPIO_FUNC219_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC219_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC219_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func219_in_sel_cfg_reg::R](gpio_func219_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC219_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func219_in_sel_cfg_reg::W](gpio_func219_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC219_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC219_IN_SEL_CFG_REG"]
pub mod gpio_func219_in_sel_cfg_reg;
#[doc = "GPIO_FUNC220_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func220_in_sel_cfg_reg](gpio_func220_in_sel_cfg_reg) module"]
pub type GPIO_FUNC220_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC220_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC220_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func220_in_sel_cfg_reg::R](gpio_func220_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC220_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func220_in_sel_cfg_reg::W](gpio_func220_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC220_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC220_IN_SEL_CFG_REG"]
pub mod gpio_func220_in_sel_cfg_reg;
#[doc = "GPIO_FUNC221_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func221_in_sel_cfg_reg](gpio_func221_in_sel_cfg_reg) module"]
pub type GPIO_FUNC221_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC221_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC221_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func221_in_sel_cfg_reg::R](gpio_func221_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC221_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func221_in_sel_cfg_reg::W](gpio_func221_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC221_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC221_IN_SEL_CFG_REG"]
pub mod gpio_func221_in_sel_cfg_reg;
#[doc = "GPIO_FUNC222_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func222_in_sel_cfg_reg](gpio_func222_in_sel_cfg_reg) module"]
pub type GPIO_FUNC222_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC222_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC222_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func222_in_sel_cfg_reg::R](gpio_func222_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC222_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func222_in_sel_cfg_reg::W](gpio_func222_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC222_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC222_IN_SEL_CFG_REG"]
pub mod gpio_func222_in_sel_cfg_reg;
#[doc = "GPIO_FUNC223_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func223_in_sel_cfg_reg](gpio_func223_in_sel_cfg_reg) module"]
pub type GPIO_FUNC223_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC223_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC223_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func223_in_sel_cfg_reg::R](gpio_func223_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC223_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func223_in_sel_cfg_reg::W](gpio_func223_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC223_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC223_IN_SEL_CFG_REG"]
pub mod gpio_func223_in_sel_cfg_reg;
#[doc = "GPIO_FUNC224_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func224_in_sel_cfg_reg](gpio_func224_in_sel_cfg_reg) module"]
pub type GPIO_FUNC224_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC224_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC224_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func224_in_sel_cfg_reg::R](gpio_func224_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC224_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func224_in_sel_cfg_reg::W](gpio_func224_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC224_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC224_IN_SEL_CFG_REG"]
pub mod gpio_func224_in_sel_cfg_reg;
#[doc = "GPIO_FUNC225_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func225_in_sel_cfg_reg](gpio_func225_in_sel_cfg_reg) module"]
pub type GPIO_FUNC225_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC225_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC225_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func225_in_sel_cfg_reg::R](gpio_func225_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC225_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func225_in_sel_cfg_reg::W](gpio_func225_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC225_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC225_IN_SEL_CFG_REG"]
pub mod gpio_func225_in_sel_cfg_reg;
#[doc = "GPIO_FUNC226_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func226_in_sel_cfg_reg](gpio_func226_in_sel_cfg_reg) module"]
pub type GPIO_FUNC226_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC226_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC226_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func226_in_sel_cfg_reg::R](gpio_func226_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC226_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func226_in_sel_cfg_reg::W](gpio_func226_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC226_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC226_IN_SEL_CFG_REG"]
pub mod gpio_func226_in_sel_cfg_reg;
#[doc = "GPIO_FUNC227_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func227_in_sel_cfg_reg](gpio_func227_in_sel_cfg_reg) module"]
pub type GPIO_FUNC227_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC227_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC227_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func227_in_sel_cfg_reg::R](gpio_func227_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC227_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func227_in_sel_cfg_reg::W](gpio_func227_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC227_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC227_IN_SEL_CFG_REG"]
pub mod gpio_func227_in_sel_cfg_reg;
#[doc = "GPIO_FUNC228_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func228_in_sel_cfg_reg](gpio_func228_in_sel_cfg_reg) module"]
pub type GPIO_FUNC228_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC228_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC228_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func228_in_sel_cfg_reg::R](gpio_func228_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC228_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func228_in_sel_cfg_reg::W](gpio_func228_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC228_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC228_IN_SEL_CFG_REG"]
pub mod gpio_func228_in_sel_cfg_reg;
#[doc = "GPIO_FUNC229_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func229_in_sel_cfg_reg](gpio_func229_in_sel_cfg_reg) module"]
pub type GPIO_FUNC229_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC229_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC229_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func229_in_sel_cfg_reg::R](gpio_func229_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC229_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func229_in_sel_cfg_reg::W](gpio_func229_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC229_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC229_IN_SEL_CFG_REG"]
pub mod gpio_func229_in_sel_cfg_reg;
#[doc = "GPIO_FUNC230_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func230_in_sel_cfg_reg](gpio_func230_in_sel_cfg_reg) module"]
pub type GPIO_FUNC230_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC230_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC230_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func230_in_sel_cfg_reg::R](gpio_func230_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC230_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func230_in_sel_cfg_reg::W](gpio_func230_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC230_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC230_IN_SEL_CFG_REG"]
pub mod gpio_func230_in_sel_cfg_reg;
#[doc = "GPIO_FUNC231_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func231_in_sel_cfg_reg](gpio_func231_in_sel_cfg_reg) module"]
pub type GPIO_FUNC231_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC231_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC231_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func231_in_sel_cfg_reg::R](gpio_func231_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC231_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func231_in_sel_cfg_reg::W](gpio_func231_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC231_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC231_IN_SEL_CFG_REG"]
pub mod gpio_func231_in_sel_cfg_reg;
#[doc = "GPIO_FUNC232_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func232_in_sel_cfg_reg](gpio_func232_in_sel_cfg_reg) module"]
pub type GPIO_FUNC232_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC232_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC232_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func232_in_sel_cfg_reg::R](gpio_func232_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC232_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func232_in_sel_cfg_reg::W](gpio_func232_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC232_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC232_IN_SEL_CFG_REG"]
pub mod gpio_func232_in_sel_cfg_reg;
#[doc = "GPIO_FUNC233_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func233_in_sel_cfg_reg](gpio_func233_in_sel_cfg_reg) module"]
pub type GPIO_FUNC233_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC233_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC233_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func233_in_sel_cfg_reg::R](gpio_func233_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC233_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func233_in_sel_cfg_reg::W](gpio_func233_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC233_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC233_IN_SEL_CFG_REG"]
pub mod gpio_func233_in_sel_cfg_reg;
#[doc = "GPIO_FUNC234_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func234_in_sel_cfg_reg](gpio_func234_in_sel_cfg_reg) module"]
pub type GPIO_FUNC234_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC234_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC234_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func234_in_sel_cfg_reg::R](gpio_func234_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC234_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func234_in_sel_cfg_reg::W](gpio_func234_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC234_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC234_IN_SEL_CFG_REG"]
pub mod gpio_func234_in_sel_cfg_reg;
#[doc = "GPIO_FUNC235_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func235_in_sel_cfg_reg](gpio_func235_in_sel_cfg_reg) module"]
pub type GPIO_FUNC235_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC235_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC235_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func235_in_sel_cfg_reg::R](gpio_func235_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC235_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func235_in_sel_cfg_reg::W](gpio_func235_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC235_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC235_IN_SEL_CFG_REG"]
pub mod gpio_func235_in_sel_cfg_reg;
#[doc = "GPIO_FUNC236_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func236_in_sel_cfg_reg](gpio_func236_in_sel_cfg_reg) module"]
pub type GPIO_FUNC236_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC236_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC236_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func236_in_sel_cfg_reg::R](gpio_func236_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC236_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func236_in_sel_cfg_reg::W](gpio_func236_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC236_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC236_IN_SEL_CFG_REG"]
pub mod gpio_func236_in_sel_cfg_reg;
#[doc = "GPIO_FUNC237_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func237_in_sel_cfg_reg](gpio_func237_in_sel_cfg_reg) module"]
pub type GPIO_FUNC237_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC237_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC237_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func237_in_sel_cfg_reg::R](gpio_func237_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC237_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func237_in_sel_cfg_reg::W](gpio_func237_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC237_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC237_IN_SEL_CFG_REG"]
pub mod gpio_func237_in_sel_cfg_reg;
#[doc = "GPIO_FUNC238_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func238_in_sel_cfg_reg](gpio_func238_in_sel_cfg_reg) module"]
pub type GPIO_FUNC238_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC238_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC238_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func238_in_sel_cfg_reg::R](gpio_func238_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC238_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func238_in_sel_cfg_reg::W](gpio_func238_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC238_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC238_IN_SEL_CFG_REG"]
pub mod gpio_func238_in_sel_cfg_reg;
#[doc = "GPIO_FUNC239_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func239_in_sel_cfg_reg](gpio_func239_in_sel_cfg_reg) module"]
pub type GPIO_FUNC239_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC239_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC239_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func239_in_sel_cfg_reg::R](gpio_func239_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC239_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func239_in_sel_cfg_reg::W](gpio_func239_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC239_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC239_IN_SEL_CFG_REG"]
pub mod gpio_func239_in_sel_cfg_reg;
#[doc = "GPIO_FUNC240_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func240_in_sel_cfg_reg](gpio_func240_in_sel_cfg_reg) module"]
pub type GPIO_FUNC240_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC240_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC240_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func240_in_sel_cfg_reg::R](gpio_func240_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC240_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func240_in_sel_cfg_reg::W](gpio_func240_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC240_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC240_IN_SEL_CFG_REG"]
pub mod gpio_func240_in_sel_cfg_reg;
#[doc = "GPIO_FUNC241_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func241_in_sel_cfg_reg](gpio_func241_in_sel_cfg_reg) module"]
pub type GPIO_FUNC241_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC241_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC241_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func241_in_sel_cfg_reg::R](gpio_func241_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC241_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func241_in_sel_cfg_reg::W](gpio_func241_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC241_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC241_IN_SEL_CFG_REG"]
pub mod gpio_func241_in_sel_cfg_reg;
#[doc = "GPIO_FUNC242_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func242_in_sel_cfg_reg](gpio_func242_in_sel_cfg_reg) module"]
pub type GPIO_FUNC242_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC242_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC242_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func242_in_sel_cfg_reg::R](gpio_func242_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC242_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func242_in_sel_cfg_reg::W](gpio_func242_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC242_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC242_IN_SEL_CFG_REG"]
pub mod gpio_func242_in_sel_cfg_reg;
#[doc = "GPIO_FUNC243_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func243_in_sel_cfg_reg](gpio_func243_in_sel_cfg_reg) module"]
pub type GPIO_FUNC243_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC243_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC243_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func243_in_sel_cfg_reg::R](gpio_func243_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC243_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func243_in_sel_cfg_reg::W](gpio_func243_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC243_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC243_IN_SEL_CFG_REG"]
pub mod gpio_func243_in_sel_cfg_reg;
#[doc = "GPIO_FUNC244_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func244_in_sel_cfg_reg](gpio_func244_in_sel_cfg_reg) module"]
pub type GPIO_FUNC244_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC244_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC244_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func244_in_sel_cfg_reg::R](gpio_func244_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC244_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func244_in_sel_cfg_reg::W](gpio_func244_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC244_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC244_IN_SEL_CFG_REG"]
pub mod gpio_func244_in_sel_cfg_reg;
#[doc = "GPIO_FUNC245_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func245_in_sel_cfg_reg](gpio_func245_in_sel_cfg_reg) module"]
pub type GPIO_FUNC245_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC245_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC245_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func245_in_sel_cfg_reg::R](gpio_func245_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC245_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func245_in_sel_cfg_reg::W](gpio_func245_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC245_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC245_IN_SEL_CFG_REG"]
pub mod gpio_func245_in_sel_cfg_reg;
#[doc = "GPIO_FUNC246_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func246_in_sel_cfg_reg](gpio_func246_in_sel_cfg_reg) module"]
pub type GPIO_FUNC246_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC246_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC246_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func246_in_sel_cfg_reg::R](gpio_func246_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC246_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func246_in_sel_cfg_reg::W](gpio_func246_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC246_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC246_IN_SEL_CFG_REG"]
pub mod gpio_func246_in_sel_cfg_reg;
#[doc = "GPIO_FUNC247_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func247_in_sel_cfg_reg](gpio_func247_in_sel_cfg_reg) module"]
pub type GPIO_FUNC247_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC247_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC247_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func247_in_sel_cfg_reg::R](gpio_func247_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC247_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func247_in_sel_cfg_reg::W](gpio_func247_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC247_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC247_IN_SEL_CFG_REG"]
pub mod gpio_func247_in_sel_cfg_reg;
#[doc = "GPIO_FUNC248_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func248_in_sel_cfg_reg](gpio_func248_in_sel_cfg_reg) module"]
pub type GPIO_FUNC248_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC248_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC248_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func248_in_sel_cfg_reg::R](gpio_func248_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC248_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func248_in_sel_cfg_reg::W](gpio_func248_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC248_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC248_IN_SEL_CFG_REG"]
pub mod gpio_func248_in_sel_cfg_reg;
#[doc = "GPIO_FUNC249_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func249_in_sel_cfg_reg](gpio_func249_in_sel_cfg_reg) module"]
pub type GPIO_FUNC249_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC249_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC249_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func249_in_sel_cfg_reg::R](gpio_func249_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC249_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func249_in_sel_cfg_reg::W](gpio_func249_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC249_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC249_IN_SEL_CFG_REG"]
pub mod gpio_func249_in_sel_cfg_reg;
#[doc = "GPIO_FUNC250_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func250_in_sel_cfg_reg](gpio_func250_in_sel_cfg_reg) module"]
pub type GPIO_FUNC250_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC250_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC250_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func250_in_sel_cfg_reg::R](gpio_func250_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC250_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func250_in_sel_cfg_reg::W](gpio_func250_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC250_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC250_IN_SEL_CFG_REG"]
pub mod gpio_func250_in_sel_cfg_reg;
#[doc = "GPIO_FUNC251_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func251_in_sel_cfg_reg](gpio_func251_in_sel_cfg_reg) module"]
pub type GPIO_FUNC251_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC251_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC251_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func251_in_sel_cfg_reg::R](gpio_func251_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC251_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func251_in_sel_cfg_reg::W](gpio_func251_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC251_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC251_IN_SEL_CFG_REG"]
pub mod gpio_func251_in_sel_cfg_reg;
#[doc = "GPIO_FUNC252_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func252_in_sel_cfg_reg](gpio_func252_in_sel_cfg_reg) module"]
pub type GPIO_FUNC252_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC252_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC252_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func252_in_sel_cfg_reg::R](gpio_func252_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC252_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func252_in_sel_cfg_reg::W](gpio_func252_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC252_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC252_IN_SEL_CFG_REG"]
pub mod gpio_func252_in_sel_cfg_reg;
#[doc = "GPIO_FUNC253_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func253_in_sel_cfg_reg](gpio_func253_in_sel_cfg_reg) module"]
pub type GPIO_FUNC253_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC253_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC253_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func253_in_sel_cfg_reg::R](gpio_func253_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC253_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func253_in_sel_cfg_reg::W](gpio_func253_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC253_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC253_IN_SEL_CFG_REG"]
pub mod gpio_func253_in_sel_cfg_reg;
#[doc = "GPIO_FUNC254_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func254_in_sel_cfg_reg](gpio_func254_in_sel_cfg_reg) module"]
pub type GPIO_FUNC254_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC254_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC254_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func254_in_sel_cfg_reg::R](gpio_func254_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC254_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func254_in_sel_cfg_reg::W](gpio_func254_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC254_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC254_IN_SEL_CFG_REG"]
pub mod gpio_func254_in_sel_cfg_reg;
#[doc = "GPIO_FUNC255_IN_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func255_in_sel_cfg_reg](gpio_func255_in_sel_cfg_reg) module"]
pub type GPIO_FUNC255_IN_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC255_IN_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC255_IN_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func255_in_sel_cfg_reg::R](gpio_func255_in_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC255_IN_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func255_in_sel_cfg_reg::W](gpio_func255_in_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC255_IN_SEL_CFG_REG {}
#[doc = "GPIO_FUNC255_IN_SEL_CFG_REG"]
pub mod gpio_func255_in_sel_cfg_reg;
#[doc = "GPIO_FUNC0_OUT_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func0_out_sel_cfg_reg](gpio_func0_out_sel_cfg_reg) module"]
pub type GPIO_FUNC0_OUT_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC0_OUT_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC0_OUT_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func0_out_sel_cfg_reg::R](gpio_func0_out_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC0_OUT_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func0_out_sel_cfg_reg::W](gpio_func0_out_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC0_OUT_SEL_CFG_REG {}
#[doc = "GPIO_FUNC0_OUT_SEL_CFG_REG"]
pub mod gpio_func0_out_sel_cfg_reg;
#[doc = "GPIO_FUNC1_OUT_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func1_out_sel_cfg_reg](gpio_func1_out_sel_cfg_reg) module"]
pub type GPIO_FUNC1_OUT_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC1_OUT_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC1_OUT_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func1_out_sel_cfg_reg::R](gpio_func1_out_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC1_OUT_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func1_out_sel_cfg_reg::W](gpio_func1_out_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC1_OUT_SEL_CFG_REG {}
#[doc = "GPIO_FUNC1_OUT_SEL_CFG_REG"]
pub mod gpio_func1_out_sel_cfg_reg;
#[doc = "GPIO_FUNC2_OUT_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func2_out_sel_cfg_reg](gpio_func2_out_sel_cfg_reg) module"]
pub type GPIO_FUNC2_OUT_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC2_OUT_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC2_OUT_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func2_out_sel_cfg_reg::R](gpio_func2_out_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC2_OUT_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func2_out_sel_cfg_reg::W](gpio_func2_out_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC2_OUT_SEL_CFG_REG {}
#[doc = "GPIO_FUNC2_OUT_SEL_CFG_REG"]
pub mod gpio_func2_out_sel_cfg_reg;
#[doc = "GPIO_FUNC3_OUT_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func3_out_sel_cfg_reg](gpio_func3_out_sel_cfg_reg) module"]
pub type GPIO_FUNC3_OUT_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC3_OUT_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC3_OUT_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func3_out_sel_cfg_reg::R](gpio_func3_out_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC3_OUT_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func3_out_sel_cfg_reg::W](gpio_func3_out_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC3_OUT_SEL_CFG_REG {}
#[doc = "GPIO_FUNC3_OUT_SEL_CFG_REG"]
pub mod gpio_func3_out_sel_cfg_reg;
#[doc = "GPIO_FUNC4_OUT_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func4_out_sel_cfg_reg](gpio_func4_out_sel_cfg_reg) module"]
pub type GPIO_FUNC4_OUT_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC4_OUT_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC4_OUT_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func4_out_sel_cfg_reg::R](gpio_func4_out_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC4_OUT_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func4_out_sel_cfg_reg::W](gpio_func4_out_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC4_OUT_SEL_CFG_REG {}
#[doc = "GPIO_FUNC4_OUT_SEL_CFG_REG"]
pub mod gpio_func4_out_sel_cfg_reg;
#[doc = "GPIO_FUNC5_OUT_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func5_out_sel_cfg_reg](gpio_func5_out_sel_cfg_reg) module"]
pub type GPIO_FUNC5_OUT_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC5_OUT_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC5_OUT_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func5_out_sel_cfg_reg::R](gpio_func5_out_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC5_OUT_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func5_out_sel_cfg_reg::W](gpio_func5_out_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC5_OUT_SEL_CFG_REG {}
#[doc = "GPIO_FUNC5_OUT_SEL_CFG_REG"]
pub mod gpio_func5_out_sel_cfg_reg;
#[doc = "GPIO_FUNC6_OUT_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func6_out_sel_cfg_reg](gpio_func6_out_sel_cfg_reg) module"]
pub type GPIO_FUNC6_OUT_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC6_OUT_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC6_OUT_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func6_out_sel_cfg_reg::R](gpio_func6_out_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC6_OUT_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func6_out_sel_cfg_reg::W](gpio_func6_out_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC6_OUT_SEL_CFG_REG {}
#[doc = "GPIO_FUNC6_OUT_SEL_CFG_REG"]
pub mod gpio_func6_out_sel_cfg_reg;
#[doc = "GPIO_FUNC7_OUT_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func7_out_sel_cfg_reg](gpio_func7_out_sel_cfg_reg) module"]
pub type GPIO_FUNC7_OUT_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC7_OUT_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC7_OUT_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func7_out_sel_cfg_reg::R](gpio_func7_out_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC7_OUT_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func7_out_sel_cfg_reg::W](gpio_func7_out_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC7_OUT_SEL_CFG_REG {}
#[doc = "GPIO_FUNC7_OUT_SEL_CFG_REG"]
pub mod gpio_func7_out_sel_cfg_reg;
#[doc = "GPIO_FUNC8_OUT_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func8_out_sel_cfg_reg](gpio_func8_out_sel_cfg_reg) module"]
pub type GPIO_FUNC8_OUT_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC8_OUT_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC8_OUT_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func8_out_sel_cfg_reg::R](gpio_func8_out_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC8_OUT_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func8_out_sel_cfg_reg::W](gpio_func8_out_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC8_OUT_SEL_CFG_REG {}
#[doc = "GPIO_FUNC8_OUT_SEL_CFG_REG"]
pub mod gpio_func8_out_sel_cfg_reg;
#[doc = "GPIO_FUNC9_OUT_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func9_out_sel_cfg_reg](gpio_func9_out_sel_cfg_reg) module"]
pub type GPIO_FUNC9_OUT_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC9_OUT_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC9_OUT_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func9_out_sel_cfg_reg::R](gpio_func9_out_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC9_OUT_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func9_out_sel_cfg_reg::W](gpio_func9_out_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC9_OUT_SEL_CFG_REG {}
#[doc = "GPIO_FUNC9_OUT_SEL_CFG_REG"]
pub mod gpio_func9_out_sel_cfg_reg;
#[doc = "GPIO_FUNC10_OUT_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func10_out_sel_cfg_reg](gpio_func10_out_sel_cfg_reg) module"]
pub type GPIO_FUNC10_OUT_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC10_OUT_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC10_OUT_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func10_out_sel_cfg_reg::R](gpio_func10_out_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC10_OUT_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func10_out_sel_cfg_reg::W](gpio_func10_out_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC10_OUT_SEL_CFG_REG {}
#[doc = "GPIO_FUNC10_OUT_SEL_CFG_REG"]
pub mod gpio_func10_out_sel_cfg_reg;
#[doc = "GPIO_FUNC11_OUT_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func11_out_sel_cfg_reg](gpio_func11_out_sel_cfg_reg) module"]
pub type GPIO_FUNC11_OUT_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC11_OUT_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC11_OUT_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func11_out_sel_cfg_reg::R](gpio_func11_out_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC11_OUT_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func11_out_sel_cfg_reg::W](gpio_func11_out_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC11_OUT_SEL_CFG_REG {}
#[doc = "GPIO_FUNC11_OUT_SEL_CFG_REG"]
pub mod gpio_func11_out_sel_cfg_reg;
#[doc = "GPIO_FUNC12_OUT_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func12_out_sel_cfg_reg](gpio_func12_out_sel_cfg_reg) module"]
pub type GPIO_FUNC12_OUT_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC12_OUT_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC12_OUT_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func12_out_sel_cfg_reg::R](gpio_func12_out_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC12_OUT_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func12_out_sel_cfg_reg::W](gpio_func12_out_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC12_OUT_SEL_CFG_REG {}
#[doc = "GPIO_FUNC12_OUT_SEL_CFG_REG"]
pub mod gpio_func12_out_sel_cfg_reg;
#[doc = "GPIO_FUNC13_OUT_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func13_out_sel_cfg_reg](gpio_func13_out_sel_cfg_reg) module"]
pub type GPIO_FUNC13_OUT_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC13_OUT_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC13_OUT_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func13_out_sel_cfg_reg::R](gpio_func13_out_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC13_OUT_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func13_out_sel_cfg_reg::W](gpio_func13_out_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC13_OUT_SEL_CFG_REG {}
#[doc = "GPIO_FUNC13_OUT_SEL_CFG_REG"]
pub mod gpio_func13_out_sel_cfg_reg;
#[doc = "GPIO_FUNC14_OUT_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func14_out_sel_cfg_reg](gpio_func14_out_sel_cfg_reg) module"]
pub type GPIO_FUNC14_OUT_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC14_OUT_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC14_OUT_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func14_out_sel_cfg_reg::R](gpio_func14_out_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC14_OUT_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func14_out_sel_cfg_reg::W](gpio_func14_out_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC14_OUT_SEL_CFG_REG {}
#[doc = "GPIO_FUNC14_OUT_SEL_CFG_REG"]
pub mod gpio_func14_out_sel_cfg_reg;
#[doc = "GPIO_FUNC15_OUT_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func15_out_sel_cfg_reg](gpio_func15_out_sel_cfg_reg) module"]
pub type GPIO_FUNC15_OUT_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC15_OUT_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC15_OUT_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func15_out_sel_cfg_reg::R](gpio_func15_out_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC15_OUT_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func15_out_sel_cfg_reg::W](gpio_func15_out_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC15_OUT_SEL_CFG_REG {}
#[doc = "GPIO_FUNC15_OUT_SEL_CFG_REG"]
pub mod gpio_func15_out_sel_cfg_reg;
#[doc = "GPIO_FUNC16_OUT_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func16_out_sel_cfg_reg](gpio_func16_out_sel_cfg_reg) module"]
pub type GPIO_FUNC16_OUT_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC16_OUT_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC16_OUT_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func16_out_sel_cfg_reg::R](gpio_func16_out_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC16_OUT_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func16_out_sel_cfg_reg::W](gpio_func16_out_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC16_OUT_SEL_CFG_REG {}
#[doc = "GPIO_FUNC16_OUT_SEL_CFG_REG"]
pub mod gpio_func16_out_sel_cfg_reg;
#[doc = "GPIO_FUNC17_OUT_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func17_out_sel_cfg_reg](gpio_func17_out_sel_cfg_reg) module"]
pub type GPIO_FUNC17_OUT_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC17_OUT_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC17_OUT_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func17_out_sel_cfg_reg::R](gpio_func17_out_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC17_OUT_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func17_out_sel_cfg_reg::W](gpio_func17_out_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC17_OUT_SEL_CFG_REG {}
#[doc = "GPIO_FUNC17_OUT_SEL_CFG_REG"]
pub mod gpio_func17_out_sel_cfg_reg;
#[doc = "GPIO_FUNC18_OUT_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func18_out_sel_cfg_reg](gpio_func18_out_sel_cfg_reg) module"]
pub type GPIO_FUNC18_OUT_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC18_OUT_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC18_OUT_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func18_out_sel_cfg_reg::R](gpio_func18_out_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC18_OUT_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func18_out_sel_cfg_reg::W](gpio_func18_out_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC18_OUT_SEL_CFG_REG {}
#[doc = "GPIO_FUNC18_OUT_SEL_CFG_REG"]
pub mod gpio_func18_out_sel_cfg_reg;
#[doc = "GPIO_FUNC19_OUT_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func19_out_sel_cfg_reg](gpio_func19_out_sel_cfg_reg) module"]
pub type GPIO_FUNC19_OUT_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC19_OUT_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC19_OUT_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func19_out_sel_cfg_reg::R](gpio_func19_out_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC19_OUT_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func19_out_sel_cfg_reg::W](gpio_func19_out_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC19_OUT_SEL_CFG_REG {}
#[doc = "GPIO_FUNC19_OUT_SEL_CFG_REG"]
pub mod gpio_func19_out_sel_cfg_reg;
#[doc = "GPIO_FUNC20_OUT_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func20_out_sel_cfg_reg](gpio_func20_out_sel_cfg_reg) module"]
pub type GPIO_FUNC20_OUT_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC20_OUT_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC20_OUT_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func20_out_sel_cfg_reg::R](gpio_func20_out_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC20_OUT_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func20_out_sel_cfg_reg::W](gpio_func20_out_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC20_OUT_SEL_CFG_REG {}
#[doc = "GPIO_FUNC20_OUT_SEL_CFG_REG"]
pub mod gpio_func20_out_sel_cfg_reg;
#[doc = "GPIO_FUNC21_OUT_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func21_out_sel_cfg_reg](gpio_func21_out_sel_cfg_reg) module"]
pub type GPIO_FUNC21_OUT_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC21_OUT_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC21_OUT_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func21_out_sel_cfg_reg::R](gpio_func21_out_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC21_OUT_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func21_out_sel_cfg_reg::W](gpio_func21_out_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC21_OUT_SEL_CFG_REG {}
#[doc = "GPIO_FUNC21_OUT_SEL_CFG_REG"]
pub mod gpio_func21_out_sel_cfg_reg;
#[doc = "GPIO_FUNC22_OUT_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func22_out_sel_cfg_reg](gpio_func22_out_sel_cfg_reg) module"]
pub type GPIO_FUNC22_OUT_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC22_OUT_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC22_OUT_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func22_out_sel_cfg_reg::R](gpio_func22_out_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC22_OUT_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func22_out_sel_cfg_reg::W](gpio_func22_out_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC22_OUT_SEL_CFG_REG {}
#[doc = "GPIO_FUNC22_OUT_SEL_CFG_REG"]
pub mod gpio_func22_out_sel_cfg_reg;
#[doc = "GPIO_FUNC23_OUT_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func23_out_sel_cfg_reg](gpio_func23_out_sel_cfg_reg) module"]
pub type GPIO_FUNC23_OUT_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC23_OUT_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC23_OUT_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func23_out_sel_cfg_reg::R](gpio_func23_out_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC23_OUT_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func23_out_sel_cfg_reg::W](gpio_func23_out_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC23_OUT_SEL_CFG_REG {}
#[doc = "GPIO_FUNC23_OUT_SEL_CFG_REG"]
pub mod gpio_func23_out_sel_cfg_reg;
#[doc = "GPIO_FUNC24_OUT_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func24_out_sel_cfg_reg](gpio_func24_out_sel_cfg_reg) module"]
pub type GPIO_FUNC24_OUT_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC24_OUT_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC24_OUT_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func24_out_sel_cfg_reg::R](gpio_func24_out_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC24_OUT_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func24_out_sel_cfg_reg::W](gpio_func24_out_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC24_OUT_SEL_CFG_REG {}
#[doc = "GPIO_FUNC24_OUT_SEL_CFG_REG"]
pub mod gpio_func24_out_sel_cfg_reg;
#[doc = "GPIO_FUNC25_OUT_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func25_out_sel_cfg_reg](gpio_func25_out_sel_cfg_reg) module"]
pub type GPIO_FUNC25_OUT_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC25_OUT_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC25_OUT_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func25_out_sel_cfg_reg::R](gpio_func25_out_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC25_OUT_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func25_out_sel_cfg_reg::W](gpio_func25_out_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC25_OUT_SEL_CFG_REG {}
#[doc = "GPIO_FUNC25_OUT_SEL_CFG_REG"]
pub mod gpio_func25_out_sel_cfg_reg;
#[doc = "GPIO_FUNC26_OUT_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func26_out_sel_cfg_reg](gpio_func26_out_sel_cfg_reg) module"]
pub type GPIO_FUNC26_OUT_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC26_OUT_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC26_OUT_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func26_out_sel_cfg_reg::R](gpio_func26_out_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC26_OUT_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func26_out_sel_cfg_reg::W](gpio_func26_out_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC26_OUT_SEL_CFG_REG {}
#[doc = "GPIO_FUNC26_OUT_SEL_CFG_REG"]
pub mod gpio_func26_out_sel_cfg_reg;
#[doc = "GPIO_FUNC27_OUT_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func27_out_sel_cfg_reg](gpio_func27_out_sel_cfg_reg) module"]
pub type GPIO_FUNC27_OUT_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC27_OUT_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC27_OUT_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func27_out_sel_cfg_reg::R](gpio_func27_out_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC27_OUT_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func27_out_sel_cfg_reg::W](gpio_func27_out_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC27_OUT_SEL_CFG_REG {}
#[doc = "GPIO_FUNC27_OUT_SEL_CFG_REG"]
pub mod gpio_func27_out_sel_cfg_reg;
#[doc = "GPIO_FUNC28_OUT_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func28_out_sel_cfg_reg](gpio_func28_out_sel_cfg_reg) module"]
pub type GPIO_FUNC28_OUT_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC28_OUT_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC28_OUT_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func28_out_sel_cfg_reg::R](gpio_func28_out_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC28_OUT_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func28_out_sel_cfg_reg::W](gpio_func28_out_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC28_OUT_SEL_CFG_REG {}
#[doc = "GPIO_FUNC28_OUT_SEL_CFG_REG"]
pub mod gpio_func28_out_sel_cfg_reg;
#[doc = "GPIO_FUNC29_OUT_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func29_out_sel_cfg_reg](gpio_func29_out_sel_cfg_reg) module"]
pub type GPIO_FUNC29_OUT_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC29_OUT_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC29_OUT_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func29_out_sel_cfg_reg::R](gpio_func29_out_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC29_OUT_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func29_out_sel_cfg_reg::W](gpio_func29_out_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC29_OUT_SEL_CFG_REG {}
#[doc = "GPIO_FUNC29_OUT_SEL_CFG_REG"]
pub mod gpio_func29_out_sel_cfg_reg;
#[doc = "GPIO_FUNC30_OUT_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func30_out_sel_cfg_reg](gpio_func30_out_sel_cfg_reg) module"]
pub type GPIO_FUNC30_OUT_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC30_OUT_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC30_OUT_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func30_out_sel_cfg_reg::R](gpio_func30_out_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC30_OUT_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func30_out_sel_cfg_reg::W](gpio_func30_out_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC30_OUT_SEL_CFG_REG {}
#[doc = "GPIO_FUNC30_OUT_SEL_CFG_REG"]
pub mod gpio_func30_out_sel_cfg_reg;
#[doc = "GPIO_FUNC31_OUT_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func31_out_sel_cfg_reg](gpio_func31_out_sel_cfg_reg) module"]
pub type GPIO_FUNC31_OUT_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC31_OUT_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC31_OUT_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func31_out_sel_cfg_reg::R](gpio_func31_out_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC31_OUT_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func31_out_sel_cfg_reg::W](gpio_func31_out_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC31_OUT_SEL_CFG_REG {}
#[doc = "GPIO_FUNC31_OUT_SEL_CFG_REG"]
pub mod gpio_func31_out_sel_cfg_reg;
#[doc = "GPIO_FUNC32_OUT_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func32_out_sel_cfg_reg](gpio_func32_out_sel_cfg_reg) module"]
pub type GPIO_FUNC32_OUT_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC32_OUT_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC32_OUT_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func32_out_sel_cfg_reg::R](gpio_func32_out_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC32_OUT_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func32_out_sel_cfg_reg::W](gpio_func32_out_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC32_OUT_SEL_CFG_REG {}
#[doc = "GPIO_FUNC32_OUT_SEL_CFG_REG"]
pub mod gpio_func32_out_sel_cfg_reg;
#[doc = "GPIO_FUNC33_OUT_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func33_out_sel_cfg_reg](gpio_func33_out_sel_cfg_reg) module"]
pub type GPIO_FUNC33_OUT_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC33_OUT_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC33_OUT_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func33_out_sel_cfg_reg::R](gpio_func33_out_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC33_OUT_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func33_out_sel_cfg_reg::W](gpio_func33_out_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC33_OUT_SEL_CFG_REG {}
#[doc = "GPIO_FUNC33_OUT_SEL_CFG_REG"]
pub mod gpio_func33_out_sel_cfg_reg;
#[doc = "GPIO_FUNC34_OUT_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func34_out_sel_cfg_reg](gpio_func34_out_sel_cfg_reg) module"]
pub type GPIO_FUNC34_OUT_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC34_OUT_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC34_OUT_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func34_out_sel_cfg_reg::R](gpio_func34_out_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC34_OUT_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func34_out_sel_cfg_reg::W](gpio_func34_out_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC34_OUT_SEL_CFG_REG {}
#[doc = "GPIO_FUNC34_OUT_SEL_CFG_REG"]
pub mod gpio_func34_out_sel_cfg_reg;
#[doc = "GPIO_FUNC35_OUT_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func35_out_sel_cfg_reg](gpio_func35_out_sel_cfg_reg) module"]
pub type GPIO_FUNC35_OUT_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC35_OUT_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC35_OUT_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func35_out_sel_cfg_reg::R](gpio_func35_out_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC35_OUT_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func35_out_sel_cfg_reg::W](gpio_func35_out_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC35_OUT_SEL_CFG_REG {}
#[doc = "GPIO_FUNC35_OUT_SEL_CFG_REG"]
pub mod gpio_func35_out_sel_cfg_reg;
#[doc = "GPIO_FUNC36_OUT_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func36_out_sel_cfg_reg](gpio_func36_out_sel_cfg_reg) module"]
pub type GPIO_FUNC36_OUT_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC36_OUT_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC36_OUT_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func36_out_sel_cfg_reg::R](gpio_func36_out_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC36_OUT_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func36_out_sel_cfg_reg::W](gpio_func36_out_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC36_OUT_SEL_CFG_REG {}
#[doc = "GPIO_FUNC36_OUT_SEL_CFG_REG"]
pub mod gpio_func36_out_sel_cfg_reg;
#[doc = "GPIO_FUNC37_OUT_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func37_out_sel_cfg_reg](gpio_func37_out_sel_cfg_reg) module"]
pub type GPIO_FUNC37_OUT_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC37_OUT_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC37_OUT_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func37_out_sel_cfg_reg::R](gpio_func37_out_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC37_OUT_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func37_out_sel_cfg_reg::W](gpio_func37_out_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC37_OUT_SEL_CFG_REG {}
#[doc = "GPIO_FUNC37_OUT_SEL_CFG_REG"]
pub mod gpio_func37_out_sel_cfg_reg;
#[doc = "GPIO_FUNC38_OUT_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func38_out_sel_cfg_reg](gpio_func38_out_sel_cfg_reg) module"]
pub type GPIO_FUNC38_OUT_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC38_OUT_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC38_OUT_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func38_out_sel_cfg_reg::R](gpio_func38_out_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC38_OUT_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func38_out_sel_cfg_reg::W](gpio_func38_out_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC38_OUT_SEL_CFG_REG {}
#[doc = "GPIO_FUNC38_OUT_SEL_CFG_REG"]
pub mod gpio_func38_out_sel_cfg_reg;
#[doc = "GPIO_FUNC39_OUT_SEL_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_func39_out_sel_cfg_reg](gpio_func39_out_sel_cfg_reg) module"]
pub type GPIO_FUNC39_OUT_SEL_CFG_REG = crate::Reg<u32, _GPIO_FUNC39_OUT_SEL_CFG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_FUNC39_OUT_SEL_CFG_REG;
#[doc = "`read()` method returns [gpio_func39_out_sel_cfg_reg::R](gpio_func39_out_sel_cfg_reg::R) reader structure"]
impl crate::Readable for GPIO_FUNC39_OUT_SEL_CFG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_func39_out_sel_cfg_reg::W](gpio_func39_out_sel_cfg_reg::W) writer structure"]
impl crate::Writable for GPIO_FUNC39_OUT_SEL_CFG_REG {}
#[doc = "GPIO_FUNC39_OUT_SEL_CFG_REG"]
pub mod gpio_func39_out_sel_cfg_reg;