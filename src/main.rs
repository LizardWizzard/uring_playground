use std::fs;
use std::io;
use std::os::unix::io::AsRawFd;

use io_uring::types::Fd;
use io_uring::{cqueue, squeue, IoUring};
use sys::Lba;
use sys::NvmeUringCmd;

mod sys;

fn main() -> io::Result<()> {
    // Params
    let path = "/dev/ng0n1";
    let lba: u64 = 4096;
    let num_blocks: u32 = 1;

    // sudo nvme id-ns /dev/nvme0n1
    let nsid = 1;

    let builder = IoUring::<squeue::Entry128, cqueue::Entry32>::builder();

    let mut ring = builder.build(128)?;

    let fd = fs::File::open(path)?;

    let mut buf = vec![0u8; 1024];
//     let text = "
// One Ring to rule them all, One Ring to find them,
// One Ring to bring them all and in the darkness bind them
//     ";
//     buf[..112].copy_from_slice(text.as_bytes());
    let nvme_cmd = unsafe { NvmeUringCmd::read(nsid, Lba(lba), num_blocks, &mut buf) };
    println!("cmd: {:?}", nvme_cmd);

    unsafe {
        ring.submission()
            .push(
                &nvme_cmd
                    .into_uring_cmd(Fd(fd.as_raw_fd()))
                    .build()
                    .user_data(0x22),
            )
            .expect("submission queue is full");
    }
    ring.submit_and_wait(1)?;

    let cqe = ring.completion().next().expect("completion queue is empty");

    assert_eq!(cqe.user_data(), 0x22);

    let content = String::from_utf8_lossy(&buf);
    println!("bytes read: {:?}", content);
    println!("bytes read: {:?}", buf);

    println!("Hello, world!");
    Ok(())
}
