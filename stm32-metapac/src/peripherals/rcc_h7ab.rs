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
    #[doc = "clock control register"]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "RCC HSI configuration register"]
    #[inline(always)]
    pub const fn hsicfgr(self) -> crate::common::Reg<regs::Hsicfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
    #[doc = "RCC Clock Recovery RC Register"]
    #[inline(always)]
    pub const fn crrcr(self) -> crate::common::Reg<regs::Crrcr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize) as _) }
    }
    #[doc = "RCC CSI configuration register"]
    #[inline(always)]
    pub const fn csicfgr(self) -> crate::common::Reg<regs::Csicfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(12usize) as _) }
    }
    #[doc = "RCC Clock Configuration Register"]
    #[inline(always)]
    pub const fn cfgr(self) -> crate::common::Reg<regs::Cfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize) as _) }
    }
    #[doc = "RCC Domain 1 Clock Configuration Register"]
    #[inline(always)]
    pub const fn d1cfgr(self) -> crate::common::Reg<regs::D1cfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(24usize) as _) }
    }
    #[doc = "RCC Domain 2 Clock Configuration Register"]
    #[inline(always)]
    pub const fn d2cfgr(self) -> crate::common::Reg<regs::D2cfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(28usize) as _) }
    }
    #[doc = "RCC Domain 3 Clock Configuration Register"]
    #[inline(always)]
    pub const fn d3cfgr(self) -> crate::common::Reg<regs::D3cfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(32usize) as _) }
    }
    #[doc = "RCC PLLs Clock Source Selection Register"]
    #[inline(always)]
    pub const fn pllckselr(self) -> crate::common::Reg<regs::Pllckselr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(40usize) as _) }
    }
    #[doc = "RCC PLLs Configuration Register"]
    #[inline(always)]
    pub const fn pllcfgr(self) -> crate::common::Reg<regs::Pllcfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(44usize) as _) }
    }
    #[doc = "RCC PLL1 Dividers Configuration Register"]
    #[inline(always)]
    pub const fn plldivr(self, n: usize) -> crate::common::Reg<regs::Plldivr, crate::common::RW> {
        assert!(n < 3usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(48usize + n * 8usize) as _) }
    }
    #[doc = "RCC PLL1 Fractional Divider Register"]
    #[inline(always)]
    pub const fn pllfracr(self, n: usize) -> crate::common::Reg<regs::Pllfracr, crate::common::RW> {
        assert!(n < 3usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(52usize + n * 8usize) as _) }
    }
    #[doc = "RCC Domain 1 Kernel Clock Configuration Register"]
    #[inline(always)]
    pub const fn d1ccipr(self) -> crate::common::Reg<regs::D1ccipr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(76usize) as _) }
    }
    #[doc = "RCC Domain 2 Kernel Clock Configuration Register"]
    #[inline(always)]
    pub const fn d2ccip1r(self) -> crate::common::Reg<regs::D2ccip1r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(80usize) as _) }
    }
    #[doc = "RCC Domain 2 Kernel Clock Configuration Register"]
    #[inline(always)]
    pub const fn d2ccip2r(self) -> crate::common::Reg<regs::D2ccip2r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(84usize) as _) }
    }
    #[doc = "RCC Domain 3 Kernel Clock Configuration Register"]
    #[inline(always)]
    pub const fn d3ccipr(self) -> crate::common::Reg<regs::D3ccipr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(88usize) as _) }
    }
    #[doc = "RCC Clock Source Interrupt Enable Register"]
    #[inline(always)]
    pub const fn cier(self) -> crate::common::Reg<regs::Cier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(96usize) as _) }
    }
    #[doc = "RCC Clock Source Interrupt Flag Register"]
    #[inline(always)]
    pub const fn cifr(self) -> crate::common::Reg<regs::Cifr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(100usize) as _) }
    }
    #[doc = "RCC Clock Source Interrupt Clear Register"]
    #[inline(always)]
    pub const fn cicr(self) -> crate::common::Reg<regs::Cicr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(104usize) as _) }
    }
    #[doc = "RCC Backup Domain Control Register"]
    #[inline(always)]
    pub const fn bdcr(self) -> crate::common::Reg<regs::Bdcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(112usize) as _) }
    }
    #[doc = "RCC Clock Control and Status Register"]
    #[inline(always)]
    pub const fn csr(self) -> crate::common::Reg<regs::Csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(116usize) as _) }
    }
    #[doc = "RCC AHB3 Reset Register"]
    #[inline(always)]
    pub const fn ahb3rstr(self) -> crate::common::Reg<regs::Ahb3rstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(124usize) as _) }
    }
    #[doc = "RCC AHB1 Peripheral Reset Register"]
    #[inline(always)]
    pub const fn ahb1rstr(self) -> crate::common::Reg<regs::Ahb1rstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(128usize) as _) }
    }
    #[doc = "RCC AHB2 Peripheral Reset Register"]
    #[inline(always)]
    pub const fn ahb2rstr(self) -> crate::common::Reg<regs::Ahb2rstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(132usize) as _) }
    }
    #[doc = "RCC AHB4 Peripheral Reset Register"]
    #[inline(always)]
    pub const fn ahb4rstr(self) -> crate::common::Reg<regs::Ahb4rstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(136usize) as _) }
    }
    #[doc = "RCC APB3 Peripheral Reset Register"]
    #[inline(always)]
    pub const fn apb3rstr(self) -> crate::common::Reg<regs::Apb3rstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(140usize) as _) }
    }
    #[doc = "RCC APB1 Peripheral Reset Register"]
    #[inline(always)]
    pub const fn apb1lrstr(self) -> crate::common::Reg<regs::Apb1lrstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(144usize) as _) }
    }
    #[doc = "RCC APB1 Peripheral Reset Register"]
    #[inline(always)]
    pub const fn apb1hrstr(self) -> crate::common::Reg<regs::Apb1hrstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(148usize) as _) }
    }
    #[doc = "RCC APB2 Peripheral Reset Register"]
    #[inline(always)]
    pub const fn apb2rstr(self) -> crate::common::Reg<regs::Apb2rstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(152usize) as _) }
    }
    #[doc = "RCC APB4 Peripheral Reset Register"]
    #[inline(always)]
    pub const fn apb4rstr(self) -> crate::common::Reg<regs::Apb4rstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(156usize) as _) }
    }
    #[doc = "Global Control Register"]
    #[inline(always)]
    pub const fn gcr(self) -> crate::common::Reg<regs::Gcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(160usize) as _) }
    }
    #[doc = "RCC D3 Autonomous mode Register"]
    #[inline(always)]
    pub const fn d3amr(self) -> crate::common::Reg<regs::D3amr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(168usize) as _) }
    }
    #[doc = "RCC Reset Status Register"]
    #[inline(always)]
    pub const fn rsr(self) -> crate::common::Reg<regs::Rsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(304usize) as _) }
    }
    #[doc = "RCC AHB3 Clock Register"]
    #[inline(always)]
    pub const fn ahb3enr(self) -> crate::common::Reg<regs::Ahb3enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(308usize) as _) }
    }
    #[doc = "RCC AHB1 Clock Register"]
    #[inline(always)]
    pub const fn ahb1enr(self) -> crate::common::Reg<regs::Ahb1enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(312usize) as _) }
    }
    #[doc = "RCC AHB2 Clock Register"]
    #[inline(always)]
    pub const fn ahb2enr(self) -> crate::common::Reg<regs::Ahb2enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(316usize) as _) }
    }
    #[doc = "RCC AHB4 Clock Register"]
    #[inline(always)]
    pub const fn ahb4enr(self) -> crate::common::Reg<regs::Ahb4enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(320usize) as _) }
    }
    #[doc = "RCC APB3 Clock Register"]
    #[inline(always)]
    pub const fn apb3enr(self) -> crate::common::Reg<regs::Apb3enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(324usize) as _) }
    }
    #[doc = "RCC APB1 Clock Register"]
    #[inline(always)]
    pub const fn apb1lenr(self) -> crate::common::Reg<regs::Apb1lenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(328usize) as _) }
    }
    #[doc = "RCC APB1 Clock Register"]
    #[inline(always)]
    pub const fn apb1henr(self) -> crate::common::Reg<regs::Apb1henr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(332usize) as _) }
    }
    #[doc = "RCC APB2 Clock Register"]
    #[inline(always)]
    pub const fn apb2enr(self) -> crate::common::Reg<regs::Apb2enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(336usize) as _) }
    }
    #[doc = "RCC APB4 Clock Register"]
    #[inline(always)]
    pub const fn apb4enr(self) -> crate::common::Reg<regs::Apb4enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(340usize) as _) }
    }
    #[doc = "RCC AHB3 Sleep Clock Register"]
    #[inline(always)]
    pub const fn ahb3lpenr(self) -> crate::common::Reg<regs::Ahb3lpenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(348usize) as _) }
    }
    #[doc = "RCC AHB1 Sleep Clock Register"]
    #[inline(always)]
    pub const fn ahb1lpenr(self) -> crate::common::Reg<regs::Ahb1lpenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(352usize) as _) }
    }
    #[doc = "RCC AHB2 Sleep Clock Register"]
    #[inline(always)]
    pub const fn ahb2lpenr(self) -> crate::common::Reg<regs::Ahb2lpenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(356usize) as _) }
    }
    #[doc = "RCC AHB4 Sleep Clock Register"]
    #[inline(always)]
    pub const fn ahb4lpenr(self) -> crate::common::Reg<regs::Ahb4lpenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(360usize) as _) }
    }
    #[doc = "RCC APB3 Sleep Clock Register"]
    #[inline(always)]
    pub const fn apb3lpenr(self) -> crate::common::Reg<regs::Apb3lpenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(364usize) as _) }
    }
    #[doc = "RCC APB1 Low Sleep Clock Register"]
    #[inline(always)]
    pub const fn apb1llpenr(self) -> crate::common::Reg<regs::Apb1llpenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(368usize) as _) }
    }
    #[doc = "RCC APB1 High Sleep Clock Register"]
    #[inline(always)]
    pub const fn apb1hlpenr(self) -> crate::common::Reg<regs::Apb1hlpenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(372usize) as _) }
    }
    #[doc = "RCC APB2 Sleep Clock Register"]
    #[inline(always)]
    pub const fn apb2lpenr(self) -> crate::common::Reg<regs::Apb2lpenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(376usize) as _) }
    }
    #[doc = "RCC APB4 Sleep Clock Register"]
    #[inline(always)]
    pub const fn apb4lpenr(self) -> crate::common::Reg<regs::Apb4lpenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(380usize) as _) }
    }
}
pub mod regs {
    #[doc = "RCC AHB1 Clock Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb1enr(pub u32);
    impl Ahb1enr {
        #[doc = "DMA1 Clock Enable"]
        #[inline(always)]
        pub const fn dma1en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DMA1 Clock Enable"]
        #[inline(always)]
        pub fn set_dma1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DMA2 Clock Enable"]
        #[inline(always)]
        pub const fn dma2en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "DMA2 Clock Enable"]
        #[inline(always)]
        pub fn set_dma2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "ADC1/2 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn adc12en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "ADC1/2 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_adc12en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "ART Clock Enable"]
        #[inline(always)]
        pub const fn arten(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "ART Clock Enable"]
        #[inline(always)]
        pub fn set_arten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Ethernet MAC bus interface Clock Enable"]
        #[inline(always)]
        pub const fn eth1macen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Ethernet MAC bus interface Clock Enable"]
        #[inline(always)]
        pub fn set_eth1macen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Ethernet Transmission Clock Enable"]
        #[inline(always)]
        pub const fn eth1txen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Ethernet Transmission Clock Enable"]
        #[inline(always)]
        pub fn set_eth1txen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Ethernet Reception Clock Enable"]
        #[inline(always)]
        pub const fn eth1rxen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Ethernet Reception Clock Enable"]
        #[inline(always)]
        pub fn set_eth1rxen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "USB_OTG_HS Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn usb_otg_hsen(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "USB_OTG_HS Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_usb_otg_hsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "USB_OTG_HS ULPI clock enable"]
        #[inline(always)]
        pub const fn usb_otg_hs_ulpien(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "USB_OTG_HS ULPI clock enable"]
        #[inline(always)]
        pub fn set_usb_otg_hs_ulpien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "USB_OTG_FS Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn usb_otg_fsen(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "USB_OTG_FS Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_usb_otg_fsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "USB_OTG_FS ULPI clock enable"]
        #[inline(always)]
        pub const fn usb_otg_fs_ulpien(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "USB_OTG_FS ULPI clock enable"]
        #[inline(always)]
        pub fn set_usb_otg_fs_ulpien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
    }
    impl Default for Ahb1enr {
        #[inline(always)]
        fn default() -> Ahb1enr {
            Ahb1enr(0)
        }
    }
    #[doc = "RCC AHB1 Sleep Clock Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb1lpenr(pub u32);
    impl Ahb1lpenr {
        #[doc = "DMA1 Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn dma1lpen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DMA1 Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_dma1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DMA2 Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn dma2lpen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "DMA2 Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_dma2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "ADC1/2 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn adc12lpen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "ADC1/2 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_adc12lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "ART Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn artlpen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "ART Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_artlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Ethernet MAC bus interface Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn eth1maclpen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Ethernet MAC bus interface Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_eth1maclpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Ethernet Transmission Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn eth1txlpen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Ethernet Transmission Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_eth1txlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Ethernet Reception Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn eth1rxlpen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Ethernet Reception Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_eth1rxlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "USB_OTG_HS peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn usb_otg_hslpen(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "USB_OTG_HS peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_usb_otg_hslpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "USB_PHY1 clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn usb_otg_hs_ulpilpen(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "USB_PHY1 clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_usb_otg_hs_ulpilpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "USB_OTG_FS peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn usb_otg_fslpen(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "USB_OTG_FS peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_usb_otg_fslpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "USB_PHY2 clocks enable during CSleep mode"]
        #[inline(always)]
        pub const fn usb_otg_fs_ulpilpen(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "USB_PHY2 clocks enable during CSleep mode"]
        #[inline(always)]
        pub fn set_usb_otg_fs_ulpilpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
    }
    impl Default for Ahb1lpenr {
        #[inline(always)]
        fn default() -> Ahb1lpenr {
            Ahb1lpenr(0)
        }
    }
    #[doc = "RCC AHB1 Peripheral Reset Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb1rstr(pub u32);
    impl Ahb1rstr {
        #[doc = "DMA1 block reset"]
        #[inline(always)]
        pub const fn dma1rst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DMA1 block reset"]
        #[inline(always)]
        pub fn set_dma1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DMA2 block reset"]
        #[inline(always)]
        pub const fn dma2rst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "DMA2 block reset"]
        #[inline(always)]
        pub fn set_dma2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "ADC1&2 block reset"]
        #[inline(always)]
        pub const fn adc12rst(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "ADC1&2 block reset"]
        #[inline(always)]
        pub fn set_adc12rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "ART block reset"]
        #[inline(always)]
        pub const fn artrst(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "ART block reset"]
        #[inline(always)]
        pub fn set_artrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "ETH1MAC block reset"]
        #[inline(always)]
        pub const fn eth1macrst(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "ETH1MAC block reset"]
        #[inline(always)]
        pub fn set_eth1macrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "USB_OTG_HS block reset"]
        #[inline(always)]
        pub const fn usb_otg_hsrst(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "USB_OTG_HS block reset"]
        #[inline(always)]
        pub fn set_usb_otg_hsrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "USB_OTG_FS block reset"]
        #[inline(always)]
        pub const fn usb_otg_fsrst(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "USB_OTG_FS block reset"]
        #[inline(always)]
        pub fn set_usb_otg_fsrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
    }
    impl Default for Ahb1rstr {
        #[inline(always)]
        fn default() -> Ahb1rstr {
            Ahb1rstr(0)
        }
    }
    #[doc = "RCC AHB2 Clock Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb2enr(pub u32);
    impl Ahb2enr {
        #[doc = "DCMI peripheral clock"]
        #[inline(always)]
        pub const fn dcmien(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DCMI peripheral clock"]
        #[inline(always)]
        pub fn set_dcmien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "CRYPT peripheral clock enable"]
        #[inline(always)]
        pub const fn crypten(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "CRYPT peripheral clock enable"]
        #[inline(always)]
        pub fn set_crypten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "HASH peripheral clock enable"]
        #[inline(always)]
        pub const fn hashen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "HASH peripheral clock enable"]
        #[inline(always)]
        pub fn set_hashen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "RNG peripheral clocks enable"]
        #[inline(always)]
        pub const fn rngen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "RNG peripheral clocks enable"]
        #[inline(always)]
        pub fn set_rngen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "SDMMC2 and SDMMC2 delay clock enable"]
        #[inline(always)]
        pub const fn sdmmc2en(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "SDMMC2 and SDMMC2 delay clock enable"]
        #[inline(always)]
        pub fn set_sdmmc2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "BDMA1 clock enable"]
        #[inline(always)]
        pub const fn bdma1en(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "BDMA1 clock enable"]
        #[inline(always)]
        pub fn set_bdma1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "FMAC enable"]
        #[inline(always)]
        pub const fn fmacen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "FMAC enable"]
        #[inline(always)]
        pub fn set_fmacen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "CORDIC enable"]
        #[inline(always)]
        pub const fn cordicen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "CORDIC enable"]
        #[inline(always)]
        pub fn set_cordicen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "SRAM1 block enable"]
        #[inline(always)]
        pub const fn sram1en(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM1 block enable"]
        #[inline(always)]
        pub fn set_sram1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "SRAM2 block enable"]
        #[inline(always)]
        pub const fn sram2en(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 block enable"]
        #[inline(always)]
        pub fn set_sram2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "SRAM3 block enable"]
        #[inline(always)]
        pub const fn sram3en(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM3 block enable"]
        #[inline(always)]
        pub fn set_sram3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ahb2enr {
        #[inline(always)]
        fn default() -> Ahb2enr {
            Ahb2enr(0)
        }
    }
    #[doc = "RCC AHB2 Sleep Clock Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb2lpenr(pub u32);
    impl Ahb2lpenr {
        #[doc = "DCMI peripheral clock enable during csleep mode"]
        #[inline(always)]
        pub const fn dcmilpen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DCMI peripheral clock enable during csleep mode"]
        #[inline(always)]
        pub fn set_dcmilpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "CRYPT peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn cryptlpen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "CRYPT peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_cryptlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "HASH peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn hashlpen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "HASH peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_hashlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "RNG peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn rnglpen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "RNG peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_rnglpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "SDMMC2 and SDMMC2 Delay Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn sdmmc2lpen(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "SDMMC2 and SDMMC2 Delay Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_sdmmc2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "BDMA1 Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn bdma1lpen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "BDMA1 Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_bdma1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "FMAC enable during CSleep Mode"]
        #[inline(always)]
        pub const fn fmaclpen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "FMAC enable during CSleep Mode"]
        #[inline(always)]
        pub fn set_fmaclpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "CORDIC enable during CSleep Mode"]
        #[inline(always)]
        pub const fn cordiclpen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "CORDIC enable during CSleep Mode"]
        #[inline(always)]
        pub fn set_cordiclpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "SRAM1 Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn sram1lpen(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM1 Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_sram1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "SRAM2 Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn sram2lpen(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_sram2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "SRAM3 Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn sram3lpen(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM3 Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_sram3lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ahb2lpenr {
        #[inline(always)]
        fn default() -> Ahb2lpenr {
            Ahb2lpenr(0)
        }
    }
    #[doc = "RCC AHB2 Peripheral Reset Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb2rstr(pub u32);
    impl Ahb2rstr {
        #[doc = "DCMI block reset"]
        #[inline(always)]
        pub const fn dcmirst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DCMI block reset"]
        #[inline(always)]
        pub fn set_dcmirst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Cryptography block reset"]
        #[inline(always)]
        pub const fn cryptrst(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Cryptography block reset"]
        #[inline(always)]
        pub fn set_cryptrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Hash block reset"]
        #[inline(always)]
        pub const fn hashrst(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Hash block reset"]
        #[inline(always)]
        pub fn set_hashrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Random Number Generator block reset"]
        #[inline(always)]
        pub const fn rngrst(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Random Number Generator block reset"]
        #[inline(always)]
        pub fn set_rngrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "SDMMC2 and SDMMC2 Delay block reset"]
        #[inline(always)]
        pub const fn sdmmc2rst(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "SDMMC2 and SDMMC2 Delay block reset"]
        #[inline(always)]
        pub fn set_sdmmc2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "BDMA1 block reset"]
        #[inline(always)]
        pub const fn bdma1rst(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "BDMA1 block reset"]
        #[inline(always)]
        pub fn set_bdma1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "FMAC reset"]
        #[inline(always)]
        pub const fn fmacrst(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "FMAC reset"]
        #[inline(always)]
        pub fn set_fmacrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "CORDIC reset"]
        #[inline(always)]
        pub const fn cordicrst(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "CORDIC reset"]
        #[inline(always)]
        pub fn set_cordicrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Ahb2rstr {
        #[inline(always)]
        fn default() -> Ahb2rstr {
            Ahb2rstr(0)
        }
    }
    #[doc = "RCC AHB3 Clock Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb3enr(pub u32);
    impl Ahb3enr {
        #[doc = "MDMA Peripheral Clock Enable"]
        #[inline(always)]
        pub const fn mdmaen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "MDMA Peripheral Clock Enable"]
        #[inline(always)]
        pub fn set_mdmaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DMA2D Peripheral Clock Enable"]
        #[inline(always)]
        pub const fn dma2den(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "DMA2D Peripheral Clock Enable"]
        #[inline(always)]
        pub fn set_dma2den(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "JPGDEC Peripheral Clock Enable"]
        #[inline(always)]
        pub const fn jpgdecen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "JPGDEC Peripheral Clock Enable"]
        #[inline(always)]
        pub fn set_jpgdecen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "FMC Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn fmcen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "FMC Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_fmcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "QUADSPI and QUADSPI Delay Clock Enable"]
        #[inline(always)]
        pub const fn quadspien(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "QUADSPI and QUADSPI Delay Clock Enable"]
        #[inline(always)]
        pub fn set_quadspien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "SDMMC1 and SDMMC1 Delay Clock Enable"]
        #[inline(always)]
        pub const fn sdmmc1en(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "SDMMC1 and SDMMC1 Delay Clock Enable"]
        #[inline(always)]
        pub fn set_sdmmc1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "OCTOSPI2 and OCTOSPI2 delay block enable"]
        #[inline(always)]
        pub const fn octospi2en(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "OCTOSPI2 and OCTOSPI2 delay block enable"]
        #[inline(always)]
        pub fn set_octospi2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "OCTOSPI IO manager enable"]
        #[inline(always)]
        pub const fn iomngren(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "OCTOSPI IO manager enable"]
        #[inline(always)]
        pub fn set_iomngren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "OTFDEC1 enable"]
        #[inline(always)]
        pub const fn otfd1en(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "OTFDEC1 enable"]
        #[inline(always)]
        pub fn set_otfd1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "OTFDEC2 enable"]
        #[inline(always)]
        pub const fn otfd2en(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "OTFDEC2 enable"]
        #[inline(always)]
        pub fn set_otfd2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "D1 DTCM1 block enable"]
        #[inline(always)]
        pub const fn dtcm1en(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "D1 DTCM1 block enable"]
        #[inline(always)]
        pub fn set_dtcm1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "D1 DTCM2 block enable"]
        #[inline(always)]
        pub const fn dtcm2en(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "D1 DTCM2 block enable"]
        #[inline(always)]
        pub fn set_dtcm2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "D1 ITCM block enable"]
        #[inline(always)]
        pub const fn itcm1en(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "D1 ITCM block enable"]
        #[inline(always)]
        pub fn set_itcm1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "AXISRAM block enable"]
        #[inline(always)]
        pub const fn axisramen(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "AXISRAM block enable"]
        #[inline(always)]
        pub fn set_axisramen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ahb3enr {
        #[inline(always)]
        fn default() -> Ahb3enr {
            Ahb3enr(0)
        }
    }
    #[doc = "RCC AHB3 Sleep Clock Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb3lpenr(pub u32);
    impl Ahb3lpenr {
        #[doc = "MDMA Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn mdmalpen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "MDMA Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_mdmalpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DMA2D Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn dma2dlpen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "DMA2D Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_dma2dlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "JPGDEC Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn jpgdeclpen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "JPGDEC Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_jpgdeclpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "FLASH Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn flashlpen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "FLASH Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_flashlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "FMC Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn fmclpen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "FMC Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_fmclpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "QUADSPI and QUADSPI Delay Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn qspilpen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "QUADSPI and QUADSPI Delay Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_qspilpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "SDMMC1 and SDMMC1 Delay Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn sdmmc1lpen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "SDMMC1 and SDMMC1 Delay Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_sdmmc1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "OCTOSPI2 and OCTOSPI2 delay block enable during CSleep Mode"]
        #[inline(always)]
        pub const fn octospi2lpen(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "OCTOSPI2 and OCTOSPI2 delay block enable during CSleep Mode"]
        #[inline(always)]
        pub fn set_octospi2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "OCTOSPI IO manager enable during CSleep Mode"]
        #[inline(always)]
        pub const fn iomngrlpen(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "OCTOSPI IO manager enable during CSleep Mode"]
        #[inline(always)]
        pub fn set_iomngrlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "OTFDEC1 enable during CSleep Mode"]
        #[inline(always)]
        pub const fn otfd1lpen(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "OTFDEC1 enable during CSleep Mode"]
        #[inline(always)]
        pub fn set_otfd1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "OTFDEC2 enable during CSleep Mode"]
        #[inline(always)]
        pub const fn otfd2lpen(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "OTFDEC2 enable during CSleep Mode"]
        #[inline(always)]
        pub fn set_otfd2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "D1DTCM1 Block Clock Enable During CSleep mode"]
        #[inline(always)]
        pub const fn d1dtcm1lpen(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "D1DTCM1 Block Clock Enable During CSleep mode"]
        #[inline(always)]
        pub fn set_d1dtcm1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "D1 DTCM2 Block Clock Enable During CSleep mode"]
        #[inline(always)]
        pub const fn dtcm2lpen(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "D1 DTCM2 Block Clock Enable During CSleep mode"]
        #[inline(always)]
        pub fn set_dtcm2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "D1ITCM Block Clock Enable During CSleep mode"]
        #[inline(always)]
        pub const fn itcmlpen(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "D1ITCM Block Clock Enable During CSleep mode"]
        #[inline(always)]
        pub fn set_itcmlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "AXISRAM Block Clock Enable During CSleep mode"]
        #[inline(always)]
        pub const fn axisramlpen(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "AXISRAM Block Clock Enable During CSleep mode"]
        #[inline(always)]
        pub fn set_axisramlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ahb3lpenr {
        #[inline(always)]
        fn default() -> Ahb3lpenr {
            Ahb3lpenr(0)
        }
    }
    #[doc = "RCC AHB3 Reset Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb3rstr(pub u32);
    impl Ahb3rstr {
        #[doc = "MDMA block reset"]
        #[inline(always)]
        pub const fn mdmarst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "MDMA block reset"]
        #[inline(always)]
        pub fn set_mdmarst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DMA2D block reset"]
        #[inline(always)]
        pub const fn dma2drst(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "DMA2D block reset"]
        #[inline(always)]
        pub fn set_dma2drst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "JPGDEC block reset"]
        #[inline(always)]
        pub const fn jpgdecrst(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "JPGDEC block reset"]
        #[inline(always)]
        pub fn set_jpgdecrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "FMC block reset"]
        #[inline(always)]
        pub const fn fmcrst(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "FMC block reset"]
        #[inline(always)]
        pub fn set_fmcrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "QUADSPI and QUADSPI delay block reset"]
        #[inline(always)]
        pub const fn qspirst(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "QUADSPI and QUADSPI delay block reset"]
        #[inline(always)]
        pub fn set_qspirst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "SDMMC1 and SDMMC1 delay block reset"]
        #[inline(always)]
        pub const fn sdmmc1rst(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "SDMMC1 and SDMMC1 delay block reset"]
        #[inline(always)]
        pub fn set_sdmmc1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "OCTOSPI2 and OCTOSPI2 delay block reset"]
        #[inline(always)]
        pub const fn octospi2rst(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "OCTOSPI2 and OCTOSPI2 delay block reset"]
        #[inline(always)]
        pub fn set_octospi2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "OCTOSPI IO manager reset"]
        #[inline(always)]
        pub const fn iomngrrst(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "OCTOSPI IO manager reset"]
        #[inline(always)]
        pub fn set_iomngrrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "OTFDEC1 reset"]
        #[inline(always)]
        pub const fn otfd1rst(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "OTFDEC1 reset"]
        #[inline(always)]
        pub fn set_otfd1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "OTFDEC2 reset"]
        #[inline(always)]
        pub const fn otfd2rst(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "OTFDEC2 reset"]
        #[inline(always)]
        pub fn set_otfd2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "CPU reset"]
        #[inline(always)]
        pub const fn cpurst(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "CPU reset"]
        #[inline(always)]
        pub fn set_cpurst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ahb3rstr {
        #[inline(always)]
        fn default() -> Ahb3rstr {
            Ahb3rstr(0)
        }
    }
    #[doc = "RCC AHB4 Clock Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb4enr(pub u32);
    impl Ahb4enr {
        #[doc = "0GPIO peripheral clock enable"]
        #[inline(always)]
        pub const fn gpioaen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[inline(always)]
        pub fn set_gpioaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[inline(always)]
        pub const fn gpioben(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[inline(always)]
        pub fn set_gpioben(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[inline(always)]
        pub const fn gpiocen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[inline(always)]
        pub fn set_gpiocen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[inline(always)]
        pub const fn gpioden(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[inline(always)]
        pub fn set_gpioden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[inline(always)]
        pub const fn gpioeen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[inline(always)]
        pub fn set_gpioeen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[inline(always)]
        pub const fn gpiofen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[inline(always)]
        pub fn set_gpiofen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[inline(always)]
        pub const fn gpiogen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[inline(always)]
        pub fn set_gpiogen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[inline(always)]
        pub const fn gpiohen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[inline(always)]
        pub fn set_gpiohen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[inline(always)]
        pub const fn gpioien(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[inline(always)]
        pub fn set_gpioien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[inline(always)]
        pub const fn gpiojen(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[inline(always)]
        pub fn set_gpiojen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[inline(always)]
        pub const fn gpioken(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[inline(always)]
        pub fn set_gpioken(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "CRC peripheral clock enable"]
        #[inline(always)]
        pub const fn crcen(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "CRC peripheral clock enable"]
        #[inline(always)]
        pub fn set_crcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "BDMA2 and DMAMUX2 Clock Enable"]
        #[inline(always)]
        pub const fn bdma2en(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "BDMA2 and DMAMUX2 Clock Enable"]
        #[inline(always)]
        pub fn set_bdma2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "BDMA and DMAMUX2 Clock Enable"]
        #[inline(always)]
        pub const fn bdmaen(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "BDMA and DMAMUX2 Clock Enable"]
        #[inline(always)]
        pub fn set_bdmaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "ADC3 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn adc3en(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "ADC3 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_adc3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "HSEM peripheral clock enable"]
        #[inline(always)]
        pub const fn hsemen(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "HSEM peripheral clock enable"]
        #[inline(always)]
        pub fn set_hsemen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Backup RAM Clock Enable"]
        #[inline(always)]
        pub const fn bkpsramen(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Backup RAM Clock Enable"]
        #[inline(always)]
        pub fn set_bkpsramen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
    }
    impl Default for Ahb4enr {
        #[inline(always)]
        fn default() -> Ahb4enr {
            Ahb4enr(0)
        }
    }
    #[doc = "RCC AHB4 Sleep Clock Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb4lpenr(pub u32);
    impl Ahb4lpenr {
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn gpioalpen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_gpioalpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn gpioblpen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_gpioblpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn gpioclpen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_gpioclpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn gpiodlpen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_gpiodlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn gpioelpen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_gpioelpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn gpioflpen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_gpioflpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn gpioglpen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_gpioglpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn gpiohlpen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_gpiohlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn gpioilpen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_gpioilpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn gpiojlpen(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_gpiojlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn gpioklpen(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_gpioklpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "CRC peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn crclpen(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "CRC peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_crclpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "BDMA2 Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn bdma2lpen(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "BDMA2 Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_bdma2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "BDMA Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn bdmalpen(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "BDMA Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_bdmalpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "ADC3 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn adc3lpen(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "ADC3 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_adc3lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Backup RAM Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn bkpsramlpen(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Backup RAM Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_bkpsramlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "SRAM4 Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn sram4lpen(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM4 Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_sram4lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
    }
    impl Default for Ahb4lpenr {
        #[inline(always)]
        fn default() -> Ahb4lpenr {
            Ahb4lpenr(0)
        }
    }
    #[doc = "RCC AHB4 Peripheral Reset Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb4rstr(pub u32);
    impl Ahb4rstr {
        #[doc = "GPIO block reset"]
        #[inline(always)]
        pub const fn gpioarst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO block reset"]
        #[inline(always)]
        pub fn set_gpioarst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "GPIO block reset"]
        #[inline(always)]
        pub const fn gpiobrst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO block reset"]
        #[inline(always)]
        pub fn set_gpiobrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "GPIO block reset"]
        #[inline(always)]
        pub const fn gpiocrst(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO block reset"]
        #[inline(always)]
        pub fn set_gpiocrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "GPIO block reset"]
        #[inline(always)]
        pub const fn gpiodrst(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO block reset"]
        #[inline(always)]
        pub fn set_gpiodrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "GPIO block reset"]
        #[inline(always)]
        pub const fn gpioerst(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO block reset"]
        #[inline(always)]
        pub fn set_gpioerst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "GPIO block reset"]
        #[inline(always)]
        pub const fn gpiofrst(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO block reset"]
        #[inline(always)]
        pub fn set_gpiofrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "GPIO block reset"]
        #[inline(always)]
        pub const fn gpiogrst(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO block reset"]
        #[inline(always)]
        pub fn set_gpiogrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "GPIO block reset"]
        #[inline(always)]
        pub const fn gpiohrst(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO block reset"]
        #[inline(always)]
        pub fn set_gpiohrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "GPIO block reset"]
        #[inline(always)]
        pub const fn gpioirst(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO block reset"]
        #[inline(always)]
        pub fn set_gpioirst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "GPIO block reset"]
        #[inline(always)]
        pub const fn gpiojrst(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO block reset"]
        #[inline(always)]
        pub fn set_gpiojrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "GPIO block reset"]
        #[inline(always)]
        pub const fn gpiokrst(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO block reset"]
        #[inline(always)]
        pub fn set_gpiokrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "CRC block reset"]
        #[inline(always)]
        pub const fn crcrst(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "CRC block reset"]
        #[inline(always)]
        pub fn set_crcrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "BDMA2 block reset"]
        #[inline(always)]
        pub const fn bdma2rst(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "BDMA2 block reset"]
        #[inline(always)]
        pub fn set_bdma2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "BDMA block reset"]
        #[inline(always)]
        pub const fn bdmarst(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "BDMA block reset"]
        #[inline(always)]
        pub fn set_bdmarst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "ADC3 block reset"]
        #[inline(always)]
        pub const fn adc3rst(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "ADC3 block reset"]
        #[inline(always)]
        pub fn set_adc3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "HSEM block reset"]
        #[inline(always)]
        pub const fn hsemrst(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "HSEM block reset"]
        #[inline(always)]
        pub fn set_hsemrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
    }
    impl Default for Ahb4rstr {
        #[inline(always)]
        fn default() -> Ahb4rstr {
            Ahb4rstr(0)
        }
    }
    #[doc = "RCC APB1 Clock Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1henr(pub u32);
    impl Apb1henr {
        #[doc = "Clock Recovery System peripheral clock enable"]
        #[inline(always)]
        pub const fn crsen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Clock Recovery System peripheral clock enable"]
        #[inline(always)]
        pub fn set_crsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "SWPMI Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn swpen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "SWPMI Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_swpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "OPAMP peripheral clock enable"]
        #[inline(always)]
        pub const fn opampen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "OPAMP peripheral clock enable"]
        #[inline(always)]
        pub fn set_opampen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "MDIOS peripheral clock enable"]
        #[inline(always)]
        pub const fn mdiosen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "MDIOS peripheral clock enable"]
        #[inline(always)]
        pub fn set_mdiosen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "FDCAN Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn fdcanen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "FDCAN Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_fdcanen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "TIM23 block enable"]
        #[inline(always)]
        pub const fn tim23en(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "TIM23 block enable"]
        #[inline(always)]
        pub fn set_tim23en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "TIM24 block enable"]
        #[inline(always)]
        pub const fn tim24en(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "TIM24 block enable"]
        #[inline(always)]
        pub fn set_tim24en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
    }
    impl Default for Apb1henr {
        #[inline(always)]
        fn default() -> Apb1henr {
            Apb1henr(0)
        }
    }
    #[doc = "RCC APB1 High Sleep Clock Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1hlpenr(pub u32);
    impl Apb1hlpenr {
        #[doc = "Clock Recovery System peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn crslpen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Clock Recovery System peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_crslpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "SWPMI Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn swplpen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "SWPMI Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_swplpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "OPAMP peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn opamplpen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "OPAMP peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_opamplpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "MDIOS peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn mdioslpen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "MDIOS peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_mdioslpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "FDCAN Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn fdcanlpen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "FDCAN Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_fdcanlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "TIM23 block enable during CSleep Mode"]
        #[inline(always)]
        pub const fn tim23lpen(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "TIM23 block enable during CSleep Mode"]
        #[inline(always)]
        pub fn set_tim23lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "TIM24 block enable during CSleep Mode"]
        #[inline(always)]
        pub const fn tim24lpen(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "TIM24 block enable during CSleep Mode"]
        #[inline(always)]
        pub fn set_tim24lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
    }
    impl Default for Apb1hlpenr {
        #[inline(always)]
        fn default() -> Apb1hlpenr {
            Apb1hlpenr(0)
        }
    }
    #[doc = "RCC APB1 Peripheral Reset Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1hrstr(pub u32);
    impl Apb1hrstr {
        #[doc = "Clock Recovery System reset"]
        #[inline(always)]
        pub const fn crsrst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Clock Recovery System reset"]
        #[inline(always)]
        pub fn set_crsrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "SWPMI block reset"]
        #[inline(always)]
        pub const fn swprst(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "SWPMI block reset"]
        #[inline(always)]
        pub fn set_swprst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "OPAMP block reset"]
        #[inline(always)]
        pub const fn opamprst(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "OPAMP block reset"]
        #[inline(always)]
        pub fn set_opamprst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "MDIOS block reset"]
        #[inline(always)]
        pub const fn mdiosrst(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "MDIOS block reset"]
        #[inline(always)]
        pub fn set_mdiosrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "FDCAN block reset"]
        #[inline(always)]
        pub const fn fdcanrst(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "FDCAN block reset"]
        #[inline(always)]
        pub fn set_fdcanrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "TIM23 block reset"]
        #[inline(always)]
        pub const fn tim23rst(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "TIM23 block reset"]
        #[inline(always)]
        pub fn set_tim23rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "TIM24 block reset"]
        #[inline(always)]
        pub const fn tim24rst(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "TIM24 block reset"]
        #[inline(always)]
        pub fn set_tim24rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
    }
    impl Default for Apb1hrstr {
        #[inline(always)]
        fn default() -> Apb1hrstr {
            Apb1hrstr(0)
        }
    }
    #[doc = "RCC APB1 Clock Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1lenr(pub u32);
    impl Apb1lenr {
        #[doc = "TIM peripheral clock enable"]
        #[inline(always)]
        pub const fn tim2en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM peripheral clock enable"]
        #[inline(always)]
        pub fn set_tim2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM peripheral clock enable"]
        #[inline(always)]
        pub const fn tim3en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TIM peripheral clock enable"]
        #[inline(always)]
        pub fn set_tim3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "TIM peripheral clock enable"]
        #[inline(always)]
        pub const fn tim4en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "TIM peripheral clock enable"]
        #[inline(always)]
        pub fn set_tim4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "TIM peripheral clock enable"]
        #[inline(always)]
        pub const fn tim5en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "TIM peripheral clock enable"]
        #[inline(always)]
        pub fn set_tim5en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "TIM peripheral clock enable"]
        #[inline(always)]
        pub const fn tim6en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "TIM peripheral clock enable"]
        #[inline(always)]
        pub fn set_tim6en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "TIM peripheral clock enable"]
        #[inline(always)]
        pub const fn tim7en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "TIM peripheral clock enable"]
        #[inline(always)]
        pub fn set_tim7en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "TIM peripheral clock enable"]
        #[inline(always)]
        pub const fn tim12en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "TIM peripheral clock enable"]
        #[inline(always)]
        pub fn set_tim12en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "TIM peripheral clock enable"]
        #[inline(always)]
        pub const fn tim13en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "TIM peripheral clock enable"]
        #[inline(always)]
        pub fn set_tim13en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "TIM peripheral clock enable"]
        #[inline(always)]
        pub const fn tim14en(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "TIM peripheral clock enable"]
        #[inline(always)]
        pub fn set_tim14en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "LPTIM1 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn lptim1en(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM1 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_lptim1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "WWDG2 peripheral clock enable"]
        #[inline(always)]
        pub const fn wwdg2en(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "WWDG2 peripheral clock enable"]
        #[inline(always)]
        pub fn set_wwdg2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "SPI2 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn spi2en(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "SPI2 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_spi2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "SPI3 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn spi3en(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "SPI3 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_spi3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "SPDIFRX Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn spdifrxen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "SPDIFRX Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_spdifrxen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "USART2 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn usart2en(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "USART2 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_usart2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "USART3 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn usart3en(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "USART3 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_usart3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "UART4 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn uart4en(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "UART4 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_uart4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "UART5 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn uart5en(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "UART5 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_uart5en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "I2C1 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn i2c1en(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_i2c1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "I2C2 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn i2c2en(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "I2C2 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_i2c2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "I2C3 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn i2c3en(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "I2C3 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_i2c3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "I2C5 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn i2c5en(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "I2C5 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_i2c5en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "HDMI-CEC peripheral clock enable"]
        #[inline(always)]
        pub const fn cecen(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "HDMI-CEC peripheral clock enable"]
        #[inline(always)]
        pub fn set_cecen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "DAC1 (containing two converters) peripheral clock enable"]
        #[inline(always)]
        pub const fn dac1en(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "DAC1 (containing two converters) peripheral clock enable"]
        #[inline(always)]
        pub fn set_dac1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "UART7 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn uart7en(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "UART7 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_uart7en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "UART8 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn uart8en(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "UART8 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_uart8en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Apb1lenr {
        #[inline(always)]
        fn default() -> Apb1lenr {
            Apb1lenr(0)
        }
    }
    #[doc = "RCC APB1 Low Sleep Clock Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1llpenr(pub u32);
    impl Apb1llpenr {
        #[doc = "TIM2 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn tim2lpen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM2 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_tim2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM3 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn tim3lpen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TIM3 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_tim3lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "TIM4 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn tim4lpen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "TIM4 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_tim4lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "TIM5 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn tim5lpen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "TIM5 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_tim5lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "TIM6 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn tim6lpen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "TIM6 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_tim6lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "TIM7 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn tim7lpen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "TIM7 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_tim7lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "TIM12 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn tim12lpen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "TIM12 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_tim12lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "TIM13 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn tim13lpen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "TIM13 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_tim13lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "TIM14 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn tim14lpen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "TIM14 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_tim14lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "LPTIM1 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn lptim1lpen(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM1 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_lptim1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "WWDG2 peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn wwdg2lpen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "WWDG2 peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_wwdg2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "SPI2 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn spi2lpen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "SPI2 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_spi2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "SPI3 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn spi3lpen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "SPI3 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_spi3lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "SPDIFRX Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn spdifrxlpen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "SPDIFRX Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_spdifrxlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "USART2 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn usart2lpen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "USART2 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_usart2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "USART3 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn usart3lpen(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "USART3 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_usart3lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "UART4 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn uart4lpen(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "UART4 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_uart4lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "UART5 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn uart5lpen(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "UART5 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_uart5lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "I2C1 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn i2c1lpen(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_i2c1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "I2C2 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn i2c2lpen(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "I2C2 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_i2c2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "I2C3 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn i2c3lpen(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "I2C3 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_i2c3lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "I2C5 block enable during CSleep Mode"]
        #[inline(always)]
        pub const fn i2c5lpen(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "I2C5 block enable during CSleep Mode"]
        #[inline(always)]
        pub fn set_i2c5lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "HDMI-CEC Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn ceclpen(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "HDMI-CEC Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_ceclpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "DAC1 (containing two converters) peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn dac1lpen(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "DAC1 (containing two converters) peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_dac1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "UART7 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn uart7lpen(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "UART7 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_uart7lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "UART8 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn uart8lpen(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "UART8 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_uart8lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Apb1llpenr {
        #[inline(always)]
        fn default() -> Apb1llpenr {
            Apb1llpenr(0)
        }
    }
    #[doc = "RCC APB1 Peripheral Reset Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1lrstr(pub u32);
    impl Apb1lrstr {
        #[doc = "TIM block reset"]
        #[inline(always)]
        pub const fn tim2rst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM block reset"]
        #[inline(always)]
        pub fn set_tim2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM block reset"]
        #[inline(always)]
        pub const fn tim3rst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TIM block reset"]
        #[inline(always)]
        pub fn set_tim3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "TIM block reset"]
        #[inline(always)]
        pub const fn tim4rst(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "TIM block reset"]
        #[inline(always)]
        pub fn set_tim4rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "TIM block reset"]
        #[inline(always)]
        pub const fn tim5rst(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "TIM block reset"]
        #[inline(always)]
        pub fn set_tim5rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "TIM block reset"]
        #[inline(always)]
        pub const fn tim6rst(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "TIM block reset"]
        #[inline(always)]
        pub fn set_tim6rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "TIM block reset"]
        #[inline(always)]
        pub const fn tim7rst(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "TIM block reset"]
        #[inline(always)]
        pub fn set_tim7rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "TIM block reset"]
        #[inline(always)]
        pub const fn tim12rst(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "TIM block reset"]
        #[inline(always)]
        pub fn set_tim12rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "TIM block reset"]
        #[inline(always)]
        pub const fn tim13rst(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "TIM block reset"]
        #[inline(always)]
        pub fn set_tim13rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "TIM block reset"]
        #[inline(always)]
        pub const fn tim14rst(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "TIM block reset"]
        #[inline(always)]
        pub fn set_tim14rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "TIM block reset"]
        #[inline(always)]
        pub const fn lptim1rst(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "TIM block reset"]
        #[inline(always)]
        pub fn set_lptim1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "SPI2 block reset"]
        #[inline(always)]
        pub const fn spi2rst(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "SPI2 block reset"]
        #[inline(always)]
        pub fn set_spi2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "SPI3 block reset"]
        #[inline(always)]
        pub const fn spi3rst(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "SPI3 block reset"]
        #[inline(always)]
        pub fn set_spi3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "SPDIFRX block reset"]
        #[inline(always)]
        pub const fn spdifrxrst(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "SPDIFRX block reset"]
        #[inline(always)]
        pub fn set_spdifrxrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "USART2 block reset"]
        #[inline(always)]
        pub const fn usart2rst(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "USART2 block reset"]
        #[inline(always)]
        pub fn set_usart2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "USART3 block reset"]
        #[inline(always)]
        pub const fn usart3rst(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "USART3 block reset"]
        #[inline(always)]
        pub fn set_usart3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "UART4 block reset"]
        #[inline(always)]
        pub const fn uart4rst(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "UART4 block reset"]
        #[inline(always)]
        pub fn set_uart4rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "UART5 block reset"]
        #[inline(always)]
        pub const fn uart5rst(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "UART5 block reset"]
        #[inline(always)]
        pub fn set_uart5rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "I2C1 block reset"]
        #[inline(always)]
        pub const fn i2c1rst(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1 block reset"]
        #[inline(always)]
        pub fn set_i2c1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "I2C2 block reset"]
        #[inline(always)]
        pub const fn i2c2rst(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "I2C2 block reset"]
        #[inline(always)]
        pub fn set_i2c2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "I2C3 block reset"]
        #[inline(always)]
        pub const fn i2c3rst(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "I2C3 block reset"]
        #[inline(always)]
        pub fn set_i2c3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "I2C5 block reset"]
        #[inline(always)]
        pub const fn i2c5rst(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "I2C5 block reset"]
        #[inline(always)]
        pub fn set_i2c5rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "HDMI-CEC block reset"]
        #[inline(always)]
        pub const fn cecrst(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "HDMI-CEC block reset"]
        #[inline(always)]
        pub fn set_cecrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "DAC1 (containing two converters) reset"]
        #[inline(always)]
        pub const fn dac1rst(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "DAC1 (containing two converters) reset"]
        #[inline(always)]
        pub fn set_dac1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "UART7 block reset"]
        #[inline(always)]
        pub const fn uart7rst(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "UART7 block reset"]
        #[inline(always)]
        pub fn set_uart7rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "UART8 block reset"]
        #[inline(always)]
        pub const fn uart8rst(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "UART8 block reset"]
        #[inline(always)]
        pub fn set_uart8rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Apb1lrstr {
        #[inline(always)]
        fn default() -> Apb1lrstr {
            Apb1lrstr(0)
        }
    }
    #[doc = "RCC APB2 Clock Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb2enr(pub u32);
    impl Apb2enr {
        #[doc = "TIM1 peripheral clock enable"]
        #[inline(always)]
        pub const fn tim1en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM1 peripheral clock enable"]
        #[inline(always)]
        pub fn set_tim1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM8 peripheral clock enable"]
        #[inline(always)]
        pub const fn tim8en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TIM8 peripheral clock enable"]
        #[inline(always)]
        pub fn set_tim8en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "USART1 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn usart1en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "USART1 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_usart1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "USART6 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn usart6en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "USART6 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_usart6en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "UART9 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn uart9en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "UART9 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_uart9en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "USART10 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn usart10en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "USART10 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_usart10en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "SPI1 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn spi1en(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "SPI1 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_spi1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "SPI4 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn spi4en(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "SPI4 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_spi4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "TIM15 peripheral clock enable"]
        #[inline(always)]
        pub const fn tim15en(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "TIM15 peripheral clock enable"]
        #[inline(always)]
        pub fn set_tim15en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "TIM16 peripheral clock enable"]
        #[inline(always)]
        pub const fn tim16en(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "TIM16 peripheral clock enable"]
        #[inline(always)]
        pub fn set_tim16en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "TIM17 peripheral clock enable"]
        #[inline(always)]
        pub const fn tim17en(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "TIM17 peripheral clock enable"]
        #[inline(always)]
        pub fn set_tim17en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "SPI5 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn spi5en(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "SPI5 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_spi5en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "SAI1 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn sai1en(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "SAI1 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_sai1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "SAI2 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn sai2en(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "SAI2 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_sai2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "SAI3 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn sai3en(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "SAI3 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_sai3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "DFSDM1 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn dfsdm1en(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "DFSDM1 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_dfsdm1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "HRTIM peripheral clock enable"]
        #[inline(always)]
        pub const fn hrtimen(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "HRTIM peripheral clock enable"]
        #[inline(always)]
        pub fn set_hrtimen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
    }
    impl Default for Apb2enr {
        #[inline(always)]
        fn default() -> Apb2enr {
            Apb2enr(0)
        }
    }
    #[doc = "RCC APB2 Sleep Clock Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb2lpenr(pub u32);
    impl Apb2lpenr {
        #[doc = "TIM1 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn tim1lpen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM1 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_tim1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM8 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn tim8lpen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TIM8 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_tim8lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "USART1 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn usart1lpen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "USART1 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_usart1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "USART6 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn usart6lpen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "USART6 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_usart6lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "SPI1 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn spi1lpen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "SPI1 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_spi1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "SPI4 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn spi4lpen(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "SPI4 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_spi4lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "TIM15 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn tim15lpen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "TIM15 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_tim15lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "TIM16 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn tim16lpen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "TIM16 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_tim16lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "TIM17 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn tim17lpen(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "TIM17 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_tim17lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "SPI5 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn spi5lpen(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "SPI5 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_spi5lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "SAI1 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn sai1lpen(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "SAI1 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_sai1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "SAI2 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn sai2lpen(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "SAI2 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_sai2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "SAI3 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn sai3lpen(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "SAI3 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_sai3lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "DFSDM1 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn dfsdm1lpen(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "DFSDM1 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_dfsdm1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "HRTIM peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn hrtimlpen(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "HRTIM peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_hrtimlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
    }
    impl Default for Apb2lpenr {
        #[inline(always)]
        fn default() -> Apb2lpenr {
            Apb2lpenr(0)
        }
    }
    #[doc = "RCC APB2 Peripheral Reset Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb2rstr(pub u32);
    impl Apb2rstr {
        #[doc = "TIM1 block reset"]
        #[inline(always)]
        pub const fn tim1rst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM1 block reset"]
        #[inline(always)]
        pub fn set_tim1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM8 block reset"]
        #[inline(always)]
        pub const fn tim8rst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TIM8 block reset"]
        #[inline(always)]
        pub fn set_tim8rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "USART1 block reset"]
        #[inline(always)]
        pub const fn usart1rst(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "USART1 block reset"]
        #[inline(always)]
        pub fn set_usart1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "USART6 block reset"]
        #[inline(always)]
        pub const fn usart6rst(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "USART6 block reset"]
        #[inline(always)]
        pub fn set_usart6rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "UART9 block reset"]
        #[inline(always)]
        pub const fn uart9rst(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "UART9 block reset"]
        #[inline(always)]
        pub fn set_uart9rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "USART10 block reset"]
        #[inline(always)]
        pub const fn usart10rst(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "USART10 block reset"]
        #[inline(always)]
        pub fn set_usart10rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "SPI1 block reset"]
        #[inline(always)]
        pub const fn spi1rst(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "SPI1 block reset"]
        #[inline(always)]
        pub fn set_spi1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "SPI4 block reset"]
        #[inline(always)]
        pub const fn spi4rst(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "SPI4 block reset"]
        #[inline(always)]
        pub fn set_spi4rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "TIM15 block reset"]
        #[inline(always)]
        pub const fn tim15rst(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "TIM15 block reset"]
        #[inline(always)]
        pub fn set_tim15rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "TIM16 block reset"]
        #[inline(always)]
        pub const fn tim16rst(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "TIM16 block reset"]
        #[inline(always)]
        pub fn set_tim16rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "TIM17 block reset"]
        #[inline(always)]
        pub const fn tim17rst(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "TIM17 block reset"]
        #[inline(always)]
        pub fn set_tim17rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "SPI5 block reset"]
        #[inline(always)]
        pub const fn spi5rst(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "SPI5 block reset"]
        #[inline(always)]
        pub fn set_spi5rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "SAI1 block reset"]
        #[inline(always)]
        pub const fn sai1rst(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "SAI1 block reset"]
        #[inline(always)]
        pub fn set_sai1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "SAI2 block reset"]
        #[inline(always)]
        pub const fn sai2rst(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "SAI2 block reset"]
        #[inline(always)]
        pub fn set_sai2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "SAI3 block reset"]
        #[inline(always)]
        pub const fn sai3rst(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "SAI3 block reset"]
        #[inline(always)]
        pub fn set_sai3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "DFSDM1 block reset"]
        #[inline(always)]
        pub const fn dfsdm1rst(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "DFSDM1 block reset"]
        #[inline(always)]
        pub fn set_dfsdm1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "HRTIM block reset"]
        #[inline(always)]
        pub const fn hrtimrst(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "HRTIM block reset"]
        #[inline(always)]
        pub fn set_hrtimrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
    }
    impl Default for Apb2rstr {
        #[inline(always)]
        fn default() -> Apb2rstr {
            Apb2rstr(0)
        }
    }
    #[doc = "RCC APB3 Clock Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb3enr(pub u32);
    impl Apb3enr {
        #[doc = "LTDC peripheral clock enable"]
        #[inline(always)]
        pub const fn ltdcen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "LTDC peripheral clock enable"]
        #[inline(always)]
        pub fn set_ltdcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "DSI Peripheral clocks enable"]
        #[inline(always)]
        pub const fn dsien(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "DSI Peripheral clocks enable"]
        #[inline(always)]
        pub fn set_dsien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "WWDG1 Clock Enable"]
        #[inline(always)]
        pub const fn wwdg1en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "WWDG1 Clock Enable"]
        #[inline(always)]
        pub fn set_wwdg1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
    }
    impl Default for Apb3enr {
        #[inline(always)]
        fn default() -> Apb3enr {
            Apb3enr(0)
        }
    }
    #[doc = "RCC APB3 Sleep Clock Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb3lpenr(pub u32);
    impl Apb3lpenr {
        #[doc = "LTDC peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn ltdclpen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "LTDC peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_ltdclpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "DSI Peripheral Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn dsilpen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "DSI Peripheral Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_dsilpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "WWDG1 Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn wwdg1lpen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "WWDG1 Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_wwdg1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
    }
    impl Default for Apb3lpenr {
        #[inline(always)]
        fn default() -> Apb3lpenr {
            Apb3lpenr(0)
        }
    }
    #[doc = "RCC APB3 Peripheral Reset Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb3rstr(pub u32);
    impl Apb3rstr {
        #[doc = "LTDC block reset"]
        #[inline(always)]
        pub const fn ltdcrst(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "LTDC block reset"]
        #[inline(always)]
        pub fn set_ltdcrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "DSI block reset"]
        #[inline(always)]
        pub const fn dsirst(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "DSI block reset"]
        #[inline(always)]
        pub fn set_dsirst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for Apb3rstr {
        #[inline(always)]
        fn default() -> Apb3rstr {
            Apb3rstr(0)
        }
    }
    #[doc = "RCC APB4 Clock Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb4enr(pub u32);
    impl Apb4enr {
        #[doc = "SYSCFG peripheral clock enable"]
        #[inline(always)]
        pub const fn syscfgen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "SYSCFG peripheral clock enable"]
        #[inline(always)]
        pub fn set_syscfgen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "LPUART1 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn lpuart1en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "LPUART1 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_lpuart1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "SPI6 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn spi6en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "SPI6 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_spi6en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "I2C4 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn i2c4en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "I2C4 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_i2c4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "LPTIM2 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn lptim2en(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM2 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_lptim2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "LPTIM3 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn lptim3en(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM3 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_lptim3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "LPTIM4 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn lptim4en(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM4 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_lptim4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "LPTIM5 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn lptim5en(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM5 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_lptim5en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "DAC2 (containing one converter) peripheral clock enable"]
        #[inline(always)]
        pub const fn dac2en(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "DAC2 (containing one converter) peripheral clock enable"]
        #[inline(always)]
        pub fn set_dac2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "COMP1/2 peripheral clock enable"]
        #[inline(always)]
        pub const fn comp12en(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "COMP1/2 peripheral clock enable"]
        #[inline(always)]
        pub fn set_comp12en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "VREF peripheral clock enable"]
        #[inline(always)]
        pub const fn vrefen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "VREF peripheral clock enable"]
        #[inline(always)]
        pub fn set_vrefen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "RTC APB Clock Enable"]
        #[inline(always)]
        pub const fn rtcapben(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "RTC APB Clock Enable"]
        #[inline(always)]
        pub fn set_rtcapben(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "SAI4 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn sai4en(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "SAI4 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_sai4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Digital temperature sensor block enable"]
        #[inline(always)]
        pub const fn dtsen(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Digital temperature sensor block enable"]
        #[inline(always)]
        pub fn set_dtsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
    }
    impl Default for Apb4enr {
        #[inline(always)]
        fn default() -> Apb4enr {
            Apb4enr(0)
        }
    }
    #[doc = "RCC APB4 Sleep Clock Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb4lpenr(pub u32);
    impl Apb4lpenr {
        #[doc = "SYSCFG peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn syscfglpen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "SYSCFG peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_syscfglpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "LPUART1 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn lpuart1lpen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "LPUART1 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_lpuart1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "SPI6 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn spi6lpen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "SPI6 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_spi6lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "I2C4 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn i2c4lpen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "I2C4 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_i2c4lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "LPTIM2 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn lptim2lpen(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM2 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_lptim2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "LPTIM3 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn lptim3lpen(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM3 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_lptim3lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "LPTIM4 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn lptim4lpen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM4 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_lptim4lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "LPTIM5 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn lptim5lpen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM5 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_lptim5lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "DAC2 (containing one converter) peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn dac2lpen(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "DAC2 (containing one converter) peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_dac2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "COMP1/2 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn comp12lpen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "COMP1/2 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_comp12lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "VREF peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn vreflpen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "VREF peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_vreflpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "RTC APB Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn rtcapblpen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "RTC APB Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_rtcapblpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "SAI4 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn sai4lpen(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "SAI4 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_sai4lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Digital temperature sensor block enable during CSleep Mode"]
        #[inline(always)]
        pub const fn dtslpen(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Digital temperature sensor block enable during CSleep Mode"]
        #[inline(always)]
        pub fn set_dtslpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
    }
    impl Default for Apb4lpenr {
        #[inline(always)]
        fn default() -> Apb4lpenr {
            Apb4lpenr(0)
        }
    }
    #[doc = "RCC APB4 Peripheral Reset Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb4rstr(pub u32);
    impl Apb4rstr {
        #[doc = "SYSCFG block reset"]
        #[inline(always)]
        pub const fn syscfgrst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "SYSCFG block reset"]
        #[inline(always)]
        pub fn set_syscfgrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "LPUART1 block reset"]
        #[inline(always)]
        pub const fn lpuart1rst(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "LPUART1 block reset"]
        #[inline(always)]
        pub fn set_lpuart1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "SPI6 block reset"]
        #[inline(always)]
        pub const fn spi6rst(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "SPI6 block reset"]
        #[inline(always)]
        pub fn set_spi6rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "I2C4 block reset"]
        #[inline(always)]
        pub const fn i2c4rst(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "I2C4 block reset"]
        #[inline(always)]
        pub fn set_i2c4rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "LPTIM2 block reset"]
        #[inline(always)]
        pub const fn lptim2rst(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM2 block reset"]
        #[inline(always)]
        pub fn set_lptim2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "LPTIM3 block reset"]
        #[inline(always)]
        pub const fn lptim3rst(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM3 block reset"]
        #[inline(always)]
        pub fn set_lptim3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "LPTIM4 block reset"]
        #[inline(always)]
        pub const fn lptim4rst(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM4 block reset"]
        #[inline(always)]
        pub fn set_lptim4rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "LPTIM5 block reset"]
        #[inline(always)]
        pub const fn lptim5rst(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM5 block reset"]
        #[inline(always)]
        pub fn set_lptim5rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "DAC2 (containing one converter) reset"]
        #[inline(always)]
        pub const fn dac2rst(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "DAC2 (containing one converter) reset"]
        #[inline(always)]
        pub fn set_dac2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "COMP12 Blocks Reset"]
        #[inline(always)]
        pub const fn comp12rst(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "COMP12 Blocks Reset"]
        #[inline(always)]
        pub fn set_comp12rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "VREF block reset"]
        #[inline(always)]
        pub const fn vrefrst(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "VREF block reset"]
        #[inline(always)]
        pub fn set_vrefrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "SAI4 block reset"]
        #[inline(always)]
        pub const fn sai4rst(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "SAI4 block reset"]
        #[inline(always)]
        pub fn set_sai4rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Digital temperature sensor block reset"]
        #[inline(always)]
        pub const fn dtsrst(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Digital temperature sensor block reset"]
        #[inline(always)]
        pub fn set_dtsrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
    }
    impl Default for Apb4rstr {
        #[inline(always)]
        fn default() -> Apb4rstr {
            Apb4rstr(0)
        }
    }
    #[doc = "RCC Backup Domain Control Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bdcr(pub u32);
    impl Bdcr {
        #[doc = "LSE oscillator enabled"]
        #[inline(always)]
        pub const fn lseon(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LSE oscillator enabled"]
        #[inline(always)]
        pub fn set_lseon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "LSE oscillator ready"]
        #[inline(always)]
        pub const fn lserdy(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "LSE oscillator ready"]
        #[inline(always)]
        pub fn set_lserdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "LSE oscillator bypass"]
        #[inline(always)]
        pub const fn lsebyp(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "LSE oscillator bypass"]
        #[inline(always)]
        pub fn set_lsebyp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "LSE oscillator driving capability"]
        #[inline(always)]
        pub const fn lsedrv(&self) -> super::vals::Lsedrv {
            let val = (self.0 >> 3usize) & 0x03;
            super::vals::Lsedrv::from_bits(val as u8)
        }
        #[doc = "LSE oscillator driving capability"]
        #[inline(always)]
        pub fn set_lsedrv(&mut self, val: super::vals::Lsedrv) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val.to_bits() as u32) & 0x03) << 3usize);
        }
        #[doc = "LSE clock security system enable"]
        #[inline(always)]
        pub const fn lsecsson(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "LSE clock security system enable"]
        #[inline(always)]
        pub fn set_lsecsson(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "LSE clock security system failure detection"]
        #[inline(always)]
        pub const fn lsecssd(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "LSE clock security system failure detection"]
        #[inline(always)]
        pub fn set_lsecssd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "RTC clock source selection"]
        #[inline(always)]
        pub const fn rtcsel(&self) -> super::vals::Rtcsel {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::Rtcsel::from_bits(val as u8)
        }
        #[doc = "RTC clock source selection"]
        #[inline(always)]
        pub fn set_rtcsel(&mut self, val: super::vals::Rtcsel) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
        #[doc = "RTC clock enable"]
        #[inline(always)]
        pub const fn rtcen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "RTC clock enable"]
        #[inline(always)]
        pub fn set_rtcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "VSwitch domain software reset"]
        #[inline(always)]
        pub const fn bdrst(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "VSwitch domain software reset"]
        #[inline(always)]
        pub fn set_bdrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Bdcr {
        #[inline(always)]
        fn default() -> Bdcr {
            Bdcr(0)
        }
    }
    #[doc = "RCC Clock Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr(pub u32);
    impl Cfgr {
        #[doc = "System clock switch"]
        #[inline(always)]
        pub const fn sw(&self) -> super::vals::Sw {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::Sw::from_bits(val as u8)
        }
        #[doc = "System clock switch"]
        #[inline(always)]
        pub fn set_sw(&mut self, val: super::vals::Sw) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "System clock switch status"]
        #[inline(always)]
        pub const fn sws(&self) -> super::vals::Sw {
            let val = (self.0 >> 3usize) & 0x07;
            super::vals::Sw::from_bits(val as u8)
        }
        #[doc = "System clock switch status"]
        #[inline(always)]
        pub fn set_sws(&mut self, val: super::vals::Sw) {
            self.0 = (self.0 & !(0x07 << 3usize)) | (((val.to_bits() as u32) & 0x07) << 3usize);
        }
        #[doc = "System clock selection after a wake up from system Stop"]
        #[inline(always)]
        pub const fn stopwuck(&self) -> super::vals::Stopwuck {
            let val = (self.0 >> 6usize) & 0x01;
            super::vals::Stopwuck::from_bits(val as u8)
        }
        #[doc = "System clock selection after a wake up from system Stop"]
        #[inline(always)]
        pub fn set_stopwuck(&mut self, val: super::vals::Stopwuck) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
        }
        #[doc = "Kernel clock selection after a wake up from system Stop"]
        #[inline(always)]
        pub const fn stopkerwuck(&self) -> super::vals::Stopwuck {
            let val = (self.0 >> 7usize) & 0x01;
            super::vals::Stopwuck::from_bits(val as u8)
        }
        #[doc = "Kernel clock selection after a wake up from system Stop"]
        #[inline(always)]
        pub fn set_stopkerwuck(&mut self, val: super::vals::Stopwuck) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
        }
        #[doc = "HSE division factor for RTC clock"]
        #[inline(always)]
        pub const fn rtcpre(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[doc = "HSE division factor for RTC clock"]
        #[inline(always)]
        pub fn set_rtcpre(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[doc = "High Resolution Timer clock prescaler selection"]
        #[inline(always)]
        pub const fn hrtimsel(&self) -> super::vals::Hrtimsel {
            let val = (self.0 >> 14usize) & 0x01;
            super::vals::Hrtimsel::from_bits(val as u8)
        }
        #[doc = "High Resolution Timer clock prescaler selection"]
        #[inline(always)]
        pub fn set_hrtimsel(&mut self, val: super::vals::Hrtimsel) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
        }
        #[doc = "Timers clocks prescaler selection"]
        #[inline(always)]
        pub const fn timpre(&self) -> super::vals::Timpre {
            let val = (self.0 >> 15usize) & 0x01;
            super::vals::Timpre::from_bits(val as u8)
        }
        #[doc = "Timers clocks prescaler selection"]
        #[inline(always)]
        pub fn set_timpre(&mut self, val: super::vals::Timpre) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
        }
        #[doc = "MCO1 prescaler"]
        #[inline(always)]
        pub const fn mco1pre(&self) -> super::vals::Mcopre {
            let val = (self.0 >> 18usize) & 0x0f;
            super::vals::Mcopre::from_bits(val as u8)
        }
        #[doc = "MCO1 prescaler"]
        #[inline(always)]
        pub fn set_mco1pre(&mut self, val: super::vals::Mcopre) {
            self.0 = (self.0 & !(0x0f << 18usize)) | (((val.to_bits() as u32) & 0x0f) << 18usize);
        }
        #[doc = "Micro-controller clock output 1"]
        #[inline(always)]
        pub const fn mco1sel(&self) -> super::vals::Mco1sel {
            let val = (self.0 >> 22usize) & 0x07;
            super::vals::Mco1sel::from_bits(val as u8)
        }
        #[doc = "Micro-controller clock output 1"]
        #[inline(always)]
        pub fn set_mco1sel(&mut self, val: super::vals::Mco1sel) {
            self.0 = (self.0 & !(0x07 << 22usize)) | (((val.to_bits() as u32) & 0x07) << 22usize);
        }
        #[doc = "MCO2 prescaler"]
        #[inline(always)]
        pub const fn mco2pre(&self) -> super::vals::Mcopre {
            let val = (self.0 >> 25usize) & 0x0f;
            super::vals::Mcopre::from_bits(val as u8)
        }
        #[doc = "MCO2 prescaler"]
        #[inline(always)]
        pub fn set_mco2pre(&mut self, val: super::vals::Mcopre) {
            self.0 = (self.0 & !(0x0f << 25usize)) | (((val.to_bits() as u32) & 0x0f) << 25usize);
        }
        #[doc = "Micro-controller clock output 2"]
        #[inline(always)]
        pub const fn mco2sel(&self) -> super::vals::Mco2sel {
            let val = (self.0 >> 29usize) & 0x07;
            super::vals::Mco2sel::from_bits(val as u8)
        }
        #[doc = "Micro-controller clock output 2"]
        #[inline(always)]
        pub fn set_mco2sel(&mut self, val: super::vals::Mco2sel) {
            self.0 = (self.0 & !(0x07 << 29usize)) | (((val.to_bits() as u32) & 0x07) << 29usize);
        }
    }
    impl Default for Cfgr {
        #[inline(always)]
        fn default() -> Cfgr {
            Cfgr(0)
        }
    }
    #[doc = "RCC Clock Source Interrupt Clear Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cicr(pub u32);
    impl Cicr {
        #[doc = "LSI ready Interrupt Clear"]
        #[inline(always)]
        pub const fn lsirdyc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LSI ready Interrupt Clear"]
        #[inline(always)]
        pub fn set_lsirdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "LSE ready Interrupt Clear"]
        #[inline(always)]
        pub const fn lserdyc(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "LSE ready Interrupt Clear"]
        #[inline(always)]
        pub fn set_lserdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "HSI ready Interrupt Clear"]
        #[inline(always)]
        pub const fn hsirdyc(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "HSI ready Interrupt Clear"]
        #[inline(always)]
        pub fn set_hsirdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "HSE ready Interrupt Clear"]
        #[inline(always)]
        pub const fn hserdyc(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "HSE ready Interrupt Clear"]
        #[inline(always)]
        pub fn set_hserdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "CSI ready Interrupt Clear"]
        #[inline(always)]
        pub const fn hse_ready_interrupt_clear(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "CSI ready Interrupt Clear"]
        #[inline(always)]
        pub fn set_hse_ready_interrupt_clear(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "RC48 ready Interrupt Clear"]
        #[inline(always)]
        pub const fn hsi48rdyc(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "RC48 ready Interrupt Clear"]
        #[inline(always)]
        pub fn set_hsi48rdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "PLL1 ready Interrupt Clear"]
        #[inline(always)]
        pub const fn pllrdyc(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 6usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "PLL1 ready Interrupt Clear"]
        #[inline(always)]
        pub fn set_pllrdyc(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 6usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "LSE clock security system Interrupt Clear"]
        #[inline(always)]
        pub const fn lsecssc(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "LSE clock security system Interrupt Clear"]
        #[inline(always)]
        pub fn set_lsecssc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "HSE clock security system Interrupt Clear"]
        #[inline(always)]
        pub const fn hsecssc(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "HSE clock security system Interrupt Clear"]
        #[inline(always)]
        pub fn set_hsecssc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
    }
    impl Default for Cicr {
        #[inline(always)]
        fn default() -> Cicr {
            Cicr(0)
        }
    }
    #[doc = "RCC Clock Source Interrupt Enable Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cier(pub u32);
    impl Cier {
        #[doc = "LSI ready Interrupt Enable"]
        #[inline(always)]
        pub const fn lsirdyie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LSI ready Interrupt Enable"]
        #[inline(always)]
        pub fn set_lsirdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "LSE ready Interrupt Enable"]
        #[inline(always)]
        pub const fn lserdyie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "LSE ready Interrupt Enable"]
        #[inline(always)]
        pub fn set_lserdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "HSI ready Interrupt Enable"]
        #[inline(always)]
        pub const fn hsirdyie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "HSI ready Interrupt Enable"]
        #[inline(always)]
        pub fn set_hsirdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "HSE ready Interrupt Enable"]
        #[inline(always)]
        pub const fn hserdyie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "HSE ready Interrupt Enable"]
        #[inline(always)]
        pub fn set_hserdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "CSI ready Interrupt Enable"]
        #[inline(always)]
        pub const fn csirdyie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "CSI ready Interrupt Enable"]
        #[inline(always)]
        pub fn set_csirdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "RC48 ready Interrupt Enable"]
        #[inline(always)]
        pub const fn hsi48rdyie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "RC48 ready Interrupt Enable"]
        #[inline(always)]
        pub fn set_hsi48rdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "PLL1 ready Interrupt Enable"]
        #[inline(always)]
        pub const fn pllrdyie(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 6usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "PLL1 ready Interrupt Enable"]
        #[inline(always)]
        pub fn set_pllrdyie(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 6usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "LSE clock security system Interrupt Enable"]
        #[inline(always)]
        pub const fn lsecssie(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "LSE clock security system Interrupt Enable"]
        #[inline(always)]
        pub fn set_lsecssie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for Cier {
        #[inline(always)]
        fn default() -> Cier {
            Cier(0)
        }
    }
    #[doc = "RCC Clock Source Interrupt Flag Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cifr(pub u32);
    impl Cifr {
        #[doc = "LSI ready Interrupt Flag"]
        #[inline(always)]
        pub const fn lsirdyf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LSI ready Interrupt Flag"]
        #[inline(always)]
        pub fn set_lsirdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "LSE ready Interrupt Flag"]
        #[inline(always)]
        pub const fn lserdyf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "LSE ready Interrupt Flag"]
        #[inline(always)]
        pub fn set_lserdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "HSI ready Interrupt Flag"]
        #[inline(always)]
        pub const fn hsirdyf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "HSI ready Interrupt Flag"]
        #[inline(always)]
        pub fn set_hsirdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "HSE ready Interrupt Flag"]
        #[inline(always)]
        pub const fn hserdyf(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "HSE ready Interrupt Flag"]
        #[inline(always)]
        pub fn set_hserdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "CSI ready Interrupt Flag"]
        #[inline(always)]
        pub const fn csirdy(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "CSI ready Interrupt Flag"]
        #[inline(always)]
        pub fn set_csirdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "RC48 ready Interrupt Flag"]
        #[inline(always)]
        pub const fn hsi48rdyf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "RC48 ready Interrupt Flag"]
        #[inline(always)]
        pub fn set_hsi48rdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "PLL1 ready Interrupt Flag"]
        #[inline(always)]
        pub const fn pllrdyf(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 6usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "PLL1 ready Interrupt Flag"]
        #[inline(always)]
        pub fn set_pllrdyf(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 6usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "LSE clock security system Interrupt Flag"]
        #[inline(always)]
        pub const fn lsecssf(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "LSE clock security system Interrupt Flag"]
        #[inline(always)]
        pub fn set_lsecssf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "HSE clock security system Interrupt Flag"]
        #[inline(always)]
        pub const fn hsecssf(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "HSE clock security system Interrupt Flag"]
        #[inline(always)]
        pub fn set_hsecssf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
    }
    impl Default for Cifr {
        #[inline(always)]
        fn default() -> Cifr {
            Cifr(0)
        }
    }
    #[doc = "clock control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "Internal high-speed clock enable"]
        #[inline(always)]
        pub const fn hsion(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Internal high-speed clock enable"]
        #[inline(always)]
        pub fn set_hsion(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "High Speed Internal clock enable in Stop mode"]
        #[inline(always)]
        pub const fn hsikeron(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "High Speed Internal clock enable in Stop mode"]
        #[inline(always)]
        pub fn set_hsikeron(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "HSI clock ready flag"]
        #[inline(always)]
        pub const fn hsirdy(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "HSI clock ready flag"]
        #[inline(always)]
        pub fn set_hsirdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "HSI clock divider"]
        #[inline(always)]
        pub const fn hsidiv(&self) -> super::vals::Hsidiv {
            let val = (self.0 >> 3usize) & 0x03;
            super::vals::Hsidiv::from_bits(val as u8)
        }
        #[doc = "HSI clock divider"]
        #[inline(always)]
        pub fn set_hsidiv(&mut self, val: super::vals::Hsidiv) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val.to_bits() as u32) & 0x03) << 3usize);
        }
        #[doc = "HSI divider flag"]
        #[inline(always)]
        pub const fn hsidivf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "HSI divider flag"]
        #[inline(always)]
        pub fn set_hsidivf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "CSI clock enable"]
        #[inline(always)]
        pub const fn csion(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "CSI clock enable"]
        #[inline(always)]
        pub fn set_csion(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "CSI clock ready flag"]
        #[inline(always)]
        pub const fn csirdy(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "CSI clock ready flag"]
        #[inline(always)]
        pub fn set_csirdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "CSI clock enable in Stop mode"]
        #[inline(always)]
        pub const fn csikeron(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "CSI clock enable in Stop mode"]
        #[inline(always)]
        pub fn set_csikeron(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "RC48 clock enable"]
        #[inline(always)]
        pub const fn hsi48on(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "RC48 clock enable"]
        #[inline(always)]
        pub fn set_hsi48on(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "RC48 clock ready flag"]
        #[inline(always)]
        pub const fn hsi48rdy(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "RC48 clock ready flag"]
        #[inline(always)]
        pub fn set_hsi48rdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "D1 domain clocks ready flag"]
        #[inline(always)]
        pub const fn d1ckrdy(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "D1 domain clocks ready flag"]
        #[inline(always)]
        pub fn set_d1ckrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "D2 domain clocks ready flag"]
        #[inline(always)]
        pub const fn d2ckrdy(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "D2 domain clocks ready flag"]
        #[inline(always)]
        pub fn set_d2ckrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "HSE clock enable"]
        #[inline(always)]
        pub const fn hseon(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "HSE clock enable"]
        #[inline(always)]
        pub fn set_hseon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "HSE clock ready flag"]
        #[inline(always)]
        pub const fn hserdy(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "HSE clock ready flag"]
        #[inline(always)]
        pub fn set_hserdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "HSE clock bypass"]
        #[inline(always)]
        pub const fn hsebyp(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "HSE clock bypass"]
        #[inline(always)]
        pub fn set_hsebyp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "HSE Clock Security System enable"]
        #[inline(always)]
        pub const fn hsecsson(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "HSE Clock Security System enable"]
        #[inline(always)]
        pub fn set_hsecsson(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "PLL1 enable"]
        #[inline(always)]
        pub const fn pllon(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 24usize + n * 2usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "PLL1 enable"]
        #[inline(always)]
        pub fn set_pllon(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 24usize + n * 2usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "PLL1 clock ready flag"]
        #[inline(always)]
        pub const fn pllrdy(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 25usize + n * 2usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "PLL1 clock ready flag"]
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
    #[doc = "RCC Clock Recovery RC Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Crrcr(pub u32);
    impl Crrcr {
        #[doc = "Internal RC 48 MHz clock calibration"]
        #[inline(always)]
        pub const fn hsi48cal(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Internal RC 48 MHz clock calibration"]
        #[inline(always)]
        pub fn set_hsi48cal(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
    }
    impl Default for Crrcr {
        #[inline(always)]
        fn default() -> Crrcr {
            Crrcr(0)
        }
    }
    #[doc = "RCC CSI configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csicfgr(pub u32);
    impl Csicfgr {
        #[doc = "CSI clock calibration"]
        #[inline(always)]
        pub const fn csical(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "CSI clock calibration"]
        #[inline(always)]
        pub fn set_csical(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "CSI clock trimming"]
        #[inline(always)]
        pub const fn csitrim(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x3f;
            val as u8
        }
        #[doc = "CSI clock trimming"]
        #[inline(always)]
        pub fn set_csitrim(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
        }
    }
    impl Default for Csicfgr {
        #[inline(always)]
        fn default() -> Csicfgr {
            Csicfgr(0)
        }
    }
    #[doc = "RCC Clock Control and Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csr(pub u32);
    impl Csr {
        #[doc = "LSI oscillator enable"]
        #[inline(always)]
        pub const fn lsion(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LSI oscillator enable"]
        #[inline(always)]
        pub fn set_lsion(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "LSI oscillator ready"]
        #[inline(always)]
        pub const fn lsirdy(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "LSI oscillator ready"]
        #[inline(always)]
        pub fn set_lsirdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Csr {
        #[inline(always)]
        fn default() -> Csr {
            Csr(0)
        }
    }
    #[doc = "RCC Domain 1 Kernel Clock Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct D1ccipr(pub u32);
    impl D1ccipr {
        #[doc = "FMC kernel clock source selection"]
        #[inline(always)]
        pub const fn fmcsel(&self) -> super::vals::Fmcsel {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Fmcsel::from_bits(val as u8)
        }
        #[doc = "FMC kernel clock source selection"]
        #[inline(always)]
        pub fn set_fmcsel(&mut self, val: super::vals::Fmcsel) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "OCTOSPI kernel clock source selection"]
        #[inline(always)]
        pub const fn octospisel(&self) -> super::vals::Fmcsel {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::Fmcsel::from_bits(val as u8)
        }
        #[doc = "OCTOSPI kernel clock source selection"]
        #[inline(always)]
        pub fn set_octospisel(&mut self, val: super::vals::Fmcsel) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
        #[doc = "kernel clock source selection"]
        #[inline(always)]
        pub const fn dsisel(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "kernel clock source selection"]
        #[inline(always)]
        pub fn set_dsisel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "SDMMC kernel clock source selection"]
        #[inline(always)]
        pub const fn sdmmcsel(&self) -> super::vals::Sdmmcsel {
            let val = (self.0 >> 16usize) & 0x01;
            super::vals::Sdmmcsel::from_bits(val as u8)
        }
        #[doc = "SDMMC kernel clock source selection"]
        #[inline(always)]
        pub fn set_sdmmcsel(&mut self, val: super::vals::Sdmmcsel) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
        }
        #[doc = "per_ck clock source selection"]
        #[inline(always)]
        pub const fn ckpersel(&self) -> super::vals::Ckpersel {
            let val = (self.0 >> 28usize) & 0x03;
            super::vals::Ckpersel::from_bits(val as u8)
        }
        #[doc = "per_ck clock source selection"]
        #[inline(always)]
        pub fn set_ckpersel(&mut self, val: super::vals::Ckpersel) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
        }
    }
    impl Default for D1ccipr {
        #[inline(always)]
        fn default() -> D1ccipr {
            D1ccipr(0)
        }
    }
    #[doc = "RCC Domain 1 Clock Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct D1cfgr(pub u32);
    impl D1cfgr {
        #[doc = "D1 domain AHB prescaler"]
        #[inline(always)]
        pub const fn hpre(&self) -> super::vals::Hpre {
            let val = (self.0 >> 0usize) & 0x0f;
            super::vals::Hpre::from_bits(val as u8)
        }
        #[doc = "D1 domain AHB prescaler"]
        #[inline(always)]
        pub fn set_hpre(&mut self, val: super::vals::Hpre) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
        }
        #[doc = "D1 domain APB3 prescaler"]
        #[inline(always)]
        pub const fn d1ppre(&self) -> super::vals::Ppre {
            let val = (self.0 >> 4usize) & 0x07;
            super::vals::Ppre::from_bits(val as u8)
        }
        #[doc = "D1 domain APB3 prescaler"]
        #[inline(always)]
        pub fn set_d1ppre(&mut self, val: super::vals::Ppre) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
        }
        #[doc = "D1 domain Core prescaler"]
        #[inline(always)]
        pub const fn d1cpre(&self) -> super::vals::Hpre {
            let val = (self.0 >> 8usize) & 0x0f;
            super::vals::Hpre::from_bits(val as u8)
        }
        #[doc = "D1 domain Core prescaler"]
        #[inline(always)]
        pub fn set_d1cpre(&mut self, val: super::vals::Hpre) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
        }
    }
    impl Default for D1cfgr {
        #[inline(always)]
        fn default() -> D1cfgr {
            D1cfgr(0)
        }
    }
    #[doc = "RCC Domain 2 Kernel Clock Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct D2ccip1r(pub u32);
    impl D2ccip1r {
        #[doc = "SAI1 and DFSDM1 kernel Aclk clock source selection"]
        #[inline(always)]
        pub const fn sai1sel(&self) -> super::vals::Saisel {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::Saisel::from_bits(val as u8)
        }
        #[doc = "SAI1 and DFSDM1 kernel Aclk clock source selection"]
        #[inline(always)]
        pub fn set_sai1sel(&mut self, val: super::vals::Saisel) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "SAI2 kernel clock source A source selection"]
        #[inline(always)]
        pub const fn sai2asel(&self) -> super::vals::Saiasel {
            let val = (self.0 >> 6usize) & 0x07;
            super::vals::Saiasel::from_bits(val as u8)
        }
        #[doc = "SAI2 kernel clock source A source selection"]
        #[inline(always)]
        pub fn set_sai2asel(&mut self, val: super::vals::Saiasel) {
            self.0 = (self.0 & !(0x07 << 6usize)) | (((val.to_bits() as u32) & 0x07) << 6usize);
        }
        #[doc = "SAI2 kernel clock source B source selection"]
        #[inline(always)]
        pub const fn sai2bsel(&self) -> super::vals::Saiasel {
            let val = (self.0 >> 9usize) & 0x07;
            super::vals::Saiasel::from_bits(val as u8)
        }
        #[doc = "SAI2 kernel clock source B source selection"]
        #[inline(always)]
        pub fn set_sai2bsel(&mut self, val: super::vals::Saiasel) {
            self.0 = (self.0 & !(0x07 << 9usize)) | (((val.to_bits() as u32) & 0x07) << 9usize);
        }
        #[doc = "SPI/I2S1,2 and 3 kernel clock source selection"]
        #[inline(always)]
        pub const fn spi123sel(&self) -> super::vals::Saisel {
            let val = (self.0 >> 12usize) & 0x07;
            super::vals::Saisel::from_bits(val as u8)
        }
        #[doc = "SPI/I2S1,2 and 3 kernel clock source selection"]
        #[inline(always)]
        pub fn set_spi123sel(&mut self, val: super::vals::Saisel) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
        }
        #[doc = "SPI4 and 5 kernel clock source selection"]
        #[inline(always)]
        pub const fn spi45sel(&self) -> super::vals::Spi45sel {
            let val = (self.0 >> 16usize) & 0x07;
            super::vals::Spi45sel::from_bits(val as u8)
        }
        #[doc = "SPI4 and 5 kernel clock source selection"]
        #[inline(always)]
        pub fn set_spi45sel(&mut self, val: super::vals::Spi45sel) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
        }
        #[doc = "SPDIFRX kernel clock source selection"]
        #[inline(always)]
        pub const fn spdifrxsel(&self) -> super::vals::Spdifrxsel {
            let val = (self.0 >> 20usize) & 0x03;
            super::vals::Spdifrxsel::from_bits(val as u8)
        }
        #[doc = "SPDIFRX kernel clock source selection"]
        #[inline(always)]
        pub fn set_spdifrxsel(&mut self, val: super::vals::Spdifrxsel) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
        }
        #[doc = "DFSDM1 kernel Clk clock source selection"]
        #[inline(always)]
        pub const fn dfsdm1sel(&self) -> super::vals::Dfsdmsel {
            let val = (self.0 >> 24usize) & 0x01;
            super::vals::Dfsdmsel::from_bits(val as u8)
        }
        #[doc = "DFSDM1 kernel Clk clock source selection"]
        #[inline(always)]
        pub fn set_dfsdm1sel(&mut self, val: super::vals::Dfsdmsel) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
        }
        #[doc = "FDCAN kernel clock source selection"]
        #[inline(always)]
        pub const fn fdcansel(&self) -> super::vals::Fdcansel {
            let val = (self.0 >> 28usize) & 0x03;
            super::vals::Fdcansel::from_bits(val as u8)
        }
        #[doc = "FDCAN kernel clock source selection"]
        #[inline(always)]
        pub fn set_fdcansel(&mut self, val: super::vals::Fdcansel) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
        }
        #[doc = "SWPMI kernel clock source selection"]
        #[inline(always)]
        pub const fn swpsel(&self) -> super::vals::Swpsel {
            let val = (self.0 >> 31usize) & 0x01;
            super::vals::Swpsel::from_bits(val as u8)
        }
        #[doc = "SWPMI kernel clock source selection"]
        #[inline(always)]
        pub fn set_swpsel(&mut self, val: super::vals::Swpsel) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
        }
    }
    impl Default for D2ccip1r {
        #[inline(always)]
        fn default() -> D2ccip1r {
            D2ccip1r(0)
        }
    }
    #[doc = "RCC Domain 2 Kernel Clock Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct D2ccip2r(pub u32);
    impl D2ccip2r {
        #[doc = "USART2/3, UART4,5, 7/8 (APB1) kernel clock source selection"]
        #[inline(always)]
        pub const fn usart234578sel(&self) -> super::vals::Usart234578sel {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::Usart234578sel::from_bits(val as u8)
        }
        #[doc = "USART2/3, UART4,5, 7/8 (APB1) kernel clock source selection"]
        #[inline(always)]
        pub fn set_usart234578sel(&mut self, val: super::vals::Usart234578sel) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "USART1, 6, 9 and 10 kernel clock source selection"]
        #[inline(always)]
        pub const fn usart16910sel(&self) -> super::vals::Usart16910sel {
            let val = (self.0 >> 3usize) & 0x07;
            super::vals::Usart16910sel::from_bits(val as u8)
        }
        #[doc = "USART1, 6, 9 and 10 kernel clock source selection"]
        #[inline(always)]
        pub fn set_usart16910sel(&mut self, val: super::vals::Usart16910sel) {
            self.0 = (self.0 & !(0x07 << 3usize)) | (((val.to_bits() as u32) & 0x07) << 3usize);
        }
        #[doc = "RNG kernel clock source selection"]
        #[inline(always)]
        pub const fn rngsel(&self) -> super::vals::Rngsel {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::Rngsel::from_bits(val as u8)
        }
        #[doc = "RNG kernel clock source selection"]
        #[inline(always)]
        pub fn set_rngsel(&mut self, val: super::vals::Rngsel) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
        #[doc = "I2C1,2,3 kernel clock source selection"]
        #[inline(always)]
        pub const fn i2c1235sel(&self) -> super::vals::I2c1235sel {
            let val = (self.0 >> 12usize) & 0x03;
            super::vals::I2c1235sel::from_bits(val as u8)
        }
        #[doc = "I2C1,2,3 kernel clock source selection"]
        #[inline(always)]
        pub fn set_i2c1235sel(&mut self, val: super::vals::I2c1235sel) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
        }
        #[doc = "USBOTG 1 and 2 kernel clock source selection"]
        #[inline(always)]
        pub const fn usbsel(&self) -> super::vals::Usbsel {
            let val = (self.0 >> 20usize) & 0x03;
            super::vals::Usbsel::from_bits(val as u8)
        }
        #[doc = "USBOTG 1 and 2 kernel clock source selection"]
        #[inline(always)]
        pub fn set_usbsel(&mut self, val: super::vals::Usbsel) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
        }
        #[doc = "HDMI-CEC kernel clock source selection"]
        #[inline(always)]
        pub const fn cecsel(&self) -> super::vals::Cecsel {
            let val = (self.0 >> 22usize) & 0x03;
            super::vals::Cecsel::from_bits(val as u8)
        }
        #[doc = "HDMI-CEC kernel clock source selection"]
        #[inline(always)]
        pub fn set_cecsel(&mut self, val: super::vals::Cecsel) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val.to_bits() as u32) & 0x03) << 22usize);
        }
        #[doc = "LPTIM1 kernel clock source selection"]
        #[inline(always)]
        pub const fn lptim1sel(&self) -> super::vals::Lptim1sel {
            let val = (self.0 >> 28usize) & 0x07;
            super::vals::Lptim1sel::from_bits(val as u8)
        }
        #[doc = "LPTIM1 kernel clock source selection"]
        #[inline(always)]
        pub fn set_lptim1sel(&mut self, val: super::vals::Lptim1sel) {
            self.0 = (self.0 & !(0x07 << 28usize)) | (((val.to_bits() as u32) & 0x07) << 28usize);
        }
    }
    impl Default for D2ccip2r {
        #[inline(always)]
        fn default() -> D2ccip2r {
            D2ccip2r(0)
        }
    }
    #[doc = "RCC Domain 2 Clock Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct D2cfgr(pub u32);
    impl D2cfgr {
        #[doc = "D2 domain APB1 prescaler"]
        #[inline(always)]
        pub const fn d2ppre1(&self) -> super::vals::Ppre {
            let val = (self.0 >> 4usize) & 0x07;
            super::vals::Ppre::from_bits(val as u8)
        }
        #[doc = "D2 domain APB1 prescaler"]
        #[inline(always)]
        pub fn set_d2ppre1(&mut self, val: super::vals::Ppre) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
        }
        #[doc = "D2 domain APB2 prescaler"]
        #[inline(always)]
        pub const fn d2ppre2(&self) -> super::vals::Ppre {
            let val = (self.0 >> 8usize) & 0x07;
            super::vals::Ppre::from_bits(val as u8)
        }
        #[doc = "D2 domain APB2 prescaler"]
        #[inline(always)]
        pub fn set_d2ppre2(&mut self, val: super::vals::Ppre) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
        }
    }
    impl Default for D2cfgr {
        #[inline(always)]
        fn default() -> D2cfgr {
            D2cfgr(0)
        }
    }
    #[doc = "RCC D3 Autonomous mode Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct D3amr(pub u32);
    impl D3amr {
        #[doc = "BDMA2 and DMAMUX Autonomous mode enable"]
        #[inline(always)]
        pub const fn bdma2amen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "BDMA2 and DMAMUX Autonomous mode enable"]
        #[inline(always)]
        pub fn set_bdma2amen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "BDMA and DMAMUX Autonomous mode enable"]
        #[inline(always)]
        pub const fn bdmaamen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "BDMA and DMAMUX Autonomous mode enable"]
        #[inline(always)]
        pub fn set_bdmaamen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "LPUART1 Autonomous mode enable"]
        #[inline(always)]
        pub const fn lpuart1amen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "LPUART1 Autonomous mode enable"]
        #[inline(always)]
        pub fn set_lpuart1amen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "SPI6 Autonomous mode enable"]
        #[inline(always)]
        pub const fn spi6amen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "SPI6 Autonomous mode enable"]
        #[inline(always)]
        pub fn set_spi6amen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "I2C4 Autonomous mode enable"]
        #[inline(always)]
        pub const fn i2c4amen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "I2C4 Autonomous mode enable"]
        #[inline(always)]
        pub fn set_i2c4amen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "LPTIM2 Autonomous mode enable"]
        #[inline(always)]
        pub const fn lptim2amen(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM2 Autonomous mode enable"]
        #[inline(always)]
        pub fn set_lptim2amen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "LPTIM3 Autonomous mode enable"]
        #[inline(always)]
        pub const fn lptim3amen(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM3 Autonomous mode enable"]
        #[inline(always)]
        pub fn set_lptim3amen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "LPTIM4 Autonomous mode enable"]
        #[inline(always)]
        pub const fn lptim4amen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM4 Autonomous mode enable"]
        #[inline(always)]
        pub fn set_lptim4amen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "LPTIM5 Autonomous mode enable"]
        #[inline(always)]
        pub const fn lptim5amen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM5 Autonomous mode enable"]
        #[inline(always)]
        pub fn set_lptim5amen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "DAC2 (containing one converter) Autonomous mode enable"]
        #[inline(always)]
        pub const fn dac2amen(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "DAC2 (containing one converter) Autonomous mode enable"]
        #[inline(always)]
        pub fn set_dac2amen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "COMP12 Autonomous mode enable"]
        #[inline(always)]
        pub const fn comp12amen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "COMP12 Autonomous mode enable"]
        #[inline(always)]
        pub fn set_comp12amen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "VREF Autonomous mode enable"]
        #[inline(always)]
        pub const fn vrefamen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "VREF Autonomous mode enable"]
        #[inline(always)]
        pub fn set_vrefamen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "RTC Autonomous mode enable"]
        #[inline(always)]
        pub const fn rtcamen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "RTC Autonomous mode enable"]
        #[inline(always)]
        pub fn set_rtcamen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "CRC Autonomous mode enable"]
        #[inline(always)]
        pub const fn crcamen(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "CRC Autonomous mode enable"]
        #[inline(always)]
        pub fn set_crcamen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "SAI4 Autonomous mode enable"]
        #[inline(always)]
        pub const fn sai4amen(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "SAI4 Autonomous mode enable"]
        #[inline(always)]
        pub fn set_sai4amen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "ADC3 Autonomous mode enable"]
        #[inline(always)]
        pub const fn adc3amen(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "ADC3 Autonomous mode enable"]
        #[inline(always)]
        pub fn set_adc3amen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Digital temperature sensor Autonomous mode enable"]
        #[inline(always)]
        pub const fn dtsamen(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Digital temperature sensor Autonomous mode enable"]
        #[inline(always)]
        pub fn set_dtsamen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Backup RAM Autonomous mode enable"]
        #[inline(always)]
        pub const fn bkpsramamen(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Backup RAM Autonomous mode enable"]
        #[inline(always)]
        pub fn set_bkpsramamen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "SRAM4 Autonomous mode enable"]
        #[inline(always)]
        pub const fn sram4amen(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM4 Autonomous mode enable"]
        #[inline(always)]
        pub fn set_sram4amen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
    }
    impl Default for D3amr {
        #[inline(always)]
        fn default() -> D3amr {
            D3amr(0)
        }
    }
    #[doc = "RCC Domain 3 Kernel Clock Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct D3ccipr(pub u32);
    impl D3ccipr {
        #[doc = "LPUART1 kernel clock source selection"]
        #[inline(always)]
        pub const fn lpuart1sel(&self) -> super::vals::Lpuartsel {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::Lpuartsel::from_bits(val as u8)
        }
        #[doc = "LPUART1 kernel clock source selection"]
        #[inline(always)]
        pub fn set_lpuart1sel(&mut self, val: super::vals::Lpuartsel) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "I2C4 kernel clock source selection"]
        #[inline(always)]
        pub const fn i2c4sel(&self) -> super::vals::I2c4sel {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::I2c4sel::from_bits(val as u8)
        }
        #[doc = "I2C4 kernel clock source selection"]
        #[inline(always)]
        pub fn set_i2c4sel(&mut self, val: super::vals::I2c4sel) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
        #[doc = "LPTIM2 kernel clock source selection"]
        #[inline(always)]
        pub const fn lptim2sel(&self) -> super::vals::Lptim2sel {
            let val = (self.0 >> 10usize) & 0x07;
            super::vals::Lptim2sel::from_bits(val as u8)
        }
        #[doc = "LPTIM2 kernel clock source selection"]
        #[inline(always)]
        pub fn set_lptim2sel(&mut self, val: super::vals::Lptim2sel) {
            self.0 = (self.0 & !(0x07 << 10usize)) | (((val.to_bits() as u32) & 0x07) << 10usize);
        }
        #[doc = "LPTIM3,4,5 kernel clock source selection"]
        #[inline(always)]
        pub const fn lptim345sel(&self) -> super::vals::Lptim2sel {
            let val = (self.0 >> 13usize) & 0x07;
            super::vals::Lptim2sel::from_bits(val as u8)
        }
        #[doc = "LPTIM3,4,5 kernel clock source selection"]
        #[inline(always)]
        pub fn set_lptim345sel(&mut self, val: super::vals::Lptim2sel) {
            self.0 = (self.0 & !(0x07 << 13usize)) | (((val.to_bits() as u32) & 0x07) << 13usize);
        }
        #[doc = "SAR ADC kernel clock source selection"]
        #[inline(always)]
        pub const fn adcsel(&self) -> super::vals::Adcsel {
            let val = (self.0 >> 16usize) & 0x03;
            super::vals::Adcsel::from_bits(val as u8)
        }
        #[doc = "SAR ADC kernel clock source selection"]
        #[inline(always)]
        pub fn set_adcsel(&mut self, val: super::vals::Adcsel) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
        }
        #[doc = "DFSDM2 kernel clock source selection"]
        #[inline(always)]
        pub const fn dfsdm2sel(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "DFSDM2 kernel clock source selection"]
        #[inline(always)]
        pub fn set_dfsdm2sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "SPI6 kernel clock source selection"]
        #[inline(always)]
        pub const fn spi6sel(&self) -> super::vals::Spi6sel {
            let val = (self.0 >> 28usize) & 0x07;
            super::vals::Spi6sel::from_bits(val as u8)
        }
        #[doc = "SPI6 kernel clock source selection"]
        #[inline(always)]
        pub fn set_spi6sel(&mut self, val: super::vals::Spi6sel) {
            self.0 = (self.0 & !(0x07 << 28usize)) | (((val.to_bits() as u32) & 0x07) << 28usize);
        }
    }
    impl Default for D3ccipr {
        #[inline(always)]
        fn default() -> D3ccipr {
            D3ccipr(0)
        }
    }
    #[doc = "RCC Domain 3 Clock Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct D3cfgr(pub u32);
    impl D3cfgr {
        #[doc = "D3 domain APB4 prescaler"]
        #[inline(always)]
        pub const fn d3ppre(&self) -> super::vals::Ppre {
            let val = (self.0 >> 4usize) & 0x07;
            super::vals::Ppre::from_bits(val as u8)
        }
        #[doc = "D3 domain APB4 prescaler"]
        #[inline(always)]
        pub fn set_d3ppre(&mut self, val: super::vals::Ppre) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
        }
    }
    impl Default for D3cfgr {
        #[inline(always)]
        fn default() -> D3cfgr {
            D3cfgr(0)
        }
    }
    #[doc = "Global Control Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gcr(pub u32);
    impl Gcr {
        #[doc = "WWDG1 reset scope control"]
        #[inline(always)]
        pub const fn ww1rsc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "WWDG1 reset scope control"]
        #[inline(always)]
        pub fn set_ww1rsc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Gcr {
        #[inline(always)]
        fn default() -> Gcr {
            Gcr(0)
        }
    }
    #[doc = "RCC HSI configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hsicfgr(pub u32);
    impl Hsicfgr {
        #[doc = "HSI clock calibration"]
        #[inline(always)]
        pub const fn hsical(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "HSI clock calibration"]
        #[inline(always)]
        pub fn set_hsical(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "HSI clock trimming"]
        #[inline(always)]
        pub const fn hsitrim(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x7f;
            val as u8
        }
        #[doc = "HSI clock trimming"]
        #[inline(always)]
        pub fn set_hsitrim(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 24usize)) | (((val as u32) & 0x7f) << 24usize);
        }
    }
    impl Default for Hsicfgr {
        #[inline(always)]
        fn default() -> Hsicfgr {
            Hsicfgr(0)
        }
    }
    #[doc = "RCC PLLs Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pllcfgr(pub u32);
    impl Pllcfgr {
        #[doc = "PLL1 fractional latch enable"]
        #[inline(always)]
        pub const fn pllfracen(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 0usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "PLL1 fractional latch enable"]
        #[inline(always)]
        pub fn set_pllfracen(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 0usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "PLL1 VCO selection"]
        #[inline(always)]
        pub const fn pllvcosel(&self, n: usize) -> super::vals::Pllvcosel {
            assert!(n < 3usize);
            let offs = 1usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            super::vals::Pllvcosel::from_bits(val as u8)
        }
        #[doc = "PLL1 VCO selection"]
        #[inline(always)]
        pub fn set_pllvcosel(&mut self, n: usize, val: super::vals::Pllvcosel) {
            assert!(n < 3usize);
            let offs = 1usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
        }
        #[doc = "PLL1 input frequency range"]
        #[inline(always)]
        pub const fn pllrge(&self, n: usize) -> super::vals::Pllrge {
            assert!(n < 3usize);
            let offs = 2usize + n * 4usize;
            let val = (self.0 >> offs) & 0x03;
            super::vals::Pllrge::from_bits(val as u8)
        }
        #[doc = "PLL1 input frequency range"]
        #[inline(always)]
        pub fn set_pllrge(&mut self, n: usize, val: super::vals::Pllrge) {
            assert!(n < 3usize);
            let offs = 2usize + n * 4usize;
            self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
        }
        #[doc = "PLL1 DIVP divider output enable"]
        #[inline(always)]
        pub const fn divpen(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 16usize + n * 3usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "PLL1 DIVP divider output enable"]
        #[inline(always)]
        pub fn set_divpen(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 16usize + n * 3usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "PLL1 DIVQ divider output enable"]
        #[inline(always)]
        pub const fn divqen(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 17usize + n * 3usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "PLL1 DIVQ divider output enable"]
        #[inline(always)]
        pub fn set_divqen(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 17usize + n * 3usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "PLL1 DIVR divider output enable"]
        #[inline(always)]
        pub const fn divren(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 18usize + n * 3usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "PLL1 DIVR divider output enable"]
        #[inline(always)]
        pub fn set_divren(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 18usize + n * 3usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Pllcfgr {
        #[inline(always)]
        fn default() -> Pllcfgr {
            Pllcfgr(0)
        }
    }
    #[doc = "RCC PLLs Clock Source Selection Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pllckselr(pub u32);
    impl Pllckselr {
        #[doc = "DIVMx and PLLs clock source selection"]
        #[inline(always)]
        pub const fn pllsrc(&self) -> super::vals::Pllsrc {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Pllsrc::from_bits(val as u8)
        }
        #[doc = "DIVMx and PLLs clock source selection"]
        #[inline(always)]
        pub fn set_pllsrc(&mut self, val: super::vals::Pllsrc) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "Prescaler for PLL1"]
        #[inline(always)]
        pub const fn divm(&self, n: usize) -> super::vals::Pllm {
            assert!(n < 3usize);
            let offs = 4usize + n * 8usize;
            let val = (self.0 >> offs) & 0x3f;
            super::vals::Pllm::from_bits(val as u8)
        }
        #[doc = "Prescaler for PLL1"]
        #[inline(always)]
        pub fn set_divm(&mut self, n: usize, val: super::vals::Pllm) {
            assert!(n < 3usize);
            let offs = 4usize + n * 8usize;
            self.0 = (self.0 & !(0x3f << offs)) | (((val.to_bits() as u32) & 0x3f) << offs);
        }
    }
    impl Default for Pllckselr {
        #[inline(always)]
        fn default() -> Pllckselr {
            Pllckselr(0)
        }
    }
    #[doc = "RCC PLL1 Dividers Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Plldivr(pub u32);
    impl Plldivr {
        #[doc = "Multiplication factor for PLL1 VCO"]
        #[inline(always)]
        pub const fn plln(&self) -> super::vals::Plln {
            let val = (self.0 >> 0usize) & 0x01ff;
            super::vals::Plln::from_bits(val as u16)
        }
        #[doc = "Multiplication factor for PLL1 VCO"]
        #[inline(always)]
        pub fn set_plln(&mut self, val: super::vals::Plln) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val.to_bits() as u32) & 0x01ff) << 0usize);
        }
        #[doc = "PLL DIVP division factor"]
        #[inline(always)]
        pub const fn pllp(&self) -> super::vals::Plldiv {
            let val = (self.0 >> 9usize) & 0x7f;
            super::vals::Plldiv::from_bits(val as u8)
        }
        #[doc = "PLL DIVP division factor"]
        #[inline(always)]
        pub fn set_pllp(&mut self, val: super::vals::Plldiv) {
            self.0 = (self.0 & !(0x7f << 9usize)) | (((val.to_bits() as u32) & 0x7f) << 9usize);
        }
        #[doc = "PLL DIVQ division factor"]
        #[inline(always)]
        pub const fn pllq(&self) -> super::vals::Plldiv {
            let val = (self.0 >> 16usize) & 0x7f;
            super::vals::Plldiv::from_bits(val as u8)
        }
        #[doc = "PLL DIVQ division factor"]
        #[inline(always)]
        pub fn set_pllq(&mut self, val: super::vals::Plldiv) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val.to_bits() as u32) & 0x7f) << 16usize);
        }
        #[doc = "PLL DIVR division factor"]
        #[inline(always)]
        pub const fn pllr(&self) -> super::vals::Plldiv {
            let val = (self.0 >> 24usize) & 0x7f;
            super::vals::Plldiv::from_bits(val as u8)
        }
        #[doc = "PLL DIVR division factor"]
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
    #[doc = "RCC PLL Fractional Divider Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pllfracr(pub u32);
    impl Pllfracr {
        #[doc = "Fractional part of the multiplication factor for PLL VCO"]
        #[inline(always)]
        pub const fn fracn(&self) -> u16 {
            let val = (self.0 >> 3usize) & 0x1fff;
            val as u16
        }
        #[doc = "Fractional part of the multiplication factor for PLL VCO"]
        #[inline(always)]
        pub fn set_fracn(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 3usize)) | (((val as u32) & 0x1fff) << 3usize);
        }
    }
    impl Default for Pllfracr {
        #[inline(always)]
        fn default() -> Pllfracr {
            Pllfracr(0)
        }
    }
    #[doc = "RCC Reset Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rsr(pub u32);
    impl Rsr {
        #[doc = "Remove reset flag"]
        #[inline(always)]
        pub const fn rmvf(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Remove reset flag"]
        #[inline(always)]
        pub fn set_rmvf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "CPU reset flag"]
        #[inline(always)]
        pub const fn cpurstf(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "CPU reset flag"]
        #[inline(always)]
        pub fn set_cpurstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "D1 domain power switch reset flag"]
        #[inline(always)]
        pub const fn d1rstf(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "D1 domain power switch reset flag"]
        #[inline(always)]
        pub fn set_d1rstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "D2 domain power switch reset flag"]
        #[inline(always)]
        pub const fn d2rstf(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "D2 domain power switch reset flag"]
        #[inline(always)]
        pub fn set_d2rstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "BOR reset flag"]
        #[inline(always)]
        pub const fn borrstf(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "BOR reset flag"]
        #[inline(always)]
        pub fn set_borrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Pin reset flag (NRST)"]
        #[inline(always)]
        pub const fn pinrstf(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Pin reset flag (NRST)"]
        #[inline(always)]
        pub fn set_pinrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "POR/PDR reset flag"]
        #[inline(always)]
        pub const fn porrstf(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "POR/PDR reset flag"]
        #[inline(always)]
        pub fn set_porrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "System reset from CPU reset flag"]
        #[inline(always)]
        pub const fn sftrstf(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "System reset from CPU reset flag"]
        #[inline(always)]
        pub fn set_sftrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Independent Watchdog reset flag"]
        #[inline(always)]
        pub const fn iwdg1rstf(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Independent Watchdog reset flag"]
        #[inline(always)]
        pub fn set_iwdg1rstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Window Watchdog reset flag"]
        #[inline(always)]
        pub const fn wwdg1rstf(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Window Watchdog reset flag"]
        #[inline(always)]
        pub fn set_wwdg1rstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Reset due to illegal D1 DStandby or CPU CStop flag"]
        #[inline(always)]
        pub const fn lpwrrstf(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Reset due to illegal D1 DStandby or CPU CStop flag"]
        #[inline(always)]
        pub fn set_lpwrrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for Rsr {
        #[inline(always)]
        fn default() -> Rsr {
            Rsr(0)
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Adcsel {
        #[doc = "pll2_p selected as peripheral clock"]
        PLL2_P = 0,
        #[doc = "pll3_r selected as peripheral clock"]
        PLL3_R = 0x01,
        #[doc = "PER selected as peripheral clock"]
        PER = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Adcsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Adcsel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Adcsel {
        #[inline(always)]
        fn from(val: u8) -> Adcsel {
            Adcsel::from_bits(val)
        }
    }
    impl From<Adcsel> for u8 {
        #[inline(always)]
        fn from(val: Adcsel) -> u8 {
            Adcsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Cecsel {
        #[doc = "LSE selected as peripheral clock"]
        LSE = 0,
        #[doc = "LSI selected as peripheral clock"]
        LSI = 0x01,
        #[doc = "csi_ker selected as peripheral clock"]
        CSI = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Cecsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cecsel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cecsel {
        #[inline(always)]
        fn from(val: u8) -> Cecsel {
            Cecsel::from_bits(val)
        }
    }
    impl From<Cecsel> for u8 {
        #[inline(always)]
        fn from(val: Cecsel) -> u8 {
            Cecsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Ckpersel {
        #[doc = "HSI selected as peripheral clock"]
        HSI = 0,
        #[doc = "CSI selected as peripheral clock"]
        CSI = 0x01,
        #[doc = "HSE selected as peripheral clock"]
        HSE = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Ckpersel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ckpersel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ckpersel {
        #[inline(always)]
        fn from(val: u8) -> Ckpersel {
            Ckpersel::from_bits(val)
        }
    }
    impl From<Ckpersel> for u8 {
        #[inline(always)]
        fn from(val: Ckpersel) -> u8 {
            Ckpersel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Dfsdmsel {
        #[doc = "rcc_pclk2 selected as peripheral clock"]
        PCLK2 = 0,
        #[doc = "System clock selected as peripheral clock"]
        SYS = 0x01,
    }
    impl Dfsdmsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Dfsdmsel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Dfsdmsel {
        #[inline(always)]
        fn from(val: u8) -> Dfsdmsel {
            Dfsdmsel::from_bits(val)
        }
    }
    impl From<Dfsdmsel> for u8 {
        #[inline(always)]
        fn from(val: Dfsdmsel) -> u8 {
            Dfsdmsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Fdcansel {
        #[doc = "HSE selected as peripheral clock"]
        HSE = 0,
        #[doc = "pll1_q selected as peripheral clock"]
        PLL1_Q = 0x01,
        #[doc = "pll2_q selected as peripheral clock"]
        PLL2_Q = 0x02,
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
    pub enum Fmcsel {
        #[doc = "rcc_hclk3 selected as peripheral clock"]
        HCLK3 = 0,
        #[doc = "pll1_q selected as peripheral clock"]
        PLL1_Q = 0x01,
        #[doc = "pll2_r selected as peripheral clock"]
        PLL2_R = 0x02,
        #[doc = "PER selected as peripheral clock"]
        PER = 0x03,
    }
    impl Fmcsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Fmcsel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Fmcsel {
        #[inline(always)]
        fn from(val: u8) -> Fmcsel {
            Fmcsel::from_bits(val)
        }
    }
    impl From<Fmcsel> for u8 {
        #[inline(always)]
        fn from(val: Fmcsel) -> u8 {
            Fmcsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Hpre {
        #[doc = "sys_ck not divided"]
        DIV1 = 0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        #[doc = "sys_ck divided by 2"]
        DIV2 = 0x08,
        #[doc = "sys_ck divided by 4"]
        DIV4 = 0x09,
        #[doc = "sys_ck divided by 8"]
        DIV8 = 0x0a,
        #[doc = "sys_ck divided by 16"]
        DIV16 = 0x0b,
        #[doc = "sys_ck divided by 64"]
        DIV64 = 0x0c,
        #[doc = "sys_ck divided by 128"]
        DIV128 = 0x0d,
        #[doc = "sys_ck divided by 256"]
        DIV256 = 0x0e,
        #[doc = "sys_ck divided by 512"]
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
    pub enum Hrtimsel {
        #[doc = "The HRTIM prescaler clock source is the same as other timers (rcc_timy_ker_ck)"]
        TIMY_KER = 0,
        #[doc = "The HRTIM prescaler clock source is the CPU clock (c_ck)"]
        C_CK = 0x01,
    }
    impl Hrtimsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Hrtimsel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Hrtimsel {
        #[inline(always)]
        fn from(val: u8) -> Hrtimsel {
            Hrtimsel::from_bits(val)
        }
    }
    impl From<Hrtimsel> for u8 {
        #[inline(always)]
        fn from(val: Hrtimsel) -> u8 {
            Hrtimsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Hsidiv {
        #[doc = "No division"]
        DIV1 = 0,
        #[doc = "Division by 2"]
        DIV2 = 0x01,
        #[doc = "Division by 4"]
        DIV4 = 0x02,
        #[doc = "Division by 8"]
        DIV8 = 0x03,
    }
    impl Hsidiv {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Hsidiv {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Hsidiv {
        #[inline(always)]
        fn from(val: u8) -> Hsidiv {
            Hsidiv::from_bits(val)
        }
    }
    impl From<Hsidiv> for u8 {
        #[inline(always)]
        fn from(val: Hsidiv) -> u8 {
            Hsidiv::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum I2c1235sel {
        #[doc = "rcc_pclk1 selected as peripheral clock"]
        PCLK1 = 0,
        #[doc = "pll3_r selected as peripheral clock"]
        PLL3_R = 0x01,
        #[doc = "hsi_ker selected as peripheral clock"]
        HSI = 0x02,
        #[doc = "csi_ker selected as peripheral clock"]
        CSI = 0x03,
    }
    impl I2c1235sel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> I2c1235sel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for I2c1235sel {
        #[inline(always)]
        fn from(val: u8) -> I2c1235sel {
            I2c1235sel::from_bits(val)
        }
    }
    impl From<I2c1235sel> for u8 {
        #[inline(always)]
        fn from(val: I2c1235sel) -> u8 {
            I2c1235sel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum I2c4sel {
        #[doc = "rcc_pclk4 selected as peripheral clock"]
        PCLK4 = 0,
        #[doc = "pll3_r selected as peripheral clock"]
        PLL3_R = 0x01,
        #[doc = "hsi_ker selected as peripheral clock"]
        HSI = 0x02,
        #[doc = "csi_ker selected as peripheral clock"]
        CSI = 0x03,
    }
    impl I2c4sel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> I2c4sel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for I2c4sel {
        #[inline(always)]
        fn from(val: u8) -> I2c4sel {
            I2c4sel::from_bits(val)
        }
    }
    impl From<I2c4sel> for u8 {
        #[inline(always)]
        fn from(val: I2c4sel) -> u8 {
            I2c4sel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Lptim1sel {
        #[doc = "rcc_pclk1 selected as peripheral clock"]
        PCLK1 = 0,
        #[doc = "pll2_p selected as peripheral clock"]
        PLL2_P = 0x01,
        #[doc = "pll3_r selected as peripheral clock"]
        PLL3_R = 0x02,
        #[doc = "LSE selected as peripheral clock"]
        LSE = 0x03,
        #[doc = "LSI selected as peripheral clock"]
        LSI = 0x04,
        #[doc = "PER selected as peripheral clock"]
        PER = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Lptim1sel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lptim1sel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Lptim1sel {
        #[inline(always)]
        fn from(val: u8) -> Lptim1sel {
            Lptim1sel::from_bits(val)
        }
    }
    impl From<Lptim1sel> for u8 {
        #[inline(always)]
        fn from(val: Lptim1sel) -> u8 {
            Lptim1sel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Lptim2sel {
        #[doc = "rcc_pclk4 selected as peripheral clock"]
        PCLK4 = 0,
        #[doc = "pll2_p selected as peripheral clock"]
        PLL2_P = 0x01,
        #[doc = "pll3_r selected as peripheral clock"]
        PLL3_R = 0x02,
        #[doc = "LSE selected as peripheral clock"]
        LSE = 0x03,
        #[doc = "LSI selected as peripheral clock"]
        LSI = 0x04,
        #[doc = "PER selected as peripheral clock"]
        PER = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Lptim2sel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lptim2sel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Lptim2sel {
        #[inline(always)]
        fn from(val: u8) -> Lptim2sel {
            Lptim2sel::from_bits(val)
        }
    }
    impl From<Lptim2sel> for u8 {
        #[inline(always)]
        fn from(val: Lptim2sel) -> u8 {
            Lptim2sel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Lpuartsel {
        #[doc = "rcc_pclk_d3 selected as peripheral clock"]
        PCLK3 = 0,
        #[doc = "pll2_q selected as peripheral clock"]
        PLL2_Q = 0x01,
        #[doc = "pll3_q selected as peripheral clock"]
        PLL3_Q = 0x02,
        #[doc = "hsi_ker selected as peripheral clock"]
        HSI = 0x03,
        #[doc = "csi_ker selected as peripheral clock"]
        CSI = 0x04,
        #[doc = "LSE selected as peripheral clock"]
        LSE = 0x05,
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
    pub enum Mco1sel {
        #[doc = "HSI selected for micro-controller clock output"]
        HSI = 0,
        #[doc = "LSE selected for micro-controller clock output"]
        LSE = 0x01,
        #[doc = "HSE selected for micro-controller clock output"]
        HSE = 0x02,
        #[doc = "pll1_q selected for micro-controller clock output"]
        PLL1_Q = 0x03,
        #[doc = "HSI48 selected for micro-controller clock output"]
        HSI48 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Mco1sel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Mco1sel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Mco1sel {
        #[inline(always)]
        fn from(val: u8) -> Mco1sel {
            Mco1sel::from_bits(val)
        }
    }
    impl From<Mco1sel> for u8 {
        #[inline(always)]
        fn from(val: Mco1sel) -> u8 {
            Mco1sel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Mco2sel {
        #[doc = "System clock selected for micro-controller clock output"]
        SYS = 0,
        #[doc = "pll2_p selected for micro-controller clock output"]
        PLL2_P = 0x01,
        #[doc = "HSE selected for micro-controller clock output"]
        HSE = 0x02,
        #[doc = "pll1_p selected for micro-controller clock output"]
        PLL1_P = 0x03,
        #[doc = "CSI selected for micro-controller clock output"]
        CSI = 0x04,
        #[doc = "LSI selected for micro-controller clock output"]
        LSI = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Mco2sel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Mco2sel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Mco2sel {
        #[inline(always)]
        fn from(val: u8) -> Mco2sel {
            Mco2sel::from_bits(val)
        }
    }
    impl From<Mco2sel> for u8 {
        #[inline(always)]
        fn from(val: Mco2sel) -> u8 {
            Mco2sel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Mcopre {
        _RESERVED_0 = 0,
        #[doc = "Divide by 1"]
        DIV1 = 0x01,
        #[doc = "Divide by 2"]
        DIV2 = 0x02,
        #[doc = "Divide by 3"]
        DIV3 = 0x03,
        #[doc = "Divide by 4"]
        DIV4 = 0x04,
        #[doc = "Divide by 5"]
        DIV5 = 0x05,
        #[doc = "Divide by 6"]
        DIV6 = 0x06,
        #[doc = "Divide by 7"]
        DIV7 = 0x07,
        #[doc = "Divide by 8"]
        DIV8 = 0x08,
        #[doc = "Divide by 9"]
        DIV9 = 0x09,
        #[doc = "Divide by 10"]
        DIV10 = 0x0a,
        #[doc = "Divide by 11"]
        DIV11 = 0x0b,
        #[doc = "Divide by 12"]
        DIV12 = 0x0c,
        #[doc = "Divide by 13"]
        DIV13 = 0x0d,
        #[doc = "Divide by 14"]
        DIV14 = 0x0e,
        #[doc = "Divide by 15"]
        DIV15 = 0x0f,
    }
    impl Mcopre {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Mcopre {
            unsafe { core::mem::transmute(val & 0x0f) }
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
        _RESERVED_0 = 0,
        DIV1 = 0x01,
        DIV2 = 0x02,
        DIV3 = 0x03,
        DIV4 = 0x04,
        DIV5 = 0x05,
        DIV6 = 0x06,
        DIV7 = 0x07,
        DIV8 = 0x08,
        DIV9 = 0x09,
        DIV10 = 0x0a,
        DIV11 = 0x0b,
        DIV12 = 0x0c,
        DIV13 = 0x0d,
        DIV14 = 0x0e,
        DIV15 = 0x0f,
        DIV16 = 0x10,
        DIV17 = 0x11,
        DIV18 = 0x12,
        DIV19 = 0x13,
        DIV20 = 0x14,
        DIV21 = 0x15,
        DIV22 = 0x16,
        DIV23 = 0x17,
        DIV24 = 0x18,
        DIV25 = 0x19,
        DIV26 = 0x1a,
        DIV27 = 0x1b,
        DIV28 = 0x1c,
        DIV29 = 0x1d,
        DIV30 = 0x1e,
        DIV31 = 0x1f,
        DIV32 = 0x20,
        DIV33 = 0x21,
        DIV34 = 0x22,
        DIV35 = 0x23,
        DIV36 = 0x24,
        DIV37 = 0x25,
        DIV38 = 0x26,
        DIV39 = 0x27,
        DIV40 = 0x28,
        DIV41 = 0x29,
        DIV42 = 0x2a,
        DIV43 = 0x2b,
        DIV44 = 0x2c,
        DIV45 = 0x2d,
        DIV46 = 0x2e,
        DIV47 = 0x2f,
        DIV48 = 0x30,
        DIV49 = 0x31,
        DIV50 = 0x32,
        DIV51 = 0x33,
        DIV52 = 0x34,
        DIV53 = 0x35,
        DIV54 = 0x36,
        DIV55 = 0x37,
        DIV56 = 0x38,
        DIV57 = 0x39,
        DIV58 = 0x3a,
        DIV59 = 0x3b,
        DIV60 = 0x3c,
        DIV61 = 0x3d,
        DIV62 = 0x3e,
        _RESERVED_3f = 0x3f,
    }
    impl Pllm {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pllm {
            unsafe { core::mem::transmute(val & 0x3f) }
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
        #[doc = "Frequency is between 1 and 2 MHz"]
        RANGE1 = 0,
        #[doc = "Frequency is between 2 and 4 MHz"]
        RANGE2 = 0x01,
        #[doc = "Frequency is between 4 and 8 MHz"]
        RANGE4 = 0x02,
        #[doc = "Frequency is between 8 and 16 MHz"]
        RANGE8 = 0x03,
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
        #[doc = "HSI selected as PLL clock"]
        HSI = 0,
        #[doc = "CSI selected as PLL clock"]
        CSI = 0x01,
        #[doc = "HSE selected as PLL clock"]
        HSE = 0x02,
        #[doc = "No clock sent to DIVMx dividers and PLLs"]
        DISABLE = 0x03,
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
    pub enum Pllvcosel {
        #[doc = "VCO frequency range 192 to 836 MHz"]
        WIDEVCO = 0,
        #[doc = "VCO frequency range 150 to 420 MHz"]
        MEDIUMVCO = 0x01,
    }
    impl Pllvcosel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pllvcosel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pllvcosel {
        #[inline(always)]
        fn from(val: u8) -> Pllvcosel {
            Pllvcosel::from_bits(val)
        }
    }
    impl From<Pllvcosel> for u8 {
        #[inline(always)]
        fn from(val: Pllvcosel) -> u8 {
            Pllvcosel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Ppre {
        #[doc = "rcc_hclk not divided"]
        DIV1 = 0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        #[doc = "rcc_hclk divided by 2"]
        DIV2 = 0x04,
        #[doc = "rcc_hclk divided by 4"]
        DIV4 = 0x05,
        #[doc = "rcc_hclk divided by 8"]
        DIV8 = 0x06,
        #[doc = "rcc_hclk divided by 16"]
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
    pub enum Rngsel {
        #[doc = "HSI48 selected as peripheral clock"]
        HSI48 = 0,
        #[doc = "pll1_q selected as peripheral clock"]
        PLL1_Q = 0x01,
        #[doc = "LSE selected as peripheral clock"]
        LSE = 0x02,
        #[doc = "LSI selected as peripheral clock"]
        LSI = 0x03,
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
        #[doc = "No clock"]
        DISABLE = 0,
        #[doc = "LSE oscillator clock used as RTC clock"]
        LSE = 0x01,
        #[doc = "LSI oscillator clock used as RTC clock"]
        LSI = 0x02,
        #[doc = "HSE oscillator clock divided by a prescaler used as RTC clock"]
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
    pub enum Saiasel {
        #[doc = "pll1_q selected as peripheral clock"]
        PLL1_Q = 0,
        #[doc = "pll2_p selected as peripheral clock"]
        PLL2_P = 0x01,
        #[doc = "pll3_p selected as peripheral clock"]
        PLL3_P = 0x02,
        #[doc = "i2s_ckin selected as peripheral clock"]
        I2S_CKIN = 0x03,
        #[doc = "PER selected as peripheral clock"]
        PER = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Saiasel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Saiasel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Saiasel {
        #[inline(always)]
        fn from(val: u8) -> Saiasel {
            Saiasel::from_bits(val)
        }
    }
    impl From<Saiasel> for u8 {
        #[inline(always)]
        fn from(val: Saiasel) -> u8 {
            Saiasel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Saisel {
        #[doc = "pll1_q selected as peripheral clock"]
        PLL1_Q = 0,
        #[doc = "pll2_p selected as peripheral clock"]
        PLL2_P = 0x01,
        #[doc = "pll3_p selected as peripheral clock"]
        PLL3_P = 0x02,
        #[doc = "I2S_CKIN selected as peripheral clock"]
        I2S_CKIN = 0x03,
        #[doc = "PER selected as peripheral clock"]
        PER = 0x04,
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
        #[doc = "pll1_q selected as peripheral clock"]
        PLL1_Q = 0,
        #[doc = "pll2_r selected as peripheral clock"]
        PLL2_R = 0x01,
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
    pub enum Spdifrxsel {
        #[doc = "pll1_q selected as peripheral clock"]
        PLL1_Q = 0,
        #[doc = "pll2_r selected as peripheral clock"]
        PLL2_R = 0x01,
        #[doc = "pll3_r selected as peripheral clock"]
        PLL3_R = 0x02,
        #[doc = "hsi_ker selected as peripheral clock"]
        HSI = 0x03,
    }
    impl Spdifrxsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Spdifrxsel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Spdifrxsel {
        #[inline(always)]
        fn from(val: u8) -> Spdifrxsel {
            Spdifrxsel::from_bits(val)
        }
    }
    impl From<Spdifrxsel> for u8 {
        #[inline(always)]
        fn from(val: Spdifrxsel) -> u8 {
            Spdifrxsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Spi45sel {
        #[doc = "APB clock selected as peripheral clock"]
        APB = 0,
        #[doc = "pll2_q selected as peripheral clock"]
        PLL2_Q = 0x01,
        #[doc = "pll3_q selected as peripheral clock"]
        PLL3_Q = 0x02,
        #[doc = "hsi_ker selected as peripheral clock"]
        HSI = 0x03,
        #[doc = "csi_ker selected as peripheral clock"]
        CSI = 0x04,
        #[doc = "HSE selected as peripheral clock"]
        HSE = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Spi45sel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Spi45sel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Spi45sel {
        #[inline(always)]
        fn from(val: u8) -> Spi45sel {
            Spi45sel::from_bits(val)
        }
    }
    impl From<Spi45sel> for u8 {
        #[inline(always)]
        fn from(val: Spi45sel) -> u8 {
            Spi45sel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Spi6sel {
        #[doc = "rcc_pclk4 selected as peripheral clock"]
        PCLK4 = 0,
        #[doc = "pll2_q selected as peripheral clock"]
        PLL2_Q = 0x01,
        #[doc = "pll3_q selected as peripheral clock"]
        PLL3_Q = 0x02,
        #[doc = "hsi_ker selected as peripheral clock"]
        HSI = 0x03,
        #[doc = "csi_ker selected as peripheral clock"]
        CSI = 0x04,
        #[doc = "HSE selected as peripheral clock"]
        HSE = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Spi6sel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Spi6sel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Spi6sel {
        #[inline(always)]
        fn from(val: u8) -> Spi6sel {
            Spi6sel::from_bits(val)
        }
    }
    impl From<Spi6sel> for u8 {
        #[inline(always)]
        fn from(val: Spi6sel) -> u8 {
            Spi6sel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Stopwuck {
        #[doc = "HSI selected as wake up clock from system Stop"]
        HSI = 0,
        #[doc = "CSI selected as wake up clock from system Stop"]
        CSI = 0x01,
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
        #[doc = "HSI selected as system clock"]
        HSI = 0,
        #[doc = "CSI selected as system clock"]
        CSI = 0x01,
        #[doc = "HSE selected as system clock"]
        HSE = 0x02,
        #[doc = "PLL1 selected as system clock"]
        PLL1_P = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Sw {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Sw {
            unsafe { core::mem::transmute(val & 0x07) }
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
    pub enum Swpsel {
        #[doc = "pclk selected as peripheral clock"]
        PCLK = 0,
        #[doc = "hsi_ker selected as peripheral clock"]
        HSI = 0x01,
    }
    impl Swpsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Swpsel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Swpsel {
        #[inline(always)]
        fn from(val: u8) -> Swpsel {
            Swpsel::from_bits(val)
        }
    }
    impl From<Swpsel> for u8 {
        #[inline(always)]
        fn from(val: Swpsel) -> u8 {
            Swpsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Timpre {
        #[doc = "Timer kernel clock equal to 2x pclk by default"]
        DEFAULTX2 = 0,
        #[doc = "Timer kernel clock equal to 4x pclk by default"]
        DEFAULTX4 = 0x01,
    }
    impl Timpre {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Timpre {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Timpre {
        #[inline(always)]
        fn from(val: u8) -> Timpre {
            Timpre::from_bits(val)
        }
    }
    impl From<Timpre> for u8 {
        #[inline(always)]
        fn from(val: Timpre) -> u8 {
            Timpre::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Usart16910sel {
        #[doc = "rcc_pclk2 selected as peripheral clock"]
        PCLK2 = 0,
        #[doc = "pll2_q selected as peripheral clock"]
        PLL2_Q = 0x01,
        #[doc = "pll3_q selected as peripheral clock"]
        PLL3_Q = 0x02,
        #[doc = "hsi_ker selected as peripheral clock"]
        HSI = 0x03,
        #[doc = "csi_ker selected as peripheral clock"]
        CSI = 0x04,
        #[doc = "LSE selected as peripheral clock"]
        LSE = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Usart16910sel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Usart16910sel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Usart16910sel {
        #[inline(always)]
        fn from(val: u8) -> Usart16910sel {
            Usart16910sel::from_bits(val)
        }
    }
    impl From<Usart16910sel> for u8 {
        #[inline(always)]
        fn from(val: Usart16910sel) -> u8 {
            Usart16910sel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Usart234578sel {
        #[doc = "rcc_pclk1 selected as peripheral clock"]
        PCLK1 = 0,
        #[doc = "pll2_q selected as peripheral clock"]
        PLL2_Q = 0x01,
        #[doc = "pll3_q selected as peripheral clock"]
        PLL3_Q = 0x02,
        #[doc = "hsi_ker selected as peripheral clock"]
        HSI = 0x03,
        #[doc = "csi_ker selected as peripheral clock"]
        CSI = 0x04,
        #[doc = "LSE selected as peripheral clock"]
        LSE = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Usart234578sel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Usart234578sel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Usart234578sel {
        #[inline(always)]
        fn from(val: u8) -> Usart234578sel {
            Usart234578sel::from_bits(val)
        }
    }
    impl From<Usart234578sel> for u8 {
        #[inline(always)]
        fn from(val: Usart234578sel) -> u8 {
            Usart234578sel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Usbsel {
        #[doc = "Disable the kernel clock"]
        DISABLE = 0,
        #[doc = "pll1_q selected as peripheral clock"]
        PLL1_Q = 0x01,
        #[doc = "pll3_q selected as peripheral clock"]
        PLL3_Q = 0x02,
        #[doc = "HSI48 selected as peripheral clock"]
        HSI48 = 0x03,
    }
    impl Usbsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Usbsel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Usbsel {
        #[inline(always)]
        fn from(val: u8) -> Usbsel {
            Usbsel::from_bits(val)
        }
    }
    impl From<Usbsel> for u8 {
        #[inline(always)]
        fn from(val: Usbsel) -> u8 {
            Usbsel::to_bits(val)
        }
    }
}
