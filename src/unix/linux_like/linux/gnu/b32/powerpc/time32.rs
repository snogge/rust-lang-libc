use crate::prelude::*;
use crate::{dev_t, pid_t, time_t};

s! {
    pub struct stat64 {
        pub st_dev: dev_t,
        pub st_ino: crate::ino64_t,
        pub st_mode: crate::mode_t,
        pub st_nlink: crate::nlink_t,
        pub st_uid: crate::uid_t,
        pub st_gid: crate::gid_t,
        pub st_rdev: dev_t,
        __pad2: c_ushort,
        pub st_size: crate::off64_t,
        pub st_blksize: crate::blksize_t,
        pub st_blocks: crate::blkcnt64_t,
        pub st_atime: time_t,
        pub st_atime_nsec: c_long,
        pub st_mtime: time_t,
        pub st_mtime_nsec: c_long,
        pub st_ctime: time_t,
        pub st_ctime_nsec: c_long,
        __glibc_reserved4: c_ulong,
        __glibc_reserved5: c_ulong,
    }

    pub struct shmid_ds {
        pub shm_perm: crate::ipc_perm,
        __glibc_reserved1: c_uint,
        pub shm_atime: time_t,
        __glibc_reserved2: c_uint,
        pub shm_dtime: time_t,
        __glibc_reserved3: c_uint,
        pub shm_ctime: time_t,
        __glibc_reserved4: c_uint,
        pub shm_segsz: size_t,
        pub shm_cpid: pid_t,
        pub shm_lpid: pid_t,
        pub shm_nattch: crate::shmatt_t,
        __glibc_reserved5: c_ulong,
        __glibc_reserved6: c_ulong,
    }
}
