use crate::{addr::VirtualAddress, error::Result, fdt::FdtHeader};
use core::{mem::size_of, slice};

#[derive(Debug)]
pub struct DeviceTree<'a> {
    data: &'a [u8],
}

impl<'a> DeviceTree<'a> {
    pub fn new(fdt_addr: VirtualAddress) -> Result<Self> {
        let fdt_header = unsafe { &*(fdt_addr.as_ptr() as *const FdtHeader) };
        if !fdt_header.is_valid() {
            return Err("Invalid FDT header".into());
        }

        let total_size = fdt_header.totalsize() as usize;
        if total_size < size_of::<FdtHeader>() {
            return Err("Invalid FDT size".into());
        }

        let data = unsafe { slice::from_raw_parts(fdt_addr.as_ptr(), total_size) };
        Ok(Self { data })
    }

    fn fdt_header(&self) -> &FdtHeader {
        unsafe { &*(self.data.as_ptr() as *const FdtHeader) }
    }
}
