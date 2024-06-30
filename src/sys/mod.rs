// #![allow(
//     clippy::unreadable_literal,
//     clippy::missing_safety_doc,
//     clippy::incorrect_clone_impl_on_copy_type
// )]

//! Nvme specification:
//! https://nvmexpress.org/wp-content/uploads/NVM-Express-NVM-Command-Set-Specification-2021.06.02-Ratified.pdf

use std::mem::transmute;

use generated::nvme_uring_cmd;
use io_uring::opcode::UringCmd80;
use rustix::ioctl::Opcode;

#[allow(non_camel_case_types)]
pub mod generated;

pub const NVME_URING_CMD_IO: Opcode = Opcode::read_write::<generated::nvme_uring_cmd>(b'N', 0x80);
pub const NVME_URING_CMD_IO_VEC: Opcode =
    Opcode::read_write::<generated::nvme_uring_cmd>(b'N', 0x81);

// Logical block address
// https://en.wikipedia.org/wiki/Logical_block_addressing
pub struct Lba(pub u64);

// Nvme opcodes as per specification.
// enum nvme_opcode in nvme.h
#[repr(C)]
pub enum NvmeOpcode {
    Flush = 0x00,
    Write = 0x01,
    Read = 0x02,
    WriteUncor = 0x04,
    Compare = 0x05,
    WriteZeroes = 0x08,
    Dsm = 0x09,
    Verify = 0x0c,
    ResvRegister = 0x0d,
    ResvReport = 0x0e,
    ResvAcquire = 0x11,
    ResvRelease = 0x15,
    ZoneMgmtSend = 0x79,
    ZoneMgmtRecv = 0x7a,
    ZoneAppend = 0x7d,
    VendorStart = 0x80,
}

#[repr(C)]
#[derive(Default, Debug)]
pub struct NvmeUringCmd {
    inner: nvme_uring_cmd,
    // pad to the command size
    pad: [u8; 8],
}

impl NvmeUringCmd {
    // TODO unsafe, because of buffers
    // TODO how to use registered buffers
    // Constructs a Read command. For details see 3.2.4 Read Command in the spec
    pub unsafe fn read(nsid: u32, starting_lba: Lba, num_blocks: u32, buf: &mut [u8]) -> Self {
        let mut cmd = nvme_uring_cmd::default();
        cmd.opcode = NvmeOpcode::Read as u8;
        cmd.nsid = nsid;
        // Starting LBA (SLBA): This field indicates the 64-bit address of the first logical block to be read
        // as part of the operation. Command Dword 10 contains bits 31:00; Command Dword 11 contains
        // bits 63: 32.
        cmd.cdw10 = (starting_lba.0 & 0xffffffff) as u32;
        cmd.cdw11 = (starting_lba.0 >> 32) as u32;
        // cdw12 can contain a lot of other stuff we're not interested in yet (TODO)
        // Number of Logical Blocks (NLB): This field indicates the number of logical blocks to be read.
        // This is a 0’s based value.
        cmd.cdw12 = num_blocks - 1;

        cmd.addr = buf.as_mut_ptr() as u64;
        cmd.data_len = buf.len() as u32; // TODO

        Self {
            inner: cmd,
            pad: Default::default(),
        }
    }

    pub unsafe fn write(nsid: u32, starting_lba: Lba, num_blocks: u32, buf: &[u8]) -> Self {
        let mut cmd = nvme_uring_cmd::default();
        cmd.opcode = NvmeOpcode::Write as u8;
        cmd.nsid = nsid;
        // Starting LBA (SLBA): This field indicates the 64-bit address of the first logical block to be read
        // as part of the operation. Command Dword 10 contains bits 31:00; Command Dword 11 contains
        // bits 63: 32.
        cmd.cdw10 = (starting_lba.0 & 0xffffffff) as u32;
        cmd.cdw11 = (starting_lba.0 >> 32) as u32;
        // cdw12 can contain a lot of other stuff we're not interested in yet (TODO)
        // Number of Logical Blocks (NLB): This field indicates the number of logical blocks to be read.
        // This is a 0’s based value.
        cmd.cdw12 = num_blocks - 1;

        cmd.addr = buf.as_ptr() as u64;
        cmd.data_len = buf.len() as u32; // TODO

        Self {
            inner: cmd,
            pad: Default::default(),
        }
    }

    pub unsafe fn into_uring_cmd(self, fd: io_uring::types::Fd) -> UringCmd80 {
        UringCmd80::new(fd, NVME_URING_CMD_IO.raw()).cmd(transmute(self))
    }
}

const _: () = assert!(std::mem::size_of::<NvmeUringCmd>() == 80);
