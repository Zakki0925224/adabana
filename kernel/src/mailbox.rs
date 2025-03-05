use crate::{addr::MmioAddress, error::Result, println};

// https://github.com/raspberrypi/firmware/wiki/Mailbox-property-interface
// https://github.com/qemu/qemu/blob/master/hw/misc/bcm2835_property.c

fn mmio_base_mailbox() -> MmioAddress {
    MmioAddress::new(0xb880)
}

fn read_mailbox_rw() -> u32 {
    mmio_base_mailbox().offset(0x00).read()
}

fn read_mailbox_peek() -> u32 {
    mmio_base_mailbox().offset(0x10).read()
}

fn read_mailbox_sender() -> u32 {
    mmio_base_mailbox().offset(0x14).read()
}

fn read_mailbox_status() -> u32 {
    mmio_base_mailbox().offset(0x18).read()
}

fn read_mailbox_config() -> u32 {
    mmio_base_mailbox().offset(0x1c).read()
}

fn write_mailbox(mbox: &Mailbox, channel: Channel) {
    let mbox_addr = mbox.inner_ptr() as u32;
    assert!(mbox_addr & 0xf == 0);
    let channel = channel as u32;
    mmio_base_mailbox().offset(0x20).write(mbox_addr | channel);
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
enum Channel {
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
enum TagId {
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
enum TagStatus {
    Request = 0,
    Response = 0x80000000,
    ResponseError = 0x80000001,
}

const TAG_LAST: u32 = 0;

#[repr(C)]
struct Tag<const N: usize>([u32; N]);

impl<const N: usize> Tag<N> {
    fn new(id: TagId, status: TagStatus) -> Self {
        assert!(N >= 4);

        let mut tags = [0; N];
        tags[0] = id as u32;
        tags[1] = ((N - 3) * 4) as u32;
        tags[2] = status as u32;
        Self(tags)
    }

    fn slice(&self) -> &[u32] {
        &self.0
    }

    fn slice_mut(&mut self) -> &mut [u32] {
        &mut self.0
    }
}

#[repr(C, align(16))]
struct Mailbox([u32; 36]);

impl Mailbox {
    fn new() -> Self {
        Self([0; 36])
    }

    fn inner_slice(&self) -> &[u32] {
        &self.0
    }

    fn inner_slice_mut(&mut self) -> &mut [u32] {
        &mut self.0
    }

    fn inner_ptr(&self) -> *const u32 {
        self.inner_slice().as_ptr()
    }

    fn read_size(&self) -> u32 {
        self.inner_slice()[0]
    }

    fn write_size(&mut self, size: u32) {
        self.inner_slice_mut()[0] = size;
    }

    fn read_request_code(&self) -> u32 {
        self.inner_slice()[1]
    }

    fn write_request_code(&mut self, code: u32) {
        self.inner_slice_mut()[1] = code;
    }

    fn write_tag(&mut self, tag: &[u32]) -> Result<usize> {
        let mut offset = 2;
        let slice_mut = self.inner_slice_mut();

        while slice_mut[offset] != TAG_LAST && offset < slice_mut.len() {
            offset += 1;
        }

        if offset + tag.len() >= slice_mut.len() {
            return Err("Mailbox buffer is full".into());
        }

        for (i, &value) in tag.iter().enumerate() {
            slice_mut[offset + i] = value;
        }

        self.write_size(self.read_size() + tag.len() as u32 * 4);

        Ok(offset)
    }

    fn call(&self, channel: Channel) -> Result<()> {
        // println!("mailbox: {:?}", self.inner_slice());

        // wait until can write to the mailbox
        while read_mailbox_status() & 0x80000000 != 0 {}

        // write
        write_mailbox(self, channel);

        loop {
            // wait until can read from the mailbox
            while read_mailbox_status() & 0x40000000 != 0 {}
            let res = read_mailbox_rw();

            if ((res & 0xf) == channel as u32) && ((res & !0xf) == self.inner_ptr() as u32) {
                break;
            }
        }

        let status = self.read_request_code();
        if status == TagStatus::Response as u32 {
            // println!("mailbox: {:?}", self.inner_slice());
            Ok(())
        } else if status == TagStatus::ResponseError as u32 {
            Err("Mailbox response error".into())
        } else {
            Err("Unknown mailbox response".into())
        }
    }
}

pub fn get_firmware_revision() -> Result<u32> {
    let mut mbox = Mailbox::new();
    let mut tag: Tag<5> = Tag::new(TagId::VideoCoreGetFirmwareVersion, TagStatus::Request);
    let tag_s = tag.slice_mut();
    tag_s[3] = 0; // response buffer
    tag_s[4] = TAG_LAST; // last
    let offset = mbox.write_tag(tag.slice())?;
    mbox.call(Channel::PropertyTags)?;
    let tag_s: &[u32] = &mbox.inner_slice()[offset..offset + 5];

    if tag_s[2] & TagStatus::Response as u32 == 0 {
        return Err("Mailbox response error".into());
    }

    Ok(tag_s[3])
}

pub fn get_board_serial() -> Result<u64> {
    let mut mbox = Mailbox::new();
    let mut tag: Tag<8> = Tag::new(TagId::HardwareGetBoardSerial, TagStatus::Request);
    let tag_s = tag.slice_mut();
    tag_s[3] = 8; // buffer size
    tag_s[4] = 8; // response buffer size
    tag_s[5] = 0; // response buffer
    tag_s[6] = 0;
    tag_s[7] = TAG_LAST; // last
    let offset = mbox.write_tag(tag.slice())?;
    mbox.call(Channel::PropertyTags)?;
    let tag_s: &[u32] = &mbox.inner_slice()[offset..offset + 8];

    if tag_s[2] & TagStatus::Response as u32 == 0 {
        return Err("Mailbox response error".into());
    }

    let serial = (tag_s[6] as u64) << 32 | tag_s[5] as u64;
    Ok(serial)
}
