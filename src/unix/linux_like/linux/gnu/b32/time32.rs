//! 32-bit specific definitions for linux-like values when gnu_time64_abi is not set

use crate::prelude::*;
use crate::{__syscall_slong_t, blkcnt_t, blksize_t, dev_t, time_t};

s! {
    pub struct stat {
        #[cfg(not(any(target_arch = "mips", target_arch = "mips32r6")))]
        pub st_dev: dev_t,
        #[cfg(any(target_arch = "mips", target_arch = "mips32r6"))]
        pub st_dev: c_ulong,

        #[cfg(not(any(target_arch = "mips", target_arch = "mips32r6")))]
        __pad1: c_short,
        #[cfg(any(target_arch = "mips", target_arch = "mips32r6"))]
        st_pad1: [c_long; 3],
        pub st_ino: crate::ino_t,
        pub st_mode: crate::mode_t,
        pub st_nlink: crate::nlink_t,
        pub st_uid: crate::uid_t,
        pub st_gid: crate::gid_t,
        #[cfg(not(any(target_arch = "mips", target_arch = "mips32r6")))]
        pub st_rdev: dev_t,
        #[cfg(any(target_arch = "mips", target_arch = "mips32r6"))]
        pub st_rdev: c_ulong,
        #[cfg(not(any(target_arch = "mips", target_arch = "mips32r6")))]
        __pad2: c_short,
        #[cfg(any(target_arch = "mips", target_arch = "mips32r6"))]
        st_pad2: [c_long; 2],
        pub st_size: crate::off_t,
        #[cfg(any(target_arch = "mips", target_arch = "mips32r6"))]
        st_pad3: c_long,
        #[cfg(not(any(target_arch = "mips", target_arch = "mips32r6")))]
        pub st_blksize: blksize_t,
        #[cfg(not(any(target_arch = "mips", target_arch = "mips32r6")))]
        pub st_blocks: blkcnt_t,
        pub st_atime: time_t,
        pub st_atime_nsec: c_long,
        pub st_mtime: time_t,
        pub st_mtime_nsec: c_long,
        pub st_ctime: time_t,
        pub st_ctime_nsec: c_long,
        #[cfg(not(any(target_arch = "mips", target_arch = "mips32r6")))]
        __unused4: c_long,
        #[cfg(not(any(target_arch = "mips", target_arch = "mips32r6")))]
        __unused5: c_long,
        #[cfg(any(target_arch = "mips", target_arch = "mips32r6"))]
        pub st_blksize: blksize_t,
        #[cfg(any(target_arch = "mips", target_arch = "mips32r6"))]
        pub st_blocks: blkcnt_t,
        #[cfg(any(target_arch = "mips", target_arch = "mips32r6"))]
        st_pad5: [c_long; 14],
    }

    pub struct timex {
        pub modes: c_uint,

        pub offset: __syscall_slong_t,
        pub freq: __syscall_slong_t,
        pub maxerror: __syscall_slong_t,
        pub esterror: __syscall_slong_t,
        pub status: c_int,
        pub constant: __syscall_slong_t,
        pub precision: __syscall_slong_t,
        pub tolerance: __syscall_slong_t,
        pub time: crate::timeval,
        pub tick: __syscall_slong_t,
        pub ppsfreq: __syscall_slong_t,
        pub jitter: __syscall_slong_t,
        pub shift: c_int,
        pub stabil: __syscall_slong_t,
        pub jitcnt: __syscall_slong_t,
        pub calcnt: __syscall_slong_t,
        pub errcnt: __syscall_slong_t,
        pub stbcnt: __syscall_slong_t,
        pub tai: c_int,
        pub __unused1: i32,
        pub __unused2: i32,
        pub __unused3: i32,
        pub __unused4: i32,
        pub __unused5: i32,
        pub __unused6: i32,
        pub __unused7: i32,
        pub __unused8: i32,
        pub __unused9: i32,
        pub __unused10: i32,
        pub __unused11: i32,
    }
}
