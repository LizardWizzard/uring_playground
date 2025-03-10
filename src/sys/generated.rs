/* automatically generated by rust-bindgen 0.69.4 */

pub type __u8 = libc::c_uchar;
pub type __u16 = libc::c_ushort;
pub type __u32 = libc::c_uint;
pub type __u64 = libc::c_ulonglong;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct nvme_uring_cmd {
    pub opcode: __u8,
    pub flags: __u8,
    pub rsvd1: __u16,
    pub nsid: __u32,
    pub cdw2: __u32,
    pub cdw3: __u32,
    pub metadata: __u64,
    pub addr: __u64,
    pub metadata_len: __u32,
    pub data_len: __u32,
    pub cdw10: __u32,
    pub cdw11: __u32,
    pub cdw12: __u32,
    pub cdw13: __u32,
    pub cdw14: __u32,
    pub cdw15: __u32,
    pub timeout_ms: __u32,
    pub rsvd2: __u32,
}
#[test]
fn bindgen_test_layout_nvme_uring_cmd() {
    const UNINIT: ::core::mem::MaybeUninit<nvme_uring_cmd> = ::core::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::core::mem::size_of::<nvme_uring_cmd>(),
        72usize,
        concat!("Size of: ", stringify!(nvme_uring_cmd))
    );
    assert_eq!(
        ::core::mem::align_of::<nvme_uring_cmd>(),
        8usize,
        concat!("Alignment of ", stringify!(nvme_uring_cmd))
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr).opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(nvme_uring_cmd),
            "::",
            stringify!(opcode)
        )
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr).flags) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(nvme_uring_cmd),
            "::",
            stringify!(flags)
        )
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr).rsvd1) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(nvme_uring_cmd),
            "::",
            stringify!(rsvd1)
        )
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr).nsid) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(nvme_uring_cmd),
            "::",
            stringify!(nsid)
        )
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr).cdw2) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(nvme_uring_cmd),
            "::",
            stringify!(cdw2)
        )
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr).cdw3) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(nvme_uring_cmd),
            "::",
            stringify!(cdw3)
        )
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr).metadata) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(nvme_uring_cmd),
            "::",
            stringify!(metadata)
        )
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr).addr) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(nvme_uring_cmd),
            "::",
            stringify!(addr)
        )
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr).metadata_len) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(nvme_uring_cmd),
            "::",
            stringify!(metadata_len)
        )
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr).data_len) as usize - ptr as usize },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(nvme_uring_cmd),
            "::",
            stringify!(data_len)
        )
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr).cdw10) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(nvme_uring_cmd),
            "::",
            stringify!(cdw10)
        )
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr).cdw11) as usize - ptr as usize },
        44usize,
        concat!(
            "Offset of field: ",
            stringify!(nvme_uring_cmd),
            "::",
            stringify!(cdw11)
        )
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr).cdw12) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(nvme_uring_cmd),
            "::",
            stringify!(cdw12)
        )
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr).cdw13) as usize - ptr as usize },
        52usize,
        concat!(
            "Offset of field: ",
            stringify!(nvme_uring_cmd),
            "::",
            stringify!(cdw13)
        )
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr).cdw14) as usize - ptr as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(nvme_uring_cmd),
            "::",
            stringify!(cdw14)
        )
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr).cdw15) as usize - ptr as usize },
        60usize,
        concat!(
            "Offset of field: ",
            stringify!(nvme_uring_cmd),
            "::",
            stringify!(cdw15)
        )
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr).timeout_ms) as usize - ptr as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(nvme_uring_cmd),
            "::",
            stringify!(timeout_ms)
        )
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr).rsvd2) as usize - ptr as usize },
        68usize,
        concat!(
            "Offset of field: ",
            stringify!(nvme_uring_cmd),
            "::",
            stringify!(rsvd2)
        )
    );
}
