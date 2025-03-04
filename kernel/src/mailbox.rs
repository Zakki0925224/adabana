use crate::addr::MmioAddress;

// https://github.com/raspberrypi/firmware/wiki/Mailbox-property-interface

fn mmio_base_mailbox_videocore() -> MmioAddress {
    MmioAddress::new(0xb880)
}

fn read_mailbox_rw() -> u32 {
    mmio_base_mailbox_videocore().offset(0x00).read()
}

fn read_mailbox_peek() -> u32 {
    mmio_base_mailbox_videocore().offset(0x10).read()
}

fn read_mailbox_sender() -> u32 {
    mmio_base_mailbox_videocore().offset(0x14).read()
}

fn read_mailbox_status() -> u32 {
    mmio_base_mailbox_videocore().offset(0x18).read()
}

fn read_mailbox_config() -> u32 {
    mmio_base_mailbox_videocore().offset(0x1c).read()
}

#[repr(u8)]
pub enum Channel {
    PowerManagement = 0,
    Framebuffer = 1,
    VirtualUart = 2,
    Vchiq = 3,
    Led = 4,
    Button = 5,
    Touchscreen = 6,
    Count = 7,
    PropertyTags = 8,
}

#[repr(u32)]
pub enum TagId {
    VideoCoreGetFirmwareVersion = 0x1,
    HardwareGetBoardModel = 0x10001,
    HardwareGetBoardRevision = 0x10002,
    HardwareGetBoardMacAddress = 0x10003,
    HardwareGetBoardSerial = 0x10004,
    HardwareGetArmMemory = 0x10005,
    HardwareGetVideoCoreMemory = 0x10006,
    HardwareGetClocks = 0x10007,
    ConfigGetCommandLine = 0x50001,
    SharedResourceManagementGetDmaChannels = 0x60001,
    PowerGetPowerState = 0x20001,
    PowerGetTiming = 0x20002,
    PowerSetPowerState = 0x28001,
    ClocksGetClockState = 0x30001,
    ClocksSetClockState = 0x38001,
    ClocksGetClockRate = 0x30002,
    ClocksGetOnboardLedState = 0x30041,
    ClocksTestOnboardLedState = 0x34041,
    ClocksSetOnboardLedState = 0x38041,
    ClocksGetClockRateMeasured = 0x30047,
    ClocksSetClockRate = 0x38002,
    ClocksGetMaxClockRate = 0x30004,
    ClocksGetMinClockRate = 0x30007,
    ClocksGetTurbo = 0x30009,
    ClocksSetTurbo = 0x38009,
    VoltageGetVoltage = 0x30003,
    VoltageSetVoltage = 0x38003,
    VoltageGetMaxVoltage = 0x30005,
    VoltageGetMinVoltage = 0x30008,
    VoltageGetTemperature = 0x30006,
    VoltageGetMaxTemperature = 0x3000a,
    VoltageAllocateMemory = 0x3000c,
    VoltageLockMemory = 0x3000d,
    VoltageUnlockMemory = 0x3000e,
    VoltageReleaseMemory = 0x3000f,
    VoltageExecuteCode = 0x30010,
    VoltageGetDispmanxResourceMemoryHandle = 0x30014,
    VoltageGetEdidBlock = 0x30020,
    FramebufferAllocateBuffer = 0x40001,
    FramebufferReleaseBuffer = 0x48001,
    FramebufferBlankScreen = 0x40002,
    FramebufferGetPhysicalWidthHeight = 0x40003,
    FramebufferTestPhysicalWidthHeight = 0x44003,
    FramebufferSetPhysicalWidthHeight = 0x48003,
    FramebufferGetVirtualWidthHeight = 0x40004,
    FramebufferTestVirtualWidthHeight = 0x44004,
    FramebufferSetVirtualWidthHeight = 0x48004,
    FramebufferGetDepth = 0x40005,
    FramebufferTestDepth = 0x44005,
    FramebufferSetDepth = 0x48005,
    FramebufferGetPixelOrder = 0x40006,
    FramebufferTestPixelOrder = 0x44006,
    FramebufferSetPixelOrder = 0x48006,
    FramebufferGetAlphaMode = 0x40007,
    FramebufferTestAlphaMode = 0x44007,
    FramebufferSetAlphaMode = 0x48007,
    FramebufferGetPitch = 0x40008,
    FramebufferGetVirtualOffset = 0x40009,
    FramebufferTestVirtualOffset = 0x44009,
    FramebufferSetVirtualOffset = 0x48009,
    FramebufferOverscan = 0x4000a,
    FramebufferTestOverscan = 0x4400a,
    FramebufferSetOverscan = 0x4800a,
    FramebufferGetPalette = 0x4000b,
    FramebufferTestPalette = 0x4400b,
    FramebufferSetPalette = 0x4800b,
    FramebufferSetCursorInfo = 0x8010,
    FramebufferSetCursorState = 0x8011,
    FramebufferSetScreenGamma = 0x8012,
}

#[repr(u32)]
pub enum Status {
    Request = 0,
    Response = 0x80000000,
    ResponseError = 0x80000001,
}

#[repr(C, align(16))]
pub struct Mailbox([u32; 36]);

impl Mailbox {
    pub fn new() -> Self {
        Self([0; 36])
    }

    fn buf(&self) -> *const u32 {
        self.0.as_ptr()
    }

    fn buf_mut(&mut self) -> *mut u32 {
        self.0.as_mut_ptr()
    }

    fn read_size(&self) -> u32 {
        unsafe { *self.buf().offset(0x00) }
    }

    fn write_size(&mut self, size: u32) {
        unsafe { *self.buf_mut().offset(0x00) = size }
    }
}
