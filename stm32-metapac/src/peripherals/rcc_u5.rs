#[doc = "Reset and clock control"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rcc {
    ptr: *mut u8,
}
unsafe impl Send for Rcc {}
unsafe impl Sync for Rcc {}
impl Rcc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "RCC clock control register"]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "RCC internal clock sources calibration register 1"]
    #[inline(always)]
    pub const fn icscr1(self) -> crate::common::Reg<regs::Icscr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize) as _) }
    }
    #[doc = "RCC internal clock sources calibration register 2"]
    #[inline(always)]
    pub const fn icscr2(self) -> crate::common::Reg<regs::Icscr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(12usize) as _) }
    }
    #[doc = "RCC internal clock sources calibration register 3"]
    #[inline(always)]
    pub const fn icscr3(self) -> crate::common::Reg<regs::Icscr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize) as _) }
    }
    #[doc = "RCC clock recovery RC register"]
    #[inline(always)]
    pub const fn crrcr(self) -> crate::common::Reg<regs::Crrcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(20usize) as _) }
    }
    #[doc = "RCC clock configuration register 1"]
    #[inline(always)]
    pub const fn cfgr1(self) -> crate::common::Reg<regs::Cfgr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(28usize) as _) }
    }
    #[doc = "RCC clock configuration register 2"]
    #[inline(always)]
    pub const fn cfgr2(self) -> crate::common::Reg<regs::Cfgr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(32usize) as _) }
    }
    #[doc = "RCC clock configuration register 3"]
    #[inline(always)]
    pub const fn cfgr3(self) -> crate::common::Reg<regs::Cfgr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(36usize) as _) }
    }
    #[doc = "RCC PLL1 configuration register"]
    #[inline(always)]
    pub const fn pll1cfgr(self) -> crate::common::Reg<regs::Pll1cfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(40usize) as _) }
    }
    #[doc = "RCC PLL2 configuration register"]
    #[inline(always)]
    pub const fn pll2cfgr(self) -> crate::common::Reg<regs::Pll23cfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(44usize) as _) }
    }
    #[doc = "RCC PLL3 configuration register"]
    #[inline(always)]
    pub const fn pll3cfgr(self) -> crate::common::Reg<regs::Pll23cfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(48usize) as _) }
    }
    #[doc = "RCC PLL1 dividers register"]
    #[inline(always)]
    pub const fn pll1divr(self) -> crate::common::Reg<regs::Plldivr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(52usize) as _) }
    }
    #[doc = "RCC PLL1 fractional divider register"]
    #[inline(always)]
    pub const fn pll1fracr(self) -> crate::common::Reg<regs::Pllfracr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(56usize) as _) }
    }
    #[doc = "RCC PLL2 dividers configuration register"]
    #[inline(always)]
    pub const fn pll2divr(self) -> crate::common::Reg<regs::Plldivr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(60usize) as _) }
    }
    #[doc = "RCC PLL2 fractional divider register"]
    #[inline(always)]
    pub const fn pll2fracr(self) -> crate::common::Reg<regs::Pllfracr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(64usize) as _) }
    }
    #[doc = "RCC PLL3 dividers configuration register"]
    #[inline(always)]
    pub const fn pll3divr(self) -> crate::common::Reg<regs::Plldivr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(68usize) as _) }
    }
    #[doc = "RCC PLL3 fractional divider register"]
    #[inline(always)]
    pub const fn pll3fracr(self) -> crate::common::Reg<regs::Pllfracr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(72usize) as _) }
    }
    #[doc = "RCC clock interrupt enable register"]
    #[inline(always)]
    pub const fn cier(self) -> crate::common::Reg<regs::Cier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(80usize) as _) }
    }
    #[doc = "RCC clock interrupt flag register"]
    #[inline(always)]
    pub const fn cifr(self) -> crate::common::Reg<regs::Cifr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(84usize) as _) }
    }
    #[doc = "RCC clock interrupt clear register"]
    #[inline(always)]
    pub const fn cicr(self) -> crate::common::Reg<regs::Cicr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(88usize) as _) }
    }
    #[doc = "RCC AHB1 peripheral reset register"]
    #[inline(always)]
    pub const fn ahb1rstr(self) -> crate::common::Reg<regs::Ahb1rstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(96usize) as _) }
    }
    #[doc = "RCC AHB2 peripheral reset register 1"]
    #[inline(always)]
    pub const fn ahb2rstr1(self) -> crate::common::Reg<regs::Ahb2rstr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(100usize) as _) }
    }
    #[doc = "RCC AHB2 peripheral reset register 2"]
    #[inline(always)]
    pub const fn ahb2rstr2(self) -> crate::common::Reg<regs::Ahb2rstr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(104usize) as _) }
    }
    #[doc = "RCC AHB3 peripheral reset register"]
    #[inline(always)]
    pub const fn ahb3rstr(self) -> crate::common::Reg<regs::Ahb3rstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(108usize) as _) }
    }
    #[doc = "RCC APB1 peripheral reset register 1"]
    #[inline(always)]
    pub const fn apb1rstr1(self) -> crate::common::Reg<regs::Apb1rstr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(116usize) as _) }
    }
    #[doc = "RCC APB1 peripheral reset register 2"]
    #[inline(always)]
    pub const fn apb1rstr2(self) -> crate::common::Reg<regs::Apb1rstr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(120usize) as _) }
    }
    #[doc = "RCC APB2 peripheral reset register"]
    #[inline(always)]
    pub const fn apb2rstr(self) -> crate::common::Reg<regs::Apb2rstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(124usize) as _) }
    }
    #[doc = "RCC APB3 peripheral reset register"]
    #[inline(always)]
    pub const fn apb3rstr(self) -> crate::common::Reg<regs::Apb3rstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(128usize) as _) }
    }
    #[doc = "RCC AHB1 peripheral clock enable register"]
    #[inline(always)]
    pub const fn ahb1enr(self) -> crate::common::Reg<regs::Ahb1enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(136usize) as _) }
    }
    #[doc = "RCC AHB2 peripheral clock enable register 1"]
    #[inline(always)]
    pub const fn ahb2enr1(self) -> crate::common::Reg<regs::Ahb2enr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(140usize) as _) }
    }
    #[doc = "RCC AHB2 peripheral clock enable register 2"]
    #[inline(always)]
    pub const fn ahb2enr2(self) -> crate::common::Reg<regs::Ahb2enr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(144usize) as _) }
    }
    #[doc = "RCC AHB3 peripheral clock enable register"]
    #[inline(always)]
    pub const fn ahb3enr(self) -> crate::common::Reg<regs::Ahb3enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(148usize) as _) }
    }
    #[doc = "RCC APB1 peripheral clock enable register 1"]
    #[inline(always)]
    pub const fn apb1enr1(self) -> crate::common::Reg<regs::Apb1enr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(156usize) as _) }
    }
    #[doc = "RCC APB1 peripheral clock enable register 2"]
    #[inline(always)]
    pub const fn apb1enr2(self) -> crate::common::Reg<regs::Apb1enr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(160usize) as _) }
    }
    #[doc = "RCC APB2 peripheral clock enable register"]
    #[inline(always)]
    pub const fn apb2enr(self) -> crate::common::Reg<regs::Apb2enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(164usize) as _) }
    }
    #[doc = "RCC APB3 peripheral clock enable register"]
    #[inline(always)]
    pub const fn apb3enr(self) -> crate::common::Reg<regs::Apb3enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(168usize) as _) }
    }
    #[doc = "RCC AHB1 peripheral clocks enable in Sleep and Stop modes register"]
    #[inline(always)]
    pub const fn ahb1smenr(self) -> crate::common::Reg<regs::Ahb1smenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(176usize) as _) }
    }
    #[doc = "RCC AHB2 peripheral clocks enable in Sleep and Stop modes register 1"]
    #[inline(always)]
    pub const fn ahb2smenr1(self) -> crate::common::Reg<regs::Ahb2smenr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(180usize) as _) }
    }
    #[doc = "RCC AHB2 peripheral clocks enable in Sleep and Stop modes register 2"]
    #[inline(always)]
    pub const fn ahb2smenr2(self) -> crate::common::Reg<regs::Ahb2smenr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(184usize) as _) }
    }
    #[doc = "RCC AHB3 peripheral clocks enable in Sleep and Stop modes register"]
    #[inline(always)]
    pub const fn ahb3smenr(self) -> crate::common::Reg<regs::Ahb3smenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(188usize) as _) }
    }
    #[doc = "RCC APB1 peripheral clocks enable in Sleep and Stop modes register 1"]
    #[inline(always)]
    pub const fn apb1smenr1(self) -> crate::common::Reg<regs::Apb1smenr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(196usize) as _) }
    }
    #[doc = "RCC APB1 peripheral clocks enable in Sleep and Stop modes register 2"]
    #[inline(always)]
    pub const fn apb1smenr2(self) -> crate::common::Reg<regs::Apb1smenr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(200usize) as _) }
    }
    #[doc = "RCC APB2 peripheral clocks enable in Sleep and Stop modes register"]
    #[inline(always)]
    pub const fn apb2smenr(self) -> crate::common::Reg<regs::Apb2smenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(204usize) as _) }
    }
    #[doc = "RCC APB3 peripheral clock enable in Sleep and Stop modes register"]
    #[inline(always)]
    pub const fn apb3smenr(self) -> crate::common::Reg<regs::Apb3smenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(208usize) as _) }
    }
    #[doc = "RCC SmartRun domain peripheral autonomous mode register"]
    #[inline(always)]
    pub const fn srdamr(self) -> crate::common::Reg<regs::Srdamr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(216usize) as _) }
    }
    #[doc = "RCC peripherals independent clock configuration register 1"]
    #[inline(always)]
    pub const fn ccipr1(self) -> crate::common::Reg<regs::Ccipr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(224usize) as _) }
    }
    #[doc = "RCC peripherals independent clock configuration register 2"]
    #[inline(always)]
    pub const fn ccipr2(self) -> crate::common::Reg<regs::Ccipr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(228usize) as _) }
    }
    #[doc = "RCC peripherals independent clock configuration register 3"]
    #[inline(always)]
    pub const fn ccipr3(self) -> crate::common::Reg<regs::Ccipr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(232usize) as _) }
    }
    #[doc = "RCC Backup domain control register"]
    #[inline(always)]
    pub const fn bdcr(self) -> crate::common::Reg<regs::Bdcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(240usize) as _) }
    }
    #[doc = "RCC control/status register"]
    #[inline(always)]
    pub const fn csr(self) -> crate::common::Reg<regs::Csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(244usize) as _) }
    }
    #[doc = "RCC secure configuration register"]
    #[inline(always)]
    pub const fn seccfgr(self) -> crate::common::Reg<regs::Seccfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(272usize) as _) }
    }
    #[doc = "RCC privilege configuration register"]
    #[inline(always)]
    pub const fn privcfgr(self) -> crate::common::Reg<regs::Privcfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(276usize) as _) }
    }
}
pub mod regs {
    #[doc = "RCC AHB1 peripheral clock enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb1enr(pub u32);
    impl Ahb1enr {
        #[doc = "GPDMA1 clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn gpdma1en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "GPDMA1 clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_gpdma1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "CORDIC clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn cordicen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "CORDIC clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_cordicen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "FMAC clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn fmacen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "FMAC clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_fmacen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "MDF1 clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn mdf1en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "MDF1 clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_mdf1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "FLASH clock enable Set and cleared by software. This bit can be disabled only when the Flash memory is in power down mode."]
        #[inline(always)]
        pub const fn flashen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "FLASH clock enable Set and cleared by software. This bit can be disabled only when the Flash memory is in power down mode."]
        #[inline(always)]
        pub fn set_flashen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "CRC clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn crcen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "CRC clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_crcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "JPEG clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn jpegen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "JPEG clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_jpegen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Touch sensing controller clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn tscen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Touch sensing controller clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_tscen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "RAMCFG clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn ramcfgen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "RAMCFG clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_ramcfgen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "DMA2D clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn dma2den(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "DMA2D clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_dma2den(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "GFXMMU clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn gfxmmuen(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "GFXMMU clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_gfxmmuen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "GPU2D clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn gpu2den(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "GPU2D clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_gpu2den(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "DCACHE2 clock enable This bit is set and reset by software. Note: DCACHE2 clock must be enabled to access memories, even if the DCACHE2 is bypassed. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn dcache2en(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "DCACHE2 clock enable This bit is set and reset by software. Note: DCACHE2 clock must be enabled to access memories, even if the DCACHE2 is bypassed. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_dcache2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "GTZC1 clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn gtzc1en(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "GTZC1 clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_gtzc1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "BKPSRAM clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn bkpsramen(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "BKPSRAM clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_bkpsramen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "DCACHE1 clock enable Set and reset by software. Note: DCACHE1 clock must be enabled when external memories are accessed through OCTOSPI1, OCTOSPI2 or FSMC, even if the DCACHE1 is bypassed."]
        #[inline(always)]
        pub const fn dcache1en(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "DCACHE1 clock enable Set and reset by software. Note: DCACHE1 clock must be enabled when external memories are accessed through OCTOSPI1, OCTOSPI2 or FSMC, even if the DCACHE1 is bypassed."]
        #[inline(always)]
        pub fn set_dcache1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "SRAM1 clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn sram1en(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM1 clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_sram1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ahb1enr {
        #[inline(always)]
        fn default() -> Ahb1enr {
            Ahb1enr(0)
        }
    }
    #[doc = "RCC AHB1 peripheral reset register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb1rstr(pub u32);
    impl Ahb1rstr {
        #[doc = "GPDMA1 reset Set and cleared by software."]
        #[inline(always)]
        pub const fn gpdma1rst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "GPDMA1 reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_gpdma1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "CORDIC reset Set and cleared by software."]
        #[inline(always)]
        pub const fn cordicrst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "CORDIC reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_cordicrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "FMAC reset Set and cleared by software."]
        #[inline(always)]
        pub const fn fmacrst(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "FMAC reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_fmacrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "MDF1 reset Set and cleared by software."]
        #[inline(always)]
        pub const fn mdf1rst(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "MDF1 reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_mdf1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "CRC reset Set and cleared by software."]
        #[inline(always)]
        pub const fn crcrst(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "CRC reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_crcrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "JPEG reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn jpegrst(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "JPEG reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_jpegrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "TSC reset Set and cleared by software."]
        #[inline(always)]
        pub const fn tscrst(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "TSC reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_tscrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "RAMCFG reset Set and cleared by software."]
        #[inline(always)]
        pub const fn ramcfgrst(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "RAMCFG reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_ramcfgrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "DMA2D reset Set and cleared by software."]
        #[inline(always)]
        pub const fn dma2drst(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "DMA2D reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_dma2drst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "GFXMMU reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn gfxmmurst(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "GFXMMU reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_gfxmmurst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "GPU2D reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn gpu2drst(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "GPU2D reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_gpu2drst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
    }
    impl Default for Ahb1rstr {
        #[inline(always)]
        fn default() -> Ahb1rstr {
            Ahb1rstr(0)
        }
    }
    #[doc = "RCC AHB1 peripheral clocks enable in Sleep and Stop modes register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb1smenr(pub u32);
    impl Ahb1smenr {
        #[doc = "GPDMA1 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub const fn gpdma1smen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "GPDMA1 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub fn set_gpdma1smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "CORDIC clocks enable during Sleep and Stop modes Set and cleared by software during Sleep mode."]
        #[inline(always)]
        pub const fn cordicsmen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "CORDIC clocks enable during Sleep and Stop modes Set and cleared by software during Sleep mode."]
        #[inline(always)]
        pub fn set_cordicsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "FMAC clocks enable during Sleep and Stop modes. Set and cleared by software."]
        #[inline(always)]
        pub const fn fmacsmen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "FMAC clocks enable during Sleep and Stop modes. Set and cleared by software."]
        #[inline(always)]
        pub fn set_fmacsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "MDF1 clocks enable during Sleep and Stop modes. Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub const fn mdf1smen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "MDF1 clocks enable during Sleep and Stop modes. Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub fn set_mdf1smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "FLASH clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn flashsmen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "FLASH clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_flashsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "CRC clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn crcsmen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "CRC clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_crcsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "JPEG clocks enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn jpegsmen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "JPEG clocks enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_jpegsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "TSC clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn tscsmen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "TSC clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_tscsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "RAMCFG clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn ramcfgsmen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "RAMCFG clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_ramcfgsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "DMA2D clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn dma2dsmen(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "DMA2D clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_dma2dsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "GFXMMU clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn gfxmmusmen(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "GFXMMU clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_gfxmmusmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "GPU2D clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn gpu2dsmen(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "GPU2D clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_gpu2dsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "DCACHE2 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn dcache2smen(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "DCACHE2 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_dcache2smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "GTZC1 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn gtzc1smen(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "GTZC1 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_gtzc1smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "BKPSRAM clocks enable during Sleep and Stop modes Set and cleared by software"]
        #[inline(always)]
        pub const fn bkpsramsmen(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "BKPSRAM clocks enable during Sleep and Stop modes Set and cleared by software"]
        #[inline(always)]
        pub fn set_bkpsramsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "ICACHE clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn icachesmen(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "ICACHE clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_icachesmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "DCACHE1 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn dcache1smen(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "DCACHE1 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_dcache1smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "SRAM1 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn sram1smen(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM1 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_sram1smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ahb1smenr {
        #[inline(always)]
        fn default() -> Ahb1smenr {
            Ahb1smenr(0)
        }
    }
    #[doc = "RCC AHB2 peripheral clock enable register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb2enr1(pub u32);
    impl Ahb2enr1 {
        #[doc = "IO port A clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn gpioaen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "IO port A clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_gpioaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "IO port B clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn gpioben(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "IO port B clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_gpioben(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "IO port C clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn gpiocen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "IO port C clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_gpiocen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "IO port D clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn gpioden(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "IO port D clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_gpioden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "IO port E clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn gpioeen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "IO port E clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_gpioeen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "IO port F clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn gpiofen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "IO port F clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_gpiofen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "IO port G clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn gpiogen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "IO port G clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_gpiogen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "IO port H clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn gpiohen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "IO port H clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_gpiohen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "IO port I clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn gpioien(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "IO port I clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_gpioien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "I/O port J clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn gpiojen(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "I/O port J clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_gpiojen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "ADC1 and ADC2 clock enable This bit is set and cleared by software. Note: This bit impacts ADC1 in STM32U535/545/575/585, and ADC1/ADC2 in�STM32U59x/5Ax/5Fx/5Gx."]
        #[inline(always)]
        pub const fn adc12en(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "ADC1 and ADC2 clock enable This bit is set and cleared by software. Note: This bit impacts ADC1 in STM32U535/545/575/585, and ADC1/ADC2 in�STM32U59x/5Ax/5Fx/5Gx."]
        #[inline(always)]
        pub fn set_adc12en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "DCMI and PSSI clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn dcmien(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "DCMI and PSSI clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_dcmien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "OTG_FS clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn usb_otg_fsen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "OTG_FS clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_usb_otg_fsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "OTG_HS clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn usb_otg_hsen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "OTG_HS clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_usb_otg_hsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "OTG_HS PHY clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn usb_otg_hs_phyen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "OTG_HS PHY clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_usb_otg_hs_phyen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "AES clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn aesen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "AES clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_aesen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "HASH clock enable Set and cleared by software"]
        #[inline(always)]
        pub const fn hashen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "HASH clock enable Set and cleared by software"]
        #[inline(always)]
        pub fn set_hashen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "RNG clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn rngen(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "RNG clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_rngen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "PKA clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn pkaen(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "PKA clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_pkaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "SAES clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn saesen(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "SAES clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_saesen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "OCTOSPIM clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn octospimen(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "OCTOSPIM clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_octospimen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "OTFDEC1 clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn otfdec1en(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "OTFDEC1 clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_otfdec1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "OTFDEC2 clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn otfdec2en(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "OTFDEC2 clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_otfdec2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "SDMMC1 clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn sdmmc1en(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "SDMMC1 clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_sdmmc1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "SDMMC2 clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn sdmmc2en(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "SDMMC2 clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_sdmmc2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "SRAM2 clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn sram2en(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_sram2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "SRAM3 clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn sram3en(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM3 clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_sram3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ahb2enr1 {
        #[inline(always)]
        fn default() -> Ahb2enr1 {
            Ahb2enr1(0)
        }
    }
    #[doc = "RCC AHB2 peripheral clock enable register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb2enr2(pub u32);
    impl Ahb2enr2 {
        #[doc = "FSMC clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn fsmcen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "FSMC clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_fsmcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "OCTOSPI1 clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn octospi1en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "OCTOSPI1 clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_octospi1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "OCTOSPI2 clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn octospi2en(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "OCTOSPI2 clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_octospi2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "HSPI1 clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn hspi1en(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "HSPI1 clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_hspi1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "SRAM6 clock enable This bit is set and reset by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn sram6en(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM6 clock enable This bit is set and reset by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_sram6en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "SRAM5 clock enable This bit is set and reset by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn sram5en(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM5 clock enable This bit is set and reset by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_sram5en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ahb2enr2 {
        #[inline(always)]
        fn default() -> Ahb2enr2 {
            Ahb2enr2(0)
        }
    }
    #[doc = "RCC AHB2 peripheral reset register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb2rstr1(pub u32);
    impl Ahb2rstr1 {
        #[doc = "IO port A reset Set and cleared by software."]
        #[inline(always)]
        pub const fn gpioarst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "IO port A reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_gpioarst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "IO port B reset Set and cleared by software."]
        #[inline(always)]
        pub const fn gpiobrst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "IO port B reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_gpiobrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "IO port C reset Set and cleared by software."]
        #[inline(always)]
        pub const fn gpiocrst(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "IO port C reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_gpiocrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "IO port D reset Set and cleared by software."]
        #[inline(always)]
        pub const fn gpiodrst(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "IO port D reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_gpiodrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "IO port E reset Set and cleared by software."]
        #[inline(always)]
        pub const fn gpioerst(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "IO port E reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_gpioerst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "IO port F reset Set and cleared by software."]
        #[inline(always)]
        pub const fn gpiofrst(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "IO port F reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_gpiofrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "IO port G reset Set and cleared by software."]
        #[inline(always)]
        pub const fn gpiogrst(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "IO port G reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_gpiogrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "IO port H reset Set and cleared by software."]
        #[inline(always)]
        pub const fn gpiohrst(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "IO port H reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_gpiohrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "IO port I reset Set and cleared by software."]
        #[inline(always)]
        pub const fn gpioirst(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "IO port I reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_gpioirst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "I/O port J reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn gpiojrst(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "I/O port J reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_gpiojrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "ADC1 and ADC2 reset This bit is set and cleared by software. Note: This bit impacts ADC1 in STM32U535/545/575/585, and ADC1/ADC2 in�STM32U59x/5Ax/5Fx/5Gx."]
        #[inline(always)]
        pub const fn adc12rst(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "ADC1 and ADC2 reset This bit is set and cleared by software. Note: This bit impacts ADC1 in STM32U535/545/575/585, and ADC1/ADC2 in�STM32U59x/5Ax/5Fx/5Gx."]
        #[inline(always)]
        pub fn set_adc12rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "DCMI and PSSI reset Set and cleared by software."]
        #[inline(always)]
        pub const fn dcmirst(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "DCMI and PSSI reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_dcmirst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "OTG_FS reset Set and cleared by software."]
        #[inline(always)]
        pub const fn usb_otg_fsrst(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "OTG_FS reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_usb_otg_fsrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "OTG_HS reset Set and cleared by software."]
        #[inline(always)]
        pub const fn usb_otg_hsrst(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "OTG_HS reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_usb_otg_hsrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "AES hardware accelerator reset Set and cleared by software."]
        #[inline(always)]
        pub const fn aesrst(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "AES hardware accelerator reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_aesrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Hash reset Set and cleared by software."]
        #[inline(always)]
        pub const fn hashrst(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Hash reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_hashrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Random number generator reset Set and cleared by software."]
        #[inline(always)]
        pub const fn rngrst(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Random number generator reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_rngrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "PKA reset Set and cleared by software."]
        #[inline(always)]
        pub const fn pkarst(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "PKA reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_pkarst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "SAES hardware accelerator reset Set and cleared by software."]
        #[inline(always)]
        pub const fn saesrst(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "SAES hardware accelerator reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_saesrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "OCTOSPIM reset Set and cleared by software."]
        #[inline(always)]
        pub const fn octospimrst(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "OCTOSPIM reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_octospimrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "OTFDEC1 reset Set and cleared by software."]
        #[inline(always)]
        pub const fn otfdec1rst(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "OTFDEC1 reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_otfdec1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "OTFDEC2 reset Set and cleared by software."]
        #[inline(always)]
        pub const fn otfdec2rst(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "OTFDEC2 reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_otfdec2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "SDMMC1 reset Set and cleared by software."]
        #[inline(always)]
        pub const fn sdmmc1rst(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "SDMMC1 reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_sdmmc1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "SDMMC2 reset Set and cleared by software."]
        #[inline(always)]
        pub const fn sdmmc2rst(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "SDMMC2 reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_sdmmc2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
    }
    impl Default for Ahb2rstr1 {
        #[inline(always)]
        fn default() -> Ahb2rstr1 {
            Ahb2rstr1(0)
        }
    }
    #[doc = "RCC AHB2 peripheral reset register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb2rstr2(pub u32);
    impl Ahb2rstr2 {
        #[doc = "Flexible memory controller reset Set and cleared by software."]
        #[inline(always)]
        pub const fn fsmcrst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Flexible memory controller reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_fsmcrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "OCTOSPI1 reset Set and cleared by software."]
        #[inline(always)]
        pub const fn octospi1rst(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "OCTOSPI1 reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_octospi1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "OCTOSPI2 reset Set and cleared by software."]
        #[inline(always)]
        pub const fn octospi2rst(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "OCTOSPI2 reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_octospi2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "HSPI1 reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn hspi1rst(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "HSPI1 reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_hspi1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
    }
    impl Default for Ahb2rstr2 {
        #[inline(always)]
        fn default() -> Ahb2rstr2 {
            Ahb2rstr2(0)
        }
    }
    #[doc = "RCC AHB2 peripheral clocks enable in Sleep and Stop modes register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb2smenr1(pub u32);
    impl Ahb2smenr1 {
        #[doc = "IO port A clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn gpioasmen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "IO port A clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_gpioasmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "IO port B clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn gpiobsmen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "IO port B clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_gpiobsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "IO port C clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn gpiocsmen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "IO port C clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_gpiocsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "IO port D clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn gpiodsmen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "IO port D clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_gpiodsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "IO port E clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn gpioesmen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "IO port E clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_gpioesmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "IO port F clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn gpiofsmen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "IO port F clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_gpiofsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "IO port G clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn gpiogsmen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "IO port G clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_gpiogsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "IO port H clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn gpiohsmen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "IO port H clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_gpiohsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "IO port I clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn gpioismen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "IO port I clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_gpioismen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "I/O port J clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn gpiojsmen(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "I/O port J clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_gpiojsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "ADC1 and ADC2 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit impacts ADC1 in STM32U535/545/575/585 and ADC1/ADC2 in�STM32U59x/5Ax/5Fx/5Gx."]
        #[inline(always)]
        pub const fn adc12smen(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "ADC1 and ADC2 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit impacts ADC1 in STM32U535/545/575/585 and ADC1/ADC2 in�STM32U59x/5Ax/5Fx/5Gx."]
        #[inline(always)]
        pub fn set_adc12smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "DCMI and PSSI clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn dcmismen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "DCMI and PSSI clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_dcmismen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "OTG_FS clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn usb_otg_fssmen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "OTG_FS clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_usb_otg_fssmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "OTG_HS clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn usb_otg_hssmen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "OTG_HS clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_usb_otg_hssmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "OTG_HS PHY clock enable during Sleep and Stop modes This bit is set and cleared by software Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn usb_otg_hs_physmen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "OTG_HS PHY clock enable during Sleep and Stop modes This bit is set and cleared by software Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_usb_otg_hs_physmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "AES clock enable during Sleep and Stop modes Set and cleared by software"]
        #[inline(always)]
        pub const fn aessmen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "AES clock enable during Sleep and Stop modes Set and cleared by software"]
        #[inline(always)]
        pub fn set_aessmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "HASH clock enable during Sleep and Stop modes Set and cleared by software"]
        #[inline(always)]
        pub const fn hashsmen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "HASH clock enable during Sleep and Stop modes Set and cleared by software"]
        #[inline(always)]
        pub fn set_hashsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Random number generator (RNG) clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn rngsmen(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Random number generator (RNG) clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_rngsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "PKA clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn pkasmen(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "PKA clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_pkasmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "SAES accelerator clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn saessmen(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "SAES accelerator clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_saessmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "OCTOSPIM clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn octospimsmen(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "OCTOSPIM clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_octospimsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "OTFDEC1 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn otfdec1smen(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "OTFDEC1 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_otfdec1smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "OTFDEC2 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn otfdec2smen(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "OTFDEC2 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_otfdec2smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "SDMMC1 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn sdmmc1smen(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "SDMMC1 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_sdmmc1smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "SDMMC2 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn sdmmc2smen(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "SDMMC2 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_sdmmc2smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "SRAM2 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn sram2smen(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_sram2smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "SRAM3 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn sram3smen(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM3 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_sram3smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ahb2smenr1 {
        #[inline(always)]
        fn default() -> Ahb2smenr1 {
            Ahb2smenr1(0)
        }
    }
    #[doc = "RCC AHB2 peripheral clocks enable in Sleep and Stop modes register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb2smenr2(pub u32);
    impl Ahb2smenr2 {
        #[doc = "FSMC clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn fsmcsmen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "FSMC clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_fsmcsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "OCTOSPI1 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn octospi1smen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "OCTOSPI1 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_octospi1smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "OCTOSPI2 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn octospi2smen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "OCTOSPI2 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_octospi2smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "HSPI1 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn hspi1smen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "HSPI1 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_hspi1smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "SRAM6 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn sram6smen(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM6 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_sram6smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "SRAM5 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn sram5smen(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM5 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_sram5smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ahb2smenr2 {
        #[inline(always)]
        fn default() -> Ahb2smenr2 {
            Ahb2smenr2(0)
        }
    }
    #[doc = "RCC AHB3 peripheral clock enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb3enr(pub u32);
    impl Ahb3enr {
        #[doc = "LPGPIO1 enable Set and cleared by software."]
        #[inline(always)]
        pub const fn lpgpio1en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LPGPIO1 enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_lpgpio1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "PWR clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn pwren(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "PWR clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_pwren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "ADC4 clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn adc4en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "ADC4 clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_adc4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "DAC1 clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn dac1en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "DAC1 clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_dac1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "LPDMA1 clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn lpdma1en(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "LPDMA1 clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_lpdma1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "ADF1 clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn adf1en(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "ADF1 clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_adf1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "GTZC2 clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn gtzc2en(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "GTZC2 clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_gtzc2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "SRAM4 clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn sram4en(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM4 clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_sram4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ahb3enr {
        #[inline(always)]
        fn default() -> Ahb3enr {
            Ahb3enr(0)
        }
    }
    #[doc = "RCC AHB3 peripheral reset register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb3rstr(pub u32);
    impl Ahb3rstr {
        #[doc = "LPGPIO1 reset Set and cleared by software."]
        #[inline(always)]
        pub const fn lpgpio1rst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LPGPIO1 reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_lpgpio1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "ADC4 reset Set and cleared by software."]
        #[inline(always)]
        pub const fn adc4rst(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "ADC4 reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_adc4rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "DAC1 reset Set and cleared by software."]
        #[inline(always)]
        pub const fn dac1rst(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "DAC1 reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_dac1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "LPDMA1 reset Set and cleared by software."]
        #[inline(always)]
        pub const fn lpdma1rst(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "LPDMA1 reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_lpdma1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "ADF1 reset Set and cleared by software."]
        #[inline(always)]
        pub const fn adf1rst(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "ADF1 reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_adf1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
    }
    impl Default for Ahb3rstr {
        #[inline(always)]
        fn default() -> Ahb3rstr {
            Ahb3rstr(0)
        }
    }
    #[doc = "RCC AHB3 peripheral clocks enable in Sleep and Stop modes register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb3smenr(pub u32);
    impl Ahb3smenr {
        #[doc = "LPGPIO1 enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn lpgpio1smen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LPGPIO1 enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_lpgpio1smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "PWR clock enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn pwrsmen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "PWR clock enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_pwrsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "ADC4 clock enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub const fn adc4smen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "ADC4 clock enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub fn set_adc4smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "DAC1 clock enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub const fn dac1smen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "DAC1 clock enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub fn set_dac1smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "LPDMA1 clock enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub const fn lpdma1smen(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "LPDMA1 clock enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub fn set_lpdma1smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "ADF1 clock enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub const fn adf1smen(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "ADF1 clock enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub fn set_adf1smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "GTZC2 clock enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn gtzc2smen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "GTZC2 clock enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_gtzc2smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "SRAM4 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn sram4smen(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM4 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_sram4smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ahb3smenr {
        #[inline(always)]
        fn default() -> Ahb3smenr {
            Ahb3smenr(0)
        }
    }
    #[doc = "RCC APB1 peripheral clock enable register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1enr1(pub u32);
    impl Apb1enr1 {
        #[doc = "TIM2 clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn tim2en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM2 clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_tim2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM3 clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn tim3en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TIM3 clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_tim3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "TIM4 clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn tim4en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "TIM4 clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_tim4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "TIM5 clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn tim5en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "TIM5 clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_tim5en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "TIM6 clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn tim6en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "TIM6 clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_tim6en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "TIM7 clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn tim7en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "TIM7 clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_tim7en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "WWDG clock enable Set by software to enable the window watchdog clock. Reset by hardware system reset. This bit can also be set by hardware if the WWDG_SW option bit is reset."]
        #[inline(always)]
        pub const fn wwdgen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "WWDG clock enable Set by software to enable the window watchdog clock. Reset by hardware system reset. This bit can also be set by hardware if the WWDG_SW option bit is reset."]
        #[inline(always)]
        pub fn set_wwdgen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "SPI2 clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn spi2en(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "SPI2 clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_spi2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "USART2 clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn usart2en(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "USART2 clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_usart2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "USART3 clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn usart3en(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "USART3 clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_usart3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "UART4 clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn uart4en(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "UART4 clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_uart4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "UART5 clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn uart5en(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "UART5 clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_uart5en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "I2C1 clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn i2c1en(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1 clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_i2c1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "I2C2 clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn i2c2en(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "I2C2 clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_i2c2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "CRS clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn crsen(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "CRS clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_crsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "USART6 clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn usart6en(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "USART6 clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_usart6en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
    }
    impl Default for Apb1enr1 {
        #[inline(always)]
        fn default() -> Apb1enr1 {
            Apb1enr1(0)
        }
    }
    #[doc = "RCC APB1 peripheral clock enable register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1enr2(pub u32);
    impl Apb1enr2 {
        #[doc = "I2C4 clock enable Set and cleared by software"]
        #[inline(always)]
        pub const fn i2c4en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "I2C4 clock enable Set and cleared by software"]
        #[inline(always)]
        pub fn set_i2c4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "LPTIM2 clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn lptim2en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM2 clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_lptim2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "I2C5 clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn i2c5en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "I2C5 clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_i2c5en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "I2C6 clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn i2c6en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "I2C6 clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_i2c6en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "FDCAN1 clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn fdcan1en(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "FDCAN1 clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_fdcan1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "UCPD1 clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn ucpd1en(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "UCPD1 clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_ucpd1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for Apb1enr2 {
        #[inline(always)]
        fn default() -> Apb1enr2 {
            Apb1enr2(0)
        }
    }
    #[doc = "RCC APB1 peripheral reset register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1rstr1(pub u32);
    impl Apb1rstr1 {
        #[doc = "TIM2 reset Set and cleared by software."]
        #[inline(always)]
        pub const fn tim2rst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM2 reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_tim2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM3 reset Set and cleared by software."]
        #[inline(always)]
        pub const fn tim3rst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TIM3 reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_tim3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "TIM4 reset Set and cleared by software."]
        #[inline(always)]
        pub const fn tim4rst(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "TIM4 reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_tim4rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "TIM5 reset Set and cleared by software."]
        #[inline(always)]
        pub const fn tim5rst(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "TIM5 reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_tim5rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "TIM6 reset Set and cleared by software."]
        #[inline(always)]
        pub const fn tim6rst(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "TIM6 reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_tim6rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "TIM7 reset Set and cleared by software."]
        #[inline(always)]
        pub const fn tim7rst(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "TIM7 reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_tim7rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "SPI2 reset Set and cleared by software."]
        #[inline(always)]
        pub const fn spi2rst(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "SPI2 reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_spi2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "USART2 reset Set and cleared by software."]
        #[inline(always)]
        pub const fn usart2rst(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "USART2 reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_usart2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "USART3 reset Set and cleared by software."]
        #[inline(always)]
        pub const fn usart3rst(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "USART3 reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_usart3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "UART4 reset Set and cleared by software."]
        #[inline(always)]
        pub const fn uart4rst(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "UART4 reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_uart4rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "UART5 reset Set and cleared by software."]
        #[inline(always)]
        pub const fn uart5rst(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "UART5 reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_uart5rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "I2C1 reset Set and cleared by software."]
        #[inline(always)]
        pub const fn i2c1rst(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1 reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_i2c1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "I2C2 reset Set and cleared by software."]
        #[inline(always)]
        pub const fn i2c2rst(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "I2C2 reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_i2c2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "CRS reset Set and cleared by software."]
        #[inline(always)]
        pub const fn crsrst(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "CRS reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_crsrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "USART6 reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn usart6rst(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "USART6 reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_usart6rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
    }
    impl Default for Apb1rstr1 {
        #[inline(always)]
        fn default() -> Apb1rstr1 {
            Apb1rstr1(0)
        }
    }
    #[doc = "RCC APB1 peripheral reset register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1rstr2(pub u32);
    impl Apb1rstr2 {
        #[doc = "I2C4 reset Set and cleared by software"]
        #[inline(always)]
        pub const fn i2c4rst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "I2C4 reset Set and cleared by software"]
        #[inline(always)]
        pub fn set_i2c4rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "LPTIM2 reset Set and cleared by software."]
        #[inline(always)]
        pub const fn lptim2rst(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM2 reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_lptim2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "I2C5 reset This bit is set and cleared by software Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn i2c5rst(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "I2C5 reset This bit is set and cleared by software Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_i2c5rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "I2C6 reset This bit is set and cleared by software Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn i2c6rst(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "I2C6 reset This bit is set and cleared by software Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_i2c6rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "FDCAN1 reset Set and cleared by software."]
        #[inline(always)]
        pub const fn fdcan1rst(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "FDCAN1 reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_fdcan1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "UCPD1 reset Set and cleared by software."]
        #[inline(always)]
        pub const fn ucpd1rst(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "UCPD1 reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_ucpd1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for Apb1rstr2 {
        #[inline(always)]
        fn default() -> Apb1rstr2 {
            Apb1rstr2(0)
        }
    }
    #[doc = "RCC APB1 peripheral clocks enable in Sleep and Stop modes register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1smenr1(pub u32);
    impl Apb1smenr1 {
        #[doc = "TIM2 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn tim2smen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM2 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_tim2smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM3 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn tim3smen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TIM3 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_tim3smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "TIM4 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn tim4smen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "TIM4 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_tim4smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "TIM5 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn tim5smen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "TIM5 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_tim5smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "TIM6 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn tim6smen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "TIM6 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_tim6smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "TIM7 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn tim7smen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "TIM7 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_tim7smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Window watchdog clocks enable during Sleep and Stop modes Set and cleared by software. This bit is forced to 1 by hardware when the hardware WWDG option is activated."]
        #[inline(always)]
        pub const fn wwdgsmen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Window watchdog clocks enable during Sleep and Stop modes Set and cleared by software. This bit is forced to 1 by hardware when the hardware WWDG option is activated."]
        #[inline(always)]
        pub fn set_wwdgsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "SPI2 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub const fn spi2smen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "SPI2 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub fn set_spi2smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "USART2 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub const fn usart2smen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "USART2 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub fn set_usart2smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "USART3 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub const fn usart3smen(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "USART3 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub fn set_usart3smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "UART4 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub const fn uart4smen(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "UART4 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub fn set_uart4smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "UART5 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub const fn uart5smen(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "UART5 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub fn set_uart5smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "I2C1 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub const fn i2c1smen(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub fn set_i2c1smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "I2C2 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub const fn i2c2smen(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "I2C2 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub fn set_i2c2smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "CRS clock enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn crssmen(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "CRS clock enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_crssmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "USART6 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn usart6smen(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "USART6 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_usart6smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
    }
    impl Default for Apb1smenr1 {
        #[inline(always)]
        fn default() -> Apb1smenr1 {
            Apb1smenr1(0)
        }
    }
    #[doc = "RCC APB1 peripheral clocks enable in Sleep and Stop modes register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1smenr2(pub u32);
    impl Apb1smenr2 {
        #[doc = "I2C4 clocks enable during Sleep and Stop modes Set and cleared by software Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub const fn i2c4smen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "I2C4 clocks enable during Sleep and Stop modes Set and cleared by software Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub fn set_i2c4smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "LPTIM2 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub const fn lptim2smen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM2 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub fn set_lptim2smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "I2C5 clock enable during Sleep and Stop modes This bit is set and cleared by software Note: This bit must be set to allow the peripheral to wake up from Stop modes. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn i2c5smen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "I2C5 clock enable during Sleep and Stop modes This bit is set and cleared by software Note: This bit must be set to allow the peripheral to wake up from Stop modes. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_i2c5smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "I2C6 clock enable during Sleep and Stop modes This bit is set and cleared by software Note: This bit must be set to allow the peripheral to wake up from Stop modes. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn i2c6smen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "I2C6 clock enable during Sleep and Stop modes This bit is set and cleared by software Note: This bit must be set to allow the peripheral to wake up from Stop modes. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_i2c6smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "FDCAN1 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn fdcan1smen(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "FDCAN1 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_fdcan1smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "UCPD1 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn ucpd1smen(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "UCPD1 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_ucpd1smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for Apb1smenr2 {
        #[inline(always)]
        fn default() -> Apb1smenr2 {
            Apb1smenr2(0)
        }
    }
    #[doc = "RCC APB2 peripheral clock enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb2enr(pub u32);
    impl Apb2enr {
        #[doc = "TIM1 clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn tim1en(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "TIM1 clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_tim1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "SPI1 clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn spi1en(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "SPI1 clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_spi1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "TIM8 clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn tim8en(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "TIM8 clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_tim8en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "USART1clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn usart1en(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "USART1clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_usart1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "TIM15 clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn tim15en(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "TIM15 clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_tim15en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "TIM16 clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn tim16en(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "TIM16 clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_tim16en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "TIM17 clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn tim17en(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "TIM17 clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_tim17en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "SAI1 clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn sai1en(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "SAI1 clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_sai1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "SAI2 clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn sai2en(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "SAI2 clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_sai2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "USB clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn usben(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "USB clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_usben(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "GFXTIM clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn gfxtimen(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "GFXTIM clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_gfxtimen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "LTDC clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn ltdcen(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "LTDC clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_ltdcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "DSI clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn dsien(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "DSI clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_dsien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
    }
    impl Default for Apb2enr {
        #[inline(always)]
        fn default() -> Apb2enr {
            Apb2enr(0)
        }
    }
    #[doc = "RCC APB2 peripheral reset register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb2rstr(pub u32);
    impl Apb2rstr {
        #[doc = "TIM1 reset Set and cleared by software."]
        #[inline(always)]
        pub const fn tim1rst(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "TIM1 reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_tim1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "SPI1 reset Set and cleared by software."]
        #[inline(always)]
        pub const fn spi1rst(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "SPI1 reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_spi1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "TIM8 reset Set and cleared by software."]
        #[inline(always)]
        pub const fn tim8rst(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "TIM8 reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_tim8rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "USART1 reset Set and cleared by software."]
        #[inline(always)]
        pub const fn usart1rst(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "USART1 reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_usart1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "TIM15 reset Set and cleared by software."]
        #[inline(always)]
        pub const fn tim15rst(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "TIM15 reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_tim15rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "TIM16 reset Set and cleared by software."]
        #[inline(always)]
        pub const fn tim16rst(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "TIM16 reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_tim16rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "TIM17 reset Set and cleared by software."]
        #[inline(always)]
        pub const fn tim17rst(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "TIM17 reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_tim17rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "SAI1 reset Set and cleared by software."]
        #[inline(always)]
        pub const fn sai1rst(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "SAI1 reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_sai1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "SAI2 reset Set and cleared by software."]
        #[inline(always)]
        pub const fn sai2rst(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "SAI2 reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_sai2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "USB reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn usbrst(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "USB reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_usbrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "GFXTIM reset This bit is set and cleared by software. Note: .This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn gfxtimrst(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "GFXTIM reset This bit is set and cleared by software. Note: .This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_gfxtimrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "LTDC reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn ltdcrst(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "LTDC reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_ltdcrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "DSI reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn dsirst(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "DSI reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_dsirst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
    }
    impl Default for Apb2rstr {
        #[inline(always)]
        fn default() -> Apb2rstr {
            Apb2rstr(0)
        }
    }
    #[doc = "RCC APB2 peripheral clocks enable in Sleep and Stop modes register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb2smenr(pub u32);
    impl Apb2smenr {
        #[doc = "TIM1 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn tim1smen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "TIM1 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_tim1smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "SPI1 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub const fn spi1smen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "SPI1 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub fn set_spi1smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "TIM8 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn tim8smen(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "TIM8 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_tim8smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "USART1clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub const fn usart1smen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "USART1clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub fn set_usart1smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "TIM15 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn tim15smen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "TIM15 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_tim15smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "TIM16 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn tim16smen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "TIM16 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_tim16smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "TIM17 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn tim17smen(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "TIM17 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_tim17smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "SAI1 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn sai1smen(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "SAI1 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_sai1smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "SAI2 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn sai2smen(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "SAI2 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_sai2smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "USB clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn usbsmen(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "USB clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_usbsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "GFXTIM clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn gfxtimsmen(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "GFXTIM clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_gfxtimsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "LTDC clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn ltdcsmen(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "LTDC clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_ltdcsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "DSI clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn dsismen(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "DSI clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_dsismen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
    }
    impl Default for Apb2smenr {
        #[inline(always)]
        fn default() -> Apb2smenr {
            Apb2smenr(0)
        }
    }
    #[doc = "RCC APB3 peripheral clock enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb3enr(pub u32);
    impl Apb3enr {
        #[doc = "SYSCFG clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn syscfgen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "SYSCFG clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_syscfgen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "SPI3 clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn spi3en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "SPI3 clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_spi3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "LPUART1 clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn lpuart1en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "LPUART1 clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_lpuart1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "I2C3 clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn i2c3en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "I2C3 clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_i2c3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "LPTIM1 clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn lptim1en(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM1 clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_lptim1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "LPTIM3 clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn lptim3en(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM3 clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_lptim3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "LPTIM4 clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn lptim4en(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM4 clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_lptim4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "OPAMP clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn opampen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "OPAMP clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_opampen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "COMP clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn compen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "COMP clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_compen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "VREFBUF clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn vrefen(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "VREFBUF clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_vrefen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "RTC and TAMP APB clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn rtcapben(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "RTC and TAMP APB clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_rtcapben(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
    }
    impl Default for Apb3enr {
        #[inline(always)]
        fn default() -> Apb3enr {
            Apb3enr(0)
        }
    }
    #[doc = "RCC APB3 peripheral reset register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb3rstr(pub u32);
    impl Apb3rstr {
        #[doc = "SYSCFG reset Set and cleared by software."]
        #[inline(always)]
        pub const fn syscfgrst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "SYSCFG reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_syscfgrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "SPI3 reset Set and cleared by software."]
        #[inline(always)]
        pub const fn spi3rst(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "SPI3 reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_spi3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "LPUART1 reset Set and cleared by software."]
        #[inline(always)]
        pub const fn lpuart1rst(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "LPUART1 reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_lpuart1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "I2C3 reset Set and cleared by software."]
        #[inline(always)]
        pub const fn i2c3rst(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "I2C3 reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_i2c3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "LPTIM1 reset Set and cleared by software."]
        #[inline(always)]
        pub const fn lptim1rst(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM1 reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_lptim1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "LPTIM3 reset Set and cleared by software."]
        #[inline(always)]
        pub const fn lptim3rst(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM3 reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_lptim3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "LPTIM4 reset Set and cleared by software."]
        #[inline(always)]
        pub const fn lptim4rst(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM4 reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_lptim4rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "OPAMP reset Set and cleared by software."]
        #[inline(always)]
        pub const fn opamprst(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "OPAMP reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_opamprst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "COMP reset Set and cleared by software."]
        #[inline(always)]
        pub const fn comprst(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "COMP reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_comprst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "VREFBUF reset Set and cleared by software."]
        #[inline(always)]
        pub const fn vrefrst(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "VREFBUF reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_vrefrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
    }
    impl Default for Apb3rstr {
        #[inline(always)]
        fn default() -> Apb3rstr {
            Apb3rstr(0)
        }
    }
    #[doc = "RCC APB3 peripheral clock enable in Sleep and Stop modes register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb3smenr(pub u32);
    impl Apb3smenr {
        #[doc = "SYSCFG clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn syscfgsmen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "SYSCFG clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_syscfgsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "SPI3 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub const fn spi3smen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "SPI3 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub fn set_spi3smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "LPUART1 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub const fn lpuart1smen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "LPUART1 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub fn set_lpuart1smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "I2C3 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub const fn i2c3smen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "I2C3 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub fn set_i2c3smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "LPTIM1 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub const fn lptim1smen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM1 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub fn set_lptim1smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "LPTIM3 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub const fn lptim3smen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM3 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub fn set_lptim3smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "LPTIM4 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub const fn lptim4smen(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM4 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub fn set_lptim4smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "OPAMP clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn opampsmen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "OPAMP clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_opampsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "COMP clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn compsmen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "COMP clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_compsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "VREFBUF clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn vrefsmen(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "VREFBUF clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_vrefsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "RTC and TAMP APB clock enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub const fn rtcapbsmen(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "RTC and TAMP APB clock enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub fn set_rtcapbsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
    }
    impl Default for Apb3smenr {
        #[inline(always)]
        fn default() -> Apb3smenr {
            Apb3smenr(0)
        }
    }
    #[doc = "RCC Backup domain control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bdcr(pub u32);
    impl Bdcr {
        #[doc = "LSE oscillator enable Set and cleared by software."]
        #[inline(always)]
        pub const fn lseon(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LSE oscillator enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_lseon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "LSE oscillator ready Set and cleared by hardware to indicate when the external 32 kHz oscillator is stable. After the LSEON bit is cleared, LSERDY goes low after six external low-speed oscillator clock cycles."]
        #[inline(always)]
        pub const fn lserdy(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "LSE oscillator ready Set and cleared by hardware to indicate when the external 32 kHz oscillator is stable. After the LSEON bit is cleared, LSERDY goes low after six external low-speed oscillator clock cycles."]
        #[inline(always)]
        pub fn set_lserdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "LSE oscillator bypass Set and cleared by software to bypass oscillator in debug mode. This bit can be written only when the external 32 kHz oscillator is disabled (LSEON = 0 and LSERDY = 0)."]
        #[inline(always)]
        pub const fn lsebyp(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "LSE oscillator bypass Set and cleared by software to bypass oscillator in debug mode. This bit can be written only when the external 32 kHz oscillator is disabled (LSEON = 0 and LSERDY = 0)."]
        #[inline(always)]
        pub fn set_lsebyp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "LSE oscillator drive capability Set by software to modulate the drive capability of the LSE oscillator. This field can be written only when the external 32 kHz oscillator is disabled (LSEON = 0 and LSERDY = 0). Note: The oscillator is in 'Xtal mode when it is not in bypass mode."]
        #[inline(always)]
        pub const fn lsedrv(&self) -> super::vals::Lsedrv {
            let val = (self.0 >> 3usize) & 0x03;
            super::vals::Lsedrv::from_bits(val as u8)
        }
        #[doc = "LSE oscillator drive capability Set by software to modulate the drive capability of the LSE oscillator. This field can be written only when the external 32 kHz oscillator is disabled (LSEON = 0 and LSERDY = 0). Note: The oscillator is in 'Xtal mode when it is not in bypass mode."]
        #[inline(always)]
        pub fn set_lsedrv(&mut self, val: super::vals::Lsedrv) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val.to_bits() as u32) & 0x03) << 3usize);
        }
        #[doc = "CSS on LSE enable Set by software to enable the CSS on LSE. LSECSSON must be enabled after the LSE oscillator is enabled (LSEON bit enabled) and ready (LSERDY flag set by hardware), and after the RTCSEL bit is selected. Once enabled, this bit cannot be disabled, except after a LSE failure detection (LSECSSD = 1). In that case, the software must disable the LSECSSON bit."]
        #[inline(always)]
        pub const fn lsecsson(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "CSS on LSE enable Set by software to enable the CSS on LSE. LSECSSON must be enabled after the LSE oscillator is enabled (LSEON bit enabled) and ready (LSERDY flag set by hardware), and after the RTCSEL bit is selected. Once enabled, this bit cannot be disabled, except after a LSE failure detection (LSECSSD = 1). In that case, the software must disable the LSECSSON bit."]
        #[inline(always)]
        pub fn set_lsecsson(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "CSS on LSE failure Detection Set by hardware to indicate when a failure is detected by the CCS on the external 32 kHz oscillator (LSE)."]
        #[inline(always)]
        pub const fn lsecssd(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "CSS on LSE failure Detection Set by hardware to indicate when a failure is detected by the CCS on the external 32 kHz oscillator (LSE)."]
        #[inline(always)]
        pub fn set_lsecssd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "LSE system clock (LSESYS) enable Set by software to enable always the LSE system clock generated by RCC. This clock can be used by any peripheral when its source clock is the LSE or at system level in case of one of the LSCOSEL, MCO, MSI PLL mode or CSS on LSE is needed. The LSESYS clock can be generated even if LSESYSEN= 0 if the LSE clock is requested by the CSS on LSE, by a peripheral or any other source clock using LSE."]
        #[inline(always)]
        pub const fn lsesysen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "LSE system clock (LSESYS) enable Set by software to enable always the LSE system clock generated by RCC. This clock can be used by any peripheral when its source clock is the LSE or at system level in case of one of the LSCOSEL, MCO, MSI PLL mode or CSS on LSE is needed. The LSESYS clock can be generated even if LSESYSEN= 0 if the LSE clock is requested by the CSS on LSE, by a peripheral or any other source clock using LSE."]
        #[inline(always)]
        pub fn set_lsesysen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "RTC and TAMP clock source selection Set by software to select the clock source for the RTC and TAMP . Once the RTC and TAMP clock source has been selected, it cannot be changed anymore unless the Backup domain is reset, or unless a failure is detected on LSE (LSECSSD is set). The BDRST bit can be used to reset them."]
        #[inline(always)]
        pub const fn rtcsel(&self) -> super::vals::Rtcsel {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::Rtcsel::from_bits(val as u8)
        }
        #[doc = "RTC and TAMP clock source selection Set by software to select the clock source for the RTC and TAMP . Once the RTC and TAMP clock source has been selected, it cannot be changed anymore unless the Backup domain is reset, or unless a failure is detected on LSE (LSECSSD is set). The BDRST bit can be used to reset them."]
        #[inline(always)]
        pub fn set_rtcsel(&mut self, val: super::vals::Rtcsel) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
        #[doc = "LSE system clock (LSESYS) ready Set and cleared by hardware to indicate when the LSE system clock is stable.When the LSESYSEN bit is set, the LSESYSRDY flag is set after two LSE clock cycles. The LSE clock must be already enabled and stable (LSEON and LSERDY are set). When the LSEON bit is cleared, LSERDY goes low after six external low-speed oscillator clock cycles."]
        #[inline(always)]
        pub const fn lsesysrdy(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "LSE system clock (LSESYS) ready Set and cleared by hardware to indicate when the LSE system clock is stable.When the LSESYSEN bit is set, the LSESYSRDY flag is set after two LSE clock cycles. The LSE clock must be already enabled and stable (LSEON and LSERDY are set). When the LSEON bit is cleared, LSERDY goes low after six external low-speed oscillator clock cycles."]
        #[inline(always)]
        pub fn set_lsesysrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "LSE clock glitch filter enable Set and cleared by hardware to enable the LSE glitch filter. This bit can be written only when the LSE is disabled (LSEON = 0 and LSERDY = 0)"]
        #[inline(always)]
        pub const fn lsegfon(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "LSE clock glitch filter enable Set and cleared by hardware to enable the LSE glitch filter. This bit can be written only when the LSE is disabled (LSEON = 0 and LSERDY = 0)"]
        #[inline(always)]
        pub fn set_lsegfon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "RTC and TAMP clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn rtcen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "RTC and TAMP clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_rtcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Backup domain software reset Set and cleared by software."]
        #[inline(always)]
        pub const fn bdrst(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Backup domain software reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_bdrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Low-speed clock output (LSCO) enable Set and cleared by software."]
        #[inline(always)]
        pub const fn lscoen(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Low-speed clock output (LSCO) enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_lscoen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Low-speed clock output selection Set and cleared by software."]
        #[inline(always)]
        pub const fn lscosel(&self) -> super::vals::Lscosel {
            let val = (self.0 >> 25usize) & 0x01;
            super::vals::Lscosel::from_bits(val as u8)
        }
        #[doc = "Low-speed clock output selection Set and cleared by software."]
        #[inline(always)]
        pub fn set_lscosel(&mut self, val: super::vals::Lscosel) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
        }
        #[doc = "LSI oscillator enable Set and cleared by software."]
        #[inline(always)]
        pub const fn lsion(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "LSI oscillator enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_lsion(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "LSI oscillator ready Set and cleared by hardware to indicate when the LSI oscillator is stable. After the LSION bit is cleared, LSIRDY goes low after three internal low-speed oscillator clock cycles. This bit is set when the LSI is used by IWDG or RTC, even if LSION = 0."]
        #[inline(always)]
        pub const fn lsirdy(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "LSI oscillator ready Set and cleared by hardware to indicate when the LSI oscillator is stable. After the LSION bit is cleared, LSIRDY goes low after three internal low-speed oscillator clock cycles. This bit is set when the LSI is used by IWDG or RTC, even if LSION = 0."]
        #[inline(always)]
        pub fn set_lsirdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Low-speed clock divider configuration Set and cleared by software to enable the LSI division. This bit can be written only when the LSI is disabled (LSION = 0 and LSIRDY = 0). If the LSI was previously enabled, it is necessary to wait for at least 60 Î¼s after clearing LSION bit (synchronization time for LSI to be really disabled), before writing LSIPREDIV. The LSIPREDIV cannot be changed if the LSI is used by the IWDG or by the RTC."]
        #[inline(always)]
        pub const fn lsiprediv(&self) -> super::vals::Lsiprediv {
            let val = (self.0 >> 28usize) & 0x01;
            super::vals::Lsiprediv::from_bits(val as u8)
        }
        #[doc = "Low-speed clock divider configuration Set and cleared by software to enable the LSI division. This bit can be written only when the LSI is disabled (LSION = 0 and LSIRDY = 0). If the LSI was previously enabled, it is necessary to wait for at least 60 Î¼s after clearing LSION bit (synchronization time for LSI to be really disabled), before writing LSIPREDIV. The LSIPREDIV cannot be changed if the LSI is used by the IWDG or by the RTC."]
        #[inline(always)]
        pub fn set_lsiprediv(&mut self, val: super::vals::Lsiprediv) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
        }
    }
    impl Default for Bdcr {
        #[inline(always)]
        fn default() -> Bdcr {
            Bdcr(0)
        }
    }
    #[doc = "RCC peripherals independent clock configuration register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccipr1(pub u32);
    impl Ccipr1 {
        #[doc = "USART1 kernel clock source selection This bits are used to select the USART1 kernel clock source. Note: The USART1 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI or LSE."]
        #[inline(always)]
        pub const fn usart1sel(&self) -> super::vals::Usartsel {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Usartsel::from_bits(val as u8)
        }
        #[doc = "USART1 kernel clock source selection This bits are used to select the USART1 kernel clock source. Note: The USART1 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI or LSE."]
        #[inline(always)]
        pub fn set_usart1sel(&mut self, val: super::vals::Usartsel) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "USART2 kernel clock source selection This bits are used to select the USART2 kernel clock source. Note: The USART2 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI or LSE."]
        #[inline(always)]
        pub const fn usart2sel(&self) -> super::vals::Usartsel {
            let val = (self.0 >> 2usize) & 0x03;
            super::vals::Usartsel::from_bits(val as u8)
        }
        #[doc = "USART2 kernel clock source selection This bits are used to select the USART2 kernel clock source. Note: The USART2 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI or LSE."]
        #[inline(always)]
        pub fn set_usart2sel(&mut self, val: super::vals::Usartsel) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
        }
        #[doc = "USART3 kernel clock source selection This bits are used to select the USART3 kernel clock source. Note: The USART3 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI or LSE."]
        #[inline(always)]
        pub const fn usart3sel(&self) -> super::vals::Usartsel {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::Usartsel::from_bits(val as u8)
        }
        #[doc = "USART3 kernel clock source selection This bits are used to select the USART3 kernel clock source. Note: The USART3 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI or LSE."]
        #[inline(always)]
        pub fn set_usart3sel(&mut self, val: super::vals::Usartsel) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
        #[doc = "UART4 kernel clock source selection This bits are used to select the UART4 kernel clock source. Note: The UART4 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI or LSE."]
        #[inline(always)]
        pub const fn uart4sel(&self) -> super::vals::Uartsel {
            let val = (self.0 >> 6usize) & 0x03;
            super::vals::Uartsel::from_bits(val as u8)
        }
        #[doc = "UART4 kernel clock source selection This bits are used to select the UART4 kernel clock source. Note: The UART4 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI or LSE."]
        #[inline(always)]
        pub fn set_uart4sel(&mut self, val: super::vals::Uartsel) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
        }
        #[doc = "UART5 kernel clock source selection These bits are used to select the UART5 kernel clock source. Note: The UART5 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI or LSE."]
        #[inline(always)]
        pub const fn uart5sel(&self) -> super::vals::Uartsel {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::Uartsel::from_bits(val as u8)
        }
        #[doc = "UART5 kernel clock source selection These bits are used to select the UART5 kernel clock source. Note: The UART5 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI or LSE."]
        #[inline(always)]
        pub fn set_uart5sel(&mut self, val: super::vals::Uartsel) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
        #[doc = "I2C1 kernel clock source selection These bits are used to select the I2C1 kernel clock source. Note: The I2C1 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI or MSIK."]
        #[inline(always)]
        pub const fn i2c1sel(&self) -> super::vals::Icsel {
            let val = (self.0 >> 10usize) & 0x03;
            super::vals::Icsel::from_bits(val as u8)
        }
        #[doc = "I2C1 kernel clock source selection These bits are used to select the I2C1 kernel clock source. Note: The I2C1 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI or MSIK."]
        #[inline(always)]
        pub fn set_i2c1sel(&mut self, val: super::vals::Icsel) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
        }
        #[doc = "I2C2 kernel clock source selection These bits are used to select the I2C2 kernel clock source. Note: The I2C2 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI or MSIK."]
        #[inline(always)]
        pub const fn i2c2sel(&self) -> super::vals::Icsel {
            let val = (self.0 >> 12usize) & 0x03;
            super::vals::Icsel::from_bits(val as u8)
        }
        #[doc = "I2C2 kernel clock source selection These bits are used to select the I2C2 kernel clock source. Note: The I2C2 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI or MSIK."]
        #[inline(always)]
        pub fn set_i2c2sel(&mut self, val: super::vals::Icsel) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
        }
        #[doc = "I2C4 kernel clock source selection These bits are used to select the I2C4 kernel clock source. Note: The I2C4 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI or MSIK."]
        #[inline(always)]
        pub const fn i2c4sel(&self) -> super::vals::Icsel {
            let val = (self.0 >> 14usize) & 0x03;
            super::vals::Icsel::from_bits(val as u8)
        }
        #[doc = "I2C4 kernel clock source selection These bits are used to select the I2C4 kernel clock source. Note: The I2C4 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI or MSIK."]
        #[inline(always)]
        pub fn set_i2c4sel(&mut self, val: super::vals::Icsel) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
        }
        #[doc = "SPI2 kernel clock source selection These bits are used to select the SPI2 kernel clock source. Note: The SPI2 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI or MSIK."]
        #[inline(always)]
        pub const fn spi2sel(&self) -> super::vals::Spisel {
            let val = (self.0 >> 16usize) & 0x03;
            super::vals::Spisel::from_bits(val as u8)
        }
        #[doc = "SPI2 kernel clock source selection These bits are used to select the SPI2 kernel clock source. Note: The SPI2 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI or MSIK."]
        #[inline(always)]
        pub fn set_spi2sel(&mut self, val: super::vals::Spisel) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
        }
        #[doc = "Low-power timer 2 kernel clock source selection These bits are used to select the LPTIM2 kernel clock source. Note: The LPTIM2 is functional in Stop 0 and Stop 1 mode only when the kernel clock is LSI, LSE or HSI if HSIKERON = 1."]
        #[inline(always)]
        pub const fn lptim2sel(&self) -> super::vals::Lptimsel {
            let val = (self.0 >> 18usize) & 0x03;
            super::vals::Lptimsel::from_bits(val as u8)
        }
        #[doc = "Low-power timer 2 kernel clock source selection These bits are used to select the LPTIM2 kernel clock source. Note: The LPTIM2 is functional in Stop 0 and Stop 1 mode only when the kernel clock is LSI, LSE or HSI if HSIKERON = 1."]
        #[inline(always)]
        pub fn set_lptim2sel(&mut self, val: super::vals::Lptimsel) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
        }
        #[doc = "SPI1 kernel clock source selection These bits are used to select the SPI1 kernel clock source. Note: The SPI1 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI or MSIK."]
        #[inline(always)]
        pub const fn spi1sel(&self) -> super::vals::Spisel {
            let val = (self.0 >> 20usize) & 0x03;
            super::vals::Spisel::from_bits(val as u8)
        }
        #[doc = "SPI1 kernel clock source selection These bits are used to select the SPI1 kernel clock source. Note: The SPI1 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI or MSIK."]
        #[inline(always)]
        pub fn set_spi1sel(&mut self, val: super::vals::Spisel) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
        }
        #[doc = "SysTick clock source selection These bits are used to select the SysTick clock source. Note: When LSE or LSI is selected, the AHB frequency must be at least four times higher than the LSI or LSE frequency. In addition, a jitter up to one HCLK cycle is introduced, due to the LSE or LSI sampling with HCLK in the SysTick circuitry."]
        #[inline(always)]
        pub const fn systicksel(&self) -> super::vals::Systicksel {
            let val = (self.0 >> 22usize) & 0x03;
            super::vals::Systicksel::from_bits(val as u8)
        }
        #[doc = "SysTick clock source selection These bits are used to select the SysTick clock source. Note: When LSE or LSI is selected, the AHB frequency must be at least four times higher than the LSI or LSE frequency. In addition, a jitter up to one HCLK cycle is introduced, due to the LSE or LSI sampling with HCLK in the SysTick circuitry."]
        #[inline(always)]
        pub fn set_systicksel(&mut self, val: super::vals::Systicksel) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val.to_bits() as u32) & 0x03) << 22usize);
        }
        #[doc = "FDCAN1 kernel clock source selection These bits are used to select the FDCAN1 kernel clock source."]
        #[inline(always)]
        pub const fn fdcan1sel(&self) -> super::vals::Fdcansel {
            let val = (self.0 >> 24usize) & 0x03;
            super::vals::Fdcansel::from_bits(val as u8)
        }
        #[doc = "FDCAN1 kernel clock source selection These bits are used to select the FDCAN1 kernel clock source."]
        #[inline(always)]
        pub fn set_fdcan1sel(&mut self, val: super::vals::Fdcansel) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
        }
        #[doc = "intermediate clock source selection These bits are used to select the clock source used by OTG_FS and SDMMC."]
        #[inline(always)]
        pub const fn iclksel(&self) -> super::vals::Iclksel {
            let val = (self.0 >> 26usize) & 0x03;
            super::vals::Iclksel::from_bits(val as u8)
        }
        #[doc = "intermediate clock source selection These bits are used to select the clock source used by OTG_FS and SDMMC."]
        #[inline(always)]
        pub fn set_iclksel(&mut self, val: super::vals::Iclksel) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
        }
        #[doc = "Clocks sources for TIM16,TIM17 and LPTIM2 internal input capture When the TIMICSEL2 bit is set, the TIM16, TIM17 and LPTIM2 internal input capture can be connected either to HSI/256, MSI/4 or MSI/1024. Depending on TIMICSEL\\[1:0\\]
value, MSI is either MSIK or MSIS. When TIMICSEL2 is cleared, the HSI, MSIK and MSIS clock sources cannot be selected as TIM16, TIM17 or LPTIM2 internal input capture. 0xx: HSI, MSIK and MSIS dividers disabled Note: The clock division must be disabled (TIMICSEL configured to 0xx) before selecting or changing a clock sources division."]
        #[inline(always)]
        pub const fn timicsel(&self) -> super::vals::Timicsel {
            let val = (self.0 >> 29usize) & 0x07;
            super::vals::Timicsel::from_bits(val as u8)
        }
        #[doc = "Clocks sources for TIM16,TIM17 and LPTIM2 internal input capture When the TIMICSEL2 bit is set, the TIM16, TIM17 and LPTIM2 internal input capture can be connected either to HSI/256, MSI/4 or MSI/1024. Depending on TIMICSEL\\[1:0\\]
value, MSI is either MSIK or MSIS. When TIMICSEL2 is cleared, the HSI, MSIK and MSIS clock sources cannot be selected as TIM16, TIM17 or LPTIM2 internal input capture. 0xx: HSI, MSIK and MSIS dividers disabled Note: The clock division must be disabled (TIMICSEL configured to 0xx) before selecting or changing a clock sources division."]
        #[inline(always)]
        pub fn set_timicsel(&mut self, val: super::vals::Timicsel) {
            self.0 = (self.0 & !(0x07 << 29usize)) | (((val.to_bits() as u32) & 0x07) << 29usize);
        }
    }
    impl Default for Ccipr1 {
        #[inline(always)]
        fn default() -> Ccipr1 {
            Ccipr1(0)
        }
    }
    #[doc = "RCC peripherals independent clock configuration register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccipr2(pub u32);
    impl Ccipr2 {
        #[doc = "MDF1 kernel clock source selection These bits are used to select the MDF1 kernel clock source. others: reserved"]
        #[inline(always)]
        pub const fn mdf1sel(&self) -> super::vals::Mdfsel {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::Mdfsel::from_bits(val as u8)
        }
        #[doc = "MDF1 kernel clock source selection These bits are used to select the MDF1 kernel clock source. others: reserved"]
        #[inline(always)]
        pub fn set_mdf1sel(&mut self, val: super::vals::Mdfsel) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "SAI1 kernel clock source selection These bits are used to select the SAI1 kernel clock source. others: reserved Note: If the selected clock is the external clock and this clock is stopped, a switch to another clock is impossible."]
        #[inline(always)]
        pub const fn sai1sel(&self) -> super::vals::Saisel {
            let val = (self.0 >> 5usize) & 0x07;
            super::vals::Saisel::from_bits(val as u8)
        }
        #[doc = "SAI1 kernel clock source selection These bits are used to select the SAI1 kernel clock source. others: reserved Note: If the selected clock is the external clock and this clock is stopped, a switch to another clock is impossible."]
        #[inline(always)]
        pub fn set_sai1sel(&mut self, val: super::vals::Saisel) {
            self.0 = (self.0 & !(0x07 << 5usize)) | (((val.to_bits() as u32) & 0x07) << 5usize);
        }
        #[doc = "SAI2 kernel clock source selection These bits are used to select the SAI2 kernel clock source. others: reserved Note: If the selected clock is the external clock and this clock is stopped, a switch to another clock is impossible."]
        #[inline(always)]
        pub const fn sai2sel(&self) -> super::vals::Saisel {
            let val = (self.0 >> 8usize) & 0x07;
            super::vals::Saisel::from_bits(val as u8)
        }
        #[doc = "SAI2 kernel clock source selection These bits are used to select the SAI2 kernel clock source. others: reserved Note: If the selected clock is the external clock and this clock is stopped, a switch to another clock is impossible."]
        #[inline(always)]
        pub fn set_sai2sel(&mut self, val: super::vals::Saisel) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
        }
        #[doc = "SAES kernel clock source selection This bit is used to select the SAES kernel clock source."]
        #[inline(always)]
        pub const fn saessel(&self) -> super::vals::Saessel {
            let val = (self.0 >> 11usize) & 0x01;
            super::vals::Saessel::from_bits(val as u8)
        }
        #[doc = "SAES kernel clock source selection This bit is used to select the SAES kernel clock source."]
        #[inline(always)]
        pub fn set_saessel(&mut self, val: super::vals::Saessel) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
        }
        #[doc = "RNGSEL kernel clock source selection These bits are used to select the RNG kernel clock source."]
        #[inline(always)]
        pub const fn rngsel(&self) -> super::vals::Rngsel {
            let val = (self.0 >> 12usize) & 0x03;
            super::vals::Rngsel::from_bits(val as u8)
        }
        #[doc = "RNGSEL kernel clock source selection These bits are used to select the RNG kernel clock source."]
        #[inline(always)]
        pub fn set_rngsel(&mut self, val: super::vals::Rngsel) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
        }
        #[doc = "SDMMC1 and SDMMC2 kernel clock source selection This bit is used to select the SDMMC kernel clock source. It is recommended to change this bit only after reset and before enabling the SDMMC."]
        #[inline(always)]
        pub const fn sdmmcsel(&self) -> super::vals::Sdmmcsel {
            let val = (self.0 >> 14usize) & 0x01;
            super::vals::Sdmmcsel::from_bits(val as u8)
        }
        #[doc = "SDMMC1 and SDMMC2 kernel clock source selection This bit is used to select the SDMMC kernel clock source. It is recommended to change this bit only after reset and before enabling the SDMMC."]
        #[inline(always)]
        pub fn set_sdmmcsel(&mut self, val: super::vals::Sdmmcsel) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
        }
        #[doc = "DSI kernel clock source selection This bit is used to select the DSI kernel clock source. This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. Note: If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn dsisel(&self) -> super::vals::Dsisel {
            let val = (self.0 >> 15usize) & 0x01;
            super::vals::Dsisel::from_bits(val as u8)
        }
        #[doc = "DSI kernel clock source selection This bit is used to select the DSI kernel clock source. This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. Note: If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_dsisel(&mut self, val: super::vals::Dsisel) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
        }
        #[doc = "USART6 kernel clock source selection These bits are used to select the USART6 kernel clock source. The USART6 is functional in Stop 0 and Stop 1 modes only when the kernel clock is HSI or LSE. Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bitfield as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn usart6sel(&self) -> super::vals::Usartsel {
            let val = (self.0 >> 16usize) & 0x03;
            super::vals::Usartsel::from_bits(val as u8)
        }
        #[doc = "USART6 kernel clock source selection These bits are used to select the USART6 kernel clock source. The USART6 is functional in Stop 0 and Stop 1 modes only when the kernel clock is HSI or LSE. Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bitfield as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_usart6sel(&mut self, val: super::vals::Usartsel) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
        }
        #[doc = "LTDC kernel clock source selection This bit is used to select the LTDC kernel clock source. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn ltdcsel(&self) -> super::vals::Ltdcsel {
            let val = (self.0 >> 18usize) & 0x01;
            super::vals::Ltdcsel::from_bits(val as u8)
        }
        #[doc = "LTDC kernel clock source selection This bit is used to select the LTDC kernel clock source. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_ltdcsel(&mut self, val: super::vals::Ltdcsel) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
        }
        #[doc = "OCTOSPI1 and OCTOSPI2 kernel clock source selection These bits are used to select the OCTOSPI1 and OCTOSPI2 kernel clock source."]
        #[inline(always)]
        pub const fn octospisel(&self) -> super::vals::Octospisel {
            let val = (self.0 >> 20usize) & 0x03;
            super::vals::Octospisel::from_bits(val as u8)
        }
        #[doc = "OCTOSPI1 and OCTOSPI2 kernel clock source selection These bits are used to select the OCTOSPI1 and OCTOSPI2 kernel clock source."]
        #[inline(always)]
        pub fn set_octospisel(&mut self, val: super::vals::Octospisel) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
        }
        #[doc = "HSPI1 kernel clock source selection These bits are used to select the HSPI1 kernel clock source. Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bitfield as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn hspi1sel(&self) -> super::vals::Hspisel {
            let val = (self.0 >> 22usize) & 0x03;
            super::vals::Hspisel::from_bits(val as u8)
        }
        #[doc = "HSPI1 kernel clock source selection These bits are used to select the HSPI1 kernel clock source. Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bitfield as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_hspi1sel(&mut self, val: super::vals::Hspisel) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val.to_bits() as u32) & 0x03) << 22usize);
        }
        #[doc = "I2C5 kernel clock source selection These bits are used to select the I2C5 kernel clock source. The I2C5 is functional in Stop 0 and Stop 1 modes only when the kernel clock is HSI�or MSIK. Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bitfield as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn i2c5sel(&self) -> super::vals::Icsel {
            let val = (self.0 >> 24usize) & 0x03;
            super::vals::Icsel::from_bits(val as u8)
        }
        #[doc = "I2C5 kernel clock source selection These bits are used to select the I2C5 kernel clock source. The I2C5 is functional in Stop 0 and Stop 1 modes only when the kernel clock is HSI�or MSIK. Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bitfield as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_i2c5sel(&mut self, val: super::vals::Icsel) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
        }
        #[doc = "I2C6 kernel clock source selection These bits are used to select the I2C6 kernel clock source. The I2C6 is functional in Stop 0 and Stop 1 modes only when the kernel clock is HSI�or MSIK. Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bitfield as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn i2c6sel(&self) -> super::vals::Icsel {
            let val = (self.0 >> 26usize) & 0x03;
            super::vals::Icsel::from_bits(val as u8)
        }
        #[doc = "I2C6 kernel clock source selection These bits are used to select the I2C6 kernel clock source. The I2C6 is functional in Stop 0 and Stop 1 modes only when the kernel clock is HSI�or MSIK. Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bitfield as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_i2c6sel(&mut self, val: super::vals::Icsel) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
        }
        #[doc = "OTG_HS PHY kernel clock source selection These bits are used to select the OTG_HS PHY kernel clock source. Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bitfield as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn otghssel(&self) -> super::vals::Otghssel {
            let val = (self.0 >> 30usize) & 0x03;
            super::vals::Otghssel::from_bits(val as u8)
        }
        #[doc = "OTG_HS PHY kernel clock source selection These bits are used to select the OTG_HS PHY kernel clock source. Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bitfield as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_otghssel(&mut self, val: super::vals::Otghssel) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val.to_bits() as u32) & 0x03) << 30usize);
        }
    }
    impl Default for Ccipr2 {
        #[inline(always)]
        fn default() -> Ccipr2 {
            Ccipr2(0)
        }
    }
    #[doc = "RCC peripherals independent clock configuration register 3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccipr3(pub u32);
    impl Ccipr3 {
        #[doc = "LPUART1 kernel clock source selection These bits are used to select the LPUART1 kernel clock source. others: reserved Note: The LPUART1 is functional in Stop 0, Stop 1 and Stop 2 modes only when the kernel clock is HSI, LSE or MSIK."]
        #[inline(always)]
        pub const fn lpuart1sel(&self) -> super::vals::Lpuartsel {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::Lpuartsel::from_bits(val as u8)
        }
        #[doc = "LPUART1 kernel clock source selection These bits are used to select the LPUART1 kernel clock source. others: reserved Note: The LPUART1 is functional in Stop 0, Stop 1 and Stop 2 modes only when the kernel clock is HSI, LSE or MSIK."]
        #[inline(always)]
        pub fn set_lpuart1sel(&mut self, val: super::vals::Lpuartsel) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "SPI3 kernel clock source selection These bits are used to select the SPI3 kernel clock source. Note: The SPI3 is functional in Stop 0, Stop 1 and Stop 2 modes only when the kernel clock is HSI or MSIK."]
        #[inline(always)]
        pub const fn spi3sel(&self) -> super::vals::Spisel {
            let val = (self.0 >> 3usize) & 0x03;
            super::vals::Spisel::from_bits(val as u8)
        }
        #[doc = "SPI3 kernel clock source selection These bits are used to select the SPI3 kernel clock source. Note: The SPI3 is functional in Stop 0, Stop 1 and Stop 2 modes only when the kernel clock is HSI or MSIK."]
        #[inline(always)]
        pub fn set_spi3sel(&mut self, val: super::vals::Spisel) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val.to_bits() as u32) & 0x03) << 3usize);
        }
        #[doc = "I2C3 kernel clock source selection These bits are used to select the I2C3 kernel clock source. Note: The I2C3 is functional in Stop 0, Stop 1 and Stop 2 modes only when the kernel clock is HSI or MSIK."]
        #[inline(always)]
        pub const fn i2c3sel(&self) -> super::vals::Icsel {
            let val = (self.0 >> 6usize) & 0x03;
            super::vals::Icsel::from_bits(val as u8)
        }
        #[doc = "I2C3 kernel clock source selection These bits are used to select the I2C3 kernel clock source. Note: The I2C3 is functional in Stop 0, Stop 1 and Stop 2 modes only when the kernel clock is HSI or MSIK."]
        #[inline(always)]
        pub fn set_i2c3sel(&mut self, val: super::vals::Icsel) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
        }
        #[doc = "LPTIM3 and LPTIM4 kernel clock source selection These bits are used to select the LPTIM3 and LPTIM4 kernel clock source. Note: The LPTIM3 and LPTIM4 are functional in Stop 0, Stop 1 and Stop 2 modes only when the kernel clock is LSI, LSE, HSI with HSIKERON = 1 or MSIK with MSIKERON = 1."]
        #[inline(always)]
        pub const fn lptim34sel(&self) -> super::vals::Lptimsel {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::Lptimsel::from_bits(val as u8)
        }
        #[doc = "LPTIM3 and LPTIM4 kernel clock source selection These bits are used to select the LPTIM3 and LPTIM4 kernel clock source. Note: The LPTIM3 and LPTIM4 are functional in Stop 0, Stop 1 and Stop 2 modes only when the kernel clock is LSI, LSE, HSI with HSIKERON = 1 or MSIK with MSIKERON = 1."]
        #[inline(always)]
        pub fn set_lptim34sel(&mut self, val: super::vals::Lptimsel) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
        #[doc = "LPTIM1 kernel clock source selection These bits are used to select the LPTIM1 kernel clock source. Note: The LPTIM1 is functional in Stop 0, Stop 1 and Stop 2 modes only when the kernel clock is LSI, LSE, HSI with HSIKERON = 1 or MSIK with MSIKERON = 1."]
        #[inline(always)]
        pub const fn lptim1sel(&self) -> super::vals::Lptimsel {
            let val = (self.0 >> 10usize) & 0x03;
            super::vals::Lptimsel::from_bits(val as u8)
        }
        #[doc = "LPTIM1 kernel clock source selection These bits are used to select the LPTIM1 kernel clock source. Note: The LPTIM1 is functional in Stop 0, Stop 1 and Stop 2 modes only when the kernel clock is LSI, LSE, HSI with HSIKERON = 1 or MSIK with MSIKERON = 1."]
        #[inline(always)]
        pub fn set_lptim1sel(&mut self, val: super::vals::Lptimsel) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
        }
        #[doc = "ADC1, ADC4 and DAC1 kernel clock source selection These bits are used to select the ADC1, ADC4 and DAC1 kernel clock source. others: reserved Note: The ADC1, ADC4 and DAC1 are functional in Stop 0, Stop 1 and Stop 2 modes only when the kernel clock is HSI or MSIK (only ADC4 and DAC1 are functional in Stop 2 mode)."]
        #[inline(always)]
        pub const fn adcdacsel(&self) -> super::vals::Adcdacsel {
            let val = (self.0 >> 12usize) & 0x07;
            super::vals::Adcdacsel::from_bits(val as u8)
        }
        #[doc = "ADC1, ADC4 and DAC1 kernel clock source selection These bits are used to select the ADC1, ADC4 and DAC1 kernel clock source. others: reserved Note: The ADC1, ADC4 and DAC1 are functional in Stop 0, Stop 1 and Stop 2 modes only when the kernel clock is HSI or MSIK (only ADC4 and DAC1 are functional in Stop 2 mode)."]
        #[inline(always)]
        pub fn set_adcdacsel(&mut self, val: super::vals::Adcdacsel) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
        }
        #[doc = "DAC1 sample and hold clock source selection This bit is used to select the DAC1 sample and hold clock source."]
        #[inline(always)]
        pub const fn dac1sel(&self) -> super::vals::Dacsel {
            let val = (self.0 >> 15usize) & 0x01;
            super::vals::Dacsel::from_bits(val as u8)
        }
        #[doc = "DAC1 sample and hold clock source selection This bit is used to select the DAC1 sample and hold clock source."]
        #[inline(always)]
        pub fn set_dac1sel(&mut self, val: super::vals::Dacsel) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
        }
        #[doc = "ADF1 kernel clock source selection These bits are used to select the ADF1 kernel clock source. others: reserved Note: The ADF1 is functional in Stop 0, Stop 1 and Stop 2 modes only when the kernel clock is AUDIOCLK or MSIK."]
        #[inline(always)]
        pub const fn adf1sel(&self) -> super::vals::Adfsel {
            let val = (self.0 >> 16usize) & 0x07;
            super::vals::Adfsel::from_bits(val as u8)
        }
        #[doc = "ADF1 kernel clock source selection These bits are used to select the ADF1 kernel clock source. others: reserved Note: The ADF1 is functional in Stop 0, Stop 1 and Stop 2 modes only when the kernel clock is AUDIOCLK or MSIK."]
        #[inline(always)]
        pub fn set_adf1sel(&mut self, val: super::vals::Adfsel) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
        }
    }
    impl Default for Ccipr3 {
        #[inline(always)]
        fn default() -> Ccipr3 {
            Ccipr3(0)
        }
    }
    #[doc = "RCC clock configuration register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr1(pub u32);
    impl Cfgr1 {
        #[doc = "system clock switch Set and cleared by software to select system clock source (SYSCLK). Configured by hardware to force MSIS oscillator selection when exiting Standby or Shutdown mode. Configured by hardware to force MSIS or HSI oscillator selection when exiting Stop mode or in case of HSE oscillator failure, depending on STOPWUCK value."]
        #[inline(always)]
        pub const fn sw(&self) -> super::vals::Sw {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Sw::from_bits(val as u8)
        }
        #[doc = "system clock switch Set and cleared by software to select system clock source (SYSCLK). Configured by hardware to force MSIS oscillator selection when exiting Standby or Shutdown mode. Configured by hardware to force MSIS or HSI oscillator selection when exiting Stop mode or in case of HSE oscillator failure, depending on STOPWUCK value."]
        #[inline(always)]
        pub fn set_sw(&mut self, val: super::vals::Sw) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "system clock switch status Set and cleared by hardware to indicate which clock source is used as system clock."]
        #[inline(always)]
        pub const fn sws(&self) -> super::vals::Sw {
            let val = (self.0 >> 2usize) & 0x03;
            super::vals::Sw::from_bits(val as u8)
        }
        #[doc = "system clock switch status Set and cleared by hardware to indicate which clock source is used as system clock."]
        #[inline(always)]
        pub fn set_sws(&mut self, val: super::vals::Sw) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
        }
        #[doc = "wakeup from Stop and CSS backup clock selection Set and cleared by software to select the system clock used when exiting Stop mode. The selected clock is also used as emergency clock for the clock security system on HSE. Warning: STOPWUCK must not be modified when the CSS is enabled by HSECSSON bit in RCC_CR and the system clock is HSE (SWS = 10) or a switch on HSE is requested (SW = 10)."]
        #[inline(always)]
        pub const fn stopwuck(&self) -> super::vals::Stopwuck {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::Stopwuck::from_bits(val as u8)
        }
        #[doc = "wakeup from Stop and CSS backup clock selection Set and cleared by software to select the system clock used when exiting Stop mode. The selected clock is also used as emergency clock for the clock security system on HSE. Warning: STOPWUCK must not be modified when the CSS is enabled by HSECSSON bit in RCC_CR and the system clock is HSE (SWS = 10) or a switch on HSE is requested (SW = 10)."]
        #[inline(always)]
        pub fn set_stopwuck(&mut self, val: super::vals::Stopwuck) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "wakeup from Stop kernel clock automatic enable selection Set and cleared by software to enable automatically another oscillator when exiting Stop mode. This oscillator can be used as independent kernel clock by peripherals."]
        #[inline(always)]
        pub const fn stopkerwuck(&self) -> super::vals::Stopkerwuck {
            let val = (self.0 >> 5usize) & 0x01;
            super::vals::Stopkerwuck::from_bits(val as u8)
        }
        #[doc = "wakeup from Stop kernel clock automatic enable selection Set and cleared by software to enable automatically another oscillator when exiting Stop mode. This oscillator can be used as independent kernel clock by peripherals."]
        #[inline(always)]
        pub fn set_stopkerwuck(&mut self, val: super::vals::Stopkerwuck) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
        }
        #[doc = "microcontroller clock output Set and cleared by software. Others: reserved Note: This clock output may have some truncated cycles at startup or during MCO clock source switching."]
        #[inline(always)]
        pub const fn mcosel(&self) -> super::vals::Mcosel {
            let val = (self.0 >> 24usize) & 0x0f;
            super::vals::Mcosel::from_bits(val as u8)
        }
        #[doc = "microcontroller clock output Set and cleared by software. Others: reserved Note: This clock output may have some truncated cycles at startup or during MCO clock source switching."]
        #[inline(always)]
        pub fn set_mcosel(&mut self, val: super::vals::Mcosel) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
        }
        #[doc = "microcontroller clock output prescaler Set and cleared by software. It is highly recommended to change this prescaler before MCO output is enabled. Others: not allowed"]
        #[inline(always)]
        pub const fn mcopre(&self) -> super::vals::Mcopre {
            let val = (self.0 >> 28usize) & 0x07;
            super::vals::Mcopre::from_bits(val as u8)
        }
        #[doc = "microcontroller clock output prescaler Set and cleared by software. It is highly recommended to change this prescaler before MCO output is enabled. Others: not allowed"]
        #[inline(always)]
        pub fn set_mcopre(&mut self, val: super::vals::Mcopre) {
            self.0 = (self.0 & !(0x07 << 28usize)) | (((val.to_bits() as u32) & 0x07) << 28usize);
        }
    }
    impl Default for Cfgr1 {
        #[inline(always)]
        fn default() -> Cfgr1 {
            Cfgr1(0)
        }
    }
    #[doc = "RCC clock configuration register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr2(pub u32);
    impl Cfgr2 {
        #[doc = "AHB prescaler Set and cleared by software to control the division factor of the AHB clock (HCLK). Depending on the device voltage range, the software must set these bits correctly to ensure that the system frequency does not exceed the maximum allowed frequency (for more details, refer to ). After a write operation to these bits and before decreasing the voltage range, this register must be read to be sure that the new value is taken into account. 0xxx: SYSCLK not divided"]
        #[inline(always)]
        pub const fn hpre(&self) -> super::vals::Hpre {
            let val = (self.0 >> 0usize) & 0x0f;
            super::vals::Hpre::from_bits(val as u8)
        }
        #[doc = "AHB prescaler Set and cleared by software to control the division factor of the AHB clock (HCLK). Depending on the device voltage range, the software must set these bits correctly to ensure that the system frequency does not exceed the maximum allowed frequency (for more details, refer to ). After a write operation to these bits and before decreasing the voltage range, this register must be read to be sure that the new value is taken into account. 0xxx: SYSCLK not divided"]
        #[inline(always)]
        pub fn set_hpre(&mut self, val: super::vals::Hpre) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
        }
        #[doc = "APB1 prescaler Set and cleared by software to control the division factor of the APB1 clock (PCLK1). 0xx: HCLK not divided"]
        #[inline(always)]
        pub const fn ppre1(&self) -> super::vals::Ppre {
            let val = (self.0 >> 4usize) & 0x07;
            super::vals::Ppre::from_bits(val as u8)
        }
        #[doc = "APB1 prescaler Set and cleared by software to control the division factor of the APB1 clock (PCLK1). 0xx: HCLK not divided"]
        #[inline(always)]
        pub fn set_ppre1(&mut self, val: super::vals::Ppre) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
        }
        #[doc = "APB2 prescaler Set and cleared by software to control the division factor of the APB2 clock (PCLK2). 0xx: HCLK not divided"]
        #[inline(always)]
        pub const fn ppre2(&self) -> super::vals::Ppre {
            let val = (self.0 >> 8usize) & 0x07;
            super::vals::Ppre::from_bits(val as u8)
        }
        #[doc = "APB2 prescaler Set and cleared by software to control the division factor of the APB2 clock (PCLK2). 0xx: HCLK not divided"]
        #[inline(always)]
        pub fn set_ppre2(&mut self, val: super::vals::Ppre) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
        }
        #[doc = "DSI PHY prescaler This bitfiled is set and cleared by software to control the division factor of DSI PHY bus clock (DCLK). 0xx: DCLK not divided Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bitfield as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn dpre(&self) -> super::vals::Dpre {
            let val = (self.0 >> 12usize) & 0x07;
            super::vals::Dpre::from_bits(val as u8)
        }
        #[doc = "DSI PHY prescaler This bitfiled is set and cleared by software to control the division factor of DSI PHY bus clock (DCLK). 0xx: DCLK not divided Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bitfield as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_dpre(&mut self, val: super::vals::Dpre) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
        }
        #[doc = "AHB1 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB1 peripherals (except those listed hereafter) are used and when their clocks are disabled in RCC_AHB1ENR. When this bit is set, all the AHB1 peripherals clocks are off, except for FLASH, BKPSRAM, ICACHE, DCACHE1 and SRAM1."]
        #[inline(always)]
        pub const fn ahb1dis(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "AHB1 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB1 peripherals (except those listed hereafter) are used and when their clocks are disabled in RCC_AHB1ENR. When this bit is set, all the AHB1 peripherals clocks are off, except for FLASH, BKPSRAM, ICACHE, DCACHE1 and SRAM1."]
        #[inline(always)]
        pub fn set_ahb1dis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "AHB2_1 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB2 peripherals from RCC_AHB2ENR1 (except SRAM2 and SRAM3) are used and when their clocks are disabled in RCC_AHB2ENR1. When this bit is set, all the AHB2 peripherals clocks from RCC_AHB2ENR1 are off, except for SRAM2 and SRAM3."]
        #[inline(always)]
        pub const fn ahb2dis1(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "AHB2_1 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB2 peripherals from RCC_AHB2ENR1 (except SRAM2 and SRAM3) are used and when their clocks are disabled in RCC_AHB2ENR1. When this bit is set, all the AHB2 peripherals clocks from RCC_AHB2ENR1 are off, except for SRAM2 and SRAM3."]
        #[inline(always)]
        pub fn set_ahb2dis1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "AHB2_2 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB2 peripherals from RCC_AHB2ENR2 are used and when their clocks are disabled in RCC_AHB2ENR2. When this bit is set, all the AHB2 peripherals clocks from RCC_AHB2EBNR2 are off."]
        #[inline(always)]
        pub const fn ahb2dis2(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "AHB2_2 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB2 peripherals from RCC_AHB2ENR2 are used and when their clocks are disabled in RCC_AHB2ENR2. When this bit is set, all the AHB2 peripherals clocks from RCC_AHB2EBNR2 are off."]
        #[inline(always)]
        pub fn set_ahb2dis2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "APB1 clock disable This bit can be set in order to further reduce power consumption, when none of the APB1 peripherals (except IWDG) are used and when their clocks are disabled in RCC_APB1ENR. When this bit is set, all the APB1 peripherals clocks are off, except for IWDG."]
        #[inline(always)]
        pub const fn apb1dis(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "APB1 clock disable This bit can be set in order to further reduce power consumption, when none of the APB1 peripherals (except IWDG) are used and when their clocks are disabled in RCC_APB1ENR. When this bit is set, all the APB1 peripherals clocks are off, except for IWDG."]
        #[inline(always)]
        pub fn set_apb1dis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "APB2 clock disable This bit can be set in order to further reduce power consumption, when none of the APB2 peripherals are used and when their clocks are disabled in RCC_APB2ENR. When this bit is set, all the APB2 peripherals clocks are off."]
        #[inline(always)]
        pub const fn apb2dis(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "APB2 clock disable This bit can be set in order to further reduce power consumption, when none of the APB2 peripherals are used and when their clocks are disabled in RCC_APB2ENR. When this bit is set, all the APB2 peripherals clocks are off."]
        #[inline(always)]
        pub fn set_apb2dis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
    }
    impl Default for Cfgr2 {
        #[inline(always)]
        fn default() -> Cfgr2 {
            Cfgr2(0)
        }
    }
    #[doc = "RCC clock configuration register 3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr3(pub u32);
    impl Cfgr3 {
        #[doc = "APB3 prescaler Set and cleared by software to control the division factor of the APB3 clock (PCLK3). 0xx: HCLK not divided"]
        #[inline(always)]
        pub const fn ppre3(&self) -> super::vals::Ppre {
            let val = (self.0 >> 4usize) & 0x07;
            super::vals::Ppre::from_bits(val as u8)
        }
        #[doc = "APB3 prescaler Set and cleared by software to control the division factor of the APB3 clock (PCLK3). 0xx: HCLK not divided"]
        #[inline(always)]
        pub fn set_ppre3(&mut self, val: super::vals::Ppre) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
        }
        #[doc = "AHB3 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB3 peripherals (except SRAM4) are used and when their clocks are disabled in RCC_AHB3ENR. When this bit is set, all the AHB3 peripherals clocks are off, except for SRAM4."]
        #[inline(always)]
        pub const fn ahb3dis(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "AHB3 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB3 peripherals (except SRAM4) are used and when their clocks are disabled in RCC_AHB3ENR. When this bit is set, all the AHB3 peripherals clocks are off, except for SRAM4."]
        #[inline(always)]
        pub fn set_ahb3dis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "APB3 clock disable This bit can be set in order to further reduce power consumption, when none of the APB3 peripherals from RCC_APB3ENR are used and when their clocks are disabled in RCC_APB3ENR. When this bit is set, all the APB3 peripherals clocks are off."]
        #[inline(always)]
        pub const fn apb3dis(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "APB3 clock disable This bit can be set in order to further reduce power consumption, when none of the APB3 peripherals from RCC_APB3ENR are used and when their clocks are disabled in RCC_APB3ENR. When this bit is set, all the APB3 peripherals clocks are off."]
        #[inline(always)]
        pub fn set_apb3dis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Cfgr3 {
        #[inline(always)]
        fn default() -> Cfgr3 {
            Cfgr3(0)
        }
    }
    #[doc = "RCC clock interrupt clear register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cicr(pub u32);
    impl Cicr {
        #[doc = "LSI ready interrupt clear Writing this bit to 1 clears the LSIRDYF flag. Writing 0 has no effect."]
        #[inline(always)]
        pub const fn lsirdyc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LSI ready interrupt clear Writing this bit to 1 clears the LSIRDYF flag. Writing 0 has no effect."]
        #[inline(always)]
        pub fn set_lsirdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "LSE ready interrupt clear Writing this bit to 1 clears the LSERDYF flag. Writing 0 has no effect."]
        #[inline(always)]
        pub const fn lserdyc(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "LSE ready interrupt clear Writing this bit to 1 clears the LSERDYF flag. Writing 0 has no effect."]
        #[inline(always)]
        pub fn set_lserdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "MSIS ready interrupt clear Writing this bit to 1 clears the MSISRDYF flag. Writing 0 has no effect."]
        #[inline(always)]
        pub const fn msisrdyc(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "MSIS ready interrupt clear Writing this bit to 1 clears the MSISRDYF flag. Writing 0 has no effect."]
        #[inline(always)]
        pub fn set_msisrdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "HSI ready interrupt clear Writing this bit to 1 clears the HSIRDYF flag. Writing 0 has no effect."]
        #[inline(always)]
        pub const fn hsirdyc(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "HSI ready interrupt clear Writing this bit to 1 clears the HSIRDYF flag. Writing 0 has no effect."]
        #[inline(always)]
        pub fn set_hsirdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "HSE ready interrupt clear Writing this bit to 1 clears the HSERDYF flag. Writing 0 has no effect."]
        #[inline(always)]
        pub const fn hserdyc(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "HSE ready interrupt clear Writing this bit to 1 clears the HSERDYF flag. Writing 0 has no effect."]
        #[inline(always)]
        pub fn set_hserdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "HSI48 ready interrupt clear Writing this bit to 1 clears the HSI48RDYF flag. Writing 0 has no effect."]
        #[inline(always)]
        pub const fn hsi48rdyc(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "HSI48 ready interrupt clear Writing this bit to 1 clears the HSI48RDYF flag. Writing 0 has no effect."]
        #[inline(always)]
        pub fn set_hsi48rdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "PLL1 ready interrupt clear Writing this bit to 1 clears the PLL1RDYF flag. Writing 0 has no effect."]
        #[inline(always)]
        pub const fn pllrdyc(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 6usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "PLL1 ready interrupt clear Writing this bit to 1 clears the PLL1RDYF flag. Writing 0 has no effect."]
        #[inline(always)]
        pub fn set_pllrdyc(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 6usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Clock security system interrupt clear Writing this bit to 1 clears the CSSF flag. Writing 0 has no effect."]
        #[inline(always)]
        pub const fn cssc(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Clock security system interrupt clear Writing this bit to 1 clears the CSSF flag. Writing 0 has no effect."]
        #[inline(always)]
        pub fn set_cssc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "MSIK oscillator ready interrupt clear Writing this bit to 1 clears the MSIKRDYF flag. Writing 0 has no effect."]
        #[inline(always)]
        pub const fn msikrdyc(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "MSIK oscillator ready interrupt clear Writing this bit to 1 clears the MSIKRDYF flag. Writing 0 has no effect."]
        #[inline(always)]
        pub fn set_msikrdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "SHSI oscillator ready interrupt clear Writing this bit to 1 clears the SHSIRDYF flag. Writing 0 has no effect."]
        #[inline(always)]
        pub const fn shsirdyc(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "SHSI oscillator ready interrupt clear Writing this bit to 1 clears the SHSIRDYF flag. Writing 0 has no effect."]
        #[inline(always)]
        pub fn set_shsirdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
    }
    impl Default for Cicr {
        #[inline(always)]
        fn default() -> Cicr {
            Cicr(0)
        }
    }
    #[doc = "RCC clock interrupt enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cier(pub u32);
    impl Cier {
        #[doc = "LSI ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the LSI oscillator stabilization."]
        #[inline(always)]
        pub const fn lsirdyie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LSI ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the LSI oscillator stabilization."]
        #[inline(always)]
        pub fn set_lsirdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "LSE ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the LSE oscillator stabilization."]
        #[inline(always)]
        pub const fn lserdyie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "LSE ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the LSE oscillator stabilization."]
        #[inline(always)]
        pub fn set_lserdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "MSIS ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the MSIS oscillator stabilization."]
        #[inline(always)]
        pub const fn msisrdyie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "MSIS ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the MSIS oscillator stabilization."]
        #[inline(always)]
        pub fn set_msisrdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "HSI ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSI oscillator stabilization."]
        #[inline(always)]
        pub const fn hsirdyie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "HSI ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSI oscillator stabilization."]
        #[inline(always)]
        pub fn set_hsirdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "HSE ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSE oscillator stabilization."]
        #[inline(always)]
        pub const fn hserdyie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "HSE ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSE oscillator stabilization."]
        #[inline(always)]
        pub fn set_hserdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "HSI48 ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSI48 oscillator stabilization."]
        #[inline(always)]
        pub const fn hsi48rdyie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "HSI48 ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSI48 oscillator stabilization."]
        #[inline(always)]
        pub fn set_hsi48rdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "PLL ready interrupt enable Set and cleared by software to enable/disable interrupt caused by PLL1 lock."]
        #[inline(always)]
        pub const fn pllrdyie(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 6usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "PLL ready interrupt enable Set and cleared by software to enable/disable interrupt caused by PLL1 lock."]
        #[inline(always)]
        pub fn set_pllrdyie(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 6usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "MSIK ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the MSIK oscillator stabilization."]
        #[inline(always)]
        pub const fn msikrdyie(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "MSIK ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the MSIK oscillator stabilization."]
        #[inline(always)]
        pub fn set_msikrdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "SHSI ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the SHSI oscillator stabilization."]
        #[inline(always)]
        pub const fn shsirdyie(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "SHSI ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the SHSI oscillator stabilization."]
        #[inline(always)]
        pub fn set_shsirdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
    }
    impl Default for Cier {
        #[inline(always)]
        fn default() -> Cier {
            Cier(0)
        }
    }
    #[doc = "RCC clock interrupt flag register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cifr(pub u32);
    impl Cifr {
        #[doc = "LSI ready interrupt flag Set by hardware when the LSI clock becomes stable and LSIRDYIE is set. Cleared by software setting the LSIRDYC bit."]
        #[inline(always)]
        pub const fn lsirdyf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LSI ready interrupt flag Set by hardware when the LSI clock becomes stable and LSIRDYIE is set. Cleared by software setting the LSIRDYC bit."]
        #[inline(always)]
        pub fn set_lsirdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "LSE ready interrupt flag Set by hardware when the LSE clock becomes stable and LSERDYIE is set. Cleared by software setting the LSERDYC bit."]
        #[inline(always)]
        pub const fn lserdyf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "LSE ready interrupt flag Set by hardware when the LSE clock becomes stable and LSERDYIE is set. Cleared by software setting the LSERDYC bit."]
        #[inline(always)]
        pub fn set_lserdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "MSIS ready interrupt flag Set by hardware when the MSIS clock becomes stable and MSISRDYIE is set. Cleared by software setting the MSISRDYC bit."]
        #[inline(always)]
        pub const fn msisrdyf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "MSIS ready interrupt flag Set by hardware when the MSIS clock becomes stable and MSISRDYIE is set. Cleared by software setting the MSISRDYC bit."]
        #[inline(always)]
        pub fn set_msisrdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "HSI ready interrupt flag Set by hardware when the HSI clock becomes stable and HSIRDYIE is set in a response to setting the HSION (see RCC_CR). When HSION is not set but the HSI oscillator is enabled by the peripheral through a clock request, this bit is not set and no interrupt is generated. Cleared by software setting the HSIRDYC bit."]
        #[inline(always)]
        pub const fn hsirdyf(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "HSI ready interrupt flag Set by hardware when the HSI clock becomes stable and HSIRDYIE is set in a response to setting the HSION (see RCC_CR). When HSION is not set but the HSI oscillator is enabled by the peripheral through a clock request, this bit is not set and no interrupt is generated. Cleared by software setting the HSIRDYC bit."]
        #[inline(always)]
        pub fn set_hsirdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "HSE ready interrupt flag Set by hardware when the HSE clock becomes stable and HSERDYIE is set. Cleared by software setting the HSERDYC bit."]
        #[inline(always)]
        pub const fn hserdyf(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "HSE ready interrupt flag Set by hardware when the HSE clock becomes stable and HSERDYIE is set. Cleared by software setting the HSERDYC bit."]
        #[inline(always)]
        pub fn set_hserdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "HSI48 ready interrupt flag Set by hardware when the HSI48 clock becomes stable and HSI48RDYIE is set. Cleared by software setting the HSI48RDYC bit."]
        #[inline(always)]
        pub const fn hsi48rdyf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "HSI48 ready interrupt flag Set by hardware when the HSI48 clock becomes stable and HSI48RDYIE is set. Cleared by software setting the HSI48RDYC bit."]
        #[inline(always)]
        pub fn set_hsi48rdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "PLL1 ready interrupt flag Set by hardware when the PLL1 locks and PLL1RDYIE is set. Cleared by software setting the PLL1RDYC bit."]
        #[inline(always)]
        pub const fn pllrdyf(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 6usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "PLL1 ready interrupt flag Set by hardware when the PLL1 locks and PLL1RDYIE is set. Cleared by software setting the PLL1RDYC bit."]
        #[inline(always)]
        pub fn set_pllrdyf(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 6usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Clock security system interrupt flag Set by hardware when a failure is detected in the HSE oscillator. Cleared by software setting the CSSC bit."]
        #[inline(always)]
        pub const fn cssf(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Clock security system interrupt flag Set by hardware when a failure is detected in the HSE oscillator. Cleared by software setting the CSSC bit."]
        #[inline(always)]
        pub fn set_cssf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "MSIK ready interrupt flag Set by hardware when the MSIK clock becomes stable and MSIKRDYIE is set. Cleared by software setting the MSIKRDYC bit."]
        #[inline(always)]
        pub const fn msikrdyf(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "MSIK ready interrupt flag Set by hardware when the MSIK clock becomes stable and MSIKRDYIE is set. Cleared by software setting the MSIKRDYC bit."]
        #[inline(always)]
        pub fn set_msikrdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "SHSI ready interrupt flag Set by hardware when the SHSI clock becomes stable and SHSIRDYIE is set. Cleared by software setting the SHSIRDYC bit."]
        #[inline(always)]
        pub const fn shsirdyf(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "SHSI ready interrupt flag Set by hardware when the SHSI clock becomes stable and SHSIRDYIE is set. Cleared by software setting the SHSIRDYC bit."]
        #[inline(always)]
        pub fn set_shsirdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
    }
    impl Default for Cifr {
        #[inline(always)]
        fn default() -> Cifr {
            Cifr(0)
        }
    }
    #[doc = "RCC clock control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "MSIS clock enable Set and cleared by software. Cleared by hardware to stop the MSIS oscillator when entering Stop, Standby or Shutdown mode. Set by hardware to force the MSIS oscillator ON when exiting Standby or Shutdown mode. Set by hardware to force the MSIS oscillator ON when STOPWUCK = 0 when exiting Stop modes or in case of a failure of the HSE oscillator. Set by hardware when used directly or indirectly as system clock."]
        #[inline(always)]
        pub const fn msison(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "MSIS clock enable Set and cleared by software. Cleared by hardware to stop the MSIS oscillator when entering Stop, Standby or Shutdown mode. Set by hardware to force the MSIS oscillator ON when exiting Standby or Shutdown mode. Set by hardware to force the MSIS oscillator ON when STOPWUCK = 0 when exiting Stop modes or in case of a failure of the HSE oscillator. Set by hardware when used directly or indirectly as system clock."]
        #[inline(always)]
        pub fn set_msison(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "MSI enable for some peripheral kernels Set and cleared by software to force MSI ON even in Stop modes. Keeping the MSI ON in Stop mode allows the communication speed not to be reduced by the MSI startup time. This bit has no effect on MSISON and MSIKON values (see autonomous mode for more details). The MSIKERON must be configured at 0 before entering Stop 3 mode."]
        #[inline(always)]
        pub const fn msikeron(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "MSI enable for some peripheral kernels Set and cleared by software to force MSI ON even in Stop modes. Keeping the MSI ON in Stop mode allows the communication speed not to be reduced by the MSI startup time. This bit has no effect on MSISON and MSIKON values (see autonomous mode for more details). The MSIKERON must be configured at 0 before entering Stop 3 mode."]
        #[inline(always)]
        pub fn set_msikeron(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "MSIS clock ready flag Set by hardware to indicate that the MSIS oscillator is stable. This bit is set only when MSIS is enabled by software by setting MSISON. Note: Once the MSISON bit is cleared, MSISRDY goes low after six MSIS clock cycles."]
        #[inline(always)]
        pub const fn msisrdy(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "MSIS clock ready flag Set by hardware to indicate that the MSIS oscillator is stable. This bit is set only when MSIS is enabled by software by setting MSISON. Note: Once the MSISON bit is cleared, MSISRDY goes low after six MSIS clock cycles."]
        #[inline(always)]
        pub fn set_msisrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "MSI clock PLL-mode enable Set and cleared by software to enable/disable the PLL part of the MSI clock source. MSIPLLEN must be enabled after LSE is enabled (LSEON enabled) and ready (LSERDY set by hardware). A hardware protection prevents from enabling MSIPLLEN if LSE is not ready. This bit is cleared by hardware when LSE is disabled (LSEON = 0) or when the CSS on LSE detects a LSE failure (see RCC_CSR)."]
        #[inline(always)]
        pub const fn msipllen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "MSI clock PLL-mode enable Set and cleared by software to enable/disable the PLL part of the MSI clock source. MSIPLLEN must be enabled after LSE is enabled (LSEON enabled) and ready (LSERDY set by hardware). A hardware protection prevents from enabling MSIPLLEN if LSE is not ready. This bit is cleared by hardware when LSE is disabled (LSEON = 0) or when the CSS on LSE detects a LSE failure (see RCC_CSR)."]
        #[inline(always)]
        pub fn set_msipllen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "MSIK clock enable Set and cleared by software. Cleared by hardware to stop the MSIK when entering Stop, Standby or Shutdown mode. Set by hardware to force the MSIK oscillator ON when exiting Standby or Shutdown mode. Set by hardware to force the MSIK oscillator ON when STOPWUCK = 0 or STOPKERWUCK = 0 when exiting Stop modes or in case of a failure of the HSE oscillator."]
        #[inline(always)]
        pub const fn msikon(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "MSIK clock enable Set and cleared by software. Cleared by hardware to stop the MSIK when entering Stop, Standby or Shutdown mode. Set by hardware to force the MSIK oscillator ON when exiting Standby or Shutdown mode. Set by hardware to force the MSIK oscillator ON when STOPWUCK = 0 or STOPKERWUCK = 0 when exiting Stop modes or in case of a failure of the HSE oscillator."]
        #[inline(always)]
        pub fn set_msikon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "MSIK clock ready flag Set by hardware to indicate that the MSIK is stable. This bit is set only when MSI kernel oscillator is enabled by software by setting MSIKON. Note: Once the MSIKON bit is cleared, MSIKRDY goes low after six MSIK oscillator clock cycles."]
        #[inline(always)]
        pub const fn msikrdy(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "MSIK clock ready flag Set by hardware to indicate that the MSIK is stable. This bit is set only when MSI kernel oscillator is enabled by software by setting MSIKON. Note: Once the MSIKON bit is cleared, MSIKRDY goes low after six MSIK oscillator clock cycles."]
        #[inline(always)]
        pub fn set_msikrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "MSI clock with PLL mode selection Set and cleared by software to select which MSI output clock uses the PLL mode. This bit can be written only when the MSI PLL mode is disabled (MSIPLLEN = 0). Note: If the MSI kernel clock output uses the same oscillator source than the MSI system clock output, then the PLL mode is applied to the both clocks outputs."]
        #[inline(always)]
        pub const fn msipllsel(&self) -> super::vals::Msipllsel {
            let val = (self.0 >> 6usize) & 0x01;
            super::vals::Msipllsel::from_bits(val as u8)
        }
        #[doc = "MSI clock with PLL mode selection Set and cleared by software to select which MSI output clock uses the PLL mode. This bit can be written only when the MSI PLL mode is disabled (MSIPLLEN = 0). Note: If the MSI kernel clock output uses the same oscillator source than the MSI system clock output, then the PLL mode is applied to the both clocks outputs."]
        #[inline(always)]
        pub fn set_msipllsel(&mut self, val: super::vals::Msipllsel) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
        }
        #[doc = "MSI PLL mode fast startup Set and reset by software to enable/disable the fast PLL mode start-up of the MSI clock source. This bit is used only if PLL mode is selected (MSIPLLEN = 1). The fast start-up feature is not active the first time the PLL mode is selected. The fast start-up is active when the MSI in PLL mode returns from switch off."]
        #[inline(always)]
        pub const fn msipllfast(&self) -> super::vals::Msipllfast {
            let val = (self.0 >> 7usize) & 0x01;
            super::vals::Msipllfast::from_bits(val as u8)
        }
        #[doc = "MSI PLL mode fast startup Set and reset by software to enable/disable the fast PLL mode start-up of the MSI clock source. This bit is used only if PLL mode is selected (MSIPLLEN = 1). The fast start-up feature is not active the first time the PLL mode is selected. The fast start-up is active when the MSI in PLL mode returns from switch off."]
        #[inline(always)]
        pub fn set_msipllfast(&mut self, val: super::vals::Msipllfast) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
        }
        #[doc = "HSI clock enable Set and cleared by software. Cleared by hardware to stop the HSI oscillator when entering Stop, Standby or Shutdown mode. Set by hardware to force the HSI oscillator ON when STOPWUCK = 1 when leaving Stop modes, or in case of failure of the HSE crystal oscillator. This bit is set by hardware if the HSI is used directly or indirectly as system clock."]
        #[inline(always)]
        pub const fn hsion(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "HSI clock enable Set and cleared by software. Cleared by hardware to stop the HSI oscillator when entering Stop, Standby or Shutdown mode. Set by hardware to force the HSI oscillator ON when STOPWUCK = 1 when leaving Stop modes, or in case of failure of the HSE crystal oscillator. This bit is set by hardware if the HSI is used directly or indirectly as system clock."]
        #[inline(always)]
        pub fn set_hsion(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "HSI enable for some peripheral kernels Set and cleared by software to force HSI ON even in Stop modes. Keeping the HSI ON in Stop mode allows the communication speed not to be reduced by the HSI startup time. This bit has no effect on HSION value. Refer to for more details. The HSIKERON must be configured at 0 before entering Stop 3 mode."]
        #[inline(always)]
        pub const fn hsikeron(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "HSI enable for some peripheral kernels Set and cleared by software to force HSI ON even in Stop modes. Keeping the HSI ON in Stop mode allows the communication speed not to be reduced by the HSI startup time. This bit has no effect on HSION value. Refer to for more details. The HSIKERON must be configured at 0 before entering Stop 3 mode."]
        #[inline(always)]
        pub fn set_hsikeron(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "HSI clock ready flag Set by hardware to indicate that HSI oscillator is stable. This bit is set only when HSI is enabled by software by setting HSION. Note: Once the HSION bit is cleared, HSIRDY goes low after six HSI clock cycles."]
        #[inline(always)]
        pub const fn hsirdy(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "HSI clock ready flag Set by hardware to indicate that HSI oscillator is stable. This bit is set only when HSI is enabled by software by setting HSION. Note: Once the HSION bit is cleared, HSIRDY goes low after six HSI clock cycles."]
        #[inline(always)]
        pub fn set_hsirdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "HSI48 clock enable Set and cleared by software. Cleared by hardware to stop the HSI48 when entering in Stop, Standby or Shutdown modes."]
        #[inline(always)]
        pub const fn hsi48on(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "HSI48 clock enable Set and cleared by software. Cleared by hardware to stop the HSI48 when entering in Stop, Standby or Shutdown modes."]
        #[inline(always)]
        pub fn set_hsi48on(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "HSI48 clock ready flag Set by hardware to indicate that HSI48 oscillator is stable. This bit is set only when HSI48 is enabled by software by setting HSI48ON."]
        #[inline(always)]
        pub const fn hsi48rdy(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "HSI48 clock ready flag Set by hardware to indicate that HSI48 oscillator is stable. This bit is set only when HSI48 is enabled by software by setting HSI48ON."]
        #[inline(always)]
        pub fn set_hsi48rdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "SHSI clock enable Set and cleared by software. Cleared by hardware to stop the SHSI when entering in Stop, Standby or Shutdown modes."]
        #[inline(always)]
        pub const fn shsion(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "SHSI clock enable Set and cleared by software. Cleared by hardware to stop the SHSI when entering in Stop, Standby or Shutdown modes."]
        #[inline(always)]
        pub fn set_shsion(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "SHSI clock ready flag Set by hardware to indicate that the SHSI oscillator is stable. This bit is set only when SHSI is enabled by software by setting SHSION. Note: Once the SHSION bit is cleared, SHSIRDY goes low after six SHSI clock cycles."]
        #[inline(always)]
        pub const fn shsirdy(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "SHSI clock ready flag Set by hardware to indicate that the SHSI oscillator is stable. This bit is set only when SHSI is enabled by software by setting SHSION. Note: Once the SHSION bit is cleared, SHSIRDY goes low after six SHSI clock cycles."]
        #[inline(always)]
        pub fn set_shsirdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "HSE clock enable Set and cleared by software. Cleared by hardware to stop the HSE oscillator when entering Stop, Standby or Shutdown mode. This bit cannot be reset if the HSE oscillator is used directly or indirectly as the system clock."]
        #[inline(always)]
        pub const fn hseon(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "HSE clock enable Set and cleared by software. Cleared by hardware to stop the HSE oscillator when entering Stop, Standby or Shutdown mode. This bit cannot be reset if the HSE oscillator is used directly or indirectly as the system clock."]
        #[inline(always)]
        pub fn set_hseon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "HSE clock ready flag Set by hardware to indicate that the HSE oscillator is stable. Note: Once the HSEON bit is cleared, HSERDY goes low after six HSE clock cycles."]
        #[inline(always)]
        pub const fn hserdy(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "HSE clock ready flag Set by hardware to indicate that the HSE oscillator is stable. Note: Once the HSEON bit is cleared, HSERDY goes low after six HSE clock cycles."]
        #[inline(always)]
        pub fn set_hserdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "HSE crystal oscillator bypass Set and cleared by software to bypass the oscillator with an external clock. The external clock must be enabled with the HSEON bit set, to be used by the device. The HSEBYP bit can be written only if the HSE oscillator is disabled."]
        #[inline(always)]
        pub const fn hsebyp(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "HSE crystal oscillator bypass Set and cleared by software to bypass the oscillator with an external clock. The external clock must be enabled with the HSEON bit set, to be used by the device. The HSEBYP bit can be written only if the HSE oscillator is disabled."]
        #[inline(always)]
        pub fn set_hsebyp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Clock security system enable Set by software to enable the clock security system. When CSSON is set, the clock detector is enabled by hardware when the HSE oscillator is ready, and disabled by hardware if a HSE clock failure is detected. This bit is set only and is cleared by reset."]
        #[inline(always)]
        pub const fn csson(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Clock security system enable Set by software to enable the clock security system. When CSSON is set, the clock detector is enabled by hardware when the HSE oscillator is ready, and disabled by hardware if a HSE clock failure is detected. This bit is set only and is cleared by reset."]
        #[inline(always)]
        pub fn set_csson(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "HSE external clock bypass mode Set and reset by software to select the external clock mode in bypass mode. External clock mode must be configured with HSEON bit to be used by the device. This bit can be written only if the HSE oscillator is disabled. This bit is active only if the HSE bypass mode is enabled."]
        #[inline(always)]
        pub const fn hseext(&self) -> super::vals::Hseext {
            let val = (self.0 >> 20usize) & 0x01;
            super::vals::Hseext::from_bits(val as u8)
        }
        #[doc = "HSE external clock bypass mode Set and reset by software to select the external clock mode in bypass mode. External clock mode must be configured with HSEON bit to be used by the device. This bit can be written only if the HSE oscillator is disabled. This bit is active only if the HSE bypass mode is enabled."]
        #[inline(always)]
        pub fn set_hseext(&mut self, val: super::vals::Hseext) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
        }
        #[doc = "PLL1 enable Set and cleared by software to enable the main PLL. Cleared by hardware when entering Stop, Standby or Shutdown mode. This bit cannot be reset if the PLL1 clock is used as the system clock."]
        #[inline(always)]
        pub const fn pllon(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 24usize + n * 2usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "PLL1 enable Set and cleared by software to enable the main PLL. Cleared by hardware when entering Stop, Standby or Shutdown mode. This bit cannot be reset if the PLL1 clock is used as the system clock."]
        #[inline(always)]
        pub fn set_pllon(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 24usize + n * 2usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "PLL1 clock ready flag Set by hardware to indicate that the PLL1 is locked."]
        #[inline(always)]
        pub const fn pllrdy(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 25usize + n * 2usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "PLL1 clock ready flag Set by hardware to indicate that the PLL1 is locked."]
        #[inline(always)]
        pub fn set_pllrdy(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 25usize + n * 2usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Cr {
        #[inline(always)]
        fn default() -> Cr {
            Cr(0)
        }
    }
    #[doc = "RCC clock recovery RC register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Crrcr(pub u32);
    impl Crrcr {
        #[doc = "HSI48 clock calibration These bits are initialized at startup with the factory-programmed HSI48 calibration trim value."]
        #[inline(always)]
        pub const fn hsi48cal(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "HSI48 clock calibration These bits are initialized at startup with the factory-programmed HSI48 calibration trim value."]
        #[inline(always)]
        pub fn set_hsi48cal(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Crrcr {
        #[inline(always)]
        fn default() -> Crrcr {
            Crrcr(0)
        }
    }
    #[doc = "RCC control/status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csr(pub u32);
    impl Csr {
        #[doc = "MSIK range after Standby mode Set by software to chose the MSIK frequency at startup. This range is used after exiting Standby mode until MSIRGSEL is set. After a NRST pin or a power-on reset or when exiting Shutdown mode, the range is always 4 MHz. MSIKSRANGE can be written only when MSIRGSEL = 1. others: reserved Note: Changing the MSIKSRANGE does not change the current MSIK frequency."]
        #[inline(always)]
        pub const fn msiksrange(&self) -> super::vals::Msixsrange {
            let val = (self.0 >> 8usize) & 0x0f;
            super::vals::Msixsrange::from_bits(val as u8)
        }
        #[doc = "MSIK range after Standby mode Set by software to chose the MSIK frequency at startup. This range is used after exiting Standby mode until MSIRGSEL is set. After a NRST pin or a power-on reset or when exiting Shutdown mode, the range is always 4 MHz. MSIKSRANGE can be written only when MSIRGSEL = 1. others: reserved Note: Changing the MSIKSRANGE does not change the current MSIK frequency."]
        #[inline(always)]
        pub fn set_msiksrange(&mut self, val: super::vals::Msixsrange) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
        }
        #[doc = "MSIS range after Standby mode Set by software to chose the MSIS frequency at startup. This range is used after exiting Standby mode until MSIRGSEL is set. After a NRST pin or a power-on reset or when exiting Shutdown mode, the range is always 4 MHz. MSISSRANGE can be written only when MSIRGSEL = 1. others: reserved Note: Changing the MSISSRANGE does not change the current MSIS frequency."]
        #[inline(always)]
        pub const fn msissrange(&self) -> super::vals::Msixsrange {
            let val = (self.0 >> 12usize) & 0x0f;
            super::vals::Msixsrange::from_bits(val as u8)
        }
        #[doc = "MSIS range after Standby mode Set by software to chose the MSIS frequency at startup. This range is used after exiting Standby mode until MSIRGSEL is set. After a NRST pin or a power-on reset or when exiting Shutdown mode, the range is always 4 MHz. MSISSRANGE can be written only when MSIRGSEL = 1. others: reserved Note: Changing the MSISSRANGE does not change the current MSIS frequency."]
        #[inline(always)]
        pub fn set_msissrange(&mut self, val: super::vals::Msixsrange) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
        }
        #[doc = "Remove reset flag Set by software to clear the reset flags."]
        #[inline(always)]
        pub const fn rmvf(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Remove reset flag Set by software to clear the reset flags."]
        #[inline(always)]
        pub fn set_rmvf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Option byte loader reset flag Set by hardware when a reset from the option byte loading occurs. Cleared by writing to the RMVF bit."]
        #[inline(always)]
        pub const fn oblrstf(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Option byte loader reset flag Set by hardware when a reset from the option byte loading occurs. Cleared by writing to the RMVF bit."]
        #[inline(always)]
        pub fn set_oblrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "NRST pin reset flag Set by hardware when a reset from the NRST pin occurs. Cleared by writing to the RMVF bit."]
        #[inline(always)]
        pub const fn pinrstf(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "NRST pin reset flag Set by hardware when a reset from the NRST pin occurs. Cleared by writing to the RMVF bit."]
        #[inline(always)]
        pub fn set_pinrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "BOR flag Set by hardware when a BOR occurs. Cleared by writing to the RMVF bit."]
        #[inline(always)]
        pub const fn borrstf(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "BOR flag Set by hardware when a BOR occurs. Cleared by writing to the RMVF bit."]
        #[inline(always)]
        pub fn set_borrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Software reset flag Set by hardware when a software reset occurs. Cleared by writing to the RMVF bit."]
        #[inline(always)]
        pub const fn sftrstf(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Software reset flag Set by hardware when a software reset occurs. Cleared by writing to the RMVF bit."]
        #[inline(always)]
        pub fn set_sftrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Independent watchdog reset flag Set by hardware when an independent watchdog reset domain occurs. Cleared by writing to the RMVF bit."]
        #[inline(always)]
        pub const fn iwdgrstf(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Independent watchdog reset flag Set by hardware when an independent watchdog reset domain occurs. Cleared by writing to the RMVF bit."]
        #[inline(always)]
        pub fn set_iwdgrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Window watchdog reset flag Set by hardware when a window watchdog reset occurs. Cleared by writing to the RMVF bit."]
        #[inline(always)]
        pub const fn wwdgrstf(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Window watchdog reset flag Set by hardware when a window watchdog reset occurs. Cleared by writing to the RMVF bit."]
        #[inline(always)]
        pub fn set_wwdgrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Low-power reset flag Set by hardware when a reset occurs due to Stop, Standby or Shutdown mode entry, whereas the corresponding nRST_STOP, nRST_STBY or nRST_SHDW option bit is cleared. Cleared by writing to the RMVF bit."]
        #[inline(always)]
        pub const fn lpwrrstf(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Low-power reset flag Set by hardware when a reset occurs due to Stop, Standby or Shutdown mode entry, whereas the corresponding nRST_STOP, nRST_STBY or nRST_SHDW option bit is cleared. Cleared by writing to the RMVF bit."]
        #[inline(always)]
        pub fn set_lpwrrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Csr {
        #[inline(always)]
        fn default() -> Csr {
            Csr(0)
        }
    }
    #[doc = "RCC internal clock sources calibration register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icscr1(pub u32);
    impl Icscr1 {
        #[doc = "MSIRC3 clock calibration for MSI ranges 12 to 15 These bits are initialized at startup with the factory-programmed MSIRC3 calibration trim value for ranges 12 to 15. When MSITRIM3 is written, MSICAL3 is updated with the sum of MSITRIM3\\[4:0\\]
and the factory calibration trim value MSIRC2\\[4:0\\]. There is no hardware protection to limit a potential overflow due to the addition of MSITRIM bitfield and factory program bitfield for this calibration value. Control must be managed by software at user level."]
        #[inline(always)]
        pub const fn msical3(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "MSIRC3 clock calibration for MSI ranges 12 to 15 These bits are initialized at startup with the factory-programmed MSIRC3 calibration trim value for ranges 12 to 15. When MSITRIM3 is written, MSICAL3 is updated with the sum of MSITRIM3\\[4:0\\]
and the factory calibration trim value MSIRC2\\[4:0\\]. There is no hardware protection to limit a potential overflow due to the addition of MSITRIM bitfield and factory program bitfield for this calibration value. Control must be managed by software at user level."]
        #[inline(always)]
        pub fn set_msical3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "MSIRC2 clock calibration for MSI ranges 8 to 11 These bits are initialized at startup with the factory-programmed MSIRC2 calibration trim value for ranges 8 to 11. When MSITRIM2 is written, MSICAL2 is updated with the sum of MSITRIM2\\[4:0\\]
and the factory calibration trim value MSIRC2\\[4:0\\]. There is no hardware protection to limit a potential overflow due to the addition of MSITRIM bitfield and factory program bitfield for this calibration value. Control must be managed by software at user level."]
        #[inline(always)]
        pub const fn msical2(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x1f;
            val as u8
        }
        #[doc = "MSIRC2 clock calibration for MSI ranges 8 to 11 These bits are initialized at startup with the factory-programmed MSIRC2 calibration trim value for ranges 8 to 11. When MSITRIM2 is written, MSICAL2 is updated with the sum of MSITRIM2\\[4:0\\]
and the factory calibration trim value MSIRC2\\[4:0\\]. There is no hardware protection to limit a potential overflow due to the addition of MSITRIM bitfield and factory program bitfield for this calibration value. Control must be managed by software at user level."]
        #[inline(always)]
        pub fn set_msical2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 5usize)) | (((val as u32) & 0x1f) << 5usize);
        }
        #[doc = "MSIRC1 clock calibration for MSI ranges 4 to 7 These bits are initialized at startup with the factory-programmed MSIRC1 calibration trim value for ranges 4 to 7. When MSITRIM1 is written, MSICAL1 is updated with the sum of MSITRIM1\\[4:0\\]
and the factory calibration trim value MSIRC1\\[4:0\\]. There is no hardware protection to limit a potential overflow due to the addition of MSITRIM bitfield and factory program bitfield for this calibration value. Control must be managed by software at user level."]
        #[inline(always)]
        pub const fn msical1(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x1f;
            val as u8
        }
        #[doc = "MSIRC1 clock calibration for MSI ranges 4 to 7 These bits are initialized at startup with the factory-programmed MSIRC1 calibration trim value for ranges 4 to 7. When MSITRIM1 is written, MSICAL1 is updated with the sum of MSITRIM1\\[4:0\\]
and the factory calibration trim value MSIRC1\\[4:0\\]. There is no hardware protection to limit a potential overflow due to the addition of MSITRIM bitfield and factory program bitfield for this calibration value. Control must be managed by software at user level."]
        #[inline(always)]
        pub fn set_msical1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 10usize)) | (((val as u32) & 0x1f) << 10usize);
        }
        #[doc = "MSIRC0 clock calibration for MSI ranges 0 to 3 These bits are initialized at startup with the factory-programmed MSIRC0 calibration trim value for ranges 0 to 3. When MSITRIM0 is written, MSICAL0 is updated with the sum of MSITRIM0\\[4:0\\]
and the factory-programmed calibration trim value MSIRC0\\[4:0\\]."]
        #[inline(always)]
        pub const fn msical0(&self) -> u8 {
            let val = (self.0 >> 15usize) & 0x1f;
            val as u8
        }
        #[doc = "MSIRC0 clock calibration for MSI ranges 0 to 3 These bits are initialized at startup with the factory-programmed MSIRC0 calibration trim value for ranges 0 to 3. When MSITRIM0 is written, MSICAL0 is updated with the sum of MSITRIM0\\[4:0\\]
and the factory-programmed calibration trim value MSIRC0\\[4:0\\]."]
        #[inline(always)]
        pub fn set_msical0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 15usize)) | (((val as u32) & 0x1f) << 15usize);
        }
        #[doc = "MSI bias mode selection Set by software to select the MSI bias mode. By default, the MSI bias is in continuous mode in order to maintain the output clocks accuracy. Setting this bit reduces the MSI consumption under range 4 but decrease its accuracy."]
        #[inline(always)]
        pub const fn msibias(&self) -> super::vals::Msibias {
            let val = (self.0 >> 22usize) & 0x01;
            super::vals::Msibias::from_bits(val as u8)
        }
        #[doc = "MSI bias mode selection Set by software to select the MSI bias mode. By default, the MSI bias is in continuous mode in order to maintain the output clocks accuracy. Setting this bit reduces the MSI consumption under range 4 but decrease its accuracy."]
        #[inline(always)]
        pub fn set_msibias(&mut self, val: super::vals::Msibias) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
        }
        #[doc = "MSI clock range selection Set by software to select the MSIS and MSIK clocks range with MSISRANGE\\[3:0\\]
and MSIKRANGE\\[3:0\\]. Write 0 has no effect. After exiting Standby or Shutdown mode, or after a reset, this bit is at 0 and the MSIS and MSIK ranges are provided by MSISSRANGE\\[3:0\\]
and MSIKSRANGE\\[3:0\\]
in RCC_CSR."]
        #[inline(always)]
        pub const fn msirgsel(&self) -> super::vals::Msirgsel {
            let val = (self.0 >> 23usize) & 0x01;
            super::vals::Msirgsel::from_bits(val as u8)
        }
        #[doc = "MSI clock range selection Set by software to select the MSIS and MSIK clocks range with MSISRANGE\\[3:0\\]
and MSIKRANGE\\[3:0\\]. Write 0 has no effect. After exiting Standby or Shutdown mode, or after a reset, this bit is at 0 and the MSIS and MSIK ranges are provided by MSISSRANGE\\[3:0\\]
and MSIKSRANGE\\[3:0\\]
in RCC_CSR."]
        #[inline(always)]
        pub fn set_msirgsel(&mut self, val: super::vals::Msirgsel) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
        }
        #[doc = "MSIK clock ranges These bits are configured by software to choose the frequency range of MSIK oscillator when MSIRGSEL is set. 16 frequency ranges are available: Note: MSIKRANGE can be modified when MSIK is OFF (MSISON = 0) or when MSIK is ready (MSIKRDY = 1). MSIKRANGE must NOT be modified when MSIK is ON and NOT ready (MSIKON = 1 and MSIKRDY = 0) MSIKRANGE is kept when the device wakes up from Stop mode, except when the MSIK range is above 24 MHz. In this case MSIKRANGE is changed by hardware into Range 2 (24 MHz)."]
        #[inline(always)]
        pub const fn msikrange(&self) -> super::vals::Msirange {
            let val = (self.0 >> 24usize) & 0x0f;
            super::vals::Msirange::from_bits(val as u8)
        }
        #[doc = "MSIK clock ranges These bits are configured by software to choose the frequency range of MSIK oscillator when MSIRGSEL is set. 16 frequency ranges are available: Note: MSIKRANGE can be modified when MSIK is OFF (MSISON = 0) or when MSIK is ready (MSIKRDY = 1). MSIKRANGE must NOT be modified when MSIK is ON and NOT ready (MSIKON = 1 and MSIKRDY = 0) MSIKRANGE is kept when the device wakes up from Stop mode, except when the MSIK range is above 24 MHz. In this case MSIKRANGE is changed by hardware into Range 2 (24 MHz)."]
        #[inline(always)]
        pub fn set_msikrange(&mut self, val: super::vals::Msirange) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
        }
        #[doc = "MSIS clock ranges These bits are configured by software to choose the frequency range of MSIS oscillator when MSIRGSEL is set. 16 frequency ranges are available: Note: MSISRANGE can be modified when MSIS is OFF (MSISON = 0) or when MSIS is ready (MSISRDY = 1). MSISRANGE must NOT be modified when MSIS is ON and NOT ready (MSISON = 1 and MSISRDY = 0) MSISRANGE is kept when the device wakes up from Stop mode, except when the MSIS range is above 24 MHz. In this case MSISRANGE is changed by hardware into Range 2 (24 MHz)."]
        #[inline(always)]
        pub const fn msisrange(&self) -> super::vals::Msirange {
            let val = (self.0 >> 28usize) & 0x0f;
            super::vals::Msirange::from_bits(val as u8)
        }
        #[doc = "MSIS clock ranges These bits are configured by software to choose the frequency range of MSIS oscillator when MSIRGSEL is set. 16 frequency ranges are available: Note: MSISRANGE can be modified when MSIS is OFF (MSISON = 0) or when MSIS is ready (MSISRDY = 1). MSISRANGE must NOT be modified when MSIS is ON and NOT ready (MSISON = 1 and MSISRDY = 0) MSISRANGE is kept when the device wakes up from Stop mode, except when the MSIS range is above 24 MHz. In this case MSISRANGE is changed by hardware into Range 2 (24 MHz)."]
        #[inline(always)]
        pub fn set_msisrange(&mut self, val: super::vals::Msirange) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val.to_bits() as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for Icscr1 {
        #[inline(always)]
        fn default() -> Icscr1 {
            Icscr1(0)
        }
    }
    #[doc = "RCC internal clock sources calibration register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icscr2(pub u32);
    impl Icscr2 {
        #[doc = "MSI clock trimming for ranges 12 to 15 These bits provide an additional user-programmable trimming value that is added to the factory-programmed calibration trim value MSIRC3\\[4:0\\]
bits. It can be programmed to adjust to voltage and temperature variations that influence the frequency of the MSI."]
        #[inline(always)]
        pub const fn msitrim3(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "MSI clock trimming for ranges 12 to 15 These bits provide an additional user-programmable trimming value that is added to the factory-programmed calibration trim value MSIRC3\\[4:0\\]
bits. It can be programmed to adjust to voltage and temperature variations that influence the frequency of the MSI."]
        #[inline(always)]
        pub fn set_msitrim3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "MSI clock trimming for ranges 8 to 11 These bits provide an additional user-programmable trimming value that is added to the factory-programmed calibration trim value MSIRC2\\[4:0\\]
bits. It can be programmed to adjust to voltage and temperature variations that influence the frequency of the MSI."]
        #[inline(always)]
        pub const fn msitrim2(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x1f;
            val as u8
        }
        #[doc = "MSI clock trimming for ranges 8 to 11 These bits provide an additional user-programmable trimming value that is added to the factory-programmed calibration trim value MSIRC2\\[4:0\\]
bits. It can be programmed to adjust to voltage and temperature variations that influence the frequency of the MSI."]
        #[inline(always)]
        pub fn set_msitrim2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 5usize)) | (((val as u32) & 0x1f) << 5usize);
        }
        #[doc = "MSI clock trimming for ranges 4 to 7 These bits provide an additional user-programmable trimming value that is added to the factory-programmed calibration trim value MSIRC1\\[4:0\\]
bits. It can be programmed to adjust to voltage and temperature variations that influence the frequency of the MSI."]
        #[inline(always)]
        pub const fn msitrim1(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x1f;
            val as u8
        }
        #[doc = "MSI clock trimming for ranges 4 to 7 These bits provide an additional user-programmable trimming value that is added to the factory-programmed calibration trim value MSIRC1\\[4:0\\]
bits. It can be programmed to adjust to voltage and temperature variations that influence the frequency of the MSI."]
        #[inline(always)]
        pub fn set_msitrim1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 10usize)) | (((val as u32) & 0x1f) << 10usize);
        }
        #[doc = "MSI clock trimming for ranges 0 to 3 These bits provide an additional user-programmable trimming value that is added to the factory-programmed calibration trim value MSIRC0\\[4:0\\]
bits. It can be programmed to adjust to voltage and temperature variations that influence the frequency of the MSI."]
        #[inline(always)]
        pub const fn msitrim0(&self) -> u8 {
            let val = (self.0 >> 15usize) & 0x1f;
            val as u8
        }
        #[doc = "MSI clock trimming for ranges 0 to 3 These bits provide an additional user-programmable trimming value that is added to the factory-programmed calibration trim value MSIRC0\\[4:0\\]
bits. It can be programmed to adjust to voltage and temperature variations that influence the frequency of the MSI."]
        #[inline(always)]
        pub fn set_msitrim0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 15usize)) | (((val as u32) & 0x1f) << 15usize);
        }
    }
    impl Default for Icscr2 {
        #[inline(always)]
        fn default() -> Icscr2 {
            Icscr2(0)
        }
    }
    #[doc = "RCC internal clock sources calibration register 3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icscr3(pub u32);
    impl Icscr3 {
        #[doc = "HSI clock calibration These bits are initialized at startup with the factory-programmed HSI calibration trim value. When HSITRIM is written, HSICAL is updated with the sum of HSITRIM and the factory trim value."]
        #[inline(always)]
        pub const fn hsical(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "HSI clock calibration These bits are initialized at startup with the factory-programmed HSI calibration trim value. When HSITRIM is written, HSICAL is updated with the sum of HSITRIM and the factory trim value."]
        #[inline(always)]
        pub fn set_hsical(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "HSI clock trimming These bits provide an additional user-programmable trimming value that is added to the HSICAL\\[11:0\\]
bits. It can be programmed to adjust to voltage and temperature variations that influence the frequency of the HSI."]
        #[inline(always)]
        pub const fn hsitrim(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "HSI clock trimming These bits provide an additional user-programmable trimming value that is added to the HSICAL\\[11:0\\]
bits. It can be programmed to adjust to voltage and temperature variations that influence the frequency of the HSI."]
        #[inline(always)]
        pub fn set_hsitrim(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
    }
    impl Default for Icscr3 {
        #[inline(always)]
        fn default() -> Icscr3 {
            Icscr3(0)
        }
    }
    #[doc = "RCC PLL configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pll1cfgr(pub u32);
    impl Pll1cfgr {
        #[doc = "PLL entry clock source Set and cleared by software to select PLL clock source. These bits can be written only when the PLL is disabled. In order to save power, when no PLL is used, the value of PLLSRC must be 0."]
        #[inline(always)]
        pub const fn pllsrc(&self) -> super::vals::Pllsrc {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Pllsrc::from_bits(val as u8)
        }
        #[doc = "PLL entry clock source Set and cleared by software to select PLL clock source. These bits can be written only when the PLL is disabled. In order to save power, when no PLL is used, the value of PLLSRC must be 0."]
        #[inline(always)]
        pub fn set_pllsrc(&mut self, val: super::vals::Pllsrc) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "PLL input frequency range Set and reset by software to select the proper reference frequency range used for PLL. This bit must be written before enabling the PLL. 00-01-10: PLL input (ref1_ck) clock range frequency between 4 and 8 MHz"]
        #[inline(always)]
        pub const fn pllrge(&self) -> super::vals::Pllrge {
            let val = (self.0 >> 2usize) & 0x03;
            super::vals::Pllrge::from_bits(val as u8)
        }
        #[doc = "PLL input frequency range Set and reset by software to select the proper reference frequency range used for PLL. This bit must be written before enabling the PLL. 00-01-10: PLL input (ref1_ck) clock range frequency between 4 and 8 MHz"]
        #[inline(always)]
        pub fn set_pllrge(&mut self, val: super::vals::Pllrge) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
        }
        #[doc = "PLL fractional latch enable Set and reset by software to latch the content of PLLFRACN into the Î£Î modulator. In order to latch the PLLFRACN value into the Î£Î modulator, PLLFRACEN must be set to 0, then set to 1: the transition 0 to 1 transfers the content of PLLFRACN into the modulator (see for details)."]
        #[inline(always)]
        pub const fn pllfracen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "PLL fractional latch enable Set and reset by software to latch the content of PLLFRACN into the Î£Î modulator. In order to latch the PLLFRACN value into the Î£Î modulator, PLLFRACEN must be set to 0, then set to 1: the transition 0 to 1 transfers the content of PLLFRACN into the modulator (see for details)."]
        #[inline(always)]
        pub fn set_pllfracen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Prescaler for PLL Set and cleared by software to configure the prescaler of the PLL. The VCO1 input frequency is PLL input clock frequency/PLLM. This bit can be written only when the PLL is disabled (PLLON = 0 and PLLRDY = 0). ..."]
        #[inline(always)]
        pub const fn pllm(&self) -> super::vals::Pllm {
            let val = (self.0 >> 8usize) & 0x0f;
            super::vals::Pllm::from_bits(val as u8)
        }
        #[doc = "Prescaler for PLL Set and cleared by software to configure the prescaler of the PLL. The VCO1 input frequency is PLL input clock frequency/PLLM. This bit can be written only when the PLL is disabled (PLLON = 0 and PLLRDY = 0). ..."]
        #[inline(always)]
        pub fn set_pllm(&mut self, val: super::vals::Pllm) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
        }
        #[doc = "Prescaler for EPOD booster input clock Set and cleared by software to configure the prescaler of the PLL, used for the EPOD booster. The EPOD booster input frequency is PLL input clock frequency/PLLMBOOST. This bit can be written only when the PLL is disabled (PLLON = 0 and PLLRDY = 0) and EPOD Boost mode is disabled (see ). others: reserved"]
        #[inline(always)]
        pub const fn pllmboost(&self) -> super::vals::Pllmboost {
            let val = (self.0 >> 12usize) & 0x0f;
            super::vals::Pllmboost::from_bits(val as u8)
        }
        #[doc = "Prescaler for EPOD booster input clock Set and cleared by software to configure the prescaler of the PLL, used for the EPOD booster. The EPOD booster input frequency is PLL input clock frequency/PLLMBOOST. This bit can be written only when the PLL is disabled (PLLON = 0 and PLLRDY = 0) and EPOD Boost mode is disabled (see ). others: reserved"]
        #[inline(always)]
        pub fn set_pllmboost(&mut self, val: super::vals::Pllmboost) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
        }
        #[doc = "PLL DIVP divider output enable Set and reset by software to enable the PLL_p_ck output of the PLL. To save power, PLLPEN and PLLP bits must be set to 0 when the PLL_p_ck is not used. This bit can be written only when the PLL is disabled (PLLON = 0 and PLLRDY = 0)."]
        #[inline(always)]
        pub const fn pllpen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "PLL DIVP divider output enable Set and reset by software to enable the PLL_p_ck output of the PLL. To save power, PLLPEN and PLLP bits must be set to 0 when the PLL_p_ck is not used. This bit can be written only when the PLL is disabled (PLLON = 0 and PLLRDY = 0)."]
        #[inline(always)]
        pub fn set_pllpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "PLL DIVQ divider output enable Set and reset by software to enable the PLL_q_ck output of the PLL. To save power, PLLQEN and PLLQ bits must be set to 0 when the PLL_q_ck is not used. This bit can be written only when the PLL is disabled (PLLON = 0 and PLLRDY = 0)."]
        #[inline(always)]
        pub const fn pllqen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "PLL DIVQ divider output enable Set and reset by software to enable the PLL_q_ck output of the PLL. To save power, PLLQEN and PLLQ bits must be set to 0 when the PLL_q_ck is not used. This bit can be written only when the PLL is disabled (PLLON = 0 and PLLRDY = 0)."]
        #[inline(always)]
        pub fn set_pllqen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "PLL DIVR divider output enable Set and reset by software to enable the PLL_r_ck output of the PLL. To save power, PLLRENPLL2REN and PLLR bits must be set to 0 when the PLL_r_ck is not used. This bit can be written only when the PLL is disabled (PLLON = 0 and PLLRDY = 0)."]
        #[inline(always)]
        pub const fn pllren(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "PLL DIVR divider output enable Set and reset by software to enable the PLL_r_ck output of the PLL. To save power, PLLRENPLL2REN and PLLR bits must be set to 0 when the PLL_r_ck is not used. This bit can be written only when the PLL is disabled (PLLON = 0 and PLLRDY = 0)."]
        #[inline(always)]
        pub fn set_pllren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
    }
    impl Default for Pll1cfgr {
        #[inline(always)]
        fn default() -> Pll1cfgr {
            Pll1cfgr(0)
        }
    }
    #[doc = "RCC PLL configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pll23cfgr(pub u32);
    impl Pll23cfgr {
        #[doc = "PLL entry clock source Set and cleared by software to select PLL clock source. These bits can be written only when the PLL is disabled. In order to save power, when no PLL is used, the value of PLLSRC must be 0."]
        #[inline(always)]
        pub const fn pllsrc(&self) -> super::vals::Pllsrc {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Pllsrc::from_bits(val as u8)
        }
        #[doc = "PLL entry clock source Set and cleared by software to select PLL clock source. These bits can be written only when the PLL is disabled. In order to save power, when no PLL is used, the value of PLLSRC must be 0."]
        #[inline(always)]
        pub fn set_pllsrc(&mut self, val: super::vals::Pllsrc) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "PLL input frequency range Set and reset by software to select the proper reference frequency range used for PLL. This bit must be written before enabling the PLL. 00-01-10: PLL input (ref1_ck) clock range frequency between 4 and 8 MHz"]
        #[inline(always)]
        pub const fn pllrge(&self) -> super::vals::Pllrge {
            let val = (self.0 >> 2usize) & 0x03;
            super::vals::Pllrge::from_bits(val as u8)
        }
        #[doc = "PLL input frequency range Set and reset by software to select the proper reference frequency range used for PLL. This bit must be written before enabling the PLL. 00-01-10: PLL input (ref1_ck) clock range frequency between 4 and 8 MHz"]
        #[inline(always)]
        pub fn set_pllrge(&mut self, val: super::vals::Pllrge) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
        }
        #[doc = "PLL fractional latch enable Set and reset by software to latch the content of PLLFRACN into the Î£Î modulator. In order to latch the PLLFRACN value into the Î£Î modulator, PLLFRACEN must be set to 0, then set to 1: the transition 0 to 1 transfers the content of PLLFRACN into the modulator (see for details)."]
        #[inline(always)]
        pub const fn pllfracen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "PLL fractional latch enable Set and reset by software to latch the content of PLLFRACN into the Î£Î modulator. In order to latch the PLLFRACN value into the Î£Î modulator, PLLFRACEN must be set to 0, then set to 1: the transition 0 to 1 transfers the content of PLLFRACN into the modulator (see for details)."]
        #[inline(always)]
        pub fn set_pllfracen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Prescaler for PLL Set and cleared by software to configure the prescaler of the PLL. The VCO1 input frequency is PLL input clock frequency/PLLM. This bit can be written only when the PLL is disabled (PLLON = 0 and PLLRDY = 0). ..."]
        #[inline(always)]
        pub const fn pllm(&self) -> super::vals::Pllm {
            let val = (self.0 >> 8usize) & 0x0f;
            super::vals::Pllm::from_bits(val as u8)
        }
        #[doc = "Prescaler for PLL Set and cleared by software to configure the prescaler of the PLL. The VCO1 input frequency is PLL input clock frequency/PLLM. This bit can be written only when the PLL is disabled (PLLON = 0 and PLLRDY = 0). ..."]
        #[inline(always)]
        pub fn set_pllm(&mut self, val: super::vals::Pllm) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
        }
        #[doc = "PLL DIVP divider output enable Set and reset by software to enable the PLL_p_ck output of the PLL. To save power, PLLPEN and PLLP bits must be set to 0 when the PLL_p_ck is not used. This bit can be written only when the PLL is disabled (PLLON = 0 and PLLRDY = 0)."]
        #[inline(always)]
        pub const fn pllpen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "PLL DIVP divider output enable Set and reset by software to enable the PLL_p_ck output of the PLL. To save power, PLLPEN and PLLP bits must be set to 0 when the PLL_p_ck is not used. This bit can be written only when the PLL is disabled (PLLON = 0 and PLLRDY = 0)."]
        #[inline(always)]
        pub fn set_pllpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "PLL DIVQ divider output enable Set and reset by software to enable the PLL_q_ck output of the PLL. To save power, PLLQEN and PLLQ bits must be set to 0 when the PLL_q_ck is not used. This bit can be written only when the PLL is disabled (PLLON = 0 and PLLRDY = 0)."]
        #[inline(always)]
        pub const fn pllqen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "PLL DIVQ divider output enable Set and reset by software to enable the PLL_q_ck output of the PLL. To save power, PLLQEN and PLLQ bits must be set to 0 when the PLL_q_ck is not used. This bit can be written only when the PLL is disabled (PLLON = 0 and PLLRDY = 0)."]
        #[inline(always)]
        pub fn set_pllqen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "PLL DIVR divider output enable Set and reset by software to enable the PLL_r_ck output of the PLL. To save power, PLLRENPLL2REN and PLLR bits must be set to 0 when the PLL_r_ck is not used. This bit can be written only when the PLL is disabled (PLLON = 0 and PLLRDY = 0)."]
        #[inline(always)]
        pub const fn pllren(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "PLL DIVR divider output enable Set and reset by software to enable the PLL_r_ck output of the PLL. To save power, PLLRENPLL2REN and PLLR bits must be set to 0 when the PLL_r_ck is not used. This bit can be written only when the PLL is disabled (PLLON = 0 and PLLRDY = 0)."]
        #[inline(always)]
        pub fn set_pllren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
    }
    impl Default for Pll23cfgr {
        #[inline(always)]
        fn default() -> Pll23cfgr {
            Pll23cfgr(0)
        }
    }
    #[doc = "RCC PLL1 dividers register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Plldivr(pub u32);
    impl Plldivr {
        #[doc = "Multiplication factor for PLL1 VCO Set and reset by software to control the multiplication factor of the VCO. These bits can be written only when the PLL is disabled (PLL1ON = 0 and PLL1RDY = 0). ... ... Others: reserved VCO output frequency = Fref1_ck x PLL1N, when fractional value 0 has been loaded into PLL1FRACN, with: PLL1N between 4 and 512 input frequency Fref1_ck between 4 and 16 MHz"]
        #[inline(always)]
        pub const fn plln(&self) -> super::vals::Plln {
            let val = (self.0 >> 0usize) & 0x01ff;
            super::vals::Plln::from_bits(val as u16)
        }
        #[doc = "Multiplication factor for PLL1 VCO Set and reset by software to control the multiplication factor of the VCO. These bits can be written only when the PLL is disabled (PLL1ON = 0 and PLL1RDY = 0). ... ... Others: reserved VCO output frequency = Fref1_ck x PLL1N, when fractional value 0 has been loaded into PLL1FRACN, with: PLL1N between 4 and 512 input frequency Fref1_ck between 4 and 16 MHz"]
        #[inline(always)]
        pub fn set_plln(&mut self, val: super::vals::Plln) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val.to_bits() as u32) & 0x01ff) << 0usize);
        }
        #[doc = "PLL1 DIVP division factor Set and reset by software to control the frequency of the pll1_p_ck clock. These bits can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). Note that odd division factors are not allowed. ..."]
        #[inline(always)]
        pub const fn pllp(&self) -> super::vals::Plldiv {
            let val = (self.0 >> 9usize) & 0x7f;
            super::vals::Plldiv::from_bits(val as u8)
        }
        #[doc = "PLL1 DIVP division factor Set and reset by software to control the frequency of the pll1_p_ck clock. These bits can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). Note that odd division factors are not allowed. ..."]
        #[inline(always)]
        pub fn set_pllp(&mut self, val: super::vals::Plldiv) {
            self.0 = (self.0 & !(0x7f << 9usize)) | (((val.to_bits() as u32) & 0x7f) << 9usize);
        }
        #[doc = "PLL1 DIVQ division factor Set and reset by software to control the frequency of the pll1_q_ck clock. These bits can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). ..."]
        #[inline(always)]
        pub const fn pllq(&self) -> super::vals::Plldiv {
            let val = (self.0 >> 16usize) & 0x7f;
            super::vals::Plldiv::from_bits(val as u8)
        }
        #[doc = "PLL1 DIVQ division factor Set and reset by software to control the frequency of the pll1_q_ck clock. These bits can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). ..."]
        #[inline(always)]
        pub fn set_pllq(&mut self, val: super::vals::Plldiv) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val.to_bits() as u32) & 0x7f) << 16usize);
        }
        #[doc = "PLL1 DIVR division factor Set and reset by software to control the frequency of the pll1_r_ck clock. These bits can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). ..."]
        #[inline(always)]
        pub const fn pllr(&self) -> super::vals::Plldiv {
            let val = (self.0 >> 24usize) & 0x7f;
            super::vals::Plldiv::from_bits(val as u8)
        }
        #[doc = "PLL1 DIVR division factor Set and reset by software to control the frequency of the pll1_r_ck clock. These bits can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). ..."]
        #[inline(always)]
        pub fn set_pllr(&mut self, val: super::vals::Plldiv) {
            self.0 = (self.0 & !(0x7f << 24usize)) | (((val.to_bits() as u32) & 0x7f) << 24usize);
        }
    }
    impl Default for Plldivr {
        #[inline(always)]
        fn default() -> Plldivr {
            Plldivr(0)
        }
    }
    #[doc = "RCC PLL1 fractional divider register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pllfracr(pub u32);
    impl Pllfracr {
        #[doc = "Fractional part of the multiplication factor for PLL1 VCO Set and reset by software to control the fractional part of the multiplication factor of the VCO. These bits can be written at any time, allowing dynamic fine-tuning of the PLL1 VCO. VCO output frequency = Fref1_ck x (PLL1N + (PLL1FRACN / 213)), with: PLL1N must be between 4 and 512. PLL1FRACN can be between 0 and 213- 1. The input frequency Fref1_ck must be between 4 and 16 MHz. To change the FRACN value on-the-fly even if the PLL is enabled, the application must proceed as follows: Set the bit PLL1FRACEN to 0. Write the new fractional value into PLL1FRACN. Set the bit PLL1FRACEN to 1."]
        #[inline(always)]
        pub const fn pllfracn(&self) -> u16 {
            let val = (self.0 >> 3usize) & 0x1fff;
            val as u16
        }
        #[doc = "Fractional part of the multiplication factor for PLL1 VCO Set and reset by software to control the fractional part of the multiplication factor of the VCO. These bits can be written at any time, allowing dynamic fine-tuning of the PLL1 VCO. VCO output frequency = Fref1_ck x (PLL1N + (PLL1FRACN / 213)), with: PLL1N must be between 4 and 512. PLL1FRACN can be between 0 and 213- 1. The input frequency Fref1_ck must be between 4 and 16 MHz. To change the FRACN value on-the-fly even if the PLL is enabled, the application must proceed as follows: Set the bit PLL1FRACEN to 0. Write the new fractional value into PLL1FRACN. Set the bit PLL1FRACEN to 1."]
        #[inline(always)]
        pub fn set_pllfracn(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 3usize)) | (((val as u32) & 0x1fff) << 3usize);
        }
    }
    impl Default for Pllfracr {
        #[inline(always)]
        fn default() -> Pllfracr {
            Pllfracr(0)
        }
    }
    #[doc = "RCC privilege configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Privcfgr(pub u32);
    impl Privcfgr {
        #[doc = "RCC secure functions privilege configuration Set and reset by software. This bit can be written only by a secure privileged access."]
        #[inline(always)]
        pub const fn spriv(&self) -> super::vals::Priv {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Priv::from_bits(val as u8)
        }
        #[doc = "RCC secure functions privilege configuration Set and reset by software. This bit can be written only by a secure privileged access."]
        #[inline(always)]
        pub fn set_spriv(&mut self, val: super::vals::Priv) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "RCC non-secure functions privilege configuration Set and reset by software. This bit can be written only by privileged access, secure or non-secure."]
        #[inline(always)]
        pub const fn nspriv(&self) -> super::vals::Priv {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Priv::from_bits(val as u8)
        }
        #[doc = "RCC non-secure functions privilege configuration Set and reset by software. This bit can be written only by privileged access, secure or non-secure."]
        #[inline(always)]
        pub fn set_nspriv(&mut self, val: super::vals::Priv) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Privcfgr {
        #[inline(always)]
        fn default() -> Privcfgr {
            Privcfgr(0)
        }
    }
    #[doc = "RCC secure configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Seccfgr(pub u32);
    impl Seccfgr {
        #[doc = "HSI clock configuration and status bits security Set and reset by software."]
        #[inline(always)]
        pub const fn hsisec(&self) -> super::vals::Security {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Security::from_bits(val as u8)
        }
        #[doc = "HSI clock configuration and status bits security Set and reset by software."]
        #[inline(always)]
        pub fn set_hsisec(&mut self, val: super::vals::Security) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "HSE clock configuration bits, status bits and HSE_CSS security Set and reset by software."]
        #[inline(always)]
        pub const fn hsesec(&self) -> super::vals::Security {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Security::from_bits(val as u8)
        }
        #[doc = "HSE clock configuration bits, status bits and HSE_CSS security Set and reset by software."]
        #[inline(always)]
        pub fn set_hsesec(&mut self, val: super::vals::Security) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "MSI clock configuration and status bits security Set and reset by software."]
        #[inline(always)]
        pub const fn msisec(&self) -> super::vals::Security {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::Security::from_bits(val as u8)
        }
        #[doc = "MSI clock configuration and status bits security Set and reset by software."]
        #[inline(always)]
        pub fn set_msisec(&mut self, val: super::vals::Security) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
        #[doc = "LSI clock configuration and status bits security Set and reset by software."]
        #[inline(always)]
        pub const fn lsisec(&self) -> super::vals::Security {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Security::from_bits(val as u8)
        }
        #[doc = "LSI clock configuration and status bits security Set and reset by software."]
        #[inline(always)]
        pub fn set_lsisec(&mut self, val: super::vals::Security) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "LSE clock configuration and status bits security Set and reset by software."]
        #[inline(always)]
        pub const fn lsesec(&self) -> super::vals::Security {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::Security::from_bits(val as u8)
        }
        #[doc = "LSE clock configuration and status bits security Set and reset by software."]
        #[inline(always)]
        pub fn set_lsesec(&mut self, val: super::vals::Security) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "SYSCLK clock selection, STOPWUCK bit, clock output on MCO configuration security Set and reset by software."]
        #[inline(always)]
        pub const fn sysclksec(&self) -> super::vals::Security {
            let val = (self.0 >> 5usize) & 0x01;
            super::vals::Security::from_bits(val as u8)
        }
        #[doc = "SYSCLK clock selection, STOPWUCK bit, clock output on MCO configuration security Set and reset by software."]
        #[inline(always)]
        pub fn set_sysclksec(&mut self, val: super::vals::Security) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
        }
        #[doc = "AHBx/APBx prescaler configuration bits security Set and reset by software."]
        #[inline(always)]
        pub const fn prescsec(&self) -> super::vals::Security {
            let val = (self.0 >> 6usize) & 0x01;
            super::vals::Security::from_bits(val as u8)
        }
        #[doc = "AHBx/APBx prescaler configuration bits security Set and reset by software."]
        #[inline(always)]
        pub fn set_prescsec(&mut self, val: super::vals::Security) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
        }
        #[doc = "PLL1 clock configuration and status bits security Set and reset by software."]
        #[inline(always)]
        pub const fn pllsec(&self, n: usize) -> super::vals::Security {
            assert!(n < 3usize);
            let offs = 7usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            super::vals::Security::from_bits(val as u8)
        }
        #[doc = "PLL1 clock configuration and status bits security Set and reset by software."]
        #[inline(always)]
        pub fn set_pllsec(&mut self, n: usize, val: super::vals::Security) {
            assert!(n < 3usize);
            let offs = 7usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
        }
        #[doc = "intermediate clock source selection security Set and reset by software."]
        #[inline(always)]
        pub const fn iclksec(&self) -> super::vals::Security {
            let val = (self.0 >> 10usize) & 0x01;
            super::vals::Security::from_bits(val as u8)
        }
        #[doc = "intermediate clock source selection security Set and reset by software."]
        #[inline(always)]
        pub fn set_iclksec(&mut self, val: super::vals::Security) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
        }
        #[doc = "HSI48 clock configuration and status bits security Set and reset by software."]
        #[inline(always)]
        pub const fn hsi48sec(&self) -> super::vals::Security {
            let val = (self.0 >> 11usize) & 0x01;
            super::vals::Security::from_bits(val as u8)
        }
        #[doc = "HSI48 clock configuration and status bits security Set and reset by software."]
        #[inline(always)]
        pub fn set_hsi48sec(&mut self, val: super::vals::Security) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
        }
        #[doc = "Remove reset flag security Set and reset by software."]
        #[inline(always)]
        pub const fn rmvfsec(&self) -> super::vals::Security {
            let val = (self.0 >> 12usize) & 0x01;
            super::vals::Security::from_bits(val as u8)
        }
        #[doc = "Remove reset flag security Set and reset by software."]
        #[inline(always)]
        pub fn set_rmvfsec(&mut self, val: super::vals::Security) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
        }
    }
    impl Default for Seccfgr {
        #[inline(always)]
        fn default() -> Seccfgr {
            Seccfgr(0)
        }
    }
    #[doc = "RCC SmartRun domain peripheral autonomous mode register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Srdamr(pub u32);
    impl Srdamr {
        #[doc = "SPI3 autonomous mode enable in Stop 0,1, 2 mode Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub const fn spi3amen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "SPI3 autonomous mode enable in Stop 0,1, 2 mode Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub fn set_spi3amen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "LPUART1 autonomous mode enable in Stop 0,1, 2 mode Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub const fn lpuart1amen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "LPUART1 autonomous mode enable in Stop 0,1, 2 mode Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub fn set_lpuart1amen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "I2C3 autonomous mode enable in Stop 0,1,2 mode Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub const fn i2c3amen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "I2C3 autonomous mode enable in Stop 0,1,2 mode Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub fn set_i2c3amen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "LPTIM1 autonomous mode enable in Stop 0,1,2 mode Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub const fn lptim1amen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM1 autonomous mode enable in Stop 0,1,2 mode Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub fn set_lptim1amen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "LPTIM3 autonomous mode enable in Stop 0,1,2 mode Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub const fn lptim3amen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM3 autonomous mode enable in Stop 0,1,2 mode Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub fn set_lptim3amen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "LPTIM4 autonomous mode enable in Stop 0,1,2 mode Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub const fn lptim4amen(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM4 autonomous mode enable in Stop 0,1,2 mode Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub fn set_lptim4amen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "OPAMP autonomous mode enable in Stop 0,1,2 mode Set and cleared by software."]
        #[inline(always)]
        pub const fn opampamen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "OPAMP autonomous mode enable in Stop 0,1,2 mode Set and cleared by software."]
        #[inline(always)]
        pub fn set_opampamen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "COMP autonomous mode enable in Stop 0,1,2 mode Set and cleared by software."]
        #[inline(always)]
        pub const fn compamen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "COMP autonomous mode enable in Stop 0,1,2 mode Set and cleared by software."]
        #[inline(always)]
        pub fn set_compamen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "VREFBUF autonomous mode enable in Stop 0,1,2 mode Set and cleared by software."]
        #[inline(always)]
        pub const fn vrefamen(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "VREFBUF autonomous mode enable in Stop 0,1,2 mode Set and cleared by software."]
        #[inline(always)]
        pub fn set_vrefamen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "RTC and TAMP autonomous mode enable in Stop 0,1,2 mode Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub const fn rtcapbamen(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "RTC and TAMP autonomous mode enable in Stop 0,1,2 mode Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub fn set_rtcapbamen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "ADC4 autonomous mode enable in Stop 0,1,2 mode Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub const fn adc4amen(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "ADC4 autonomous mode enable in Stop 0,1,2 mode Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub fn set_adc4amen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "LPGPIO1 autonomous mode enable in Stop 0,1,2 mode Set and cleared by software."]
        #[inline(always)]
        pub const fn lpgpio1amen(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "LPGPIO1 autonomous mode enable in Stop 0,1,2 mode Set and cleared by software."]
        #[inline(always)]
        pub fn set_lpgpio1amen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "DAC1 autonomous mode enable in Stop 0,1,2 mode Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub const fn dac1amen(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "DAC1 autonomous mode enable in Stop 0,1,2 mode Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub fn set_dac1amen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "LPDMA1 autonomous mode enable in Stop 0,1,2 mode Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub const fn lpdma1amen(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "LPDMA1 autonomous mode enable in Stop 0,1,2 mode Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub fn set_lpdma1amen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "ADF1 autonomous mode enable in Stop 0,1,2 mode Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub const fn adf1amen(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "ADF1 autonomous mode enable in Stop 0,1,2 mode Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub fn set_adf1amen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "SRAM4 autonomous mode enable in Stop 0,1,2 mode Set and cleared by software."]
        #[inline(always)]
        pub const fn sram4amen(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM4 autonomous mode enable in Stop 0,1,2 mode Set and cleared by software."]
        #[inline(always)]
        pub fn set_sram4amen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Srdamr {
        #[inline(always)]
        fn default() -> Srdamr {
            Srdamr(0)
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Adcdacsel {
        #[doc = "HCLK clock selected"]
        HCLK1 = 0,
        #[doc = "SYSCLK selected"]
        SYS = 0x01,
        #[doc = "PLL2 R (pll2_r_ck) selected"]
        PLL2_R = 0x02,
        #[doc = "HSE clock selected"]
        HSE = 0x03,
        #[doc = "HSI clock selected"]
        HSI = 0x04,
        #[doc = "MSIK clock selected"]
        MSIK = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Adcdacsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Adcdacsel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Adcdacsel {
        #[inline(always)]
        fn from(val: u8) -> Adcdacsel {
            Adcdacsel::from_bits(val)
        }
    }
    impl From<Adcdacsel> for u8 {
        #[inline(always)]
        fn from(val: Adcdacsel) -> u8 {
            Adcdacsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Adfsel {
        #[doc = "HCLK selected"]
        HCLK1 = 0,
        #[doc = "PLL1 P (pll1_p_ck) selected"]
        PLL1_P = 0x01,
        #[doc = "PLL3 Q (pll3_q_ck) selected"]
        PLL3_Q = 0x02,
        #[doc = "input pin AUDIOCLK selected"]
        AUDIOCLK = 0x03,
        #[doc = "MSIK clock selected"]
        MSIK = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Adfsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Adfsel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Adfsel {
        #[inline(always)]
        fn from(val: u8) -> Adfsel {
            Adfsel::from_bits(val)
        }
    }
    impl From<Adfsel> for u8 {
        #[inline(always)]
        fn from(val: Adfsel) -> u8 {
            Adfsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Dacsel {
        #[doc = "LSE selected"]
        LSE = 0,
        #[doc = "LSI selected"]
        LSI = 0x01,
    }
    impl Dacsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Dacsel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Dacsel {
        #[inline(always)]
        fn from(val: u8) -> Dacsel {
            Dacsel::from_bits(val)
        }
    }
    impl From<Dacsel> for u8 {
        #[inline(always)]
        fn from(val: Dacsel) -> u8 {
            Dacsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Dpre {
        #[doc = "DCLK not divided"]
        DIV1 = 0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        #[doc = "DCLK divided by 2"]
        DIV2 = 0x04,
        #[doc = "DCLK divided by 4"]
        DIV4 = 0x05,
        #[doc = "DCLK divided by 8"]
        DIV8 = 0x06,
        #[doc = "DCLK divided by 16"]
        DIV16 = 0x07,
    }
    impl Dpre {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Dpre {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Dpre {
        #[inline(always)]
        fn from(val: u8) -> Dpre {
            Dpre::from_bits(val)
        }
    }
    impl From<Dpre> for u8 {
        #[inline(always)]
        fn from(val: Dpre) -> u8 {
            Dpre::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Dsisel {
        #[doc = "PLL3 “P” (pll3_p_ck) selected"]
        PLL3_P = 0,
        #[doc = "DSI PHY PLL output selected"]
        DCLK = 0x01,
    }
    impl Dsisel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Dsisel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Dsisel {
        #[inline(always)]
        fn from(val: u8) -> Dsisel {
            Dsisel::from_bits(val)
        }
    }
    impl From<Dsisel> for u8 {
        #[inline(always)]
        fn from(val: Dsisel) -> u8 {
            Dsisel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Fdcansel {
        #[doc = "HSE clock selected"]
        HSE = 0,
        #[doc = "PLL1 Q (pll1_q_ck) selected"]
        PLL1_Q = 0x01,
        #[doc = "PLL2 P (pll2_p_ck) selected"]
        PLL2_P = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Fdcansel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Fdcansel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Fdcansel {
        #[inline(always)]
        fn from(val: u8) -> Fdcansel {
            Fdcansel::from_bits(val)
        }
    }
    impl From<Fdcansel> for u8 {
        #[inline(always)]
        fn from(val: Fdcansel) -> u8 {
            Fdcansel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Hpre {
        #[doc = "SYSCLK not divided"]
        DIV1 = 0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        #[doc = "SYSCLK divided by 2"]
        DIV2 = 0x08,
        #[doc = "SYSCLK divided by 4"]
        DIV4 = 0x09,
        #[doc = "SYSCLK divided by 8"]
        DIV8 = 0x0a,
        #[doc = "SYSCLK divided by 16"]
        DIV16 = 0x0b,
        #[doc = "SYSCLK divided by 64"]
        DIV64 = 0x0c,
        #[doc = "SYSCLK divided by 128"]
        DIV128 = 0x0d,
        #[doc = "SYSCLK divided by 256"]
        DIV256 = 0x0e,
        #[doc = "SYSCLK divided by 512"]
        DIV512 = 0x0f,
    }
    impl Hpre {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Hpre {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Hpre {
        #[inline(always)]
        fn from(val: u8) -> Hpre {
            Hpre::from_bits(val)
        }
    }
    impl From<Hpre> for u8 {
        #[inline(always)]
        fn from(val: Hpre) -> u8 {
            Hpre::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Hseext {
        #[doc = "external HSE clock analog mode"]
        ANALOG = 0,
        #[doc = "external HSE clock digital mode (through I/O Schmitt trigger)"]
        DIGITAL = 0x01,
    }
    impl Hseext {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Hseext {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Hseext {
        #[inline(always)]
        fn from(val: u8) -> Hseext {
            Hseext::from_bits(val)
        }
    }
    impl From<Hseext> for u8 {
        #[inline(always)]
        fn from(val: Hseext) -> u8 {
            Hseext::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Hspisel {
        #[doc = "SYSCLK selected"]
        SYS = 0,
        #[doc = "PLL1 “Q” (pll1_q_ck) selected, can be up to 200 MHz"]
        PLL1_Q = 0x01,
        #[doc = "PLL2 “Q” (pll2_q_ck) selected, can be up to 200 MHz"]
        PLL2_Q = 0x02,
        #[doc = "PLL3 “R” (pll3_r_ck) selected, can be up to 200 MHz"]
        PLL3_R = 0x03,
    }
    impl Hspisel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Hspisel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Hspisel {
        #[inline(always)]
        fn from(val: u8) -> Hspisel {
            Hspisel::from_bits(val)
        }
    }
    impl From<Hspisel> for u8 {
        #[inline(always)]
        fn from(val: Hspisel) -> u8 {
            Hspisel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Iclksel {
        #[doc = "HSI48 clock selected"]
        HSI48 = 0,
        #[doc = "PLL2 Q (pll2_q_ck) selected"]
        PLL2_Q = 0x01,
        #[doc = "PLL1 Q (pll1_q_ck) selected"]
        PLL1_Q = 0x02,
        #[doc = "MSIK clock selected"]
        MSIK = 0x03,
    }
    impl Iclksel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Iclksel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Iclksel {
        #[inline(always)]
        fn from(val: u8) -> Iclksel {
            Iclksel::from_bits(val)
        }
    }
    impl From<Iclksel> for u8 {
        #[inline(always)]
        fn from(val: Iclksel) -> u8 {
            Iclksel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Icsel {
        #[doc = "PCLK1 selected"]
        PCLK1 = 0,
        #[doc = "SYSCLK selected"]
        SYS = 0x01,
        #[doc = "HSI selected"]
        HSI = 0x02,
        #[doc = "MSIK selected"]
        MSIK = 0x03,
    }
    impl Icsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Icsel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Icsel {
        #[inline(always)]
        fn from(val: u8) -> Icsel {
            Icsel::from_bits(val)
        }
    }
    impl From<Icsel> for u8 {
        #[inline(always)]
        fn from(val: Icsel) -> u8 {
            Icsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Lptimsel {
        #[doc = "PCLK1 selected"]
        PCLK1 = 0,
        #[doc = "LSI selected"]
        LSI = 0x01,
        #[doc = "HSI selected"]
        HSI = 0x02,
        #[doc = "LSE selected"]
        LSE = 0x03,
    }
    impl Lptimsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lptimsel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Lptimsel {
        #[inline(always)]
        fn from(val: u8) -> Lptimsel {
            Lptimsel::from_bits(val)
        }
    }
    impl From<Lptimsel> for u8 {
        #[inline(always)]
        fn from(val: Lptimsel) -> u8 {
            Lptimsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Lpuartsel {
        #[doc = "PCLK3 selected"]
        PCLK3 = 0,
        #[doc = "SYSCLK selected"]
        SYS = 0x01,
        #[doc = "HSI selected"]
        HSI = 0x02,
        #[doc = "LSE selected"]
        LSE = 0x03,
        #[doc = "MSIK selected"]
        MSIK = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Lpuartsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lpuartsel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Lpuartsel {
        #[inline(always)]
        fn from(val: u8) -> Lpuartsel {
            Lpuartsel::from_bits(val)
        }
    }
    impl From<Lpuartsel> for u8 {
        #[inline(always)]
        fn from(val: Lpuartsel) -> u8 {
            Lpuartsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Lscosel {
        #[doc = "LSI clock selected"]
        LSI = 0,
        #[doc = "LSE clock selected"]
        LSE = 0x01,
    }
    impl Lscosel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lscosel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Lscosel {
        #[inline(always)]
        fn from(val: u8) -> Lscosel {
            Lscosel::from_bits(val)
        }
    }
    impl From<Lscosel> for u8 {
        #[inline(always)]
        fn from(val: Lscosel) -> u8 {
            Lscosel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Lsedrv {
        #[doc = "Low driving capability"]
        LOW = 0,
        #[doc = "Medium low driving capability"]
        MEDIUMLOW = 0x01,
        #[doc = "Medium high driving capability"]
        MEDIUMHIGH = 0x02,
        #[doc = "High driving capability"]
        HIGH = 0x03,
    }
    impl Lsedrv {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lsedrv {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Lsedrv {
        #[inline(always)]
        fn from(val: u8) -> Lsedrv {
            Lsedrv::from_bits(val)
        }
    }
    impl From<Lsedrv> for u8 {
        #[inline(always)]
        fn from(val: Lsedrv) -> u8 {
            Lsedrv::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Lsiprediv {
        #[doc = "LSI not divided"]
        DIV1 = 0,
        #[doc = "LSI divided by 128"]
        DIV128 = 0x01,
    }
    impl Lsiprediv {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lsiprediv {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Lsiprediv {
        #[inline(always)]
        fn from(val: u8) -> Lsiprediv {
            Lsiprediv::from_bits(val)
        }
    }
    impl From<Lsiprediv> for u8 {
        #[inline(always)]
        fn from(val: Lsiprediv) -> u8 {
            Lsiprediv::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Ltdcsel {
        #[doc = "PLL3 “R” (pll3_r_ck) selected"]
        PLL3_R = 0,
        #[doc = "PLL2 “R” (pll2_r_ck) selected"]
        PLL2_R = 0x01,
    }
    impl Ltdcsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ltdcsel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ltdcsel {
        #[inline(always)]
        fn from(val: u8) -> Ltdcsel {
            Ltdcsel::from_bits(val)
        }
    }
    impl From<Ltdcsel> for u8 {
        #[inline(always)]
        fn from(val: Ltdcsel) -> u8 {
            Ltdcsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Mcopre {
        #[doc = "MCO divided by 1"]
        DIV1 = 0,
        #[doc = "MCO divided by 2"]
        DIV2 = 0x01,
        #[doc = "MCO divided by 4"]
        DIV4 = 0x02,
        #[doc = "MCO divided by 8"]
        DIV8 = 0x03,
        #[doc = "MCO divided by 16"]
        DIV16 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Mcopre {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Mcopre {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Mcopre {
        #[inline(always)]
        fn from(val: u8) -> Mcopre {
            Mcopre::from_bits(val)
        }
    }
    impl From<Mcopre> for u8 {
        #[inline(always)]
        fn from(val: Mcopre) -> u8 {
            Mcopre::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Mcosel {
        #[doc = "MCO output disabled, no clock on MCO"]
        DISABLE = 0,
        #[doc = "SYSCLK system clock selected"]
        SYS = 0x01,
        #[doc = "MSIS clock selected"]
        MSIS = 0x02,
        #[doc = "HSI clock selected"]
        HSI = 0x03,
        #[doc = "HSE clock selected"]
        HSE = 0x04,
        #[doc = "Main PLL clock pll1_r_ck selected"]
        PLL1_R = 0x05,
        #[doc = "LSI clock selected"]
        LSI = 0x06,
        #[doc = "LSE clock selected"]
        LSE = 0x07,
        #[doc = "Internal HSI48 clock selected"]
        HSI48 = 0x08,
        #[doc = "MSIK clock selected"]
        MSIK = 0x09,
        _RESERVED_a = 0x0a,
        _RESERVED_b = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Mcosel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Mcosel {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Mcosel {
        #[inline(always)]
        fn from(val: u8) -> Mcosel {
            Mcosel::from_bits(val)
        }
    }
    impl From<Mcosel> for u8 {
        #[inline(always)]
        fn from(val: Mcosel) -> u8 {
            Mcosel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Mdfsel {
        #[doc = "HCLK selected"]
        HCLK1 = 0,
        #[doc = "PLL1 P (pll1_p_ck) selected"]
        PLL1_P = 0x01,
        #[doc = "PLL3 Q (pll3_q_ck) selected"]
        PLL3_Q = 0x02,
        #[doc = "input pin AUDIOCLK selected"]
        AUDIOCLK = 0x03,
        #[doc = "MSIK clock selected"]
        MSIK = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Mdfsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Mdfsel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Mdfsel {
        #[inline(always)]
        fn from(val: u8) -> Mdfsel {
            Mdfsel::from_bits(val)
        }
    }
    impl From<Mdfsel> for u8 {
        #[inline(always)]
        fn from(val: Mdfsel) -> u8 {
            Mdfsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Msibias {
        #[doc = "MSI bias continuous mode (clock accuracy fast settling time)"]
        CONTINUOUS = 0,
        #[doc = "MSI bias sampling mode (ultra-low-power mode)"]
        SAMPLING = 0x01,
    }
    impl Msibias {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Msibias {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Msibias {
        #[inline(always)]
        fn from(val: u8) -> Msibias {
            Msibias::from_bits(val)
        }
    }
    impl From<Msibias> for u8 {
        #[inline(always)]
        fn from(val: Msibias) -> u8 {
            Msibias::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Msipllfast {
        #[doc = "MSI PLL normal start-up"]
        NORMAL = 0,
        #[doc = "MSI PLL fast start-up"]
        FAST = 0x01,
    }
    impl Msipllfast {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Msipllfast {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Msipllfast {
        #[inline(always)]
        fn from(val: u8) -> Msipllfast {
            Msipllfast::from_bits(val)
        }
    }
    impl From<Msipllfast> for u8 {
        #[inline(always)]
        fn from(val: Msipllfast) -> u8 {
            Msipllfast::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Msipllsel {
        #[doc = "PLL mode applied to MSIK (MSI kernel) clock output"]
        MSIK = 0,
        #[doc = "PLL mode applied to MSIS (MSI system) clock output"]
        MSIS = 0x01,
    }
    impl Msipllsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Msipllsel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Msipllsel {
        #[inline(always)]
        fn from(val: u8) -> Msipllsel {
            Msipllsel::from_bits(val)
        }
    }
    impl From<Msipllsel> for u8 {
        #[inline(always)]
        fn from(val: Msipllsel) -> u8 {
            Msipllsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Msirange {
        #[doc = "range 0 around 48 MHz"]
        RANGE_48MHZ = 0,
        #[doc = "range 1 around 24 MHz"]
        RANGE_24MHZ = 0x01,
        #[doc = "range 2 around 16 MHz"]
        RANGE_16MHZ = 0x02,
        #[doc = "range 3 around 12 MHz"]
        RANGE_12MHZ = 0x03,
        #[doc = "range 4 around 4 MHz (reset value)"]
        RANGE_4MHZ = 0x04,
        #[doc = "range 5 around 2 MHz"]
        RANGE_2MHZ = 0x05,
        #[doc = "range 6 around 1.33 MHz"]
        RANGE_1_33MHZ = 0x06,
        #[doc = "range 7 around 1 MHz"]
        RANGE_1MHZ = 0x07,
        #[doc = "range 8 around 3.072 MHz"]
        RANGE_3_072MHZ = 0x08,
        #[doc = "range 9 around 1.536 MHz"]
        RANGE_1_536MHZ = 0x09,
        #[doc = "range 10 around 1.024 MHz"]
        RANGE_1_024MHZ = 0x0a,
        #[doc = "range 11 around 768 kHz"]
        RANGE_768KHZ = 0x0b,
        #[doc = "range 12 around 400 kHz"]
        RANGE_400KHZ = 0x0c,
        #[doc = "range 13 around 200 kHz"]
        RANGE_200KHZ = 0x0d,
        #[doc = "range 14 around 133 kHz"]
        RANGE_133KHZ = 0x0e,
        #[doc = "range 15 around 100 kHz"]
        RANGE_100KHZ = 0x0f,
    }
    impl Msirange {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Msirange {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Msirange {
        #[inline(always)]
        fn from(val: u8) -> Msirange {
            Msirange::from_bits(val)
        }
    }
    impl From<Msirange> for u8 {
        #[inline(always)]
        fn from(val: Msirange) -> u8 {
            Msirange::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Msirgsel {
        #[doc = "MSIS/MSIK ranges provided by MSISSRANGE\\[3:0\\]
and MSIKSRANGE\\[3:0\\]
in RCC_CSR"]
        CSR = 0,
        #[doc = "MSIS/MSIK ranges provided by MSISRANGE\\[3:0\\]
and MSIKRANGE\\[3:0\\]
in RCC_ICSCR1"]
        ICSCR1 = 0x01,
    }
    impl Msirgsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Msirgsel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Msirgsel {
        #[inline(always)]
        fn from(val: u8) -> Msirgsel {
            Msirgsel::from_bits(val)
        }
    }
    impl From<Msirgsel> for u8 {
        #[inline(always)]
        fn from(val: Msirgsel) -> u8 {
            Msirgsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Msixsrange {
        _RESERVED_0 = 0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        #[doc = "range 4 around 4M Hz (reset value)"]
        RANGE_4MHZ = 0x04,
        #[doc = "range 5 around 2 MHz"]
        RANGE_2MHZ = 0x05,
        #[doc = "range 6 around 1.5 MHz"]
        RANGE_1_5MHZ = 0x06,
        #[doc = "range 7 around 1 MHz"]
        RANGE_1MHZ = 0x07,
        #[doc = "range 8 around 3.072 MHz"]
        RANGE_3_072MHZ = 0x08,
        _RESERVED_9 = 0x09,
        _RESERVED_a = 0x0a,
        _RESERVED_b = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Msixsrange {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Msixsrange {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Msixsrange {
        #[inline(always)]
        fn from(val: u8) -> Msixsrange {
            Msixsrange::from_bits(val)
        }
    }
    impl From<Msixsrange> for u8 {
        #[inline(always)]
        fn from(val: Msixsrange) -> u8 {
            Msixsrange::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Octospisel {
        #[doc = "SYSCLK selected"]
        SYS = 0,
        #[doc = "MSIK selected"]
        MSIK = 0x01,
        #[doc = "PLL1 Q (pll1_q_ck) selected, can be up to 200 MHz"]
        PLL1_Q = 0x02,
        #[doc = "PLL2 Q (pll2_q_ck) selected, can be up to 200 MHz"]
        PLL2_Q = 0x03,
    }
    impl Octospisel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Octospisel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Octospisel {
        #[inline(always)]
        fn from(val: u8) -> Octospisel {
            Octospisel::from_bits(val)
        }
    }
    impl From<Octospisel> for u8 {
        #[inline(always)]
        fn from(val: Octospisel) -> u8 {
            Octospisel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Otghssel {
        #[doc = "HSE selected"]
        HSE = 0,
        #[doc = "PLL1 “P” (pll1_q_ck) selected,"]
        PLL1_P = 0x01,
        #[doc = "HSE/2 selected"]
        HSE_DIV_2 = 0x02,
        #[doc = "PLL1 “P” divided by 2 (pll1_p_ck/2) selected"]
        PLL1_P_DIV_2 = 0x03,
    }
    impl Otghssel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Otghssel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Otghssel {
        #[inline(always)]
        fn from(val: u8) -> Otghssel {
            Otghssel::from_bits(val)
        }
    }
    impl From<Otghssel> for u8 {
        #[inline(always)]
        fn from(val: Otghssel) -> u8 {
            Otghssel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Plldiv {
        DIV1 = 0,
        DIV2 = 0x01,
        DIV3 = 0x02,
        DIV4 = 0x03,
        DIV5 = 0x04,
        DIV6 = 0x05,
        DIV7 = 0x06,
        DIV8 = 0x07,
        DIV9 = 0x08,
        DIV10 = 0x09,
        DIV11 = 0x0a,
        DIV12 = 0x0b,
        DIV13 = 0x0c,
        DIV14 = 0x0d,
        DIV15 = 0x0e,
        DIV16 = 0x0f,
        DIV17 = 0x10,
        DIV18 = 0x11,
        DIV19 = 0x12,
        DIV20 = 0x13,
        DIV21 = 0x14,
        DIV22 = 0x15,
        DIV23 = 0x16,
        DIV24 = 0x17,
        DIV25 = 0x18,
        DIV26 = 0x19,
        DIV27 = 0x1a,
        DIV28 = 0x1b,
        DIV29 = 0x1c,
        DIV30 = 0x1d,
        DIV31 = 0x1e,
        DIV32 = 0x1f,
        DIV33 = 0x20,
        DIV34 = 0x21,
        DIV35 = 0x22,
        DIV36 = 0x23,
        DIV37 = 0x24,
        DIV38 = 0x25,
        DIV39 = 0x26,
        DIV40 = 0x27,
        DIV41 = 0x28,
        DIV42 = 0x29,
        DIV43 = 0x2a,
        DIV44 = 0x2b,
        DIV45 = 0x2c,
        DIV46 = 0x2d,
        DIV47 = 0x2e,
        DIV48 = 0x2f,
        DIV49 = 0x30,
        DIV50 = 0x31,
        DIV51 = 0x32,
        DIV52 = 0x33,
        DIV53 = 0x34,
        DIV54 = 0x35,
        DIV55 = 0x36,
        DIV56 = 0x37,
        DIV57 = 0x38,
        DIV58 = 0x39,
        DIV59 = 0x3a,
        DIV60 = 0x3b,
        DIV61 = 0x3c,
        DIV62 = 0x3d,
        DIV63 = 0x3e,
        DIV64 = 0x3f,
        DIV65 = 0x40,
        DIV66 = 0x41,
        DIV67 = 0x42,
        DIV68 = 0x43,
        DIV69 = 0x44,
        DIV70 = 0x45,
        DIV71 = 0x46,
        DIV72 = 0x47,
        DIV73 = 0x48,
        DIV74 = 0x49,
        DIV75 = 0x4a,
        DIV76 = 0x4b,
        DIV77 = 0x4c,
        DIV78 = 0x4d,
        DIV79 = 0x4e,
        DIV80 = 0x4f,
        DIV81 = 0x50,
        DIV82 = 0x51,
        DIV83 = 0x52,
        DIV84 = 0x53,
        DIV85 = 0x54,
        DIV86 = 0x55,
        DIV87 = 0x56,
        DIV88 = 0x57,
        DIV89 = 0x58,
        DIV90 = 0x59,
        DIV91 = 0x5a,
        DIV92 = 0x5b,
        DIV93 = 0x5c,
        DIV94 = 0x5d,
        DIV95 = 0x5e,
        DIV96 = 0x5f,
        DIV97 = 0x60,
        DIV98 = 0x61,
        DIV99 = 0x62,
        DIV100 = 0x63,
        DIV101 = 0x64,
        DIV102 = 0x65,
        DIV103 = 0x66,
        DIV104 = 0x67,
        DIV105 = 0x68,
        DIV106 = 0x69,
        DIV107 = 0x6a,
        DIV108 = 0x6b,
        DIV109 = 0x6c,
        DIV110 = 0x6d,
        DIV111 = 0x6e,
        DIV112 = 0x6f,
        DIV113 = 0x70,
        DIV114 = 0x71,
        DIV115 = 0x72,
        DIV116 = 0x73,
        DIV117 = 0x74,
        DIV118 = 0x75,
        DIV119 = 0x76,
        DIV120 = 0x77,
        DIV121 = 0x78,
        DIV122 = 0x79,
        DIV123 = 0x7a,
        DIV124 = 0x7b,
        DIV125 = 0x7c,
        DIV126 = 0x7d,
        DIV127 = 0x7e,
        DIV128 = 0x7f,
    }
    impl Plldiv {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Plldiv {
            unsafe { core::mem::transmute(val & 0x7f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Plldiv {
        #[inline(always)]
        fn from(val: u8) -> Plldiv {
            Plldiv::from_bits(val)
        }
    }
    impl From<Plldiv> for u8 {
        #[inline(always)]
        fn from(val: Plldiv) -> u8 {
            Plldiv::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Pllm {
        DIV1 = 0,
        DIV2 = 0x01,
        DIV3 = 0x02,
        DIV4 = 0x03,
        DIV5 = 0x04,
        DIV6 = 0x05,
        DIV7 = 0x06,
        DIV8 = 0x07,
        DIV9 = 0x08,
        DIV10 = 0x09,
        DIV11 = 0x0a,
        DIV12 = 0x0b,
        DIV13 = 0x0c,
        DIV14 = 0x0d,
        DIV15 = 0x0e,
        DIV16 = 0x0f,
    }
    impl Pllm {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pllm {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pllm {
        #[inline(always)]
        fn from(val: u8) -> Pllm {
            Pllm::from_bits(val)
        }
    }
    impl From<Pllm> for u8 {
        #[inline(always)]
        fn from(val: Pllm) -> u8 {
            Pllm::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Pllmboost {
        #[doc = "division by 1 (bypass)"]
        DIV1 = 0,
        #[doc = "division by 2"]
        DIV2 = 0x01,
        #[doc = "division by 4"]
        DIV4 = 0x02,
        #[doc = "division by 6"]
        DIV6 = 0x03,
        #[doc = "division by 8"]
        DIV8 = 0x04,
        #[doc = "division by 10"]
        DIV10 = 0x05,
        #[doc = "division by 12"]
        DIV12 = 0x06,
        #[doc = "division by 14"]
        DIV14 = 0x07,
        #[doc = "division by 16"]
        DIV16 = 0x08,
        _RESERVED_9 = 0x09,
        _RESERVED_a = 0x0a,
        _RESERVED_b = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Pllmboost {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pllmboost {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pllmboost {
        #[inline(always)]
        fn from(val: u8) -> Pllmboost {
            Pllmboost::from_bits(val)
        }
    }
    impl From<Pllmboost> for u8 {
        #[inline(always)]
        fn from(val: Pllmboost) -> u8 {
            Pllmboost::to_bits(val)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Plln(pub u16);
    impl Plln {
        pub const MUL4: Self = Self(0x03);
        pub const MUL5: Self = Self(0x04);
        pub const MUL6: Self = Self(0x05);
        pub const MUL7: Self = Self(0x06);
        pub const MUL8: Self = Self(0x07);
        pub const MUL9: Self = Self(0x08);
        pub const MUL10: Self = Self(0x09);
        pub const MUL11: Self = Self(0x0a);
        pub const MUL12: Self = Self(0x0b);
        pub const MUL13: Self = Self(0x0c);
        pub const MUL14: Self = Self(0x0d);
        pub const MUL15: Self = Self(0x0e);
        pub const MUL16: Self = Self(0x0f);
        pub const MUL17: Self = Self(0x10);
        pub const MUL18: Self = Self(0x11);
        pub const MUL19: Self = Self(0x12);
        pub const MUL20: Self = Self(0x13);
        pub const MUL21: Self = Self(0x14);
        pub const MUL22: Self = Self(0x15);
        pub const MUL23: Self = Self(0x16);
        pub const MUL24: Self = Self(0x17);
        pub const MUL25: Self = Self(0x18);
        pub const MUL26: Self = Self(0x19);
        pub const MUL27: Self = Self(0x1a);
        pub const MUL28: Self = Self(0x1b);
        pub const MUL29: Self = Self(0x1c);
        pub const MUL30: Self = Self(0x1d);
        pub const MUL31: Self = Self(0x1e);
        pub const MUL32: Self = Self(0x1f);
        pub const MUL33: Self = Self(0x20);
        pub const MUL34: Self = Self(0x21);
        pub const MUL35: Self = Self(0x22);
        pub const MUL36: Self = Self(0x23);
        pub const MUL37: Self = Self(0x24);
        pub const MUL38: Self = Self(0x25);
        pub const MUL39: Self = Self(0x26);
        pub const MUL40: Self = Self(0x27);
        pub const MUL41: Self = Self(0x28);
        pub const MUL42: Self = Self(0x29);
        pub const MUL43: Self = Self(0x2a);
        pub const MUL44: Self = Self(0x2b);
        pub const MUL45: Self = Self(0x2c);
        pub const MUL46: Self = Self(0x2d);
        pub const MUL47: Self = Self(0x2e);
        pub const MUL48: Self = Self(0x2f);
        pub const MUL49: Self = Self(0x30);
        pub const MUL50: Self = Self(0x31);
        pub const MUL51: Self = Self(0x32);
        pub const MUL52: Self = Self(0x33);
        pub const MUL53: Self = Self(0x34);
        pub const MUL54: Self = Self(0x35);
        pub const MUL55: Self = Self(0x36);
        pub const MUL56: Self = Self(0x37);
        pub const MUL57: Self = Self(0x38);
        pub const MUL58: Self = Self(0x39);
        pub const MUL59: Self = Self(0x3a);
        pub const MUL60: Self = Self(0x3b);
        pub const MUL61: Self = Self(0x3c);
        pub const MUL62: Self = Self(0x3d);
        pub const MUL63: Self = Self(0x3e);
        pub const MUL64: Self = Self(0x3f);
        pub const MUL65: Self = Self(0x40);
        pub const MUL66: Self = Self(0x41);
        pub const MUL67: Self = Self(0x42);
        pub const MUL68: Self = Self(0x43);
        pub const MUL69: Self = Self(0x44);
        pub const MUL70: Self = Self(0x45);
        pub const MUL71: Self = Self(0x46);
        pub const MUL72: Self = Self(0x47);
        pub const MUL73: Self = Self(0x48);
        pub const MUL74: Self = Self(0x49);
        pub const MUL75: Self = Self(0x4a);
        pub const MUL76: Self = Self(0x4b);
        pub const MUL77: Self = Self(0x4c);
        pub const MUL78: Self = Self(0x4d);
        pub const MUL79: Self = Self(0x4e);
        pub const MUL80: Self = Self(0x4f);
        pub const MUL81: Self = Self(0x50);
        pub const MUL82: Self = Self(0x51);
        pub const MUL83: Self = Self(0x52);
        pub const MUL84: Self = Self(0x53);
        pub const MUL85: Self = Self(0x54);
        pub const MUL86: Self = Self(0x55);
        pub const MUL87: Self = Self(0x56);
        pub const MUL88: Self = Self(0x57);
        pub const MUL89: Self = Self(0x58);
        pub const MUL90: Self = Self(0x59);
        pub const MUL91: Self = Self(0x5a);
        pub const MUL92: Self = Self(0x5b);
        pub const MUL93: Self = Self(0x5c);
        pub const MUL94: Self = Self(0x5d);
        pub const MUL95: Self = Self(0x5e);
        pub const MUL96: Self = Self(0x5f);
        pub const MUL97: Self = Self(0x60);
        pub const MUL98: Self = Self(0x61);
        pub const MUL99: Self = Self(0x62);
        pub const MUL100: Self = Self(0x63);
        pub const MUL101: Self = Self(0x64);
        pub const MUL102: Self = Self(0x65);
        pub const MUL103: Self = Self(0x66);
        pub const MUL104: Self = Self(0x67);
        pub const MUL105: Self = Self(0x68);
        pub const MUL106: Self = Self(0x69);
        pub const MUL107: Self = Self(0x6a);
        pub const MUL108: Self = Self(0x6b);
        pub const MUL109: Self = Self(0x6c);
        pub const MUL110: Self = Self(0x6d);
        pub const MUL111: Self = Self(0x6e);
        pub const MUL112: Self = Self(0x6f);
        pub const MUL113: Self = Self(0x70);
        pub const MUL114: Self = Self(0x71);
        pub const MUL115: Self = Self(0x72);
        pub const MUL116: Self = Self(0x73);
        pub const MUL117: Self = Self(0x74);
        pub const MUL118: Self = Self(0x75);
        pub const MUL119: Self = Self(0x76);
        pub const MUL120: Self = Self(0x77);
        pub const MUL121: Self = Self(0x78);
        pub const MUL122: Self = Self(0x79);
        pub const MUL123: Self = Self(0x7a);
        pub const MUL124: Self = Self(0x7b);
        pub const MUL125: Self = Self(0x7c);
        pub const MUL126: Self = Self(0x7d);
        pub const MUL127: Self = Self(0x7e);
        pub const MUL128: Self = Self(0x7f);
        pub const MUL129: Self = Self(0x80);
        pub const MUL130: Self = Self(0x81);
        pub const MUL131: Self = Self(0x82);
        pub const MUL132: Self = Self(0x83);
        pub const MUL133: Self = Self(0x84);
        pub const MUL134: Self = Self(0x85);
        pub const MUL135: Self = Self(0x86);
        pub const MUL136: Self = Self(0x87);
        pub const MUL137: Self = Self(0x88);
        pub const MUL138: Self = Self(0x89);
        pub const MUL139: Self = Self(0x8a);
        pub const MUL140: Self = Self(0x8b);
        pub const MUL141: Self = Self(0x8c);
        pub const MUL142: Self = Self(0x8d);
        pub const MUL143: Self = Self(0x8e);
        pub const MUL144: Self = Self(0x8f);
        pub const MUL145: Self = Self(0x90);
        pub const MUL146: Self = Self(0x91);
        pub const MUL147: Self = Self(0x92);
        pub const MUL148: Self = Self(0x93);
        pub const MUL149: Self = Self(0x94);
        pub const MUL150: Self = Self(0x95);
        pub const MUL151: Self = Self(0x96);
        pub const MUL152: Self = Self(0x97);
        pub const MUL153: Self = Self(0x98);
        pub const MUL154: Self = Self(0x99);
        pub const MUL155: Self = Self(0x9a);
        pub const MUL156: Self = Self(0x9b);
        pub const MUL157: Self = Self(0x9c);
        pub const MUL158: Self = Self(0x9d);
        pub const MUL159: Self = Self(0x9e);
        pub const MUL160: Self = Self(0x9f);
        pub const MUL161: Self = Self(0xa0);
        pub const MUL162: Self = Self(0xa1);
        pub const MUL163: Self = Self(0xa2);
        pub const MUL164: Self = Self(0xa3);
        pub const MUL165: Self = Self(0xa4);
        pub const MUL166: Self = Self(0xa5);
        pub const MUL167: Self = Self(0xa6);
        pub const MUL168: Self = Self(0xa7);
        pub const MUL169: Self = Self(0xa8);
        pub const MUL170: Self = Self(0xa9);
        pub const MUL171: Self = Self(0xaa);
        pub const MUL172: Self = Self(0xab);
        pub const MUL173: Self = Self(0xac);
        pub const MUL174: Self = Self(0xad);
        pub const MUL175: Self = Self(0xae);
        pub const MUL176: Self = Self(0xaf);
        pub const MUL177: Self = Self(0xb0);
        pub const MUL178: Self = Self(0xb1);
        pub const MUL179: Self = Self(0xb2);
        pub const MUL180: Self = Self(0xb3);
        pub const MUL181: Self = Self(0xb4);
        pub const MUL182: Self = Self(0xb5);
        pub const MUL183: Self = Self(0xb6);
        pub const MUL184: Self = Self(0xb7);
        pub const MUL185: Self = Self(0xb8);
        pub const MUL186: Self = Self(0xb9);
        pub const MUL187: Self = Self(0xba);
        pub const MUL188: Self = Self(0xbb);
        pub const MUL189: Self = Self(0xbc);
        pub const MUL190: Self = Self(0xbd);
        pub const MUL191: Self = Self(0xbe);
        pub const MUL192: Self = Self(0xbf);
        pub const MUL193: Self = Self(0xc0);
        pub const MUL194: Self = Self(0xc1);
        pub const MUL195: Self = Self(0xc2);
        pub const MUL196: Self = Self(0xc3);
        pub const MUL197: Self = Self(0xc4);
        pub const MUL198: Self = Self(0xc5);
        pub const MUL199: Self = Self(0xc6);
        pub const MUL200: Self = Self(0xc7);
        pub const MUL201: Self = Self(0xc8);
        pub const MUL202: Self = Self(0xc9);
        pub const MUL203: Self = Self(0xca);
        pub const MUL204: Self = Self(0xcb);
        pub const MUL205: Self = Self(0xcc);
        pub const MUL206: Self = Self(0xcd);
        pub const MUL207: Self = Self(0xce);
        pub const MUL208: Self = Self(0xcf);
        pub const MUL209: Self = Self(0xd0);
        pub const MUL210: Self = Self(0xd1);
        pub const MUL211: Self = Self(0xd2);
        pub const MUL212: Self = Self(0xd3);
        pub const MUL213: Self = Self(0xd4);
        pub const MUL214: Self = Self(0xd5);
        pub const MUL215: Self = Self(0xd6);
        pub const MUL216: Self = Self(0xd7);
        pub const MUL217: Self = Self(0xd8);
        pub const MUL218: Self = Self(0xd9);
        pub const MUL219: Self = Self(0xda);
        pub const MUL220: Self = Self(0xdb);
        pub const MUL221: Self = Self(0xdc);
        pub const MUL222: Self = Self(0xdd);
        pub const MUL223: Self = Self(0xde);
        pub const MUL224: Self = Self(0xdf);
        pub const MUL225: Self = Self(0xe0);
        pub const MUL226: Self = Self(0xe1);
        pub const MUL227: Self = Self(0xe2);
        pub const MUL228: Self = Self(0xe3);
        pub const MUL229: Self = Self(0xe4);
        pub const MUL230: Self = Self(0xe5);
        pub const MUL231: Self = Self(0xe6);
        pub const MUL232: Self = Self(0xe7);
        pub const MUL233: Self = Self(0xe8);
        pub const MUL234: Self = Self(0xe9);
        pub const MUL235: Self = Self(0xea);
        pub const MUL236: Self = Self(0xeb);
        pub const MUL237: Self = Self(0xec);
        pub const MUL238: Self = Self(0xed);
        pub const MUL239: Self = Self(0xee);
        pub const MUL240: Self = Self(0xef);
        pub const MUL241: Self = Self(0xf0);
        pub const MUL242: Self = Self(0xf1);
        pub const MUL243: Self = Self(0xf2);
        pub const MUL244: Self = Self(0xf3);
        pub const MUL245: Self = Self(0xf4);
        pub const MUL246: Self = Self(0xf5);
        pub const MUL247: Self = Self(0xf6);
        pub const MUL248: Self = Self(0xf7);
        pub const MUL249: Self = Self(0xf8);
        pub const MUL250: Self = Self(0xf9);
        pub const MUL251: Self = Self(0xfa);
        pub const MUL252: Self = Self(0xfb);
        pub const MUL253: Self = Self(0xfc);
        pub const MUL254: Self = Self(0xfd);
        pub const MUL255: Self = Self(0xfe);
        pub const MUL256: Self = Self(0xff);
        pub const MUL257: Self = Self(0x0100);
        pub const MUL258: Self = Self(0x0101);
        pub const MUL259: Self = Self(0x0102);
        pub const MUL260: Self = Self(0x0103);
        pub const MUL261: Self = Self(0x0104);
        pub const MUL262: Self = Self(0x0105);
        pub const MUL263: Self = Self(0x0106);
        pub const MUL264: Self = Self(0x0107);
        pub const MUL265: Self = Self(0x0108);
        pub const MUL266: Self = Self(0x0109);
        pub const MUL267: Self = Self(0x010a);
        pub const MUL268: Self = Self(0x010b);
        pub const MUL269: Self = Self(0x010c);
        pub const MUL270: Self = Self(0x010d);
        pub const MUL271: Self = Self(0x010e);
        pub const MUL272: Self = Self(0x010f);
        pub const MUL273: Self = Self(0x0110);
        pub const MUL274: Self = Self(0x0111);
        pub const MUL275: Self = Self(0x0112);
        pub const MUL276: Self = Self(0x0113);
        pub const MUL277: Self = Self(0x0114);
        pub const MUL278: Self = Self(0x0115);
        pub const MUL279: Self = Self(0x0116);
        pub const MUL280: Self = Self(0x0117);
        pub const MUL281: Self = Self(0x0118);
        pub const MUL282: Self = Self(0x0119);
        pub const MUL283: Self = Self(0x011a);
        pub const MUL284: Self = Self(0x011b);
        pub const MUL285: Self = Self(0x011c);
        pub const MUL286: Self = Self(0x011d);
        pub const MUL287: Self = Self(0x011e);
        pub const MUL288: Self = Self(0x011f);
        pub const MUL289: Self = Self(0x0120);
        pub const MUL290: Self = Self(0x0121);
        pub const MUL291: Self = Self(0x0122);
        pub const MUL292: Self = Self(0x0123);
        pub const MUL293: Self = Self(0x0124);
        pub const MUL294: Self = Self(0x0125);
        pub const MUL295: Self = Self(0x0126);
        pub const MUL296: Self = Self(0x0127);
        pub const MUL297: Self = Self(0x0128);
        pub const MUL298: Self = Self(0x0129);
        pub const MUL299: Self = Self(0x012a);
        pub const MUL300: Self = Self(0x012b);
        pub const MUL301: Self = Self(0x012c);
        pub const MUL302: Self = Self(0x012d);
        pub const MUL303: Self = Self(0x012e);
        pub const MUL304: Self = Self(0x012f);
        pub const MUL305: Self = Self(0x0130);
        pub const MUL306: Self = Self(0x0131);
        pub const MUL307: Self = Self(0x0132);
        pub const MUL308: Self = Self(0x0133);
        pub const MUL309: Self = Self(0x0134);
        pub const MUL310: Self = Self(0x0135);
        pub const MUL311: Self = Self(0x0136);
        pub const MUL312: Self = Self(0x0137);
        pub const MUL313: Self = Self(0x0138);
        pub const MUL314: Self = Self(0x0139);
        pub const MUL315: Self = Self(0x013a);
        pub const MUL316: Self = Self(0x013b);
        pub const MUL317: Self = Self(0x013c);
        pub const MUL318: Self = Self(0x013d);
        pub const MUL319: Self = Self(0x013e);
        pub const MUL320: Self = Self(0x013f);
        pub const MUL321: Self = Self(0x0140);
        pub const MUL322: Self = Self(0x0141);
        pub const MUL323: Self = Self(0x0142);
        pub const MUL324: Self = Self(0x0143);
        pub const MUL325: Self = Self(0x0144);
        pub const MUL326: Self = Self(0x0145);
        pub const MUL327: Self = Self(0x0146);
        pub const MUL328: Self = Self(0x0147);
        pub const MUL329: Self = Self(0x0148);
        pub const MUL330: Self = Self(0x0149);
        pub const MUL331: Self = Self(0x014a);
        pub const MUL332: Self = Self(0x014b);
        pub const MUL333: Self = Self(0x014c);
        pub const MUL334: Self = Self(0x014d);
        pub const MUL335: Self = Self(0x014e);
        pub const MUL336: Self = Self(0x014f);
        pub const MUL337: Self = Self(0x0150);
        pub const MUL338: Self = Self(0x0151);
        pub const MUL339: Self = Self(0x0152);
        pub const MUL340: Self = Self(0x0153);
        pub const MUL341: Self = Self(0x0154);
        pub const MUL342: Self = Self(0x0155);
        pub const MUL343: Self = Self(0x0156);
        pub const MUL344: Self = Self(0x0157);
        pub const MUL345: Self = Self(0x0158);
        pub const MUL346: Self = Self(0x0159);
        pub const MUL347: Self = Self(0x015a);
        pub const MUL348: Self = Self(0x015b);
        pub const MUL349: Self = Self(0x015c);
        pub const MUL350: Self = Self(0x015d);
        pub const MUL351: Self = Self(0x015e);
        pub const MUL352: Self = Self(0x015f);
        pub const MUL353: Self = Self(0x0160);
        pub const MUL354: Self = Self(0x0161);
        pub const MUL355: Self = Self(0x0162);
        pub const MUL356: Self = Self(0x0163);
        pub const MUL357: Self = Self(0x0164);
        pub const MUL358: Self = Self(0x0165);
        pub const MUL359: Self = Self(0x0166);
        pub const MUL360: Self = Self(0x0167);
        pub const MUL361: Self = Self(0x0168);
        pub const MUL362: Self = Self(0x0169);
        pub const MUL363: Self = Self(0x016a);
        pub const MUL364: Self = Self(0x016b);
        pub const MUL365: Self = Self(0x016c);
        pub const MUL366: Self = Self(0x016d);
        pub const MUL367: Self = Self(0x016e);
        pub const MUL368: Self = Self(0x016f);
        pub const MUL369: Self = Self(0x0170);
        pub const MUL370: Self = Self(0x0171);
        pub const MUL371: Self = Self(0x0172);
        pub const MUL372: Self = Self(0x0173);
        pub const MUL373: Self = Self(0x0174);
        pub const MUL374: Self = Self(0x0175);
        pub const MUL375: Self = Self(0x0176);
        pub const MUL376: Self = Self(0x0177);
        pub const MUL377: Self = Self(0x0178);
        pub const MUL378: Self = Self(0x0179);
        pub const MUL379: Self = Self(0x017a);
        pub const MUL380: Self = Self(0x017b);
        pub const MUL381: Self = Self(0x017c);
        pub const MUL382: Self = Self(0x017d);
        pub const MUL383: Self = Self(0x017e);
        pub const MUL384: Self = Self(0x017f);
        pub const MUL385: Self = Self(0x0180);
        pub const MUL386: Self = Self(0x0181);
        pub const MUL387: Self = Self(0x0182);
        pub const MUL388: Self = Self(0x0183);
        pub const MUL389: Self = Self(0x0184);
        pub const MUL390: Self = Self(0x0185);
        pub const MUL391: Self = Self(0x0186);
        pub const MUL392: Self = Self(0x0187);
        pub const MUL393: Self = Self(0x0188);
        pub const MUL394: Self = Self(0x0189);
        pub const MUL395: Self = Self(0x018a);
        pub const MUL396: Self = Self(0x018b);
        pub const MUL397: Self = Self(0x018c);
        pub const MUL398: Self = Self(0x018d);
        pub const MUL399: Self = Self(0x018e);
        pub const MUL400: Self = Self(0x018f);
        pub const MUL401: Self = Self(0x0190);
        pub const MUL402: Self = Self(0x0191);
        pub const MUL403: Self = Self(0x0192);
        pub const MUL404: Self = Self(0x0193);
        pub const MUL405: Self = Self(0x0194);
        pub const MUL406: Self = Self(0x0195);
        pub const MUL407: Self = Self(0x0196);
        pub const MUL408: Self = Self(0x0197);
        pub const MUL409: Self = Self(0x0198);
        pub const MUL410: Self = Self(0x0199);
        pub const MUL411: Self = Self(0x019a);
        pub const MUL412: Self = Self(0x019b);
        pub const MUL413: Self = Self(0x019c);
        pub const MUL414: Self = Self(0x019d);
        pub const MUL415: Self = Self(0x019e);
        pub const MUL416: Self = Self(0x019f);
        pub const MUL417: Self = Self(0x01a0);
        pub const MUL418: Self = Self(0x01a1);
        pub const MUL419: Self = Self(0x01a2);
        pub const MUL420: Self = Self(0x01a3);
        pub const MUL421: Self = Self(0x01a4);
        pub const MUL422: Self = Self(0x01a5);
        pub const MUL423: Self = Self(0x01a6);
        pub const MUL424: Self = Self(0x01a7);
        pub const MUL425: Self = Self(0x01a8);
        pub const MUL426: Self = Self(0x01a9);
        pub const MUL427: Self = Self(0x01aa);
        pub const MUL428: Self = Self(0x01ab);
        pub const MUL429: Self = Self(0x01ac);
        pub const MUL430: Self = Self(0x01ad);
        pub const MUL431: Self = Self(0x01ae);
        pub const MUL432: Self = Self(0x01af);
        pub const MUL433: Self = Self(0x01b0);
        pub const MUL434: Self = Self(0x01b1);
        pub const MUL435: Self = Self(0x01b2);
        pub const MUL436: Self = Self(0x01b3);
        pub const MUL437: Self = Self(0x01b4);
        pub const MUL438: Self = Self(0x01b5);
        pub const MUL439: Self = Self(0x01b6);
        pub const MUL440: Self = Self(0x01b7);
        pub const MUL441: Self = Self(0x01b8);
        pub const MUL442: Self = Self(0x01b9);
        pub const MUL443: Self = Self(0x01ba);
        pub const MUL444: Self = Self(0x01bb);
        pub const MUL445: Self = Self(0x01bc);
        pub const MUL446: Self = Self(0x01bd);
        pub const MUL447: Self = Self(0x01be);
        pub const MUL448: Self = Self(0x01bf);
        pub const MUL449: Self = Self(0x01c0);
        pub const MUL450: Self = Self(0x01c1);
        pub const MUL451: Self = Self(0x01c2);
        pub const MUL452: Self = Self(0x01c3);
        pub const MUL453: Self = Self(0x01c4);
        pub const MUL454: Self = Self(0x01c5);
        pub const MUL455: Self = Self(0x01c6);
        pub const MUL456: Self = Self(0x01c7);
        pub const MUL457: Self = Self(0x01c8);
        pub const MUL458: Self = Self(0x01c9);
        pub const MUL459: Self = Self(0x01ca);
        pub const MUL460: Self = Self(0x01cb);
        pub const MUL461: Self = Self(0x01cc);
        pub const MUL462: Self = Self(0x01cd);
        pub const MUL463: Self = Self(0x01ce);
        pub const MUL464: Self = Self(0x01cf);
        pub const MUL465: Self = Self(0x01d0);
        pub const MUL466: Self = Self(0x01d1);
        pub const MUL467: Self = Self(0x01d2);
        pub const MUL468: Self = Self(0x01d3);
        pub const MUL469: Self = Self(0x01d4);
        pub const MUL470: Self = Self(0x01d5);
        pub const MUL471: Self = Self(0x01d6);
        pub const MUL472: Self = Self(0x01d7);
        pub const MUL473: Self = Self(0x01d8);
        pub const MUL474: Self = Self(0x01d9);
        pub const MUL475: Self = Self(0x01da);
        pub const MUL476: Self = Self(0x01db);
        pub const MUL477: Self = Self(0x01dc);
        pub const MUL478: Self = Self(0x01dd);
        pub const MUL479: Self = Self(0x01de);
        pub const MUL480: Self = Self(0x01df);
        pub const MUL481: Self = Self(0x01e0);
        pub const MUL482: Self = Self(0x01e1);
        pub const MUL483: Self = Self(0x01e2);
        pub const MUL484: Self = Self(0x01e3);
        pub const MUL485: Self = Self(0x01e4);
        pub const MUL486: Self = Self(0x01e5);
        pub const MUL487: Self = Self(0x01e6);
        pub const MUL488: Self = Self(0x01e7);
        pub const MUL489: Self = Self(0x01e8);
        pub const MUL490: Self = Self(0x01e9);
        pub const MUL491: Self = Self(0x01ea);
        pub const MUL492: Self = Self(0x01eb);
        pub const MUL493: Self = Self(0x01ec);
        pub const MUL494: Self = Self(0x01ed);
        pub const MUL495: Self = Self(0x01ee);
        pub const MUL496: Self = Self(0x01ef);
        pub const MUL497: Self = Self(0x01f0);
        pub const MUL498: Self = Self(0x01f1);
        pub const MUL499: Self = Self(0x01f2);
        pub const MUL500: Self = Self(0x01f3);
        pub const MUL501: Self = Self(0x01f4);
        pub const MUL502: Self = Self(0x01f5);
        pub const MUL503: Self = Self(0x01f6);
        pub const MUL504: Self = Self(0x01f7);
        pub const MUL505: Self = Self(0x01f8);
        pub const MUL506: Self = Self(0x01f9);
        pub const MUL507: Self = Self(0x01fa);
        pub const MUL508: Self = Self(0x01fb);
        pub const MUL509: Self = Self(0x01fc);
        pub const MUL510: Self = Self(0x01fd);
        pub const MUL511: Self = Self(0x01fe);
        pub const MUL512: Self = Self(0x01ff);
    }
    impl Plln {
        pub const fn from_bits(val: u16) -> Plln {
            Self(val & 0x01ff)
        }
        pub const fn to_bits(self) -> u16 {
            self.0
        }
    }
    impl From<u16> for Plln {
        #[inline(always)]
        fn from(val: u16) -> Plln {
            Plln::from_bits(val)
        }
    }
    impl From<Plln> for u16 {
        #[inline(always)]
        fn from(val: Plln) -> u16 {
            Plln::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Pllrge {
        #[doc = "PLL2 input (ref2_ck) clock range frequency between 4 and 8 MHz"]
        FREQ_4TO8MHZ = 0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        #[doc = "PLL2 input (ref2_ck) clock range frequency between 8 and 16 MHz"]
        FREQ_8TO16MHZ = 0x03,
    }
    impl Pllrge {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pllrge {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pllrge {
        #[inline(always)]
        fn from(val: u8) -> Pllrge {
            Pllrge::from_bits(val)
        }
    }
    impl From<Pllrge> for u8 {
        #[inline(always)]
        fn from(val: Pllrge) -> u8 {
            Pllrge::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Pllsrc {
        #[doc = "No clock sent to PLL3"]
        DISABLE = 0,
        #[doc = "MSIS clock selected as PLL3 clock entry"]
        MSIS = 0x01,
        #[doc = "HSI clock selected as PLL3 clock entry"]
        HSI = 0x02,
        #[doc = "HSE clock selected as PLL3 clock entry"]
        HSE = 0x03,
    }
    impl Pllsrc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pllsrc {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pllsrc {
        #[inline(always)]
        fn from(val: u8) -> Pllsrc {
            Pllsrc::from_bits(val)
        }
    }
    impl From<Pllsrc> for u8 {
        #[inline(always)]
        fn from(val: Pllsrc) -> u8 {
            Pllsrc::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Ppre {
        #[doc = "HCLK not divided"]
        DIV1 = 0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        #[doc = "HCLK divided by 2"]
        DIV2 = 0x04,
        #[doc = "HCLK divided by 4"]
        DIV4 = 0x05,
        #[doc = "HCLK divided by 8"]
        DIV8 = 0x06,
        #[doc = "HCLK divided by 16"]
        DIV16 = 0x07,
    }
    impl Ppre {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ppre {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ppre {
        #[inline(always)]
        fn from(val: u8) -> Ppre {
            Ppre::from_bits(val)
        }
    }
    impl From<Ppre> for u8 {
        #[inline(always)]
        fn from(val: Ppre) -> u8 {
            Ppre::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Priv {
        #[doc = "Read and write to secure functions can be done by privileged or unprivileged access."]
        UNPRIVILEGED = 0,
        #[doc = "Read and write to secure functions can be done by privileged access only."]
        PRIVILEGED = 0x01,
    }
    impl Priv {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Priv {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Priv {
        #[inline(always)]
        fn from(val: u8) -> Priv {
            Priv::from_bits(val)
        }
    }
    impl From<Priv> for u8 {
        #[inline(always)]
        fn from(val: Priv) -> u8 {
            Priv::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Rngsel {
        #[doc = "HSI48 selected"]
        HSI48 = 0,
        #[doc = "HSI48 / 2 selected, can be used in Range 4"]
        HSI48_DIV_2 = 0x01,
        #[doc = "HSI selected"]
        HSI = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Rngsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Rngsel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Rngsel {
        #[inline(always)]
        fn from(val: u8) -> Rngsel {
            Rngsel::from_bits(val)
        }
    }
    impl From<Rngsel> for u8 {
        #[inline(always)]
        fn from(val: Rngsel) -> u8 {
            Rngsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Rtcsel {
        #[doc = "No clock selected"]
        DISABLE = 0,
        #[doc = "LSE oscillator clock selected"]
        LSE = 0x01,
        #[doc = "LSI oscillator clock selected"]
        LSI = 0x02,
        #[doc = "HSE oscillator clock divided by 32 selected"]
        HSE = 0x03,
    }
    impl Rtcsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Rtcsel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Rtcsel {
        #[inline(always)]
        fn from(val: u8) -> Rtcsel {
            Rtcsel::from_bits(val)
        }
    }
    impl From<Rtcsel> for u8 {
        #[inline(always)]
        fn from(val: Rtcsel) -> u8 {
            Rtcsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Saessel {
        #[doc = "SHSI selected"]
        SHSI = 0,
        #[doc = "SHSI / 2 selected, can be used in Range 4"]
        SHSI_DIV_2 = 0x01,
    }
    impl Saessel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Saessel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Saessel {
        #[inline(always)]
        fn from(val: u8) -> Saessel {
            Saessel::from_bits(val)
        }
    }
    impl From<Saessel> for u8 {
        #[inline(always)]
        fn from(val: Saessel) -> u8 {
            Saessel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Saisel {
        #[doc = "PLL2 P (pll2_p_ck) selected"]
        PLL2_P = 0,
        #[doc = "PLL3 P (pll3_p_ck) selected"]
        PLL3_P = 0x01,
        #[doc = "PLL1 P (pll1_p_ck) selected"]
        PLL1_P = 0x02,
        #[doc = "input pin AUDIOCLK selected"]
        AUDIOCLK = 0x03,
        #[doc = "HSI clock selected"]
        HSI = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Saisel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Saisel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Saisel {
        #[inline(always)]
        fn from(val: u8) -> Saisel {
            Saisel::from_bits(val)
        }
    }
    impl From<Saisel> for u8 {
        #[inline(always)]
        fn from(val: Saisel) -> u8 {
            Saisel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Sdmmcsel {
        #[doc = "ICLK clock selected"]
        ICLK = 0,
        #[doc = "PLL1 P (pll1_p_ck) selected, in case higher than 48 MHz is needed (for SDR50 mode)"]
        PLL1_P = 0x01,
    }
    impl Sdmmcsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Sdmmcsel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Sdmmcsel {
        #[inline(always)]
        fn from(val: u8) -> Sdmmcsel {
            Sdmmcsel::from_bits(val)
        }
    }
    impl From<Sdmmcsel> for u8 {
        #[inline(always)]
        fn from(val: Sdmmcsel) -> u8 {
            Sdmmcsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Security {
        #[doc = "non secure"]
        NON_SECURE = 0,
        #[doc = "secure"]
        SECURE = 0x01,
    }
    impl Security {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Security {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Security {
        #[inline(always)]
        fn from(val: u8) -> Security {
            Security::from_bits(val)
        }
    }
    impl From<Security> for u8 {
        #[inline(always)]
        fn from(val: Security) -> u8 {
            Security::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Spisel {
        #[doc = "PCLK2 selected"]
        PCLK2 = 0,
        #[doc = "SYSCLK selected"]
        SYS = 0x01,
        #[doc = "HSI selected"]
        HSI = 0x02,
        #[doc = "MSIK selected"]
        MSIK = 0x03,
    }
    impl Spisel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Spisel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Spisel {
        #[inline(always)]
        fn from(val: u8) -> Spisel {
            Spisel::from_bits(val)
        }
    }
    impl From<Spisel> for u8 {
        #[inline(always)]
        fn from(val: Spisel) -> u8 {
            Spisel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Stopkerwuck {
        #[doc = "MSIK oscillator automatically enabled when exiting Stop mode"]
        MSIK = 0,
        #[doc = "HSI oscillator automatically enabled when exiting Stop mode"]
        HSI = 0x01,
    }
    impl Stopkerwuck {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Stopkerwuck {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Stopkerwuck {
        #[inline(always)]
        fn from(val: u8) -> Stopkerwuck {
            Stopkerwuck::from_bits(val)
        }
    }
    impl From<Stopkerwuck> for u8 {
        #[inline(always)]
        fn from(val: Stopkerwuck) -> u8 {
            Stopkerwuck::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Stopwuck {
        #[doc = "MSIS oscillator selected as wakeup from stop clock and CSS backup clock"]
        MSIS = 0,
        #[doc = "HSI oscillator selected as wakeup from stop clock and CSS backup clock"]
        HSI = 0x01,
    }
    impl Stopwuck {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Stopwuck {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Stopwuck {
        #[inline(always)]
        fn from(val: u8) -> Stopwuck {
            Stopwuck::from_bits(val)
        }
    }
    impl From<Stopwuck> for u8 {
        #[inline(always)]
        fn from(val: Stopwuck) -> u8 {
            Stopwuck::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Sw {
        #[doc = "MSIS selected as system clock"]
        MSIS = 0,
        #[doc = "HSI selected as system clock"]
        HSI = 0x01,
        #[doc = "HSE selected as system clock"]
        HSE = 0x02,
        #[doc = "PLL pll1_r_ck selected as system clock"]
        PLL1_R = 0x03,
    }
    impl Sw {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Sw {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Sw {
        #[inline(always)]
        fn from(val: u8) -> Sw {
            Sw::from_bits(val)
        }
    }
    impl From<Sw> for u8 {
        #[inline(always)]
        fn from(val: Sw) -> u8 {
            Sw::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Systicksel {
        #[doc = "HCLK/8 selected"]
        HCLK1_DIV_8 = 0,
        #[doc = "LSI selected"]
        LSI = 0x01,
        #[doc = "LSE selected"]
        LSE = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Systicksel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Systicksel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Systicksel {
        #[inline(always)]
        fn from(val: u8) -> Systicksel {
            Systicksel::from_bits(val)
        }
    }
    impl From<Systicksel> for u8 {
        #[inline(always)]
        fn from(val: Systicksel) -> u8 {
            Systicksel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Timicsel {
        #[doc = "No sources can be selected by TIM16, TIM17 and LPTIM2 as internal input capture"]
        DISABLE = 0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        #[doc = "HSI/256, MSIS/1024 and MSIS/4 generated and can be selected by TIM16, TIM17 and LPTIM2 as internal input capture"]
        HSI256_MSIS1024_MSIS4 = 0x04,
        #[doc = "HSI/256, MSIS/1024 and MSIK/4 generated and can be selected by TIM16, TIM17 and LPTIM2 as internal input capture"]
        HSI256_MSIS1024_MSIK4 = 0x05,
        #[doc = "HSI/256, MSIK/1024 and MSIS/4 generated and can be selected by TIM16, TIM17 and LPTIM2 as internal input capture"]
        HSI256_MSIK1024_MSIS4 = 0x06,
        #[doc = "HSI/256, MSIK/1024 and MSIK/4 generated and can be selected by TIM16, TIM17 and LPTIM2 as internal input capture"]
        HSI256_MSIK1024_MSIK4 = 0x07,
    }
    impl Timicsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Timicsel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Timicsel {
        #[inline(always)]
        fn from(val: u8) -> Timicsel {
            Timicsel::from_bits(val)
        }
    }
    impl From<Timicsel> for u8 {
        #[inline(always)]
        fn from(val: Timicsel) -> u8 {
            Timicsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Uartsel {
        #[doc = "PCLK1 selected"]
        PCLK1 = 0,
        #[doc = "SYSCLK selected"]
        SYS = 0x01,
        #[doc = "HSI selected"]
        HSI = 0x02,
        #[doc = "LSE selected"]
        LSE = 0x03,
    }
    impl Uartsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Uartsel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Uartsel {
        #[inline(always)]
        fn from(val: u8) -> Uartsel {
            Uartsel::from_bits(val)
        }
    }
    impl From<Uartsel> for u8 {
        #[inline(always)]
        fn from(val: Uartsel) -> u8 {
            Uartsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Usartsel {
        #[doc = "PCLK2 selected"]
        PCLK2 = 0,
        #[doc = "SYSCLK selected"]
        SYS = 0x01,
        #[doc = "HSI selected"]
        HSI = 0x02,
        #[doc = "LSE selected"]
        LSE = 0x03,
    }
    impl Usartsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Usartsel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Usartsel {
        #[inline(always)]
        fn from(val: u8) -> Usartsel {
            Usartsel::from_bits(val)
        }
    }
    impl From<Usartsel> for u8 {
        #[inline(always)]
        fn from(val: Usartsel) -> u8 {
            Usartsel::to_bits(val)
        }
    }
}
